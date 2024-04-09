use crate::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct AppBuild {
    #[serde(rename = "appid")]
    pub app_id: i32,
    pub desc: String,
    #[serde(rename = "buildoutput")]
    pub build_output: String,
    #[serde(rename = "contentroot")]
    pub content_root: String,
    #[serde(rename = "setlive")]
    pub set_live: String,
    pub preview: i32,
    pub local: String,
    pub depots: Depots,
}

impl AppBuild {
    pub fn new(
        app_id: i32,
        desc: impl Into<String>,
        build_output: impl Into<PathBuf>,
        preview: bool,
        depot_id: i32,
        depot_path: impl Into<PathBuf>,
    ) -> Result<AppBuild> {
        Ok(AppBuild {
            app_id,
            desc: desc.into(),
            build_output: build_output.into().into_os_string().into_string()?,
            content_root: String::new(),
            set_live: String::new(),
            preview: preview as i32,
            local: String::new(),
            depots: Depots {
                depot_id,
                depot_path: depot_path.into().into_os_string().into_string()?,
            },
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct Depots {
    pub depot_id: i32,
    pub depot_path: String,
}
