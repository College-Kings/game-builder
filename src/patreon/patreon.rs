use std::{fs::File, thread, time::Duration};

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;

use crate::{build_game, BUNNY_ACCESS_KEY, BUNNY_PATH_TEMPLATE, GAME_DIR, VERSION};

pub async fn patreon(game_name: String) {
    let pc_game_name = game_name.clone();
    let mac_game_name = game_name;

    let task1 = thread::spawn(move || handle_pc_game(pc_game_name));
    tokio::time::sleep(Duration::from_secs(30)).await;
    let task2 = thread::spawn(move || handle_mac_game(mac_game_name));

    task1.join().unwrap();
    task2.join().unwrap();
}

fn handle_pc_game(game_name: String) {
    println!("Building PC Game...");
    build_game("pc", "zip");

    println!("Uploading Game...");
    upload_game(game_name, "pc".into());
}

fn handle_mac_game(game_name: String) {
    println!("Building Mac Game...");
    build_game("mac", "zip");

    println!("Uploading Game...");
    upload_game(game_name, "mac".into());
}

fn upload_game(game_name: String, os: String) {
    let game_name_without_spaces = game_name.replace(' ', "");

    let bunny_root = BUNNY_PATH_TEMPLATE.replace("{}", &game_name_without_spaces.to_lowercase());
    let url = format!(
        "{}/{}-{}-{}.zip",
        bunny_root, game_name_without_spaces, VERSION, os
    );

    let file_path = format!(
        r#"{}-dists\{}-{}.zip"#,
        GAME_DIR, game_name_without_spaces, os
    );

    let file = File::open(file_path).unwrap();
    let file_size = file.metadata().unwrap().len();

    let client = Client::builder().timeout(None).build().unwrap();

    let pb = ProgressBar::new(file_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("##-"),
    );

    let response = client
        .put(url)
        .header("Accesskey", BUNNY_ACCESS_KEY)
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
