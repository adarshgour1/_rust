use std::net::Ipv4Addr;
use std::time::{Duration, Instant};
use std::{io, mem};

use libc::{
    AF_INET, IPPROTO_ICMP, SO_RCVTIMEO, SOCK_RAW, SOL_SOCKET, recvfrom, sendto, setsockopt, socket,
};

const ICMP_ECHO_REQUEST: u8 = 8;

#[repr(C, packed)]
struct IcmpHeader {
    type_: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    sequence_number: u16,
}

fn checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut chunks = data.chunks_exact(2);
    for chunk in &mut chunks {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
        sum += word;
    }
    if let Some([last]) = chunks.remainder().first().map(|b| [*b]) {
        sum += (u16::from_be_bytes([last, 0])) as u32;
    }
    while (sum >> 16) != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

fn create_icmp_packet(seq: u16, id: u16) -> Vec<u8> {
    let payload = b"PING";
    let mut packet = Vec::with_capacity(8 + payload.len());

    let mut header = IcmpHeader {
        type_: ICMP_ECHO_REQUEST,
        code: 0,
        checksum: 0,
        identifier: id,
        sequence_number: seq,
    };

    let header_bytes =
        unsafe { std::slice::from_raw_parts((&header as *const IcmpHeader) as *const u8, 8) };

    packet.extend_from_slice(header_bytes);
    packet.extend_from_slice(payload);

    // Recalculate checksum
    let cksum = checksum(&packet);
    header.checksum = cksum;
    let header_bytes =
        unsafe { std::slice::from_raw_parts((&header as *const IcmpHeader) as *const u8, 8) };

    packet.splice(..8, header_bytes.iter().cloned());
    packet
}

pub struct PingStats {
    pub transmitted: u32,
    pub received: u32,
    pub loss: f32,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub avg: Option<f64>,
}

pub struct Pinger {
    sock: i32,
    
    #[allow(dead_code)]
    timeout: Duration,
    id: u16,
}

impl Pinger {
    pub fn new(timeout: Duration) -> io::Result<Self> {
        let sock = unsafe { socket(AF_INET, SOCK_RAW, IPPROTO_ICMP) };
        if sock < 0 {
            return Err(io::Error::last_os_error());
        }
        let timeval = libc::timeval {
            tv_sec: timeout.as_secs() as i64,
            tv_usec: 0,
        };
        unsafe {
            setsockopt(
                sock,
                SOL_SOCKET,
                SO_RCVTIMEO,
                &timeval as *const _ as *const _,
                mem::size_of_val(&timeval) as u32,
            );
        }
        Ok(Pinger {
            sock,
            timeout,
            id: std::process::id() as u16,
        })
    }

    pub fn ping(&self, target_ip: Ipv4Addr, count: u16) -> io::Result<PingStats> {
        let mut transmitted = 0;
        let mut received = 0;
        let mut rtts = Vec::new();

        let addr = libc::sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: 0,
            sin_addr: libc::in_addr {
                s_addr: u32::from(target_ip).to_be(),
            },
            sin_zero: [0; 8],
        };

        for seq in 1..=count {
            let packet = create_icmp_packet(seq, self.id);
            let start = Instant::now();

            let sent = unsafe {
                sendto(
                    self.sock,
                    packet.as_ptr() as *const _,
                    packet.len(),
                    0,
                    &addr as *const _ as *const _,
                    mem::size_of_val(&addr) as u32,
                )
            };
            transmitted += 1;
            if sent < 0 {
                eprintln!("Send failed: {}", io::Error::last_os_error());
                continue;
            }

            let mut buf = [0u8; 1024];
            let recv_len = unsafe {
                recvfrom(
                    self.sock,
                    buf.as_mut_ptr() as *mut _,
                    buf.len(),
                    0,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            };

            if recv_len < 0 {
                // eprintln!("Timeout or receive error for seq {}.", seq);
                continue;
            }

            let elapsed = start.elapsed();
            let ms = elapsed.as_secs_f64() * 1000.0;
            rtts.push(ms);
            received += 1;
            // println!("Reply from {}: seq={} time={:.2} ms", target_ip, seq, ms);
        }

        let loss = if transmitted > 0 {
            100.0 * (transmitted - received) as f32 / transmitted as f32
        } else {
            0.0
        };

        let mut min = None;
        let mut max = None;
        let mut sum = 0.0;

        for &rtt in &rtts {
            min = Some(min.map_or(rtt, |m: f64| m.min(rtt)));
            max = Some(max.map_or(rtt, |m: f64| m.max(rtt)));
            sum += rtt;
        }
        let avg = if !rtts.is_empty() {
            Some(sum / rtts.len() as f64)
        } else {
            None
        };
        
        Ok(PingStats {
            transmitted,
            received,
            loss,
            min,
            max,
            avg,
        })
    }
}
