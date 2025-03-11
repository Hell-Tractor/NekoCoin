use std::sync::OnceLock;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tracing::{debug, error, info};

use crate::constants;

pub fn db() -> &'static Pool<Sqlite> {
    static DB: OnceLock<Pool<Sqlite>> = OnceLock::new();
    DB.get_or_init(|| {
        debug!("Connecting to database");
        let Ok(db) = SqlitePoolOptions::new()
            .max_connections(constants::DB_CONNECTIONS)
            .connect_lazy(format!("sqlite:{}", constants::DB_NAME).as_str())
        else {
            error!("Failed to connect to database");
            panic!("Failed to connect to database");
        };
        info!("Connected to database");
        db
    })
}
