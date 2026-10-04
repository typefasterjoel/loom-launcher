use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub id: String,
    pub name: String,
    #[serde(alias = "mc_version")]
    pub mc_version: String,
    #[serde(alias = "mod_loader")]
    pub mod_loader: String,
    #[serde(alias = "loader_version")]
    pub loader_version: String,
    #[serde(alias = "icon_path")]
    pub icon_path: Option<String>,
}
