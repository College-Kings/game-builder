use tokio::task::JoinHandle;

use crate::Error;

pub async fn tokio_flatten<T>(handle: JoinHandle<Result<T, Error>>) -> Result<T, Error> {
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        err => err.unwrap(),
    }
}
