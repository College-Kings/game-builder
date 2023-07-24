use std::path::PathBuf;

use serde::{Deserialize, Serialize};

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
        desc: String,
        build_output: PathBuf,
        preview: bool,
        depot_id: i32,
        depot_path: PathBuf,
    ) -> AppBuild {
        AppBuild {
            app_id,
            desc,
            build_output: build_output.into_os_string().into_string().unwrap(),
            content_root: String::new(),
            set_live: String::new(),
            preview: preview as i32,
            local: String::new(),
            depots: Depots {
                depot_id,
                depot_path: depot_path.into_os_string().into_string().unwrap(),
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Depots {
    pub depot_id: i32,
    pub depot_path: String,
}
