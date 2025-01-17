use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{GAME_DIR, RENPY_DIR};

pub fn build_game(package: &str, format: &str) {
    println!("Building {package}...");

    let renpy_dir = PathBuf::from(RENPY_DIR);

    let mut renpy_child = Command::new(renpy_dir.join("renpy.exe"))
        .args([
            "launcher",
            "distribute",
            GAME_DIR,
            "--package",
            package,
            "--format",
            format,
        ])
        .stdout(Stdio::piped())
        .current_dir(&renpy_dir)
        .spawn()
        .unwrap();

    if let Some(stdout) = renpy_child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            println!("{}", line.unwrap())
        }
    }

    let status = renpy_child.wait().unwrap();
    if status.success() {
        println!("Build successful")
    } else {
        println!("Build failed: {:?}", status.code())
    }
}
