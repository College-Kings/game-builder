mod patreon;
mod steam;

use std::{
    env,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use patreon::patreon::patreon;
use steam::steam::steam;

const CONTENT_BUILDER_PATH: &str = r#"D:\Steam Build\sdk\tools\ContentBuilder"#;
const STEAM_BUILD_ACCOUNT_USERNAME: &str = "Crimson_Sky_Admin";
const STEAM_BUILD_ACCOUNT_PASSWORD: &str = r#"UFY5LDXQff&rTu2h9T3LhmEd8M6rXvV0"#;
const RENPY_DIR: &str = r#"D:\renpy-sdk"#;
const GAME_DIR: &str = r#"D:\Crimson Sky\College Kings\College-Kings-2"#;
const PREVIEW: bool = false;

const IS_STEAM: bool = true;
const VERSION: &str = "3.1.13";

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

fn main() {
    if IS_STEAM {
        println!("Starting Steam Process...");
        steam();
    } else {
        patreon();
    }

    println!("DONE!")
}
