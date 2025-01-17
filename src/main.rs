mod action;
mod bunny;
mod error;
mod regex;
pub mod renpy;
mod steam;
pub mod utils;

use action::Action;
use bunny::upload;
pub use error::{Error, Result};
use lazy_static::lazy_static;
use regex::VERSION_REGEX;
use renpy::build_game;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{fs, io, thread};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};
// use crate::patreon::{
//     launcher_upload::{generate_manifest, run},
//     upload_manifest::upload_manifest,
// };

const RENPY_DIR: &str = r"D:\renpy-sdk";

// -- Steam
const CONTENT_BUILDER_PATH: &str = r"D:\steamworks_sdk\sdk\tools\ContentBuilder";
const PREVIEW: bool = false;

lazy_static! {
    pub static ref APPS: HashMap<&'static str, Vec<u32>> = {
        let mut map = HashMap::new();
        map.insert("College Kings", vec![1463120]);
        map.insert(
            "College Kings 2",
            vec![1924480, 2100540, 2267960, 2725540, 0, 3126690],
        );
        map
    };
}

// College Kings 1
// const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\College-Kings";
// const GAME_NAME: &str = "College Kings";

// College Kings 2
const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\college-kings-2-main";
const GAME_NAME: &str = "College Kings 2";

const ACTION: Action = Action::STEAM;

fn update_steam_status(is_steam: bool) {
    let script_file_path = PathBuf::from(GAME_DIR).join("game").join("script.rpy");

    let mut file_contents = fs::read_to_string(&script_file_path).unwrap();

    if is_steam {
        file_contents = file_contents.replace(
            "define config.enable_steam = False",
            "define config.enable_steam = True",
        );
    } else {
        file_contents = file_contents.replace(
            "define config.enable_steam = True",
            "define config.enable_steam = False",
        );
    }

    fs::write(script_file_path, file_contents).unwrap();
}

fn get_version() -> Result<String> {
    let version_file_path = PathBuf::from(GAME_DIR).join("game").join("script.rpy");
    let contents = fs::read_to_string(version_file_path).unwrap();

    let version = VERSION_REGEX
        .captures(&contents)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();

    Ok(version)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let version: Arc<String> = Arc::from(get_version().unwrap());

    // upload_manifest(Path::new("manifest.json"));

    let pc_built = Arc::new(AtomicBool::new(false));
    let mac_built = Arc::new(AtomicBool::new(false));

    const DEBUG: bool = false;
    let patreon_thread =
        if ACTION.contains(Action::PATREON) || ACTION.contains(Action::OPPAIMAN) && DEBUG {
            println!("Building for Patreon...");

            let pc_built = pc_built.clone();
            let mac_built = mac_built.clone();

            thread::spawn(move || {
                update_steam_status(false);

                let pc_thread = thread::spawn(|| build_game("pc", "zip"));
                thread::sleep(Duration::from_secs(30));
                let mac_thread = thread::spawn(|| build_game("mac", "app-zip"));

                pc_thread.join().unwrap();
                pc_built.store(true, Ordering::Release);
                mac_thread.join().unwrap();
                mac_built.store(true, Ordering::Release);
                Result::Ok(())
            })
        } else {
            pc_built.store(true, Ordering::Release);
            mac_built.store(true, Ordering::Release);
            thread::spawn(|| Result::Ok(()))
        };
    let steam_thread = if ACTION.contains(Action::STEAM) {
        thread::spawn(move || {
            while !pc_built.load(Ordering::Acquire) || !mac_built.load(Ordering::Acquire) {
                thread::yield_now();
            }
            update_steam_status(true);
            build_game("market", "directory");

            Result::Ok(())
        })
    } else {
        thread::spawn(|| Result::Ok(()))
    };

    patreon_thread.join().unwrap().unwrap();
    if ACTION.contains(Action::PATREON) {
        let game_name = GAME_NAME.replace(' ', "");

        let src_path = PathBuf::from(GAME_DIR)
            .parent()
            .unwrap()
            .join(format!("{}-dists", GAME_NAME.replace(' ', "")));

        let folder = GAME_NAME.to_lowercase().replace(' ', "_");

        let pc_name = format!("{}-{}-pc.zip", game_name, version);
        let mac_name = format!("{}-{}-mac.zip", game_name, version);

        upload(&src_path.join(&pc_name), format!("{}/{}", folder, pc_name)).await;
        upload(
            &src_path.join(&mac_name),
            format!("{}/{}", folder, mac_name),
        )
        .await;
    }
    if ACTION.contains(Action::OPPAIMAN) && GAME_NAME == "College Kings 2" {
        println!("Uploading for OppaiMan...");

        let game_name = GAME_NAME.replace(' ', "");
        let temp_dir = Path::new(&GAME_DIR[0..3]).join("college-kings");
        fs::create_dir_all(&temp_dir).unwrap();

        let dists_path = PathBuf::from(GAME_DIR)
            .parent()
            .unwrap()
            .join(format!("{}-dists", game_name));

        let pc_zip_file = dists_path.join(format!("{}-{}-pc.zip", game_name, version));
        let mac_zip_file = dists_path.join(format!("{}-{}-mac.zip", game_name, version));

        let pc_folder_path = unzip(&pc_zip_file).await;
        let mac_folder_path = unzip(&mac_zip_file).await;
        let pc_game_folder = pc_folder_path.join("game");
        let mac_game_folder = mac_folder_path
            .join("Contents")
            .join("Resources")
            .join("autorun")
            .join("game");

        println!("pc_game_folder: {}", pc_game_folder.display());
        println!("mac_game_folder: {}", mac_game_folder.display());

        fs::read_dir(&pc_game_folder)
            .unwrap()
            .chain(fs::read_dir(&mac_game_folder).unwrap())
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .map(|s| s.starts_with("ep"))
                    .unwrap_or(false)
            })
            .for_each(|file| {
                fs::rename(file.path(), temp_dir.join(file.file_name()))
                    .expect("Failed to move file");
            });

        let folder = GAME_NAME.to_lowercase().replace(' ', "_");

        let entries = fs::read_dir(temp_dir)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();

        let unique_eps = entries
            .iter()
            .filter(|e| e.file_name().to_str().unwrap().ends_with("images.rpa"))
            .map(|e| e.file_name().to_str().unwrap()[2..3].parse::<u8>().unwrap());

        for ep in unique_eps {
            let entries = entries
                .iter()
                .filter(|entry| {
                    entry
                        .file_name()
                        .to_str()
                        .map(|s| s.starts_with(&format!("ep{}", ep)))
                        .unwrap_or(false)
                })
                .filter_map(|entry| match entry.file_name().into_string() {
                    Ok(file_name) => Some((file_name, entry.path())),
                    Err(_) => None,
                });

            for (file_name, file_path) in entries.clone() {
                let pc_file_path = pc_game_folder.join(&file_name);
                let mac_file_path = mac_game_folder.join(file_name);

                println!("Moving {}", file_path.display());

                fs::copy(&file_path, &pc_file_path).unwrap();
                fs::rename(&file_path, &mac_file_path).unwrap();
            }

            let pc_zip_path = zip(&pc_folder_path);
            let mac_zip_path = zip(&mac_folder_path);

            upload(
                &pc_zip_path,
                format!("{}/ep{}/{}-{}-pc.zip", folder, ep, game_name, version),
            )
            .await;
            upload(
                &mac_zip_path,
                format!("{}/ep{}/{}-{}-mac.zip", folder, ep, game_name, version),
            )
            .await;

            for (file_name, _) in entries {
                let pc_game_file_path = pc_game_folder.join(&file_name);
                let mac_game_file_path = mac_game_folder.join(file_name);
                println!("Removing: {}", pc_game_file_path.display());

                fs::remove_file(pc_game_file_path).unwrap();
                fs::remove_file(mac_game_file_path).unwrap();
            }
        }
    }
    if ACTION.contains(Action::STEAM) {
        steam_thread.join().unwrap().unwrap();
        steam::steam(&version);
    }

    update_steam_status(false);

    println!("DONE!");
}

async fn unzip(path: &Path) -> PathBuf {
    let root = path.parent().unwrap_or(Path::new(""));
    let mut out_dir = None;

    let file = File::open(path).unwrap();
    println!("Unzipping: {}", path.display());

    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => {
                if out_dir.is_none() {
                    if let Some(p) = path.components().next() {
                        out_dir = Some(root.join(p));
                    }
                }

                root.join(path)
            }
            None => continue,
        };

        if file.is_dir() {
            tokio::fs::create_dir_all(&outpath).await.unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    tokio::fs::create_dir_all(p).await.unwrap();
                }
            }

            let mut outfile = File::create(outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    out_dir.unwrap()
}

pub fn zip(folder_path: impl AsRef<Path>) -> PathBuf {
    let src_dir = folder_path.as_ref();
    let dst_file = src_dir.with_file_name(format!(
        "{}.zip",
        src_dir.file_name().unwrap().to_string_lossy()
    ));

    if !src_dir.is_dir() {
        panic!("Provided folder path is not a directory")
    }

    let file = File::create(&dst_file).expect("Failed to create zip file");
    let mut zip = ZipWriter::new(file);

    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .large_file(true)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(src_dir).expect("Failed to strip prefix");

        let clean_name = name.to_string_lossy().replace("\\", "/");

        if path.is_file() {
            println!("Adding file to zip: {}", path.display());

            zip.start_file(clean_name, options)
                .expect("Failed to add file to zip");

            let mut f = File::open(path).expect("Failed to open file for zipping");
            f.read_to_end(&mut buffer)
                .expect("Failed to read file content");
            zip.write_all(&buffer)
                .expect("Failed to write file content to zip");
            buffer.clear();
        } else if path.is_dir() {
            println!("Adding directory to zip: {}", path.display());

            zip.add_directory(clean_name, options)
                .expect("Failed to add directory to zip");
        }
    }

    zip.finish().expect("Failed to finalize zip file");
    dst_file
}
