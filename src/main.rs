mod action;
mod bunny;
mod error;
mod patreon;
mod regex;
pub mod renpy;
mod steam;
pub mod utils;

pub use crate::error::{Error, Result};
use action::Action;
use lazy_static::lazy_static;
use patreon::patreon_thread;
use regex::VERSION_REGEX;
use renpy::build_game;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use utils::tokio_flatten;
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
        map.insert("College Kings 2", vec![1924480, 2100540, 2267960, 2725540]);
        map
    };
}

// College Kings 1
// const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\College-Kings";
// const GAME_NAME: &str = "College Kings";

// College Kings 2
const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\college-kings-2-main";
const GAME_NAME: &str = "College Kings 2";

const ACTION: Action = Action::SteamBunny;

fn update_steam_status(is_steam: bool) -> Result<()> {
    let script_file_path = PathBuf::from(GAME_DIR).join("game").join("script.rpy");

    let mut file_contents = fs::read_to_string(&script_file_path)?;

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

    fs::write(script_file_path, file_contents)?;

    Ok(())
}

fn get_version() -> Result<String> {
    let version_file_path = PathBuf::from(GAME_DIR).join("game").join("script.rpy");
    let contents = fs::read_to_string(version_file_path)?;

    let version = VERSION_REGEX
        .captures(&contents)
        .ok_or_else(|| Error::VersionNotFound(contents.as_str().into()))?
        .get(1)
        .ok_or_else(|| Error::VersionNotFound(contents.as_str().into()))?
        .as_str()
        .to_string();

    Ok(version)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let version: Arc<String> = Arc::from(get_version()?);

    // upload_manifest(Path::new("manifest.json"));

    match ACTION {
        Action::Steam => {
            update_steam_status(true)?;
            build_game("market", "directory")?;
            steam::steam(&version)?;
        }
        Action::Bunny => {
            update_steam_status(false)?;
            let (pc_thread, mac_thread) = patreon_thread(version.clone())?;
            tokio::try_join!(tokio_flatten(pc_thread), tokio_flatten(mac_thread))?;
        }
        Action::SteamBunny => {
            update_steam_status(false)?;
            let (pc_thread, mac_thread) = patreon_thread(version.clone())?;

            update_steam_status(true)?;
            build_game("market", "directory")?;
            steam::steam(&version)?;

            println!("Waiting for BunnyCDN to finish uploading...");
            tokio::try_join!(tokio_flatten(pc_thread), tokio_flatten(mac_thread))?;
        }
        _ => unimplemented!("Action not implemented"),
    }

    update_steam_status(false)?;

    //     Action::All => {
    //         update_steam_status(false)?;
    //         let pc_build_thread = thread::spawn(|| build_game("pc", "zip"));
    //         time::sleep(Duration::from_secs(30)).await;
    //         let mac_build_thread = thread::spawn(|| build_game("mac", "zip"));

    //         pc_build_thread.join().map_err(Error::Thread)??;
    //         mac_build_thread.join().map_err(Error::Thread)??;

    //         let pc_upload_thread = thread::spawn({
    //             let version = version.clone();
    //             move || patreon::upload_game(&version, "pc")
    //         });
    //         let mac_upload_thread = thread::spawn({
    //             let version = version.clone();
    //             move || patreon::upload_game(&version, "mac")
    //         });

    //         update_steam_status(true)?;
    //         let steam_thread = thread::spawn(move || steam(&version));

    //         steam_thread.join().map_err(Error::Thread)??;
    //         mac_upload_thread.join().map_err(Error::Thread)??;
    //         pc_upload_thread.join().map_err(Error::Thread)??;
    //     }
    // }

    println!("DONE!");
    Ok(())
}
