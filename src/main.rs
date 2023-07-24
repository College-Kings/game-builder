mod patreon;
mod steam;

use std::{env, path::PathBuf, process::Command};

use patreon::patreon::patreon;
use steam::steam::steam;

const IS_STEAM: bool = true;
const RENPY_DIR: &str = r#"D:\renpy-sdk"#;

pub fn build_game(package: &str, game_path: &PathBuf) {
    let original_dir = env::current_dir().unwrap();
    env::set_current_dir(RENPY_DIR).unwrap();

    let output = Command::new("./renpy.exe")
        .arg("launcher")
        .arg("distribute")
        .arg("--package")
        .arg(package)
        .arg(game_path.to_str().unwrap())
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());

    env::set_current_dir(original_dir).unwrap();
}

fn main() {
    let game_path = PathBuf::from(r#"D:\Crimson Sky\College Kings\College-Kings-2"#);
    let version = "DEV";

    if IS_STEAM {
        println!("Starting Steam Process...");
        steam(game_path, version);
    } else {
        patreon();
    }
}
