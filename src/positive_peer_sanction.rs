use crate::sanction::Sanction;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// The reason for improving a peer's standing
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PositivePeerSanction {
    // positive sanctions (standing-improving)
    // We only reward events that are unlikely to occur more frequently than the
    // target block frequency. This should make it impossible for an attacker
    // to quickly ramp up their standing with peers, provided that they are on
    // the global tip.
    ValidBlocks(usize),
    NewBlockProposal,
}

impl Display for PositivePeerSanction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            PositivePeerSanction::ValidBlocks(_) => "valid blocks",
            PositivePeerSanction::NewBlockProposal => "new block proposal",
        };
        write!(f, "{string}")
    }
}

impl Sanction for PositivePeerSanction {
    fn severity(self) -> i32 {
        match self {
            PositivePeerSanction::ValidBlocks(number) => number
                .try_into()
                .map(|n: i32| n.saturating_mul(10))
                .unwrap_or(i32::MAX),
            PositivePeerSanction::NewBlockProposal => 7,
        }
    }
}

