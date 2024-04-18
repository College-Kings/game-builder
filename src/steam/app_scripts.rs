use std::{fs, path::PathBuf};

use crate::{Error, Result, CONTENT_BUILDER_PATH, PREVIEW};

pub fn create_app_scripts(app_ids: &[u32], version: &str) -> Result<()> {
    for app_id in app_ids {
        create_app_script(*app_id, version)?;
    }

    Ok(())
}

pub fn create_app_script(app_id: u32, version: &str) -> Result<()> {
    let output_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("output");
    let script_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("scripts");
    let depot_id = app_id + 1;

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
        output_dir
            .to_str()
            .ok_or_else(|| Error::InvalidPath(output_dir.clone()))?,
        PREVIEW as u8,
        depot_id,
        script_dir
            .join(format!("depot_{}.vdf", depot_id))
            .to_str()
            .ok_or_else(|| Error::InvalidPath(script_dir.clone()))?
    );

    let file_path = script_dir.join(format!("app_{}.vdf", app_id));
    fs::write(file_path, vdf_content)?;

    Ok(())
}
