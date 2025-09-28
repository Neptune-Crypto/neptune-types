use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::positive_peer_sanction::PositivePeerSanction;
use crate::negative_peer_sanction::NegativePeerSanction;
use crate::sanction::Sanction;


/// The reason for changing a peer's standing.
///
/// Sanctions can be positive (rewards) or negative (punishments).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PeerSanction {
    Positive(PositivePeerSanction),
    Negative(NegativePeerSanction),
}

impl Sanction for PeerSanction {
    fn severity(self) -> i32 {
        match self {
            PeerSanction::Positive(positive_peer_sanction) => positive_peer_sanction.severity(),
            PeerSanction::Negative(negative_peer_sanction) => negative_peer_sanction.severity(),
        }
    }
}

impl Display for PeerSanction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeerSanction::Positive(positive_peer_sanction) => write!(f, "{positive_peer_sanction}"),
            PeerSanction::Negative(negative_peer_sanction) => write!(f, "{negative_peer_sanction}"),
        }
    }
}
