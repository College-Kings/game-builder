use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{Error, Result, CONTENT_BUILDER_PATH, GAME_DIR, GAME_NAME};

pub fn create_depot_scripts(app_ids: &[u32]) -> Result<()> {
    let game_name = GAME_NAME.replace(' ', "");
    let content_root = PathBuf::from(GAME_DIR)
        .parent()
        .ok_or_else(|| Error::InvalidPath(PathBuf::from(GAME_DIR)))?
        .join(format!("{}-dists", game_name))
        .join(format!("{}-market", game_name));
    let script_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("scripts");

    let latest_episode = app_ids.len();

    for (index, app_id) in app_ids.iter().enumerate() {
        let depot_id = if index == 0 { *app_id + 1 } else { *app_id };

        let local_path = if index == 0 {
            String::from("*")
        } else {
            format!("*ep{}.rpa", index + 1)
        };

        let exclusions = if index == 0 {
            (2..=latest_episode)
                .map(|episode| format!("*ep{}.rpa", episode))
                .collect::<Vec<String>>()
        } else {
            vec![]
        };

        create_depot_script(
            script_dir.clone(),
            depot_id,
            &content_root,
            &local_path,
            exclusions,
        )?;
    }

    Ok(())
}

pub fn create_depot_script(
    script_dir: PathBuf,
    depot_id: u32,
    content_root: impl AsRef<Path>,
    local_path: &str,
    exclusions: Vec<String>,
) -> Result<()> {
    let content_root = content_root.as_ref();
    let file_exclusions = exclusions
        .iter()
        .map(|exclusion| format!(r#"    "FileExclusion" "{}""#, exclusion))
        .collect::<Vec<String>>()
        .join("\n");

    // IMPROVEMENT: Create formatter for handling VDF files
    let vdf_content = format!(
        r#""DepotBuildConfig"
{{
    "DepotID" "{}"
    "contentroot" "{}"
    "FileMapping"
    {{
        "LocalPath" "{}"
        "DepotPath" "."
        "recursive" "1"
    }}
{}
}}"#,
        depot_id,
        content_root
            .to_str()
            .ok_or_else(|| Error::InvalidPath(content_root.to_path_buf()))?,
        local_path,
        file_exclusions
    );

    let file_path = script_dir.join(format!("depot_{}.vdf", depot_id));
    fs::write(file_path, vdf_content)?;

    Ok(())
}
