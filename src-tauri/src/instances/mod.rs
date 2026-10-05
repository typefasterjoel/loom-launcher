use crate::utils::fs::sanitize_folder_name;
use std::fs;
use std::path::{Path, PathBuf};
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

    let entries = fs::read_dir(instances_dir).map_err(|error| error.to_string())?;

    let instances = entries
        .flatten()
        .filter_map(|instance| read_instance(&instance.path()))
        .collect();

    Ok(instances)
}

fn read_instance(folder: &Path) -> Option<Instance> {
    let content = fs::read_to_string(folder.join("instance.json")).ok()?;
    serde_json::from_str(&content).ok()
}

fn find_instance_folder(
    instances_dir: &Path,
    instance_id: &str,
) -> Result<Option<PathBuf>, String> {
    let entries = fs::read_dir(instances_dir).map_err(|error| error.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();

        if read_instance(&path).is_some_and(|instance| instance.id == instance_id) {
            return Ok(Some(path));
        }
    }

    Ok(None)
}

#[tauri::command]
pub fn delete_instance(instance_id: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let instances_dir = get_instances_dir(&app_handle)?;
    println!("Delete is invoked.");

    match find_instance_folder(&instances_dir, &instance_id)? {
        Some(instance_folder) => {
            println!("Found the folder: {}", instance_folder.display());
            fs::remove_dir_all(&instance_folder).map_err(|error| error.to_string())?;
            Ok(())
        }
        None => Err(format!("No instance found with id {instance_id}")),
    }
}
