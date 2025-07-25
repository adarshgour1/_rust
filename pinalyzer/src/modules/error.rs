use derive_more::{Display, From};
use crate::error::Internal;

pub type Result<T> = std::result::Result<T, Error>;



#[derive(Debug, From, Display)]
pub enum Error {
    DB(rusqlite::Error),

    #[from]
    Ip(std::net::AddrParseError),

    #[from]
    Internal(Internal),
}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::QueryReturnedNoRows => Error::Internal(Internal::NoDataFound),
            _ => Error::DB(value),
        }
    }
}

impl std::error::Error for Error {}
