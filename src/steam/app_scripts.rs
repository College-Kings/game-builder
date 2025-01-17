use std::{fs, path::PathBuf};

use crate::{Result, CONTENT_BUILDER_PATH, PREVIEW};

pub fn create_app_scripts(app_ids: &[u32], version: &str) {
    for (index, app_id) in app_ids.iter().enumerate() {
        let depot_id = if index == 0 { *app_id + 1 } else { *app_id };

        create_app_script(*app_id, depot_id, version).unwrap();
    }
}

pub fn create_app_script(app_id: u32, depot_id: u32, version: &str) -> Result<()> {
    let output_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("output");
    let script_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("scripts");

    // IMPROVEMENT: Create formatter for handling VDF files
    let vdf_content = format!(
        r#""appbuild"
{{
    "appid" "{}"
    "desc" "{}"
    "buildoutput" "{}"
    "contentroot" ""
    "setlive" ""
    "preview" "{}"
    "local" ""
    "depots"
    {{
        "{}" "{}"
    }}
}}"#,
        app_id,
        version,
        output_dir.to_str().unwrap(),
        PREVIEW as u8,
        depot_id,
        script_dir
            .join(format!("depot_{}.vdf", depot_id))
            .to_str()
            .unwrap()
    );

    let file_path = script_dir.join(format!("app_{}.vdf", app_id));
    fs::write(file_path, vdf_content).unwrap();

    Ok(())
}
