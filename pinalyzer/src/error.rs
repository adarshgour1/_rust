use derive_more::{Display, From};
use prettytable::{Table, row};
use serde::Serialize;

use crate::format::Formatter;
pub type Result<T> = std::result::Result<T, Error>;

use crate::cmd::Error as CmdError;
use crate::modules::Error as ModuleError;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    #[display("Module error: {}", _0)]
    Module(ModuleError),

    #[from]
    Cmd(CmdError),

    #[from]
    Internal(Internal),
}

impl std::error::Error for Error {}

impl Formatter for Error {
    fn json(&self) -> crate::format::Result {
        Ok(format!(r#"{{"error":{}}}"#, self))
    }

    fn table(&self) -> crate::format::Result {
        let mut table = Table::new();
        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(row!["error"]);
        table.add_row(row![self]);
        Ok(table.to_string())
    }
}

#[derive(Debug, From, Display, Serialize)]
pub enum Internal {
    #[display("no data found")]
    NoDataFound,
    #[display("no database file found, run init command first")]
    NoDatabaseFile,
    #[display("internal error")]
    InternalError,
}

impl std::error::Error for Internal {}
