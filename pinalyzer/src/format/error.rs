use derive_more::{Display, From};
use std::fmt::Debug;

pub type Result = std::result::Result<String, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[from]
    Json(serde_json::Error),

}

impl std::error::Error for Error {}

