use derive_more::From;
use std::fmt::Debug;

pub type Result = std::result::Result<String, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Json(serde_json::Error),

    #[from]
    Yaml(serde_yaml::Error),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}