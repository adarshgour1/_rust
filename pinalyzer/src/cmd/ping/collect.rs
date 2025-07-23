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

use crate::modules::device::Device;
use crate::modules::ping::PingStats as ModulePingStats;
use crate::plugin::ping::Pinger;

#[derive(Debug, Args, PartialEq)]
#[group(required = true, multiple = false)]
pub struct Collect {
    /// Ping all devices
    #[arg(long, group = "devices_id")]
    all: bool,

    /// Ping devices by id
    #[arg(long, group = "devices_id")]
    ids: Option<Vec<i32>>,

    /// Ping frequency in seconds
    #[arg(long, default_value_t = 30)]
    frequency: u64,
}

impl Collect {
    pub fn execute(&self, conn: &Connection) -> crate::Result<()> {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Handle Ctrl+C gracefully
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down gracefully...");
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl+C handler");

        let devices = self.get_devices(conn)?;
        let pinger = Pinger::new(Duration::from_secs(1))?;

        while running.load(Ordering::SeqCst) {
            for device in &devices {
                if !running.load(Ordering::SeqCst) {
                    break;
                }
                let ip: Ipv4Addr = device.ipaddr.parse()?;
                let timestamp = chrono::Utc::now();
                let stats = pinger.ping(ip, 4)?;

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
                thread::sleep(Duration::from_secs(self.frequency));
            }
        }
        Ok(())
    }

    fn get_devices(&self, conn: &Connection) -> crate::Result<Vec<Device>> {
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
