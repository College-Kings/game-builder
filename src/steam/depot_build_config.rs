use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    pub fn new<P, S>(
        depot_id: i32,
        content_root: P,
        file_mapping: FileMapping,
        file_exclusions: Vec<S>,
    ) -> Result<DepotBuildConfig>
    where
        P: Into<PathBuf>,
        S: Into<String>,
    {
        Ok(DepotBuildConfig {
            depot_id,
            content_root: content_root.into().into_os_string().into_string()?,
            file_mapping,
            file_exclusions: file_exclusions.into_iter().map(|s| s.into()).collect(),
        })
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
    pub fn new<S: Into<String>>(local_path: S, depot_path: S, recursive: bool) -> FileMapping {
        FileMapping {
            local_path: local_path.into(),
            depot_path: depot_path.into(),
            recursive: (recursive as i32).to_string(),
        }
    }
}
