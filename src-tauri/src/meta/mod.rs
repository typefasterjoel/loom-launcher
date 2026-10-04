pub mod models;
pub use models::VersionManifest;
use semver::Version;

use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use tauri::Manager;

use crate::meta::models::{FabricLoaderEntry, QuiltLoaderEntry};

const MOJANG_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const FABRIC_MANIFEST_URL: &str = "https://meta.fabricmc.net/v2/versions/loader";
const QUILT_MANIFEST_URL: &str = "https://meta.quiltmc.org/v3/versions/loader";
const CACHE_TTL_SECIONS: u64 = 60 * 60 * 24;

fn get_cache_path(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    let cache_dir = app_dir.join("cache");

    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).map_err(|error| error.to_string())?;
    }

    Ok(cache_dir.join("version_manifest.json"))
}

fn is_cache_fresh(cache_path: &PathBuf) -> bool {
    if let Ok(metadata) = fs::metadata(cache_path) {
        if let Ok(modified) = metadata.modified() {
            if let Ok(elapsed) = SystemTime::now().duration_since(modified) {
                return elapsed.as_secs() < CACHE_TTL_SECIONS;
            }
        }
    }

    false
}

#[tauri::command]
pub async fn get_minecraft_versions(
    app_handle: tauri::AppHandle,
    force_refresh: Option<bool>,
) -> Result<VersionManifest, String> {
    let cache_path = get_cache_path(&app_handle)?;
    let force = force_refresh.unwrap_or(false);

    if !force && cache_path.exists() && is_cache_fresh(&cache_path) {
        if let Ok(content) = fs::read_to_string(&cache_path) {
            if let Ok(manifest) = serde_json::from_str::<VersionManifest>(&content) {
                return Ok(manifest);
            }
        }
    }

    let response = reqwest::get(MOJANG_MANIFEST_URL)
        .await
        .map_err(|error| error.to_string())?;
    let manifest = response
        .json::<VersionManifest>()
        .await
        .map_err(|error| error.to_string())?;

    if let Ok(json_data) = serde_json::to_string_pretty(&manifest) {
        let _ = fs::write(cache_path, json_data);
    }

    Ok(manifest)
}

async fn get_fabric_loader_version(game_version: &str) -> Result<String, String> {
    let url = format!("{}/{}", FABRIC_MANIFEST_URL, game_version);

    let client = reqwest::Client::builder()
        .user_agent("CustomLauncher/1.0")
        .build()
        .map_err(|error| error.to_string())?;

    let entries: Vec<FabricLoaderEntry> = client
        .get(&url)
        .send()
        .await
        .map_err(|error| format!("Failed to fetch fabric loader entries: {}", error))?
        .json()
        .await
        .map_err(|error| format!("Failed to parse Fabric manifest: {}", error))?;

    let chosen = entries
        .iter()
        .find(|entry| entry.loader.stable)
        .or_else(|| entries.first())
        .ok_or_else(|| format!("No Fabric Loader Found for MC Version: {}", game_version))?;

    Ok(chosen.loader.version.clone())
}

async fn get_quilt_loader_version(game_version: &str) -> Result<String, String> {
    let url = format!("{}/{}", QUILT_MANIFEST_URL, game_version);

    let client = reqwest::Client::builder()
        .user_agent("CustomLauncher/1.0")
        .build()
        .map_err(|error| error.to_string())?;

    let entries: Vec<QuiltLoaderEntry> = client
        .get(&url)
        .send()
        .await
        .map_err(|error| format!("Failed to fetch quilt loader entries: {}", error))?
        .json()
        .await
        .map_err(|error| format!("Failed to parse Quilt manifest: {}", error))?;

    let parsed: Vec<Version> = entries
        .iter()
        .filter_map(|entry| Version::parse(&entry.loader.version).ok())
        .collect();

    let chosen = parsed
        .iter()
        .filter(|version| version.pre.is_empty())
        .max()
        .or_else(|| parsed.iter().max())
        .ok_or_else(|| format!("No Quilt loader found for MC {}", game_version))?;

    Ok(chosen.to_string())
}

#[tauri::command]
pub async fn get_latest_loader_version(
    game_version: String,
    loader: String,
) -> Result<String, String> {
    match loader.to_lowercase().as_str() {
        "vanilla" => Ok("none".to_string()),
        "fabric" => get_fabric_loader_version(&game_version).await,
        "quilt" => get_quilt_loader_version(&game_version).await,
        //Placeholders
        "forge" => Ok("latest".to_string()),
        "neoforge" => Ok("latest".to_string()),
        _ => Err(format!("Unsupported loader: {}", loader)),
    }
}
