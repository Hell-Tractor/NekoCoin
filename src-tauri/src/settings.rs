use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::{Error, Result};

const SETTINGS_FILE: &str = "settings.yaml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub primary_currency_code: String,
    pub theme: String,
    pub user_name: String,
    pub avatar: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            primary_currency_code: "CNY".to_string(),
            theme: "pinkPad".to_string(),
            user_name: String::new(),
            avatar: "mdi-cat".to_string(),
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
    let path = settings_path(&app)?;
    if !path.exists() {
        return Ok(Settings::default());
    }
    let content = fs::read_to_string(path)
        .map_err(|error| Error::InvalidParameter(format!("failed to read settings: {error}")))?;
    serde_yaml::from_str(&content)
        .map_err(|error| Error::InvalidParameter(format!("failed to parse settings: {error}")))
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: Settings) -> Result<()> {
    let path = settings_path(&app)?;
    let content = serde_yaml::to_string(&settings)
        .map_err(|error| Error::InvalidParameter(format!("failed to serialize settings: {error}")))?;
    let temporary_path = path.with_extension("yaml.tmp");
    fs::write(&temporary_path, content)
        .map_err(|error| Error::InvalidParameter(format!("failed to write settings: {error}")))?;
    fs::rename(temporary_path, path)
        .map_err(|error| Error::InvalidParameter(format!("failed to replace settings: {error}")))?;
    Ok(())
}