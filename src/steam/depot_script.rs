use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{CONTENT_BUILDER_PATH, GAME_DIR, GAME_NAME};

pub fn create_depot_scripts(app_ids: &[u32]) {
    let game_name = GAME_NAME.replace(' ', "");
    let content_root = PathBuf::from(GAME_DIR)
        .parent()
        .unwrap()
        .join(format!("{}-dists", game_name))
        .join(format!("{}-market", game_name));
    let script_dir = PathBuf::from(CONTENT_BUILDER_PATH).join("scripts");

    let latest_episode = app_ids.len();

    for (index, app_id) in app_ids.iter().enumerate() {
        let depot_id = if index == 0 { *app_id + 1 } else { *app_id };

        let local_path = if index == 0 {
            String::from("*")
        } else {
            format!("*ep{}*", index + 1)
        };

        let exclusions: Vec<String> = if index == 0 {
            (2..=latest_episode)
                .map(|episode| format!("*ep{}*", episode))
                .collect()
        } else {
            vec![]
        };

        create_depot_script(
            &script_dir,
            depot_id,
            &content_root,
            &local_path,
            exclusions,
        );
    }
}

pub fn create_depot_script(
    script_dir: &Path,
    depot_id: u32,
    content_root: impl AsRef<Path>,
    local_path: &str,
    exclusions: Vec<String>,
) {
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
        content_root.to_str().unwrap(),
        local_path,
        file_exclusions
    );

    let file_path = script_dir.join(format!("depot_{}.vdf", depot_id));
    fs::write(file_path, vdf_content).unwrap();
}
