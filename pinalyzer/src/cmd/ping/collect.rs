
use std::net::Ipv4Addr;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

use clap::Args;
use ctrlc;
use rusqlite::Connection;


use super::{Result};
use crate::modules::Device;
use crate::modules::PingStats as ModulePingStats;
use crate::plugin::Pinger;


// Command-line arguments for the 'collect' subcommand
#[derive(Debug, Args, PartialEq)]
pub struct Collect {
    /// Ping all devices
    #[arg(long, conflicts_with = "ids", required_unless_present = "ids")]
    all: bool,

    /// Ping devices by id
    #[arg(long, conflicts_with = "all", required_unless_present = "all")]
    ids: Option<Vec<i32>>,

    /// Ping frequency in seconds
    #[arg(long, default_value_t = 30)]
    frequency: u64,
}


impl Collect {
    // Execute the collect command: ping devices and store stats in the database
    pub fn execute(&self, conn: &Connection) -> Result<()> {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Handle Ctrl+C gracefully to allow clean shutdown
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down gracefully...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        let devices = self.get_devices(conn)?; // Get list of devices to ping
        let pinger = Pinger::new(Duration::from_secs(1))?; // Create a pinger with 1s timeout

        while running.load(Ordering::SeqCst) {
            for device in &devices {
                if !running.load(Ordering::SeqCst) {
                    break;
                }
                let ip: Ipv4Addr = device.ipaddr.parse()?; // Parse device IP address
                let timestamp = chrono::Utc::now(); // Get current timestamp
                let stats = pinger.ping(ip, 4)?; // Ping the device 4 times

                // Create a PingStats record and save it to the database
                let ping_stats = ModulePingStats::new(
                    timestamp.to_rfc3339(),
                    device.id.unwrap(),
                    stats.transmitted,
                    stats.received,
                    stats.loss,
                    stats.min,
                    stats.max,
                    stats.avg,
                );

                ping_stats.save(conn)?;
                println!("{}", ping_stats);
            }
            thread::sleep(Duration::from_secs(self.frequency)); // Wait before next ping
        }
        Ok(())
    }

    // Get the list of devices to ping, filtered by IDs if provided
    fn get_devices(&self, conn: &Connection) -> Result<Vec<Device>> {
        let devices = Device::get_all(conn)?;

        if let Some(ref ids) = self.ids {
            let filtered_devices: Vec<_> = devices
                .into_iter()
                .filter(|d| d.id.map_or(false, |id| ids.contains(&id)))
                .collect();
            Ok(filtered_devices)
        } else {
            Ok(devices)
        }
    }
}
