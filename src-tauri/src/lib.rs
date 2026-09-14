use std::fs;

use sql::db;
use sqlx::migrate::Migrator;
use tag::TagKind;
use tracing::{info, warn};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, layer::SubscriberExt, reload, util::SubscriberInitExt, fmt::writer::MakeWriterExt};
use tauri::Manager;

mod constants;
mod money;
mod sql;
mod wallet;
mod tag;
mod transaction;
mod summary;
mod settings;
mod log;
mod activity;
mod backup;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    ChronoError(#[from] chrono::ParseError),
    #[error("Invalid tag type `{given:?}`. Allow: {allow:?}")]
    InvalidTagType {
        given: TagKind,
        allow: Vec<TagKind>
    },
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Logger error: {0}")]
    LoggerError(String),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
}
type Result<T> = std::result::Result<T, Error>;

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        warn!("error occurred: {}", *self);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            init_logger(app.handle())?;
            let retention_days = settings::get_settings(app.handle().clone())?.log_retention_days;
            log::cleanup_expired_logs(app.handle(), retention_days)?;
            tauri::async_runtime::block_on(async {
                init_database(app.handle()).await.expect("Failed to initialize database");
                migrate_database().await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            wallet::controller::create_wallet,
            wallet::controller::update_wallet,
            wallet::controller::get_wallet_by_id,
            wallet::controller::retrieve_wallets,
            wallet::controller::delete_wallet,
            wallet::controller::get_sum_balance,

            tag::controller::create_tag,
            tag::controller::update_tag,
            tag::controller::get_tag_by_id,
            tag::controller::has_child_tag,
            tag::controller::retrieve_tags,
            tag::controller::delete_tag,

            activity::controller::create_activity,
            activity::controller::update_activity,
            activity::controller::get_activity_by_id,
            activity::controller::retrieve_activities,
            activity::controller::get_activity_balances,
            activity::controller::delete_activity,
            activity::controller::set_activity_open,
            activity::controller::get_expense_summary_by_activity,
            activity::controller::get_expense_summary_by_activity_tag,

            transaction::controller::create_transaction,
            transaction::controller::retrieve_transactions,
            transaction::controller::retrieve_transactions_in_wallet,
            transaction::controller::retrieve_transactions_with_tag,
            transaction::controller::retrieve_transactions_in_activity,
            transaction::controller::delete_transaction,
            transaction::controller::get_sum_balance_with_type,
            transaction::controller::get_sum_balance_in_wallet,
            transaction::controller::get_sum_balance_in_activity,
            transaction::controller::update_transaction,
            transaction::controller::get_summary_by_tag_in_wallet,
            transaction::controller::get_summary_by_tag_in_activity,
            transaction::controller::get_expense_summary_by_tag,
            transaction::controller::get_summary_by_tag_with_tag,

            summary::controller::get_summary,

            settings::get_settings,
            settings::save_settings,
            settings::reset_app,

            log::get_log_usage,
            log::clear_logs,

            backup::export_database,
            backup::import_database,
            backup::export_csv,
            backup::import_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn migrate_database() {
    info!("Preparing to migrate database...");
    static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
    MIGRATOR
        .run(db())
        .await
        .expect("Failed to migrate database");
    info!("Database migration completed");
}

async fn init_database(app: &tauri::AppHandle) -> Result<()> {
    let directory = app.path().app_data_dir()
        .map_err(|error| Error::DatabaseError(format!("failed to locate database directory: {error}")))?;
    fs::create_dir_all(&directory)
        .map_err(|error| Error::DatabaseError(format!("failed to create database directory: {error}")))?;
    let database_path = directory.join(constants::DB_NAME);
    if !database_path.exists() {
        info!("Creating database file `{}`", database_path.display());
        fs::File::create(&database_path)
            .map_err(|error| Error::DatabaseError(format!("failed to create database: {error}")))?;
    }
    sql::init(database_path).await?;
    info!("Database initialization done.");
    Ok(())
}

fn init_logger(app: &tauri::AppHandle) -> Result<()> {
    let directory = log::log_dir(app)?;
    let log_level = settings::get_settings(app.clone())
        .map(|loaded| loaded.log_level)
        .unwrap_or_else(|_| log::DEFAULT_LOG_LEVEL.to_string());

    let file_appender = rolling::daily(directory, log::LOG_PREFIX);
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    std::mem::forget(guard);

    let (filter_layer, reload_handle) = reload::Layer::new(log::env_filter_for_level(&log_level));
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt::layer().with_writer(file_writer.and(std::io::stdout)))
        .init();
    app.manage(log::LogReloadHandle::new(reload_handle));

    info!("Logger initialized (level = {})", crate::log::normalize_log_level(&log_level));
    Ok(())
}