use std::net::{self, ToSocketAddrs};

use anyhow::bail;

use crate::PingError;

#[derive(Debug, Clone)]
pub struct Config {
    pub count: u16,
    pub packet_size: usize,
    pub ttl: u32,
    pub timeout: u64,
    pub interval: u64,
    pub id: u16,
    pub sequence: u16,
    pub destination: Address,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub ip: net::IpAddr,
    pub raw: String,
}

impl Address {
    pub fn parse(host: &str) -> anyhow::Result<Address> {
        let raw = String::from(host);
        match host.parse::<net::IpAddr>().ok() {
            Some(ip) => Ok(Address { ip, raw }),
            None => {
                let new = format!("{}:{}", host, 0);
                let mut addrs = new.to_socket_addrs()?;
                if let Some(addrs) = addrs.next() {
                    Ok(Address {
                        ip: addrs.ip(),
                        raw,
                    })
                } else {
                    bail!(PingError::InvalidConfig(String::from("address")))
                }
            }
        }
    }
}
