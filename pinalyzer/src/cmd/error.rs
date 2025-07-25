use derive_more::{Display, From};
use rusqlite::ffi;

use crate::error::Internal;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    Module(crate::modules::Error),

    DB(rusqlite::Error),

    #[from]
    Format(crate::format::Error),

    #[from]
    Internal(Internal),

    #[from]
    Io(std::io::Error),

    #[from]
    Ip(std::net::AddrParseError),
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::SqliteFailure(
                ffi::Error {
                    code: ffi::ErrorCode::CannotOpen,
                    extended_code: _,
                },
                _,
            ) => Error::Internal(Internal::NoDatabaseFile),
            _ => Error::DB(value),
        }
    }
}

impl std::error::Error for Error {}
