#![allow(dead_code)]

use crate::{Error, Result};
use crate::{BUNNY_ROOT, GAME_DIR};
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

pub fn run(version: &str) -> Result<()> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S%.3f").to_string();

    generate_manifest(version, "pc", &timestamp)?;
    Ok(())
}

fn upload_file(
    root_path: &Path,
    file_path: &str,
    version: &str,
    os: &str,
    timestamp: &str,
) -> Result<()> {
    println!("Uploading {}", file_path);

    let bunny_path = format!(
        "{}/CK-{}-{}-{}",
        BUNNY_ROOT,
        version.replace('.', "-"),
        os,
        timestamp
    );

    let file = File::open(root_path)?;
    let file_size = file.metadata()?.len();

    let client = Client::builder().timeout(None).build()?;

    let response = client
        .put(format!("{}/{}", bunny_path, file_path))
        .header("Accesskey", env::var("BUNNY_ACCESS_KEY")?)
        .header("Content-Length", file_size)
        .body(file)
        .send()?;

    // Check the response status code
    if !response.status().is_success() {
        println!("Failed to upload the file. Status: {:?}", response.status());
    }

    Ok(())
}

pub fn generate_manifest(version: &str, os: &str, timestamp: &str) -> Result<()> {
    let game_path_buf = PathBuf::from(GAME_DIR)
        .parent()
        .ok_or_else(|| Error::InvalidPath(PathBuf::from(GAME_DIR)))?
        .to_path_buf()
        .join("CollegeKings-dists")
        .join("CollegeKings-pc");

    let mut manifest: HashMap<String, String> = HashMap::new();

    walk_directory_and_upload(
        &game_path_buf,
        &mut manifest,
        &game_path_buf,
        version,
        os,
        timestamp,
    )?;

    let options = serde_json::to_string_pretty(&manifest)?;

    let manifest_path = PathBuf::from(GAME_DIR).join("manifest.json");
    let mut file = File::create(manifest_path)?;
    file.write_all(options.as_bytes())?;

    Ok(())
}

fn walk_directory_and_upload(
    root: &Path,
    manifest: &mut HashMap<String, String>,
    game_path: &Path,
    version: &str,
    os: &str,
    timestamp: &str,
) -> Result<()> {
    let files = fs::read_dir(root)?;

    for entry in files {
        let entry = entry?;
        let path = entry.path();
        let file_path = path
            .strip_prefix(game_path)
            .unwrap_or(&path)
            .to_str()
            .ok_or_else(|| Error::InvalidPath(path.clone()))?;

        if path.is_file() {
            upload_file(&path, file_path, version, os, timestamp)?;
            manifest.insert(file_path.into(), sha256_checksum(&path)?);
        } else {
            walk_directory_and_upload(&path, manifest, game_path, version, os, timestamp)?;
        }
    }

    Ok(())
}

fn sha256_checksum(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    let result = hasher.finalize();

    Ok(format!("{:x}", result))
}
