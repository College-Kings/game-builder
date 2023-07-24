use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DepotBuildConfig {
    #[serde(rename = "DepotID")]
    pub depot_id: i32,
    #[serde(rename = "contentroot")]
    pub content_root: String,
    #[serde(rename = "FileMapping")]
    pub file_mapping: FileMapping,
}

impl DepotBuildConfig {
    pub fn new(
        depot_id: i32,
        content_root: PathBuf,
        file_mapping: FileMapping,
    ) -> DepotBuildConfig {
        DepotBuildConfig {
            depot_id,
            content_root: content_root.into_os_string().into_string().unwrap(),
            file_mapping,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FileMapping {
    #[serde(rename = "LocalPath")]
    pub local_path: String,
    #[serde(rename = "DepotPath")]
    pub depot_path: String,
    #[serde(rename = "recursive")]
    pub recursive: String,
}
