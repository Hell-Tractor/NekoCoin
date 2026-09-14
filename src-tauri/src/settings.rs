use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tracing::{debug, info};

use crate::{constants, sql, Error, Result};

const SETTINGS_FILE: &str = "nekocoin_settings.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub primary_currency_code: String,
    pub theme: String,
    pub user_name: String,
    pub avatar: String,
    pub locale: String,
    pub default_page: String,
    pub date_format: String,
    pub time_format: String,
    pub decimal_places: u8,
    pub thousands_separator: bool,
    pub initialized: bool,
    pub log_retention_days: u32,
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    crate::log::DEFAULT_LOG_LEVEL.to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            primary_currency_code: "CNY".to_string(),
            theme: "pinkPad".to_string(),
            user_name: String::new(),
            avatar: "mdi-cat".to_string(),
            locale: "zh-CN".to_string(),
            default_page: "home".to_string(),
            date_format: "YYYY-MM-DD".to_string(),
            time_format: "24hr".to_string(),
            decimal_places: 2,
            thousands_separator: true,
            initialized: false,
            log_retention_days: 30,
            log_level: default_log_level(),
        }
    }
}

fn settings_path(app: &AppHandle) -> Result<PathBuf> {
    let directory = app.path().app_config_dir()
        .map_err(|error| Error::InvalidParameter(format!("failed to locate settings directory: {error}")))?;
    fs::create_dir_all(&directory)
        .map_err(|error| Error::InvalidParameter(format!("failed to create settings directory: {error}")))?;
    Ok(directory.join(SETTINGS_FILE))
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<Settings> {
    debug!("Getting settings");
    let path = settings_path(&app)?;
    if !path.exists() {
        debug!("Settings file not found, returning defaults");
        return Ok(Settings::default());
    }
    let content = fs::read_to_string(path)
        .map_err(|error| Error::InvalidParameter(format!("failed to read settings: {error}")))?;
    let mut settings: Settings = serde_yaml::from_str(&content)
        .map_err(|error| Error::InvalidParameter(format!("failed to parse settings: {error}")))?;
    if !settings.initialized {
        settings.initialized = true;
    }
    settings.log_level = crate::log::normalize_log_level(&settings.log_level).to_string();
    info!("Settings loaded");
    Ok(settings)
}

#[tauri::command]
pub fn save_settings(app: AppHandle, mut settings: Settings) -> Result<()> {
    settings.log_level = crate::log::normalize_log_level(&settings.log_level).to_string();
    debug!(
        "Saving settings (locale = {}, log_retention_days = {}, log_level = {})",
        settings.locale, settings.log_retention_days, settings.log_level
    );
    let path = settings_path(&app)?;
    let content = serde_yaml::to_string(&settings)
        .map_err(|error| Error::InvalidParameter(format!("failed to serialize settings: {error}")))?;
    let temporary_path = path.with_extension("yaml.tmp");
    fs::write(&temporary_path, content)
        .map_err(|error| Error::InvalidParameter(format!("failed to write settings: {error}")))?;
    fs::rename(temporary_path, path)
        .map_err(|error| Error::InvalidParameter(format!("failed to replace settings: {error}")))?;
    if let Some(handle) = app.try_state::<crate::log::LogReloadHandle>() {
        handle.set_level(&settings.log_level)?;
    }
    info!("Settings saved");
    Ok(())
}

fn database_path(app: &AppHandle) -> Result<PathBuf> {
    let directory = app.path().app_data_dir()
        .map_err(|error| Error::InvalidParameter(format!("failed to locate database directory: {error}")))?;
    Ok(directory.join(constants::DB_NAME))
}

#[tauri::command]
pub async fn reset_app(app: AppHandle) -> Result<()> {
    debug!("Resetting app");
    sql::close().await?;

    let database_path = database_path(&app)?;
    if database_path.exists() {
        fs::remove_file(&database_path)
            .map_err(|error| Error::InvalidParameter(format!("failed to delete database: {error}")))?;
    }

    let settings_file = settings_path(&app)?;
    if settings_file.exists() {
        fs::remove_file(&settings_file)
            .map_err(|error| Error::InvalidParameter(format!("failed to delete settings: {error}")))?;
    }

    info!("App reset completed, restarting");
    app.restart()
}