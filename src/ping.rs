use std::{
    net::{self, Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use socket2::{Domain, Protocol, Socket, Type};

use crate::Config;

#[derive(Clone)]
pub struct Pinger {
    config: Config,
    dest: SocketAddr,
    socket: Arc<Socket>,
}

impl Pinger {
    pub fn new(config: Config) -> anyhow::Result<Pinger> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))?;
        let src = SocketAddr::new(net::IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0);
        let dest = SocketAddr::new(config.destination.ip, 0);
        socket.bind(&src.into())?;
        socket.set_ttl(config.ttl)?;
        socket.set_read_timeout(Some(Duration::new(config.timeout, 0)))?;
        socket.set_write_timeout(Some(Duration::new(config.timeout, 0)))?;
        Ok(Self {
            config: config,
            dest: dest,
            socket: Arc::new(socket),
        })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        todo!()
    }
}
