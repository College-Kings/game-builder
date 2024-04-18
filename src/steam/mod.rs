use std::{
    env,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

mod app_scripts;
use app_scripts::create_app_scripts;

mod depot_script;
use depot_script::create_depot_scripts;

use crate::{Error, Result, APPS, CONTENT_BUILDER_PATH, GAME_NAME};

pub fn steam(version: &str) -> Result<()> {
    println!("Starting Steam Process...");

    let app_ids = APPS
        .get(GAME_NAME)
        .ok_or_else(|| Error::InvalidGame(GAME_NAME.to_string()))?;

    create_app_scripts(app_ids, version)?;
    create_depot_scripts(app_ids)?;
    upload_apps(app_ids)?;

    Ok(())
}

fn upload_apps(app_ids: &[u32]) -> Result<()> {
    let handles: Vec<_> = app_ids
        .iter()
        .map(|app_id| {
            let app_id = *app_id;
            thread::spawn(move || upload_app(app_id))
        })
        .collect();

    for handle in handles {
        handle.join().map_err(Error::Thread)??;
    }

    Ok(())
}

fn upload_app(app_id: u32) -> Result<()> {
    let steam_cmd = PathBuf::from(CONTENT_BUILDER_PATH)
        .join("builder")
        .join("steamcmd.exe");
    let app_script = PathBuf::from(CONTENT_BUILDER_PATH)
        .join("scripts")
        .join(format!("app_{}.vdf", app_id));

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
        println!("Upload failed: {:?}", status);
    }

    Ok(())
}
