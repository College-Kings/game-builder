use std::{env, path::PathBuf, sync::Arc, thread, time::Duration};

use bunny_cdn_wrapper::BunnyStorage;
use reqwest::Response;

use crate::{renpy::build_game, update_steam_status, Error, Result, GAME_DIR, GAME_NAME};

pub async fn bunny(version: Arc<String>) -> Result<()> {
    println!("Starting Bunny Process...");

    update_steam_status(false)?;

    let pc_handler = thread::spawn({
        let version = version.clone();
        || async move {
            build_game("pc", "zip")?;
            upload_game(format!("{}-{}-pc.zip", GAME_NAME.replace(' ', ""), version)).await
        }
    });
    thread::sleep(Duration::from_secs(30));
    let mac_handler = thread::spawn({
        let version = version.clone();
        || async move {
            build_game("mac", "zip")?;
            upload_game(format!(
                "{}-{}-mac.zip",
                GAME_NAME.replace(' ', ""),
                version
            ))
            .await
        }
    });

    let responses = tokio::try_join!(
        pc_handler.join().map_err(Error::Thread)?,
        mac_handler.join().map_err(Error::Thread)?
    )?;

    println!("Responses: {:?}", responses);

    Ok(())
}

async fn upload_game(file_name: String) -> Result<Response> {
    println!("Uploading {} build...", file_name);

    let bunny_storage =
        BunnyStorage::new("collegekingsstorage", env::var("BUNNY_ACCESS_KEY")?, "de")?;

    let file_path = PathBuf::from(GAME_DIR)
        .parent()
        .ok_or_else(|| Error::InvalidPath(PathBuf::from(GAME_DIR)))?
        .join(format!("{}-dists", GAME_NAME.replace(' ', "")))
        .join(&file_name);

    println!("File Path: {:?}", file_path);

    let response = bunny_storage
        .upload(
            file_path,
            &format!(
                "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/{file_name}",
                GAME_NAME.to_lowercase().replace(' ', "_"),
            ),
        )
        .await?;

    Ok(response)
}
