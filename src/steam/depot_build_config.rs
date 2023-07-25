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
    pub file_exclusions: Vec<String>,
}

impl DepotBuildConfig {
    pub fn new(
        depot_id: i32,
        content_root: PathBuf,
        file_mapping: FileMapping,
        file_exclusions: Vec<String>,
    ) -> DepotBuildConfig {
        DepotBuildConfig {
            depot_id,
            content_root: content_root.into_os_string().into_string().unwrap(),
            file_mapping,
            file_exclusions,
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

impl FileMapping {
    pub fn new(local_path: String, depot_path: String, recursive: bool) -> FileMapping {
        FileMapping {
            local_path,
            depot_path,
            recursive: (recursive as i32).to_string(),
        }
    }
}
