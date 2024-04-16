mod action;
mod error;
mod patreon;
mod steam;

pub use crate::error::{Error, Result};
use crate::patreon::patreon;
use action::Action;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::Duration,
};
use steam::steam;
use tokio::time;
// use crate::patreon::{
//     launcher_upload::{generate_manifest, run},
//     upload_manifest::upload_manifest,
// };

const CONTENT_BUILDER_PATH: &str = r"D:\steamworks_sdk\sdk\tools\ContentBuilder";
const BUNNY_ROOT: &str = r"https://storage.bunnycdn.com/collegekingsstorage/__bcdn_perma_cache__/pullzone__collegekings__22373407/wp-content/uploads/secured/Game%20Launcher/Delta%20Patching%20Testing";
const RENPY_DIR: &str = r"D:\renpy-8.2.0-sdk";
const PREVIEW: bool = false;

const GAME_DIR: &str = r"D:\Crimson Sky\College Kings\college-kings-2-main";
const GAME_NAME: &str = "College Kings 2";
const ACTION: Action = Action::Steam;
const VERSION: &str = "3.3.16";

fn update_steam_status(is_steam: bool) -> Result<()> {
    let mut script_file_path = PathBuf::from(GAME_DIR);
    script_file_path.push("game");
    script_file_path.push("script.rpy");

    let mut file = File::open(&script_file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;

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

    let mut file = File::create(script_file_path)?;
    file.write_all(file_contents.as_bytes())?;

    Ok(())
}

fn get_version() -> Result<String> {
    let version_file_path = PathBuf::from(GAME_DIR).join("game").join("script.rpy");

    let mut file = File::open(version_file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let version = VERSION_REGEX
        .captures(&contents)
        .ok_or(Error::VersionNotFound)?
        .get(1)
        .ok_or(Error::VersionNotFound)?
        .as_str()
        .to_string();

    Ok(version)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let version: Arc<String> = Arc::from(get_version()?);
    println!("Version: {}", version);

    // upload_manifest(Path::new("manifest.json"));

    match ACTION {
        Action::Steam => steam::steam(&version)?,
        Action::Bunny => bunny::bunny(&version).await?,
        Action::SteamBunny => {
            let steam_thread = thread::spawn({
                let version = version.clone();
                move || steam::steam(&version)
            });
            sleep(Duration::from_secs(30)).await;
            let bunny_thread = thread::spawn({
                let version = version.clone();
                move || async move { bunny::bunny(&version).await }
            });

            steam_thread.join().map_err(Error::Thread)??;
            bunny_thread.join().map_err(Error::Thread)?.await?;
        }
        _ => unimplemented!("Action not implemented"),
    }

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
