#![allow(dead_code)]
use std::{collections::HashMap, fs::File, io::Write, path::PathBuf, process::Command};

use crate::{build_game, steam::app_info::AppDLC};

use super::{
    app_build::AppBuild,
    app_info::AppInfo,
    depot_build_config::{DepotBuildConfig, FileMapping},
};

const CONTENT_BUILDER_PATH: &str = r#"D:\Steam Build\sdk\tools\ContentBuilder"#;
const STEAM_BUILD_ACCOUNT_USERNAME: &str = "Crimson_Sky_Admin";
const STEAM_BUILD_ACCOUNT_PASSWORD: &str = r#"UFY5LDXQff&rTu2h9T3LhmEd8M6rXvV0"#;

pub fn steam(game_path: PathBuf, version: &str) {
    let apps_info: HashMap<&str, AppInfo> = HashMap::from([
        (
            "College Kings",
            AppInfo {
                name: "College Kings".into(),
                app_id: 1463120,
                content_path: r#"D:\Crimson Sky\College Kings\CollegeKings-dists\CollegeKings-market"#.into(),
                additional_dlc: Vec::new(),
            },
        ),
        (
            "College Kings 2",
            AppInfo {
                name: "College Kings 2".into(),
                app_id: 1924480,
                content_path: r#"D:\Crimson Sky\College Kings\CollegeKings2-dists\CollegeKings2-market"#.into(),
                additional_dlc: vec![AppDLC {
                    name: "College Kings 2 - Episode 2 \"The Pool Party\"".into(),
                    app_id: 2100540,
                    content_path: r#"D:\Crimson Sky\College Kings\CollegeKings2-dists\CollegeKings2-market\game\ep2.rpa"#.into(),
                },
                AppDLC {
                    name: "College Kings 2 - Episode 3 \"Back To Basics\"".into(),
                    app_id: 2267960,
                    content_path: r#"D:\Crimson Sky\College Kings\CollegeKings2-dists\CollegeKings2-market\game\ep3.rpa"#.into(),
                }],
            },
        ),
    ]);

    let game_name = game_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('-', " ");

    let app_info = apps_info.get(game_name.as_str()).unwrap();

    println!("Building Game...");
    // build_game("market", &game_path);

    println!("Creating Depot Script...");
    create_depot_script(app_info, &app_info.content_path);

    println!("Creating App Script...");
    create_app_script(app_info, version.into());

    println!("Uploading Game...")
    // upload_game();
}

fn create_depot_script(app_info: &AppInfo, game_path: &String) {
    let depot_id = app_info.app_id + 1;

    let depot_build = DepotBuildConfig::new(
        depot_id,
        PathBuf::from(game_path),
        FileMapping {
            local_path: "*".into(),
            depot_path: ".".into(),
            recursive: "1".into(),
        },
    );

    // IMPROVEMENT: Create formatter for handling VDF files
    let vdf_content = format!(
        r#""DepotBuildConfig"
{{
    "DepotID" "{}"
    "contentroot" "{}"
    "FileMapping"
    {{
        "LocalPath" "{}"
        "DepotPath" "{}"
        "recursive" "{}"
    }}
}}"#,
        depot_build.depot_id,
        depot_build.content_root,
        depot_build.file_mapping.local_path,
        depot_build.file_mapping.depot_path,
        depot_build.file_mapping.recursive,
    );

    let mut file_path = PathBuf::from(CONTENT_BUILDER_PATH);
    file_path.push("scripts");
    file_path.push(format!("depot_{}_DEV.vdf", depot_build.depot_id));

    let mut file = File::create(file_path).unwrap();
    file.write_all(vdf_content.as_bytes()).unwrap();
}

fn create_app_script(app_info: &AppInfo, version: String) {
    let mut build_output = PathBuf::from(CONTENT_BUILDER_PATH);
    build_output.push(r#"output"#);

    let app_build = AppBuild::new(
        app_info.app_id,
        version,
        build_output,
        true,
        1924481,
        r#"D:\Steam Build\sdk\tools\ContentBuilder\scripts\depot_1924481.vdf"#.into(),
    );

    // IMPROVEMENT: Create formatter for handling VDF files
    let vdf_content = format!(
        r#""appbuild"
{{
    "appid" "{}"
    "desc" "{}"
    "buildoutput" "{}"
    "contentroot" "{}"
    "setlive" "{}"
    "preview" "{}"
    "local" "{}"
    "depots"
    {{
        "{}" "{}"
    }}
}}"#,
        app_build.app_id,
        app_build.desc,
        app_build.build_output,
        app_build.content_root,
        app_build.set_live,
        app_build.preview,
        app_build.local,
        app_build.depots.depot_id,
        app_build.depots.depot_path,
    );

    let mut file_path = PathBuf::from(CONTENT_BUILDER_PATH);
    file_path.push("scripts");
    file_path.push(format!("app_{}_DEV.vdf", app_build.app_id));

    let mut file = File::create(file_path).unwrap();
    file.write_all(vdf_content.as_bytes()).unwrap();
}

fn upload_game() {
    let mut steam_cmd = PathBuf::from(CONTENT_BUILDER_PATH);
    steam_cmd.push(r#"builder\steamcmd.exe"#);

    let mut app_script = PathBuf::from(CONTENT_BUILDER_PATH);
    app_script.push(r#"scripts\app_1924480.vdf"#);

    let output = Command::new(steam_cmd)
        .arg("+login")
        .arg(STEAM_BUILD_ACCOUNT_USERNAME)
        .arg(STEAM_BUILD_ACCOUNT_PASSWORD)
        .arg("+run_app_build")
        .arg(app_script.to_str().unwrap())
        .arg("+quit")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
}
