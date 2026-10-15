pub mod models;
pub use models::VersionManifest;
use semver::Version;

use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use tauri::Manager;

use crate::meta::models::{FabricLoaderEntry, ForgePromotions, QuiltLoaderEntry};

const MOJANG_MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
const FABRIC_MANIFEST_URL: &str = "https://meta.fabricmc.net/v2/versions/loader";
const QUILT_MANIFEST_URL: &str = "https://meta.quiltmc.org/v3/versions/loader";
const FORGE_PROMOTIONS_URL: &str =
    "https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json";
const NEOFORGE_MAVEN_METADATA_URL: &str =
    "https://maven.neoforged.net/releases/net/neoforged/neoforge/maven-metadata.xml";
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
        .user_agent("LoomLauncher/1.0")
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
        .user_agent("LoomLauncher/1.0")
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

// Forge
async fn get_forge_loader_version(game_version: &str) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .user_agent("LoomLauncerh/1.0")
        .build()
        .map_err(|error| error.to_string())?;

    let promotions: ForgePromotions = client
        .get(FORGE_PROMOTIONS_URL)
        .send()
        .await
        .map_err(|error| format!("Failed to reach Forge promos: {error}"))?
        .json()
        .await
        .map_err(|error| format!("Failed to parse Forge promos: {error}"))?;

    let recommended_key = format!("{game_version}-recommended");
    let latest_key = format!("{game_version}-latest");

    if let Some(ver) = promotions.promos.get(&recommended_key) {
        Ok(ver.clone())
    } else if let Some(ver) = promotions.promos.get(&latest_key) {
        Ok(ver.clone())
    } else {
        Err(format!("No Forge build found for Minecraft {game_version}"))
    }
}

// Neoforge
async fn get_neoforge_loader_version(game_version: &str) -> Result<String, String> {
    let mc_prefix = game_version.strip_prefix("1.").unwrap_or(game_version);
    let expected_prefix = format!("{mc_prefix}.");

    let client = reqwest::Client::builder()
        .user_agent("LoomLauncher/1.0")
        .build()
        .map_err(|error| error.to_string())?;

    let raw_xml = client
        .get(NEOFORGE_MAVEN_METADATA_URL)
        .send()
        .await
        .map_err(|error| format!("Failed to reach Neoforge Maven: {error}"))?
        .text()
        .await
        .map_err(|error| format!("Failed to read Neoforge metadata: {error}"))?;

    let mut matching_versions: Vec<String> = Vec::new();

    for line in raw_xml.lines() {
        let trimmed = line.trim();
        if let (Some(start), Some(end)) = (trimmed.find("<version>"), trimmed.find("</version>")) {
            if start < end {
                let ver = &trimmed[start + "<version>".len()..end];
                if ver.starts_with(&expected_prefix)
                    && !ver.contains("-beta")
                    && !ver.contains("-alpha")
                {
                    matching_versions.push(ver.to_string());
                }
            }
        }
    }

    matching_versions
        .last()
        .cloned()
        .ok_or_else(|| format!("No Neoforge build found for Minecraft {game_version}"))
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
        "forge" => get_forge_loader_version(&game_version).await,
        "neoforge" => get_neoforge_loader_version(&game_version).await,
        _ => Err(format!("Unsupported loader: {}", loader)),
    }
}
