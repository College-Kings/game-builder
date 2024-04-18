use std::{sync::Arc, thread, time::Duration};

use tokio::task::JoinHandle;

use crate::{bunny::upload_game, renpy::build_game, Error, Result, GAME_NAME};

type ThreadHandle = JoinHandle<Result<()>>;

pub fn patreon_thread(version: Arc<String>) -> Result<(ThreadHandle, ThreadHandle)> {
    let pc_thread = thread::spawn(|| build_game("pc", "zip"));
    thread::sleep(Duration::from_secs(30));
    let mac_thread = thread::spawn(|| build_game("mac", "zip"));

    pc_thread.join().map_err(Error::Thread)??;
    let pc_thread = tokio::spawn(upload_game(format!(
        "{}-{}-pc.zip",
        GAME_NAME.replace(' ', ""),
        version
    )));

    mac_thread.join().map_err(Error::Thread)??;
    let mac_thread = tokio::spawn(upload_game(format!(
        "{}-{}-mac.zip",
        GAME_NAME.replace(' ', ""),
        version
    )));

    Ok((pc_thread, mac_thread))
}
