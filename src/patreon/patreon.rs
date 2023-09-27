use std::{env, fs::File, path::PathBuf, thread, time::Duration};

use reqwest::blocking::Client;
use tokio::time;

use crate::{build_game, BUNNY_PATH_ROOT, GAME_DIR, VERSION};

pub async fn patreon(game_name: String) {
    println!("Starting patreon process...");

    let pc_game_name = game_name.clone();

    let pc_build_thread = thread::spawn(|| build_game("pc", "zip"));
    time::sleep(Duration::from_secs(30)).await;
    let mac_build_thread = thread::spawn(|| build_game("mac", "zip"));

    pc_build_thread.join().unwrap();
    mac_build_thread.join().unwrap();

    let pc_upload_thread = thread::spawn(move || upload_game(&pc_game_name, "pc"));
    let mac_upload_thread = thread::spawn(move || upload_game(&game_name, "mac"));

    pc_upload_thread.join().unwrap();
    mac_upload_thread.join().unwrap();
}

pub fn upload_game(game_name: &str, os: &str) {
    println!("Uploading {} build...", os);

    let game_name_without_spaces = game_name.replace(' ', "");

    let bunny_root = BUNNY_PATH_ROOT.replace("{}", &game_name.replace(' ', "_").to_lowercase()); // BUG: BUNNY_ROOT has been changed
    let url = format!(
        "{}/{}-{}-{}.zip",
        bunny_root, game_name_without_spaces, VERSION, os
    );

    let game_path_buf = PathBuf::from(GAME_DIR);

    let file_path = format!(
        r#"{}\{}-dists\{}-{}.zip"#,
        game_path_buf.parent().unwrap().to_str().unwrap(),
        game_name_without_spaces,
        game_name_without_spaces,
        os
    );

    let file = File::open(&file_path).expect(&format!("File not found: {}", file_path));
    let file_size = file.metadata().unwrap().len();

    let client = Client::builder().timeout(None).build().unwrap();

    let response = client
        .put(url)
        .header(
            "Accesskey",
            env::var("BUNNY_ACCESS_KEY").expect("Missing Bunny Access Key"),
        )
        .header("Content-Length", file_size)
        .body(file)
        .send()
        .unwrap();

    // Check the response status code
    if response.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        println!("Failed to upload the file. Status: {:?}", response.status());
    }
}
