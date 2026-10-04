use serde::{Deserialize, Serialize};

// Minecraft Game Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LatestVersion {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionEntry {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    #[serde(default)]
    pub sha1: String,
    #[serde(rename = "complianceLevel", default)]
    pub compliance_level: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionManifest {
    pub latest: LatestVersion,
    pub versions: Vec<VersionEntry>,
}

// Fabric Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricLoaderInfo {
    pub separator: Option<String>,
    pub build: Option<u32>,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FabricLoaderEntry {
    pub loader: FabricLoaderInfo,
}

// Quilt Info
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuiltLoaderInfo {
    pub separator: Option<String>,
    pub build: Option<u32>,
    pub maven: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuiltLoaderEntry {
    pub loader: QuiltLoaderInfo,
}
