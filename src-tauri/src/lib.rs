mod config;
mod instances;
mod meta;
mod utils;

use config::{get_theme, set_theme};
use instances::{create_instance, get_instances};
use meta::{get_latest_loader_version, get_minecraft_versions};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_theme,
            set_theme,
            create_instance,
            get_instances,
            get_minecraft_versions,
            get_latest_loader_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
