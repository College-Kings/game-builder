use std::{env, path::PathBuf};

use bunny_cdn_wrapper::edge_storage::BunnyStorage;
use reqwest::Response;

use crate::{renpy::build_game, update_steam_status, Result, GAME_NAME};

const DIST_DIRECTORY: &str = r"D:\Crimson Sky\College Kings\CollegeKings2-dists\";

pub async fn bunny(version: &str) -> Result<()> {
    println!("Starting Bunny Process...");

    update_steam_status(false)?;

    build_game(&["pc", "mac"], "zip")?;

    let pc_handler = tokio::spawn(upload_game(format!(
        "{}-{}-pc.zip",
        GAME_NAME.replace(' ', ""),
        version
    )));
    let mac_handler = tokio::spawn(upload_game(format!(
        "{}-{}-mac.zip",
        GAME_NAME.replace(' ', ""),
        version
    )));

    let responses = tokio::try_join!(pc_handler, mac_handler)?;

    println!("Responses: {:?}", responses);

    Ok(())
}

async fn upload_game(file_name: String) -> Result<Response> {
    println!("Uploading {} build...", file_name);

    let bunny_storage =
        BunnyStorage::new("collegekingsstorage", env::var("BUNNY_ACCESS_KEY")?, "de")?;

    let dist_directory = PathBuf::from(DIST_DIRECTORY);

    let file_path = dist_directory.join(&file_name);
    let response = bunny_storage
        .upload(
            file_path,
            format!(
                "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/{file_name}",
                GAME_NAME.to_lowercase().replace(' ', "_"),
            ),
        )
        .await?;

    Ok(response)
}
