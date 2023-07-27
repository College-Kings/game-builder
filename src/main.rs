mod action;
mod patreon;
mod steam;

use std::{
    env,
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use action::Action;
use patreon::patreon::patreon;
use steam::steam::steam;
use tokio::task;

const CONTENT_BUILDER_PATH: &str = r#"D:\Steam Build\sdk\tools\ContentBuilder"#;
const STEAM_BUILD_ACCOUNT_USERNAME: &str = "Crimson_Sky_Admin";
const STEAM_BUILD_ACCOUNT_PASSWORD: &str = r#"UFY5LDXQff&rTu2h9T3LhmEd8M6rXvV0"#;
const BUNNY_PATH_TEMPLATE: &str = r#"https://storage.bunnycdn.com/collegekingsstorage/__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}"#;
const BUNNY_ACCESS_KEY: &str = "ba39c6ef-f9e1-4ea7-a132e6d0f8c2-0aa3-4d1a";
const RENPY_DIR: &str = r#"D:\renpy-sdk"#;
const PREVIEW: bool = false;

const GAME_DIR: &str = r#"D:\Crimson Sky\College Kings\College-Kings"#;
const ACTION: Action = Action::Patreon;
const VERSION: &str = "1.3.17";

pub fn build_game(package: &str, format: &str) {
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
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(steam_stdout) = renpy_process.stdout.take() {
        let reader = BufReader::new(steam_stdout);

        for line in reader.lines() {
            if let Ok(line) = line {
                println!("{}", line)
            }
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

#[tokio::main]
async fn main() {
    let game_name = PathBuf::from(GAME_DIR)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace('-', " ");

    match ACTION {
        Action::Steam => {
            println!("Starting Steam Process...");
            steam(game_name);
        }
        Action::Patreon => {
            println!("Starting Patreon Process...");
            patreon(game_name).await;
        }
        Action::Both => {
            println!("Starting Patreon Process...");
            let patreon_task = task::spawn(patreon(game_name.clone()));

            tokio::time::sleep(Duration::from_secs(30)).await;

            println!("Starting Steam Process...");
            steam(game_name);

            let _ = patreon_task.await;
        }
    }

    println!("DONE!")
}
