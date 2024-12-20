use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{Result, GAME_DIR, RENPY_DIR};

pub fn build_game(package: &str, format: &str) -> Result<()> {
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
        .spawn()?;

    if let Some(stdout) = renpy_child.stdout.take() {
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            println!("{}", line?)
        }
    }

    let status = renpy_child.wait()?;
    if status.success() {
        println!("Build successful")
    } else {
        println!("Build failed: {:?}", status.code())
    }

    Ok(())
}
