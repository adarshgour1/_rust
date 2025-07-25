pub mod device;
pub mod error;
pub mod init;
pub mod ping;

pub use error::{Error, Result};
pub use device::Device;
pub use ping::PingStats;