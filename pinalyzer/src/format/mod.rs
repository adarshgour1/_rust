mod error;
use clap::ValueEnum;

// reexporting Result for convenience
pub use error::{Error, Result};

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Json,
    Table,
}

pub trait Formatter {
    fn json(&self) -> Result;

    fn table(&self) -> Result;

    fn parse(&self, format: &Format) -> Result {
        match format {
            Format::Json => self.json(),
            Format::Table => self.table(),
        }
    }
}
