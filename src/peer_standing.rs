use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::peer_sanction::PeerSanction;
use crate::negative_peer_sanction::NegativePeerSanction;
use crate::positive_peer_sanction::PositivePeerSanction;
use crate::sanction::Sanction;

#[cfg(target_arch = "wasm32")]
use web_time::SystemTime;

#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;


/// This is the object that gets stored in the database to record how well a
/// peer has behaved so far.
//
// The most central methods are [PeerStanding::sanction] and
// [PeerStanding::is_bad].
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PeerStanding {
    /// The actual standing. The higher, the better.
    pub standing: i32,
    pub latest_punishment: Option<(NegativePeerSanction, SystemTime)>,
    pub latest_reward: Option<(PositivePeerSanction, SystemTime)>,
    peer_tolerance: i32,
}
#[derive(Debug, Clone, Copy, Default)]
pub struct StandingExceedsBanThreshold;

impl Display for StandingExceedsBanThreshold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "standing exceeds ban threshold")
    }
}

impl std::error::Error for StandingExceedsBanThreshold {}

impl PeerStanding {
    pub fn new(peer_tolerance: u16) -> Self {
        assert!(peer_tolerance > 0, "peer tolerance must be positive");
        Self {
            peer_tolerance: i32::from(peer_tolerance),
            standing: 0,
            latest_punishment: None,
            latest_reward: None,
        }
    }

    /// Sanction peer. If (and only if) the peer is now in
    /// [bad standing](Self::is_bad), returns an error.
    pub fn sanction(
        &mut self,
        sanction: PeerSanction,
    ) -> Result<(), StandingExceedsBanThreshold> {
        self.standing = self
            .standing
            .saturating_add(sanction.severity())
            .clamp(-self.peer_tolerance, self.peer_tolerance);
/*
        trace!(
            "new standing: {}, peer tolerance: {}",
            self.standing,
            self.peer_tolerance
        );
*/
        let now = SystemTime::now();
        match sanction {
            PeerSanction::Negative(sanction) => self.latest_punishment = Some((sanction, now)),
            PeerSanction::Positive(sanction) => self.latest_reward = Some((sanction, now)),
        }

        self.is_good()
            .then_some(())
            .ok_or(StandingExceedsBanThreshold)
    }

    /// Clear peer standing record
    pub fn clear_standing(&mut self) {
        self.standing = 0;
        self.latest_punishment = None;
        self.latest_reward = None;
    }

    pub fn is_negative(&self) -> bool {
        self.standing.is_negative()
    }

    pub fn is_bad(&self) -> bool {
        self.standing <= -self.peer_tolerance
    }

    pub fn is_good(&self) -> bool {
        !self.is_bad()
    }
}

impl Display for PeerStanding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.standing)
    }
}
