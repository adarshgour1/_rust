mod error;
use clap::ValueEnum;
pub use error::{Error, Result};

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Table,
}

pub trait Formatter {
    fn json(&self) -> Result;

    fn yaml(&self) -> Result;

    fn table(&self) -> Result;

    fn parse(&self, format: &Format) -> Result {
        match format {
            Format::Json => self.json(),
            Format::Yaml => self.yaml(),
            Format::Table => self.table(),
        }
    }
}
