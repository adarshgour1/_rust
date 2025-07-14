use derive_more::Display;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display)]
pub enum Error {
    #[display("internal error")]
    DB(rusqlite::Error),

    #[display("no data found")]
    NoDataFound,
}

impl std::error::Error for Error {}

impl From<rusqlite::Error> for Error {
    fn from(value: rusqlite::Error) -> Self {
        match value {
            rusqlite::Error::QueryReturnedNoRows => Self::NoDataFound,
            x => Self::DB(x),
        }
    }
}
