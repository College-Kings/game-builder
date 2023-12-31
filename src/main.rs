mod action;
mod patreon;
mod steam;

use crate::patreon::patreon::patreon;
use action::Action;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use steam::steam;
use tokio::time;

// use crate::patreon::{
//     launcher_upload::{generate_manifest, run},
//     upload_manifest::upload_manifest,
// };

const CONTENT_BUILDER_PATH: &str = r"D:\Steam Build\sdk\tools\ContentBuilder";
const BUNNY_PATH_ROOT: &str = r#"https://storage.bunnycdn.com/collegekingsstorage/__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/Game%20Launcher/Delta%20Patching%20Testing"#;
const RENPY_DIR: &str = r"D:\renpy-sdk";
const PREVIEW: bool = false;

const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\College-Kings-2";
const ACTION: Action = Action::Steam;
const VERSION: &str = "3.3.7";

pub fn build_game(package: &str, format: &str) {
    println!("Building {} Game...", package);

    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(RENPY_DIR).unwrap();

    let mut renpy_process = Command::new("./renpy.exe")
        .arg("launcher")
        .arg("distribute")
        .arg("--package")
        .arg(package)
        .arg("--format")
        .arg(format)
        .arg(GAME_DIR)
        .stdout(Stdio::null())
        .spawn()
        .expect("failed to execute process");

    if let Some(steam_stdout) = renpy_process.stdout.take() {
        let reader = BufReader::new(steam_stdout);

        for line in reader.lines().flatten() {
            println!("{}", line)
        }
    }

    let status = renpy_process
        .wait()
        .expect("Failed to wait for steam process.");
    if status.success() {
        println!("Build successful")
    } else {
        println!("Build failed: {:?}", status.code())
    }

    env::set_current_dir(original_dir).unwrap();
}

fn update_steam_status(is_steam: bool) {
    let mut script_file_path = PathBuf::from(GAME_DIR);
    script_file_path.push("game");
    script_file_path.push("script.rpy");

    let mut file = File::open(&script_file_path).unwrap();
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).unwrap();

    if is_steam {
        file_contents = file_contents.replace(
            "define config.enable_steam = False",
            "define config.enable_steam = True",
        );
    } else {
        file_contents = file_contents.replace(
            "define config.enable_steam = True",
            "define config.enable_steam = False",
        );
    }

    let mut file = File::create(script_file_path).unwrap();
    file.write_all(file_contents.as_bytes()).unwrap();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let game_name = PathBuf::from(GAME_DIR)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('-', " ");

    // upload_manifest(Path::new("manifest.json"));

    match ACTION {
        Action::Steam => {
            update_steam_status(true);
            steam(&game_name);
        }
        Action::Patreon => {
            update_steam_status(false);
            patreon(&game_name).await;
        }
        Action::All => {
            let pc_game_name = game_name.clone();
            let mac_game_name = game_name.clone();

            update_steam_status(false);
            let pc_build_thread = thread::spawn(|| build_game("pc", "zip"));
            time::sleep(Duration::from_secs(30)).await;
            let mac_build_thread = thread::spawn(|| build_game("mac", "zip"));

            pc_build_thread.join().unwrap();
            mac_build_thread.join().unwrap();

            let pc_upload_thread =
                thread::spawn(move || patreon::patreon::upload_game(pc_game_name, "pc"));
            let mac_upload_thread =
                thread::spawn(move || patreon::patreon::upload_game(mac_game_name, "mac"));

            update_steam_status(true);
            let steam_thread = thread::spawn(move || steam(&game_name));

            steam_thread.join().unwrap();
            mac_upload_thread.join().unwrap();
            pc_upload_thread.join().unwrap();
        }
    }

    println!("DONE!")
}
