use derive_more::From;
use serde::Serialize;
use std::fmt::{Debug};


pub type Result = std::result::Result<String, Error>;

pub trait Formatter: Serialize {
    
    fn json(&self) -> Result {
        let js = serde_json::to_string(self)?;
        Ok(js)
    }

    fn yaml(&self) -> Result {
        let yml = serde_yaml::to_string(self)?;
        Ok(yml)
    }

    fn table(&self) -> Result;

}

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Json(serde_json::Error),

    #[from]
    Yaml(serde_yaml::Error),

}
