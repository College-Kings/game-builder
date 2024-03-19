#![allow(dead_code)]

use chrono::Local;
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{BUNNY_PATH_ROOT, GAME_DIR, VERSION};

pub fn run() {
    let timestamp = Local::now().format("%Y%m%d%H%M%S%.3f").to_string();

    generate_manifest("pc", &timestamp)
}

fn upload_file(root_path: &Path, file_path: &str, os: &str, timestamp: &str) {
    println!("Uploading {}", file_path);

    let bunny_path = format!(
        "{}/CK-{}-{}-{}",
        BUNNY_PATH_ROOT,
        VERSION.replace('.', "-"),
        os,
        timestamp
    );

    let file = File::open(root_path).expect("File not found");
    let file_size = file.metadata().unwrap().len();

    let client = Client::builder().timeout(None).build().unwrap();

    let response = client
        .put(format!("{}/{}", bunny_path, file_path))
        .header(
            "Accesskey",
            env::var("BUNNY_ACCESS_KEY").expect("Missing Bunny Access Key"),
        )
        .header("Content-Length", file_size)
        .body(file)
        .send()
        .unwrap();

    // Check the response status code
    if !response.status().is_success() {
        println!("Failed to upload the file. Status: {:?}", response.status());
    }
}

pub fn generate_manifest(os: &str, timestamp: &str) {
    let mut game_path_buf = PathBuf::from(GAME_DIR);
    game_path_buf.pop();
    game_path_buf.push("CollegeKings-dists");
    game_path_buf.push("CollegeKings-pc");

    let mut manifest: HashMap<String, String> = HashMap::new();

    walk_directory_and_upload(&game_path_buf, &mut manifest, &game_path_buf, os, timestamp);

    let options = serde_json::to_string_pretty(&manifest).unwrap();

    let manifest_path = PathBuf::from(GAME_DIR).join("manifest.json");
    let mut file = File::create(manifest_path).unwrap();
    file.write_all(options.as_bytes()).unwrap();
}

fn walk_directory_and_upload(
    root: &Path,
    manifest: &mut HashMap<String, String>,
    game_path: &Path,
    os: &str,
    timestamp: &str,
) {
    let files = fs::read_dir(root).unwrap();

    for entry in files {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_path = path
            .strip_prefix(game_path)
            .unwrap_or(&path)
            .to_str()
            .unwrap();

        if path.is_file() {
            upload_file(&path, file_path, os, timestamp);
            manifest.insert(file_path.into(), sha256_checksum(&path));
        } else {
            walk_directory_and_upload(&path, manifest, game_path, os, timestamp);
        }
    }
}

fn sha256_checksum(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    format!("{:x}", result)
}
