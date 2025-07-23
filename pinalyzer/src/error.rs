use std::net::AddrParseError;

use derive_more::{Display, From};
use prettytable::{Table, row};
use rusqlite::ffi;
use serde::Serialize;

use crate::format::Formatter;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    DB(rusqlite::Error),

    #[from]
    Formatter(crate::format::Error),

    #[from]
    Io(std::io::Error),

    #[from]
    Ip(AddrParseError),
}

impl std::error::Error for Error {}

// --------------------------------------------------- ConsoleDisplayError -----------------------------------------------------
// This will be displayed as console error
#[derive(Debug, From, Display, Serialize)]
pub enum ConsoleDisplayError {
    #[display("Application is not initialize, run init command")]
    NoDatabaseExists,

    #[display("no data found")]
    NoDataFound,

    #[display("internal error")]
    Internal,

    Ip(String)
}
impl std::error::Error for ConsoleDisplayError {}

impl From<self::Error> for ConsoleDisplayError {
    fn from(value: self::Error) -> Self {
        match value {
            Error::DB(rusqlite::Error::SqliteFailure(
                ffi::Error {
                    code: ffi::ErrorCode::CannotOpen,
                    extended_code: _,
                },
                _,
            )) => Self::NoDatabaseExists,

            Error::DB(rusqlite::Error::QueryReturnedNoRows) => Self::NoDataFound,
            Error::Ip(e) => Self::Ip(e.to_string()),
            _ => Self::Internal,
        }
    }
}

impl Formatter for ConsoleDisplayError {
    fn json(&self) -> crate::format::Result {
        Ok(format!(r#"{{"error":{}}}"#, serde_json::to_string(self)?))
    }

    fn table(&self) -> crate::format::Result {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["error"]);
        table.add_row(row![self]);
        Ok(table.to_string())
    }
}
