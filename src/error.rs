use std::any::Any;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidGame(Box<str>),
    InvalidPath(std::path::PathBuf),
    VersionNotFound(Box<str>),

    Io(std::io::Error),
    EnvVar(std::env::VarError),
    OsString(std::ffi::OsString),
    Dotenvy(dotenvy::Error),
    Bunny(bunny_cdn_wrapper::Error),
    JoinError(tokio::task::JoinError),
    Thread(Box<dyn Any + Send>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Self {
        Error::EnvVar(error)
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
