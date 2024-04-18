use std::{env, path::PathBuf};

use bunny_cdn_wrapper::BunnyStorage;

use crate::{Error, Result, GAME_DIR, GAME_NAME};

pub async fn upload_game(file_name: String) -> Result<()> {
    println!("Uploading {} build...", file_name);

    let bunny_storage =
        BunnyStorage::new("collegekingsstorage", env::var("BUNNY_ACCESS_KEY")?, "de")?;

    let file_path = PathBuf::from(GAME_DIR)
        .parent()
        .ok_or_else(|| Error::InvalidPath(PathBuf::from(GAME_DIR)))?
        .join(format!("{}-dists", GAME_NAME.replace(' ', "")))
        .join(&file_name);

    let response = bunny_storage
        .upload(
            file_path,
            &format!(
                "__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/{}/{file_name}",
                GAME_NAME.to_lowercase().replace(' ', "_"),
            ),
        )
        .await?;

    if !response.status().is_success() {
        println!("Failed to upload the file. Status: {:?}", response);
    }

    Ok(())
}
