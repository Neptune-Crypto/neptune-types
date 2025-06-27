//! BlockSelector is a helper for querying blocks.
//!
//! The idea is to instantiate a BlockSelector using any of the following as
//! identifier:
//!  * A Digest
//!  * A BlockHeight
//!  * Genesis
//!  * Tip
//!
//! Then call BlockSelector::as_digest() to obtain the block's Digest, if it
//! exists.
//!
//! Public API's such as RPCs should accept a BlockSelector rather than a Digest
//! or Height.

use std::num::ParseIntError;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

use crate::block_height::BlockHeight;
// use crate::models::state::GlobalState;
use twenty_first::prelude::*;
use twenty_first::error::TryFromHexDigestError;
// use twenty_first::math::digest::Digest;

/// Provides alternatives for looking up a block.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlockSelector {
    Digest(Digest),      // Identifies block by Digest (hash)
    Height(BlockHeight), // Identifies block by Height (count from genesis)
    Genesis,             // Indicates the genesis block
    Tip,                 // Indicates the latest canonical block
}

/// BlockSelector can be written out as any of:
/// ```text
///  genesis
///  tip
///  height/<N>
///  digest/<hex>
/// ```
///
/// This is intended to be easy for humans to read and also input, ie suitable
/// for use as CLI argument.
impl std::fmt::Display for BlockSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digest(d) => write!(f, "digest/{}", d),
            Self::Height(h) => write!(f, "height/{}", h),
            Self::Genesis => write!(f, "genesis"),
            Self::Tip => write!(f, "tip"),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum BlockSelectorParseError {
    #[error("Invalid selector {0}.  Try genesis or tip")]
    InvalidSelector(String),

    #[error("Invalid pair selector {0}.  Try height/<N> or digest/<hex>")]
    InvalidPairSelector(String),

    #[error("Wrong selector length {0}.  (too many or too few '/')")]
    WrongSelectorLength(usize),

    #[error("Bad Digest")]
    BadDigest(#[from] TryFromHexDigestError),

    #[error("Bad Height")]
    BadHeight(#[from] ParseIntError),
}

impl FromStr for BlockSelector {
    type Err = BlockSelectorParseError;

    // note: this parses the output of impl Display for BlockSelector
    // note: this is used by clap parser in neptune-cli for block-info command
    //       and probably future commands as well.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 1 {
            match parts[0] {
                "genesis" => Ok(Self::Genesis),
                "tip" => Ok(Self::Tip),
                other => Err(BlockSelectorParseError::InvalidSelector(other.to_string())),
            }
        } else if parts.len() == 2 {
            match parts[0] {
                "digest" => Ok(Self::Digest(Digest::try_from_hex(parts[1])?)),
                "height" => Ok(Self::Height(parts[1].parse::<u64>()?.into())),
                other => Err(BlockSelectorParseError::InvalidPairSelector(
                    other.to_string(),
                )),
            }
        } else {
            Err(BlockSelectorParseError::WrongSelectorLength(parts.len()))
        }
    }
}

impl BlockSelector {

    pub async fn to_digest(&self) -> Option<Digest> {
        match self {
            BlockSelector::Digest(d) => Some(*d),
            _ => None,
        }
    }

    /// returns canonical chain block Digest for this selector, if it exists.
    ///
    /// note: if multiple blocks with same height are found only the digest
    /// of the block belonging to canonical chain is returned.
    pub async fn as_digest(&self, source: &impl BlockSelectorSource) -> Option<Digest> {
        match self {
            BlockSelector::Digest(d) => Some(*d),
            BlockSelector::Height(h) => source.block_digest_for_height(*h),
            BlockSelector::Genesis => source.block_digest_for_genesis(),
            BlockSelector::Tip => source.block_digest_for_tip(),
        }
    }
}

pub trait BlockSelectorSource {
    fn block_digest_for_height(&self, height: BlockHeight) -> Option<Digest>;
    fn block_digest_for_tip(&self) -> Option<Digest>;
    fn block_digest_for_genesis(&self) -> Option<Digest>;
}
