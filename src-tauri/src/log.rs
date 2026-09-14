use std::{
    fs,
    path::{Path, PathBuf},
};

use chrono::{Local, NaiveDate};
use tauri::{AppHandle, Manager};
use tracing::{debug, info, warn};

use crate::{Error, Result};

pub const LOG_PREFIX: &str = "nekocoin.log";
pub const DEFAULT_LOG_LEVEL: &str = "info";

pub struct LogReloadHandle {
    inner: tracing_subscriber::reload::Handle<
        tracing_subscriber::EnvFilter,
        tracing_subscriber::Registry,
    >,
}

pub fn normalize_log_level(level: &str) -> &'static str {
    match level.to_ascii_lowercase().as_str() {
        "error" => "error",
        "warn" => "warn",
        "info" => "info",
        "debug" => "debug",
        "trace" => "trace",
        _ => DEFAULT_LOG_LEVEL,
    }
}

pub fn env_filter_for_level(level: &str) -> tracing_subscriber::EnvFilter {
    tracing_subscriber::EnvFilter::new(format!("nekocoin_lib={}", normalize_log_level(level)))
}

impl LogReloadHandle {
    pub fn new(
        handle: tracing_subscriber::reload::Handle<
            tracing_subscriber::EnvFilter,
            tracing_subscriber::Registry,
        >,
    ) -> Self {
        Self { inner: handle }
    }

    pub fn set_level(&self, level: &str) -> Result<()> {
        let normalized = normalize_log_level(level);
        self.inner
            .reload(env_filter_for_level(normalized))
            .map_err(|error| Error::LoggerError(format!("failed to reload log filter: {error}")))?;
        info!("Log level set to {normalized}");
        Ok(())
    }
}

pub fn log_dir(app: &AppHandle) -> Result<PathBuf> {
    let directory = app
        .path()
        .app_log_dir()
        .map_err(|error| Error::LoggerError(format!("failed to locate log directory: {error}")))?;
    fs::create_dir_all(&directory).map_err(|error| {
        Error::LoggerError(format!("failed to create log directory: {error}"))
    })?;
    Ok(directory)
}

fn parse_log_date(filename: &str) -> Option<NaiveDate> {
    let suffix = filename.strip_prefix(&format!("{LOG_PREFIX}."))?;
    NaiveDate::parse_from_str(suffix, "%Y-%m-%d").ok()
}

fn is_today_log(filename: &str, today: NaiveDate) -> bool {
    parse_log_date(filename)
        .map(|date| date == today)
        .unwrap_or(false)
}

fn delete_log_file(path: &Path, filename: &str) -> Result<()> {
    fs::remove_file(path).map_err(|error| {
        Error::LoggerError(format!("failed to delete log file `{filename}`: {error}"))
    })?;
    debug!("Deleted log file `{filename}`");
    Ok(())
}

pub fn get_log_usage_bytes(directory: &Path) -> Result<u64> {
    if !directory.exists() {
        return Ok(0);
    }

    let mut total = 0;
    for entry in fs::read_dir(directory).map_err(|error| {
        Error::LoggerError(format!("failed to read log directory: {error}"))
    })? {
        let entry = entry.map_err(|error| {
            Error::LoggerError(format!("failed to read log directory entry: {error}"))
        })?;
        if entry.path().is_file() {
            let size = entry.metadata().map_err(|error| {
                Error::LoggerError(format!("failed to read log file metadata: {error}"))
            })?;
            total += size.len();
        }
    }
    Ok(total)
}

pub fn cleanup_expired_logs(app: &AppHandle, retention_days: u32) -> Result<()> {
    if retention_days == 0 {
        debug!("Skipping log cleanup because retention is disabled");
        return Ok(());
    }

    debug!("Cleaning up expired logs (retention_days = {retention_days})");
    let directory = log_dir(app)?;
    if !directory.exists() {
        debug!("Log directory does not exist, skipping cleanup");
        return Ok(());
    }

    let today = Local::now().date_naive();
    let cutoff = today - chrono::Duration::days(retention_days as i64);
    let mut deleted = 0u32;
    let mut failed = 0u32;

    for entry in fs::read_dir(&directory).map_err(|error| {
        Error::LoggerError(format!("failed to read log directory: {error}"))
    })? {
        let entry = entry.map_err(|error| {
            Error::LoggerError(format!("failed to read log directory entry: {error}"))
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = entry.file_name().to_string_lossy().into_owned();
        let should_delete = match parse_log_date(&filename) {
            Some(date) => date < cutoff,
            None => true,
        };

        if should_delete {
            match delete_log_file(&path, &filename) {
                Ok(()) => deleted += 1,
                Err(error) => {
                    failed += 1;
                    warn!("{error}");
                }
            }
        }
    }

    info!(
        "Log cleanup completed: {deleted} file(s) deleted, {failed} failed (cutoff = {cutoff})"
    );
    Ok(())
}

#[tauri::command]
pub fn get_log_usage(app: AppHandle) -> Result<u64> {
    debug!("Getting log usage");
    let usage = get_log_usage_bytes(&log_dir(&app)?)?;
    info!("Log usage: {usage} byte(s)");
    Ok(usage)
}

#[tauri::command]
pub fn clear_logs(app: AppHandle) -> Result<()> {
    debug!("Clearing logs");
    let directory = log_dir(&app)?;
    if !directory.exists() {
        debug!("Log directory does not exist, skipping clear");
        return Ok(());
    }

    let today = Local::now().date_naive();
    let mut deleted = 0u32;
    let mut skipped = 0u32;
    let mut failed = 0u32;

    for entry in fs::read_dir(&directory).map_err(|error| {
        Error::LoggerError(format!("failed to read log directory: {error}"))
    })? {
        let entry = entry.map_err(|error| {
            Error::LoggerError(format!("failed to read log directory entry: {error}"))
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let filename = entry.file_name().to_string_lossy().into_owned();
        if is_today_log(&filename, today) {
            skipped += 1;
            debug!("Skipping today's log file `{filename}`");
            continue;
        }

        match delete_log_file(&path, &filename) {
            Ok(()) => deleted += 1,
            Err(error) => {
                failed += 1;
                warn!("{error}");
            }
        }
    }

    info!(
        "Logs cleared: {deleted} file(s) deleted, {skipped} file(s) skipped, {failed} failed"
    );
    Ok(())
}
