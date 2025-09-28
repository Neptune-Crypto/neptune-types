use std::net::SocketAddr;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeerConnectionInfo {
    pub port_for_incoming_connections: Option<u16>,
    pub connected_address: SocketAddr,
    pub inbound: bool,
}

impl PeerConnectionInfo {
    pub fn new(
        port_for_incoming_connections: Option<u16>,
        connected_address: SocketAddr,
        inbound: bool,
    ) -> Self {
        Self {
            port_for_incoming_connections,
            connected_address,
            inbound,
        }
    }
}

