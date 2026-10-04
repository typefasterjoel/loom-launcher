use crate::utils::fs::sanitize_folder_name;
use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use uuid::Uuid;

pub mod models;
pub use models::Instance;

pub fn get_instances_dir(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;

    let instances_dir = app_dir.join("instances");

    if !instances_dir.exists() {
        fs::create_dir_all(&instances_dir).map_err(|error| error.to_string())?;
    }

    Ok(instances_dir)
}

#[tauri::command]
pub fn create_instance(
    app_handle: tauri::AppHandle,
    name: String,
    mc_version: String,
    mod_loader: String,
    loader_version: String,
) -> Result<Instance, String> {
    let instance_dir = get_instances_dir(&app_handle)?;
    let raw_uuid = Uuid::new_v4().to_string();

    let folder_name = format!("{} ({})", sanitize_folder_name(&name), raw_uuid);
    let instance_folder = instance_dir.join(&folder_name);

    // Create folder;
    fs::create_dir_all(instance_folder.join("mods")).map_err(|error| error.to_string())?;
    fs::create_dir_all(instance_folder.join("saves")).map_err(|error| error.to_string())?;
    fs::create_dir_all(instance_folder.join("resourcepacks")).map_err(|error| error.to_string())?;

    let new_instance = Instance {
        id: raw_uuid,
        name,
        mc_version,
        mod_loader,
        loader_version,
        icon_path: None,
    };

    let config_file = instance_folder.join("instance.json");
    let json_data =
        serde_json::to_string_pretty(&new_instance).map_err(|error| error.to_string())?;
    fs::write(config_file, json_data).map_err(|error| error.to_string())?;

    Ok(new_instance)
}

#[tauri::command]
pub fn get_instances(app_handle: tauri::AppHandle) -> Result<Vec<Instance>, String> {
    let instances_dir = get_instances_dir(&app_handle)?;
    let mut instances = Vec::new();

    let entries = fs::read_dir(instances_dir).map_err(|error| error.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() {
            let config_file = path.join("instance.json");

            if config_file.exists() {
                if let Ok(content) = fs::read_to_string(config_file) {
                    if let Ok(instance) = serde_json::from_str::<Instance>(&content) {
                        instances.push(instance)
                    }
                }
            }
        }
    }

    Ok(instances)
}
