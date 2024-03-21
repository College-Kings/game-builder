use std::{any::Any, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    EnvVar(std::env::VarError),
    SerdeJson(serde_json::Error),
    OsString(std::ffi::OsString),
    Thread(Box<dyn Any + Send>),
    GameNotFound,
    InvalidPath(PathBuf),
}

// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

//     }
// }

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Reqwest(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::EnvVar(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}

impl From<std::ffi::OsString> for Error {
    fn from(error: std::ffi::OsString) -> Self {
        Error::OsString(error)
    }
}
