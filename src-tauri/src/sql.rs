use std::{path::PathBuf, sync::OnceLock};

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::{debug, info};

use crate::Error;
use crate::constants;
use crate::Result;

static DB: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub fn db() -> &'static Pool<Sqlite> {
    DB.get().expect("Database has not been initialized")
}

pub async fn close() -> Result<()> {
    if let Some(pool) = DB.get() {
        pool.close().await;
    }
    Ok(())
}

pub async fn init(path: PathBuf) -> Result<()> {
    debug!("Connecting to database");

    let database_url = format!("sqlite://{}", path.display());

    let db = SqlitePoolOptions::new()
        .max_connections(constants::DB_CONNECTIONS)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("PRAGMA foreign_keys = ON")
                    .execute(conn)
                    .await?;

                Ok(())
            })
        })
        .connect(&database_url)
        .await?;

    DB.set(db)
        .map_err(|_| Error::DatabaseError(format!("Database has already been initialized")))?;

    info!("Connected to database");

    Ok(())
}
