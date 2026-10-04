use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConfig {
    theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
        }
    }
}

pub fn get_config_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let config_dir = app_handle
        .path()
        .app_config_dir()
        .map_err(|error| error.to_string())?;

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|error| error.to_string())?;
    }

    Ok(config_dir.join("config.json"))
}

pub fn load_config(app_handle: &tauri::AppHandle) -> Result<AppConfig, String> {
    let config_path = get_config_path(app_handle)?;

    Ok(fs::read_to_string(&config_path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default())
}

#[tauri::command]
pub fn get_theme(app_handle: tauri::AppHandle) -> String {
    let config_path = match get_config_path(&app_handle) {
        Ok(p) => p,
        Err(_) => return "dark".into(),
    };

    if let Ok(data) = fs::read_to_string(config_path) {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&data) {
            return config.theme;
        }
    }

    "dark".into()
}

#[tauri::command]
pub fn set_theme(app_handle: tauri::AppHandle, theme: String) -> Result<(), String> {
    let config_path = get_config_path(&app_handle)?;
    let mut config = load_config(&app_handle)?;
    config.theme = theme;
    let json = serde_json::to_string_pretty(&config).map_err(|error| error.to_string())?;
    fs::write(config_path, json).map_err(|error| error.to_string())?;

    Ok(())
}
