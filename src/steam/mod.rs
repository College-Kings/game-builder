mod app_build;
mod app_info;
mod depot_build_config;

use crate::renpy::build_game;
use crate::{update_steam_status, CONTENT_BUILDER_PATH, GAME_NAME, PREVIEW};
use crate::{Error, Result};
use app_build::AppBuild;
use app_info::AppInfo;
use depot_build_config::{DepotBuildConfig, FileMapping};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

const CONTENT_PATH: &str = r"D:\Crimson Sky\College Kings\CollegeKings-dists\CollegeKings-market";

pub fn steam(version: &str) -> Result<()> {
    println!("Starting Steam Process...");

    update_steam_status(true)?;

    let apps_info: HashMap<&str, AppInfo> = HashMap::from([
        (
            "College Kings",
            AppInfo::new("College Kings", 1463120, CONTENT_PATH),
        ),
        (
            "College Kings 2",
            AppInfo {
                name: "College Kings 2".into(),
                app_id: 1924480,
                content_path: CONTENT_PATH.to_string(),
                additional_dlc: vec![
                    AppInfo::new(
                        "College Kings 2 - Episode 2 \"The Pool Party\"",
                        2100540,
                        "*ep2.rpa",
                    ),
                    AppInfo::new(
                        "College Kings 2 - Episode 3 \"Back To Basics\"",
                        2267960,
                        "*ep3.rpa",
                    ),
                ],
            },
        ),
    ]);

    let app_info = apps_info
        .get(GAME_NAME)
        .ok_or_else(|| Error::GameNotFound)?;

    println!("Building Game...");
    build_game("market", "directory")?;

    println!("Creating Depot Script...");
    create_depot_script(app_info, None)?;

    println!("Creating App Script...");
    create_app_script(app_info, version, None)?;

    println!("Uploading Game...");
    upload_game(app_info)?;

    for dlc_info in app_info.additional_dlc.iter() {
        println!("Creating DLC Depot Script...");
        create_depot_script(app_info, Some(dlc_info))?;

        println!("Creating DLC App Script...");
        create_app_script(app_info, version, Some(dlc_info))?;

        println!("Uploading DLC {}", dlc_info.name);
        upload_game(dlc_info)?;
    }

    Ok(())
}

fn create_depot_script(app_info: &AppInfo, dlc_info: Option<&AppInfo>) -> Result<()> {
    let depot_build = match dlc_info {
        Some(dlc_info) => DepotBuildConfig::new(
            dlc_info.app_id,
            PathBuf::from(&app_info.content_path),
            FileMapping::new(&dlc_info.content_path, ".", true),
            Vec::<String>::new(),
        )?,
        None => DepotBuildConfig::new(
            app_info.app_id + 1,
            PathBuf::from(&app_info.content_path),
            FileMapping::new("*", ".", true),
            app_info.additional_dlc.iter().map(|dlc| &dlc.content_path),
        )?,
    };

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
{}
}}"#,
        depot_build.depot_id,
        depot_build.content_root,
        depot_build.file_mapping.local_path,
        depot_build.file_mapping.depot_path,
        depot_build.file_mapping.recursive,
        depot_build
            .file_exclusions
            .into_iter()
            .map(|f| format!(r#"    "FileExclusion" "{}""#, f))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let mut file_path = PathBuf::from(CONTENT_BUILDER_PATH);
    file_path.push("scripts");
    file_path.push(format!("depot_{}.vdf", depot_build.depot_id));

    let mut file = File::create(file_path)?;
    file.write_all(vdf_content.as_bytes())?;
    Ok(())
}

fn create_app_script(app_info: &AppInfo, version: &str, dlc_info: Option<&AppInfo>) -> Result<()> {
    let mut build_output = PathBuf::from(CONTENT_BUILDER_PATH);
    build_output.push("output");

    let app_build = match dlc_info {
        Some(dlc_info) => AppBuild::new(
            dlc_info.app_id,
            version,
            build_output,
            PREVIEW,
            dlc_info.app_id,
            format!(
                r"{}\scripts\depot_{}.vdf",
                CONTENT_BUILDER_PATH, dlc_info.app_id
            ),
        )?,
        None => AppBuild::new(
            app_info.app_id,
            version,
            build_output,
            PREVIEW,
            app_info.app_id + 1,
            format!(
                r"{}\scripts\depot_{}.vdf",
                CONTENT_BUILDER_PATH,
                app_info.app_id + 1
            ),
        )?,
    };

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
    file_path.push(format!("app_{}.vdf", app_build.app_id));

    let mut file = File::create(file_path)?;
    file.write_all(vdf_content.as_bytes())?;
    Ok(())
}

fn upload_game(app_info: &AppInfo) -> Result<()> {
    let mut steam_cmd = PathBuf::from(CONTENT_BUILDER_PATH);
    steam_cmd.push(r"builder\steamcmd.exe");

    let mut app_script = PathBuf::from(CONTENT_BUILDER_PATH);
    app_script.push(format!(r"scripts\app_{}.vdf", app_info.app_id));

    let mut steam_process = Command::new(steam_cmd)
        .arg("+login")
        .arg(env::var("STEAM_BUILD_ACCOUNT_USERNAME")?)
        .arg(env::var("STEAM_BUILD_ACCOUNT_PASSWORD")?)
        .arg("+run_app_build")
        .arg(
            app_script
                .to_str()
                .ok_or_else(|| Error::InvalidPath(app_script.clone()))?,
        )
        .arg("+quit")
        .stdout(Stdio::piped())
        .spawn()?;

    if let Some(steam_stdout) = steam_process.stdout.take() {
        let reader = BufReader::new(steam_stdout);

        for line in reader.lines() {
            let line = line?;
            println!("{}", line)
        }
    }

    let status = steam_process.wait()?;
    if status.success() {
        println!("Upload successful");
    } else {
        println!("Upload failed: {:?}", status.code());
    }

    Ok(())
}
