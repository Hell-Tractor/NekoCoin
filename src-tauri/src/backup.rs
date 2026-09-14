use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use csv::StringRecord;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tauri_plugin_fs::{FsExt, OpenOptions};
use tracing::{debug, info, warn};

use crate::constants;
use crate::sql::{self, db};
use crate::{Error, Result};

const CSV_FORMAT: &str = "NekoCoin-CSV-1";

fn database_path(app: &AppHandle) -> Result<PathBuf> {
    let directory = app.path().app_data_dir()
        .map_err(|error| Error::InvalidParameter(format!("failed to locate database directory: {error}")))?;
    Ok(directory.join(constants::DB_NAME))
}

fn optional_u32(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        trimmed.parse().ok()
    }
}

fn required_u32(value: &str, field: &str) -> Result<u32> {
    value.trim().parse().map_err(|_| Error::InvalidParameter(format!("invalid {field}: {value}")))
}

fn required_i32(value: &str, field: &str) -> Result<i32> {
    value.trim().parse().map_err(|_| Error::InvalidParameter(format!("invalid {field}: {value}")))
}

fn required_u8(value: &str, field: &str) -> Result<u8> {
    value.trim().parse().map_err(|_| Error::InvalidParameter(format!("invalid {field}: {value}")))
}

fn field<'a>(row: &'a StringRecord, index: usize, name: &str) -> Result<&'a str> {
    row.get(index).ok_or_else(|| Error::InvalidParameter(format!("missing column {name}")))
}

fn sqlite_sidecar_paths(path: &Path) -> [PathBuf; 2] {
    [
        PathBuf::from(format!("{}-wal", path.display())),
        PathBuf::from(format!("{}-shm", path.display())),
    ]
}

fn remove_sqlite_sidecars(path: &Path) {
    for sidecar in sqlite_sidecar_paths(path) {
        let _ = fs::remove_file(sidecar);
    }
}

fn cleanup_sqlite_file(path: &Path) {
    let _ = fs::remove_file(path);
    remove_sqlite_sidecars(path);
}

fn copy_sqlite_sidecars(source: &Path, destination: &Path) {
    for suffix in ["-wal", "-shm"] {
        let sidecar = PathBuf::from(format!("{}{suffix}", source.display()));
        if sidecar.exists() {
            let target = PathBuf::from(format!("{}{suffix}", destination.display()));
            let _ = fs::copy(&sidecar, &target);
        }
    }
}

async fn checkpoint_database() -> Result<()> {
    sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
        .execute(db())
        .await?;
    Ok(())
}

async fn validate_import_database(work_path: &Path) -> Result<()> {
    let database_url = format!("sqlite://{}", work_path.display());
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .map_err(|error| Error::InvalidParameter(format!("not a valid SQLite database: {error}")))?;

    let fingerprint: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('_sqlx_migrations', 'wallets')",
    )
    .fetch_one(&pool)
    .await
    .map_err(|error| Error::InvalidParameter(format!("not a NekoCoin database: {error}")))?;

    if fingerprint == 0 {
        pool.close().await;
        return Err(Error::InvalidParameter("not a NekoCoin database".to_string()));
    }

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|error| Error::InvalidParameter(format!("database migration failed: {error}")))?;

    for query in [
        "SELECT currency_code, color, icon FROM wallets LIMIT 0",
        "SELECT parent_id FROM tags LIMIT 0",
        "SELECT split_id FROM transactions LIMIT 0",
        "SELECT tag_id FROM activities LIMIT 0",
    ] {
        sqlx::query(query)
            .execute(&pool)
            .await
            .map_err(|error| Error::InvalidParameter(format!("database schema is invalid: {error}")))?;
    }

    sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
        .execute(&pool)
        .await?;
    pool.close().await;
    Ok(())
}

fn open_picked_read(app: &AppHandle, path: FilePath) -> Result<fs::File> {
    let mut options = OpenOptions::new();
    options.read(true);
    Ok(app.fs().open(path, options)?)
}

fn open_picked_write(app: &AppHandle, path: FilePath) -> Result<fs::File> {
    let mut options = OpenOptions::new();
    options.write(true).create(true).truncate(true);
    Ok(app.fs().open(path, options)?)
}

fn copy_to_picked(app: &AppHandle, source: &Path, destination: FilePath) -> Result<()> {
    let mut input = fs::File::open(source)?;
    let mut output = open_picked_write(app, destination)?;
    std::io::copy(&mut input, &mut output)?;
    output.flush()?;
    Ok(())
}

fn copy_from_picked(app: &AppHandle, source: FilePath, destination: &Path) -> Result<()> {
    let mut input = open_picked_read(app, source)?;
    let mut output = fs::File::create(destination)?;
    std::io::copy(&mut input, &mut output)?;
    output.flush()?;
    Ok(())
}

async fn pick_save_path(window: tauri::WebviewWindow, file_name: &'static str, filter_name: &'static str, extensions: &'static [&str]) -> Result<Option<FilePath>> {
    let picked = tauri::async_runtime::spawn_blocking(move || {
        window
            .dialog()
            .file()
            .set_file_name(file_name)
            .add_filter(filter_name, extensions)
            .blocking_save_file()
    })
    .await
    .map_err(|error| Error::InvalidParameter(format!("failed to open save dialog: {error}")))?;
    Ok(picked)
}

async fn pick_open_path(window: tauri::WebviewWindow, filter_name: &'static str, extensions: &'static [&str]) -> Result<Option<FilePath>> {
    let picked = tauri::async_runtime::spawn_blocking(move || {
        window
            .dialog()
            .file()
            .add_filter(filter_name, extensions)
            .blocking_pick_file()
    })
    .await
    .map_err(|error| Error::InvalidParameter(format!("failed to open file dialog: {error}")))?;
    Ok(picked)
}

#[tauri::command]
pub async fn export_database(app: AppHandle, window: tauri::WebviewWindow) -> Result<bool> {
    let Some(path) = pick_save_path(window, "nekocoin.db", "SQLite", &["db", "sqlite"]).await? else {
        debug!("Database export cancelled");
        return Ok(false);
    };
    debug!("Exporting database to {path}");
    checkpoint_database().await?;
    let source = database_path(&app)?;
    copy_to_picked(&app, &source, path.clone())?;
    info!("Database exported to {path}");
    Ok(true)
}

#[tauri::command]
pub async fn import_database(app: AppHandle, window: tauri::WebviewWindow) -> Result<bool> {
    let Some(source) = pick_open_path(window, "SQLite", &["db", "sqlite"]).await? else {
        debug!("Database import cancelled");
        return Ok(false);
    };
    debug!("Importing database from {source}");

    let destination = database_path(&app)?;
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    let work_path = destination.with_file_name("nekocoin.import.tmp.db");
    let backup_path = destination.with_file_name("nekocoin.db.bak");
    cleanup_sqlite_file(&work_path);
    let sidecar_source = source.clone();
    if let Err(error) = copy_from_picked(&app, source.clone(), &work_path) {
        cleanup_sqlite_file(&work_path);
        return Err(error);
    }
    if let Some(path) = sidecar_source.as_path() {
        copy_sqlite_sidecars(path, &work_path);
    }

    if let Err(error) = validate_import_database(&work_path).await {
        cleanup_sqlite_file(&work_path);
        return Err(error);
    }

    if let Err(error) = checkpoint_database().await {
        cleanup_sqlite_file(&work_path);
        return Err(error);
    }
    if destination.exists() {
        if let Err(error) = fs::copy(&destination, &backup_path) {
            cleanup_sqlite_file(&work_path);
            return Err(error.into());
        }
    }

    sql::close().await?;
    if let Err(error) = fs::copy(&work_path, &destination) {
        warn!("Failed to replace live database, restoring previous file: {error}");
        if backup_path.exists() {
            let _ = fs::copy(&backup_path, &destination);
        }
        cleanup_sqlite_file(&work_path);
        return Err(Error::InvalidParameter(format!(
            "failed to replace database ({error}); previous database restored, please restart the app"
        )));
    }

    remove_sqlite_sidecars(&destination);
    cleanup_sqlite_file(&work_path);
    let _ = fs::remove_file(&backup_path);
    info!("Database imported from {source}, restarting");
    app.restart()
}

#[tauri::command]
pub async fn export_csv(app: AppHandle, window: tauri::WebviewWindow) -> Result<bool> {
    let Some(path) = pick_save_path(window, "nekocoin.csv", "CSV", &["csv"]).await? else {
        debug!("CSV export cancelled");
        return Ok(false);
    };
    debug!("Exporting CSV to {path}");
    let mut writer = csv::WriterBuilder::new().flexible(true).from_writer(open_picked_write(&app, path.clone())?);
    writer.write_record([CSV_FORMAT])?;

    writer.write_record(["[wallets]"])?;
    writer.write_record(["id", "name", "remark", "balance", "currency_code", "color", "icon"])?;
    let wallets = sqlx::query("SELECT id, name, remark, balance, currency_code, color, icon FROM wallets ORDER BY id")
        .fetch_all(db())
        .await?;
    for row in wallets {
        writer.write_record([
            row.get::<u32, _>("id").to_string(),
            row.try_get::<String, _>("name").unwrap_or_default(),
            row.try_get::<String, _>("remark").unwrap_or_default(),
            row.get::<i32, _>("balance").to_string(),
            row.try_get::<String, _>("currency_code").unwrap_or_default(),
            row.try_get::<String, _>("color").unwrap_or_default(),
            row.try_get::<String, _>("icon").unwrap_or_default(),
        ])?;
    }

    writer.write_record(["[tags]"])?;
    writer.write_record(["id", "name", "remark", "color", "icon", "kind", "parent_id"])?;
    let tags = sqlx::query("SELECT id, name, remark, color, icon, kind, parent_id FROM tags ORDER BY id")
        .fetch_all(db())
        .await?;
    for row in tags {
        let parent_id = row.try_get::<Option<u32>, _>("parent_id").ok().flatten().map(|id| id.to_string()).unwrap_or_default();
        writer.write_record([
            row.get::<u32, _>("id").to_string(),
            row.try_get::<String, _>("name").unwrap_or_default(),
            row.try_get::<String, _>("remark").unwrap_or_default(),
            row.try_get::<String, _>("color").unwrap_or_default(),
            row.try_get::<String, _>("icon").unwrap_or_default(),
            row.get::<i32, _>("kind").to_string(),
            parent_id,
        ])?;
    }

    writer.write_record(["[activities]"])?;
    writer.write_record(["id", "name", "remark", "color", "icon", "open", "tag_id"])?;
    let activities = sqlx::query("SELECT id, name, remark, color, icon, open, tag_id FROM activities ORDER BY id")
        .fetch_all(db())
        .await?;
    for row in activities {
        writer.write_record([
            row.get::<u32, _>("id").to_string(),
            row.try_get::<String, _>("name").unwrap_or_default(),
            row.try_get::<String, _>("remark").unwrap_or_default(),
            row.try_get::<String, _>("color").unwrap_or_default(),
            row.try_get::<String, _>("icon").unwrap_or_default(),
            row.get::<i32, _>("open").to_string(),
            row.get::<u32, _>("tag_id").to_string(),
        ])?;
    }

    writer.write_record(["[transaction_splits]"])?;
    writer.write_record(["id", "count", "expense", "receive_wallet_id"])?;
    let splits = sqlx::query("SELECT id, count, expense, receive_wallet_id FROM transaction_splits ORDER BY id")
        .fetch_all(db())
        .await?;
    for row in splits {
        writer.write_record([
            row.get::<u32, _>("id").to_string(),
            row.get::<u32, _>("count").to_string(),
            row.get::<i32, _>("expense").to_string(),
            row.get::<u32, _>("receive_wallet_id").to_string(),
        ])?;
    }

    writer.write_record(["[transactions]"])?;
    writer.write_record(["id", "remark", "wallet_id", "to_wallet_id", "tag_id", "activity_id", "amount", "time", "split_id"])?;
    let transactions = sqlx::query("SELECT id, remark, wallet_id, to_wallet_id, tag_id, activity_id, amount, time, split_id FROM transactions ORDER BY id")
        .fetch_all(db())
        .await?;
    for row in transactions {
        let to_wallet_id = row.try_get::<Option<u32>, _>("to_wallet_id").ok().flatten().map(|id| id.to_string()).unwrap_or_default();
        let activity_id = row.try_get::<Option<u32>, _>("activity_id").ok().flatten().map(|id| id.to_string()).unwrap_or_default();
        let split_id = row.try_get::<Option<u32>, _>("split_id").ok().flatten().map(|id| id.to_string()).unwrap_or_default();
        writer.write_record([
            row.get::<u32, _>("id").to_string(),
            row.try_get::<String, _>("remark").unwrap_or_default(),
            row.get::<u32, _>("wallet_id").to_string(),
            to_wallet_id,
            row.get::<u32, _>("tag_id").to_string(),
            activity_id,
            row.get::<i32, _>("amount").to_string(),
            row.get::<String, _>("time"),
            split_id,
        ])?;
    }

    writer.flush()?;
    info!("CSV exported to {path}");
    Ok(true)
}

#[tauri::command]
pub async fn import_csv(app: AppHandle, window: tauri::WebviewWindow) -> Result<bool> {
    let Some(path) = pick_open_path(window, "CSV", &["csv"]).await? else {
        debug!("CSV import cancelled");
        return Ok(false);
    };
    debug!("Importing CSV from {path}");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(open_picked_read(&app, path.clone())?);
    let mut records = reader.records();
    let first = records.next().ok_or_else(|| Error::InvalidParameter("CSV file is empty".to_string()))??;
    if first.get(0) != Some(CSV_FORMAT) {
        return Err(Error::InvalidParameter("not a NekoCoin CSV export".to_string()));
    }

    let mut section = String::new();
    let mut wallets = Vec::new();
    let mut tags = Vec::new();
    let mut activities = Vec::new();
    let mut splits = Vec::new();
    let mut transactions = Vec::new();

    for record in records {
        let record = record?;
        let first_cell = record.get(0).unwrap_or("").trim();
        if first_cell.is_empty() {
            continue;
        }
        if first_cell.starts_with('[') && first_cell.ends_with(']') {
            section = first_cell.to_string();
            continue;
        }
        if first_cell == "id" {
            continue;
        }
        match section.as_str() {
            "[wallets]" => wallets.push((
                required_u32(field(&record, 0, "id")?, "wallet id")?,
                field(&record, 1, "name")?.to_string(),
                field(&record, 2, "remark")?.to_string(),
                required_i32(field(&record, 3, "balance")?, "balance")?,
                field(&record, 4, "currency_code")?.to_string(),
                field(&record, 5, "color")?.to_string(),
                field(&record, 6, "icon")?.to_string(),
            )),
            "[tags]" => tags.push((
                required_u32(field(&record, 0, "id")?, "tag id")?,
                field(&record, 1, "name")?.to_string(),
                field(&record, 2, "remark")?.to_string(),
                field(&record, 3, "color")?.to_string(),
                field(&record, 4, "icon")?.to_string(),
                required_u8(field(&record, 5, "kind")?, "kind")?,
                optional_u32(field(&record, 6, "parent_id")?),
            )),
            "[activities]" => activities.push((
                required_u32(field(&record, 0, "id")?, "activity id")?,
                field(&record, 1, "name")?.to_string(),
                field(&record, 2, "remark")?.to_string(),
                field(&record, 3, "color")?.to_string(),
                field(&record, 4, "icon")?.to_string(),
                required_i32(field(&record, 5, "open")?, "open")?,
                required_u32(field(&record, 6, "tag_id")?, "tag_id")?,
            )),
            "[transaction_splits]" => splits.push((
                required_u32(field(&record, 0, "id")?, "split id")?,
                required_u32(field(&record, 1, "count")?, "count")?,
                required_i32(field(&record, 2, "expense")?, "expense")?,
                required_u32(field(&record, 3, "receive_wallet_id")?, "receive_wallet_id")?,
            )),
            "[transactions]" => transactions.push((
                required_u32(field(&record, 0, "id")?, "transaction id")?,
                field(&record, 1, "remark")?.to_string(),
                required_u32(field(&record, 2, "wallet_id")?, "wallet_id")?,
                optional_u32(field(&record, 3, "to_wallet_id")?),
                required_u32(field(&record, 4, "tag_id")?, "tag_id")?,
                optional_u32(field(&record, 5, "activity_id")?),
                required_i32(field(&record, 6, "amount")?, "amount")?,
                field(&record, 7, "time")?.to_string(),
                optional_u32(field(&record, 8, "split_id")?),
            )),
            _ => {}
        }
    }

    let mut tx = db().begin().await?;
    sqlx::query("PRAGMA foreign_keys = OFF").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM transactions").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM transaction_splits").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM activities").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM tags").execute(&mut *tx).await?;
    sqlx::query("DELETE FROM wallets").execute(&mut *tx).await?;

    for (id, name, remark, balance, currency_code, color, icon) in wallets {
        sqlx::query("INSERT INTO wallets (id, name, remark, balance, currency_code, color, icon) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(id).bind(name).bind(remark).bind(balance).bind(currency_code).bind(color).bind(icon)
            .execute(&mut *tx)
            .await?;
    }
    for (id, name, remark, color, icon, kind, parent_id) in tags {
        sqlx::query("INSERT INTO tags (id, name, remark, color, icon, kind, parent_id) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(id).bind(name).bind(remark).bind(color).bind(icon).bind(kind).bind(parent_id)
            .execute(&mut *tx)
            .await?;
    }
    for (id, name, remark, color, icon, open, tag_id) in activities {
        sqlx::query("INSERT INTO activities (id, name, remark, color, icon, open, tag_id) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(id).bind(name).bind(remark).bind(color).bind(icon).bind(open).bind(tag_id)
            .execute(&mut *tx)
            .await?;
    }
    for (id, count, expense, receive_wallet_id) in splits {
        sqlx::query("INSERT INTO transaction_splits (id, count, expense, receive_wallet_id) VALUES ($1, $2, $3, $4)")
            .bind(id).bind(count).bind(expense).bind(receive_wallet_id)
            .execute(&mut *tx)
            .await?;
    }
    for (id, remark, wallet_id, to_wallet_id, tag_id, activity_id, amount, time, split_id) in transactions {
        sqlx::query("INSERT INTO transactions (id, remark, wallet_id, to_wallet_id, tag_id, activity_id, amount, time, split_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(id).bind(remark).bind(wallet_id).bind(to_wallet_id).bind(tag_id).bind(activity_id).bind(amount).bind(time).bind(split_id)
            .execute(&mut *tx)
            .await?;
    }
    sqlx::query("DELETE FROM sqlite_sequence").execute(&mut *tx).await?;
    for table in ["wallets", "tags", "activities", "transaction_splits", "transactions"] {
        sqlx::query(&format!(
            "INSERT INTO sqlite_sequence (name, seq) SELECT '{table}', IFNULL(MAX(id), 0) FROM {table}"
        ))
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query("PRAGMA foreign_keys = ON").execute(&mut *tx).await?;
    tx.commit().await?;
    info!("CSV imported from {path}, restarting");
    app.restart()
}
