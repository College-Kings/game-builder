use std::any::Any;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    EnvVar(std::env::VarError),
    SerdeJson(serde_json::Error),
    OsString(std::ffi::OsString),
    Dotenvy(dotenvy::Error),
    Bunny(bunny_cdn_wrapper::Error),
    JoinError(tokio::task::JoinError),
    Thread(Box<dyn Any + Send>),
    GameNotFound,
    InvalidPath(std::path::PathBuf),
    VersionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "IO error: {}", error),
            Error::Reqwest(error) => write!(f, "Reqwest error: {}", error),
            Error::EnvVar(error) => write!(f, "Environment variable error: {}", error),
            Error::SerdeJson(error) => write!(f, "Serde JSON error: {}", error),
            Error::OsString(error) => write!(f, "OsString error: {:?}", error),
            Error::Dotenvy(error) => write!(f, "Dotenvy error: {}", error),
            Error::Bunny(error) => write!(f, "Bunny error: {}", error),
            Error::JoinError(error) => write!(f, "Join error: {}", error),
            Error::Thread(_) => write!(f, "Thread error"),
            Error::GameNotFound => write!(f, "Game not found"),
            Error::InvalidPath(path) => write!(f, "Invalid path: {:?}", path),
            Error::VersionNotFound => write!(f, "Version not found"),
        }
    }
}

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

impl From<dotenvy::Error> for Error {
    fn from(error: dotenvy::Error) -> Self {
        Error::Dotenvy(error)
    }
}

impl From<bunny_cdn_wrapper::Error> for Error {
    fn from(error: bunny_cdn_wrapper::Error) -> Self {
        Error::Bunny(error)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Self {
        Error::JoinError(error)
    }
}
