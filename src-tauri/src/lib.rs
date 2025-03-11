use std::{fs, path::Path};

use sql::db;
use sqlx::migrate::Migrator;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter};

mod constants;
mod money;
mod sql;
mod wallet;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logger();
    init_database();

    tauri::async_runtime::block_on(migrate_database());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![wallet::controller::create_wallet])
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

fn init_database() {
    if !Path::new(constants::DB_NAME).exists() {
        info!("Database file not exists. Creating file `{}`", constants::DB_NAME);
        fs::File::create(constants::DB_NAME).expect("Failed to create database file.");
    }
    info!("Database initialization done.");
}

fn init_logger() {
    let file_appender = rolling::daily("logs", "latest.log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = tracing_subscriber::fmt()
        .with_writer(file_writer.and(std::io::stdout))
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    info!("Logger initialized");
}
