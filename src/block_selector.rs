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
use crate::block_height::BlockHeight;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;
use twenty_first::prelude::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BlockSelectorLiteral {
    Genesis,
    Tip,
}


/// Provides alternatives for looking up a block.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum BlockSelector {
    Special(BlockSelectorLiteral),
    Digest(Digest),
    Height(BlockHeight),
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
            Self::Digest(d) => write!(f, "{}", d),
            Self::Height(h) => write!(f, "{}", h),
            Self::Special(BlockSelectorLiteral::Genesis) => write!(f, "genesis"),
            Self::Special(BlockSelectorLiteral::Tip) => write!(f, "tip"),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum BlockSelectorParseError {
    #[error("Invalid selector {0}. Try genesis or tip")]
    InvalidSelector(String),
}

impl FromStr for BlockSelector {
    type Err = BlockSelectorParseError;

    // note: this parses the output of impl Display for BlockSelector
    // note: this is used by clap parser in neptune-cli for block-info command
    //       and probably future commands as well.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "genesis" => Ok(Self::Special(BlockSelectorLiteral::Genesis)),
            "tip" => Ok(Self::Special(BlockSelectorLiteral::Tip)),
            _ => {
                if let Ok(d) = Digest::try_from_hex(s) {
                    Ok(Self::Digest(d))
                } else if let Ok(h) = s.parse::<u64>() {
                    Ok(Self::Height(h.into()))
                } else {
                    Err(BlockSelectorParseError::InvalidSelector(s.to_string()))
                }
            }
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
            BlockSelector::Special(BlockSelectorLiteral::Tip) => source.block_digest_for_tip(),
            BlockSelector::Special(BlockSelectorLiteral::Genesis) => source.block_digest_for_genesis(),
            BlockSelector::Digest(d) => Some(*d),
            BlockSelector::Height(h) => source.block_digest_for_height(*h),
        }
    }
}

pub trait BlockSelectorSource {
    fn block_digest_for_height(&self, height: BlockHeight) -> Option<Digest>;
    fn block_digest_for_tip(&self) -> Option<Digest>;
    fn block_digest_for_genesis(&self) -> Option<Digest>;
}

#[cfg(test)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(non_snake_case)]
mod generated_tests {
    use super::*;
    use crate::test_shared::*;
    use bincode;
    use serde::{Deserialize, Serialize};
    pub mod nc {
        pub use neptune_cash::protocol::consensus::block::block_selector::BlockSelector;
        pub use neptune_cash::protocol::consensus::block::block_selector::BlockSelectorLiteral;
    }
    #[test]
    fn test_bincode_serialization_for_block_selector() {
        let original_instance = BlockSelector::Special(BlockSelectorLiteral::Tip); 
        let nc_instance = nc::BlockSelector::Special(nc::BlockSelectorLiteral::Tip);
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_block_selector() {
        let original_instance = BlockSelector::Special(BlockSelectorLiteral::Tip);
        let nc_instance = nc::BlockSelector::Special(nc::BlockSelectorLiteral::Tip);
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_block_selector() {
        let original_instance = BlockSelector::Special(BlockSelectorLiteral::Tip);
        let nc_instance = nc::BlockSelector::Special(nc::BlockSelectorLiteral::Tip);
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
