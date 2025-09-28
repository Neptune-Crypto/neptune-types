#[cfg(target_arch = "wasm32")]
use web_time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

use std::net::SocketAddr;

use serde::Deserialize;
use serde::Serialize;

use crate::peer_connection_info::PeerConnectionInfo;
use crate::peer_standing::PeerStanding;
use crate::handshake_data::HandshakeData;

pub type InstanceId = u128;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeerInfo {
    peer_connection_info: PeerConnectionInfo,
    instance_id: InstanceId,
    pub own_timestamp_connection_established: SystemTime,
    pub peer_timestamp_connection_established: SystemTime,
    pub standing: PeerStanding,
    version: String,
    is_archival_node: bool,
}

impl PeerInfo {
    pub fn new(
        peer_connection_info: PeerConnectionInfo,
        peer_handshake: &HandshakeData,
        connection_established: SystemTime,
        peer_tolerance: u16,
    ) -> Self {
        assert!(peer_tolerance > 0, "Peer tolerance must be positive");
        let standing = PeerStanding::new(peer_tolerance);
        Self {
            peer_connection_info,
            instance_id: peer_handshake.instance_id,
            own_timestamp_connection_established: connection_established,
            peer_timestamp_connection_established: peer_handshake.timestamp,
            standing,
            version: peer_handshake.version.to_string(),
            is_archival_node: peer_handshake.is_archival_node,
        }
    }

    /// Infallible absolute difference between two timestamps, in seconds.
    fn system_time_diff_seconds(peer: SystemTime, own: SystemTime) -> i128 {
        let peer = peer
            .duration_since(UNIX_EPOCH)
            .map(|d| i128::from(d.as_secs()))
            .unwrap_or_else(|e| -i128::from(e.duration().as_secs()));

        let own = own
            .duration_since(UNIX_EPOCH)
            .map(|d| i128::from(d.as_secs()))
            .unwrap_or_else(|e| -i128::from(e.duration().as_secs()));

        own - peer
    }

    /// Return the difference in time as reported by peer and client in seconds.
    /// The returned value is `peer clock - own clock`. So the amount of time
    /// that the connected peer is ahead of this client's clock. Negative value
    /// if peer clock is behind our clock.
    pub fn time_difference_in_seconds(&self) -> i128 {
        Self::system_time_diff_seconds(
            self.peer_timestamp_connection_established,
            self.own_timestamp_connection_established,
        )
    }

    pub fn with_standing(mut self, standing: PeerStanding) -> Self {
        self.standing = standing;
        self
    }

    pub fn instance_id(&self) -> u128 {
        self.instance_id
    }

    pub fn standing(&self) -> PeerStanding {
        self.standing
    }

    pub fn connected_address(&self) -> SocketAddr {
        self.peer_connection_info.connected_address
    }

    pub fn connection_established(&self) -> SystemTime {
        self.own_timestamp_connection_established
    }

    pub fn is_archival_node(&self) -> bool {
        self.is_archival_node
    }

    pub fn connection_is_inbound(&self) -> bool {
        self.peer_connection_info.inbound
    }

    pub fn connection_is_outbound(&self) -> bool {
        !self.connection_is_inbound()
    }

    /// returns the neptune-core version-string reported by the peer.
    ///
    /// note: the peer might not be honest.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return the socket address that the peer is expected to listen on. Returns `None` if peer does not accept
    /// incoming connections.
    pub fn listen_address(&self) -> Option<SocketAddr> {
        self.peer_connection_info
            .port_for_incoming_connections
            .map(|port| SocketAddr::new(self.peer_connection_info.connected_address.ip(), port))
    }

    #[cfg(all(test, feature = "original-tests"))]
    pub(crate) fn set_connection_established(&mut self, new_timestamp: SystemTime) {
        self.own_timestamp_connection_established = new_timestamp;
    }
}

#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use test_strategy::proptest;

    use super::*;

    #[test]
    fn time_difference_in_seconds_simple() {
        let now = SystemTime::now();
        let and_now = SystemTime::now();
        assert!(PeerInfo::system_time_diff_seconds(now, and_now) < 10);
    }

    #[proptest]
    fn time_difference_doesnt_crash(now: SystemTime, and_now: SystemTime) {
        PeerInfo::system_time_diff_seconds(now, and_now);
    }
}
