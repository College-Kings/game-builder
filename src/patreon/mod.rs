pub mod launcher_upload;
pub mod upload_manifest;

use std::{env, fs::File, path::PathBuf, thread, time::Duration};

use crate::error::{Error, Result};
use reqwest::blocking::Client;
use tokio::time;

use crate::{build_game, BUNNY_ROOT, GAME_DIR, GAME_NAME, VERSION};

pub async fn patreon() -> Result<()> {
    println!("Starting patreon process...");

    let pc_build_thread = thread::spawn(|| build_game("pc", "zip"));
    time::sleep(Duration::from_secs(30)).await;
    let mac_build_thread = thread::spawn(|| build_game("mac", "zip"));

    pc_build_thread.join().map_err(Error::Thread)??;
    mac_build_thread.join().map_err(Error::Thread)??;

    let pc_upload_thread = thread::spawn(|| upload_game("pc"));
    let mac_upload_thread = thread::spawn(|| upload_game("mac"));

    pc_upload_thread.join().map_err(Error::Thread)??;
    mac_upload_thread.join().map_err(Error::Thread)??;

    Ok(())
}

pub fn upload_game(os: &str) -> Result<()> {
    println!("Uploading {} build...", os);

    let game_name_without_spaces = GAME_NAME.replace(' ', "");

    let bunny_root = BUNNY_ROOT.replace("{}", &GAME_NAME.replace(' ', "_").to_lowercase()); // BUG: BUNNY_ROOT has been changed
    let url = format!(
        "{}/{}-{}-{}.zip",
        bunny_root, game_name_without_spaces, VERSION, os
    );

    let game_path_buf = PathBuf::from(GAME_DIR);

    let file_path = format!(
        r"{}\{}-dists\{}-{}.zip",
        game_path_buf
            .parent()
            .ok_or_else(|| Error::InvalidPath(game_path_buf.clone()))?
            .to_str()
            .ok_or_else(|| Error::InvalidPath(game_path_buf.clone()))?,
        game_name_without_spaces,
        game_name_without_spaces,
        os
    );

    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();

    let client = Client::builder().timeout(None).build()?;

    let response = client
        .put(url)
        .header("Accesskey", env::var("BUNNY_ACCESS_KEY")?)
        .header("Content-Length", file_size)
        .body(file)
        .send()?;

    // Check the response status code
    if response.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        println!("Failed to upload the file. Status: {:?}", response.status());
    }

    Ok(())
}
