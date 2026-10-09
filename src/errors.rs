use std::fmt::Display;

use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O Error {0}")]
    IOError(#[from] std::io::Error),
    #[error("I/O Error {0}")]
    IOCoreError(#[from] iocore::Error),
    #[error("ID3 Error {0}")]
    ID3Error(#[from] id3::Error),
    #[error("Input Error {0}")]
    InputError(String),
    #[error("Logging Error {0}")]
    LoggingError(#[from] log::SetLoggerError),
    #[error("Logging Setup Error {0}")]
    FernInitError(#[from] fern::InitError),

    #[error("Runtime Error {0}")]
    RuntimeError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Exit {
    Success,
    Error(Error),
}
impl std::process::Termination for Exit {
    fn report(self) -> std::process::ExitCode {
        match &self {
            Exit::Success => std::process::ExitCode::from(0),
            Exit::Error(error) => {
                eprintln!("{}", error);
                std::process::ExitCode::from(1)
            }
        }
    }
}
impl<T> From<std::result::Result<T, Error>> for Exit {
    fn from(result: std::result::Result<T, Error>) -> Exit {
        match result {
            Ok(_) => Exit::Success,
            Err(e) => Exit::Error(e),
        }
    }
}
