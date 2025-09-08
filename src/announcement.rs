use std::fmt::Display;
use std::fmt::LowerHex;
use std::num::ParseIntError;

use get_size2::GetSize;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;

#[cfg(feature = "tasm-object")]
use tasm_lib::prelude::TasmObject;

/// Represents arbitrary data that can be stored in a transaction on the public
/// blockchain.
///
/// These are typically used for transmitting encrypted UTXO notifications, so
/// that a recipient can identify and claim the UTXO.
///
/// See [Transaction](super::Transaction).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, GetSize, BFieldCodec, Default)]
#[cfg_attr(any(test, feature = "arbitrary-impls"), derive(arbitrary::Arbitrary))]
#[cfg_attr(feature = "tasm-object", derive(TasmObject))]
pub struct Announcement {
    pub message: Vec<BFieldElement>,
}

impl Announcement {
    pub fn new(message: Vec<BFieldElement>) -> Self {
        Self { message }
    }
}

impl LowerHex for Announcement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.message {
            // big-endian (Arabic)
            write!(f, "{:016x}", m.value())?;
        }
        Ok(())
    }
}

impl Display for Announcement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // add hex delimiter, then use hex formatter
        write!(f, "0x{:x}", self)
    }
}

#[derive(Debug, Clone)]
pub enum ParsePublicAnnouncementError {
    TooShort,
    BadHexDelimiter,
    BadLengthAlignment,
    ParseIntError(ParseIntError),
    NonCanonicalRepresentation,
}

impl TryFrom<String> for Announcement {
    type Error = ParsePublicAnnouncementError;

    fn try_from(unparsed: String) -> Result<Self, Self::Error> {
        const BFE_HEX_LEN: usize = 16;
        let (delimiter, payload) = unparsed
            .split_at_checked(2)
            .ok_or(ParsePublicAnnouncementError::TooShort)?;

        let _hex_delimiter_is_valid = (delimiter == "0x")
            .then_some(true)
            .ok_or(ParsePublicAnnouncementError::BadHexDelimiter)?;

        let _payload_length_aligns_with_bfes = payload
            .len()
            .is_multiple_of(BFE_HEX_LEN)
            .then_some(true)
            .ok_or(ParsePublicAnnouncementError::BadLengthAlignment)?;

        let mut bfes = vec![];
        for chunk in &payload.chars().chunks(BFE_HEX_LEN) {
            let substring: String = chunk.collect();
            let representant = u64::from_str_radix(&substring, 16)
                .map_err(ParsePublicAnnouncementError::ParseIntError)?;

            let _representation_is_canonical = (representant <= BFieldElement::MAX)
                .then_some(true)
                .ok_or(ParsePublicAnnouncementError::NonCanonicalRepresentation)?;
            bfes.push(BFieldElement::new(representant));
        }

        Ok(Self { message: bfes })
    }
}

#[cfg(all(test, feature = "original-tests"))]
#[cfg(test)]
mod tests {
    use proptest::prop_assert_eq;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;

    use super::*;

    #[proptest]
    fn try_from_string_inverts_display_format(#[strategy(arb())] announcement: Announcement) {
        let as_hex = format!("{}", announcement);
        let as_announcement_again = Announcement::try_from(as_hex).unwrap();
        prop_assert_eq!(announcement, as_announcement_again);
    }

    #[proptest]
    fn try_from_string_cannot_crash(s: String) {
        let _announcement = Announcement::try_from(s); // no crash
    }
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
        pub use neptune_cash::models::blockchain::transaction::announcement::Announcement;
    }
    #[test]
    fn test_bincode_serialization_for_announcement() {
        let original_instance: Announcement = Announcement::default();
        let nc_instance: nc::Announcement = nc::Announcement::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_public_announcement() {
        let original_instance: Announcement = Announcement::default();
        let nc_instance: nc::Announcement = nc::Announcement::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_public_announcement() {
        let original_instance: Announcement = Announcement::default();
        let nc_instance: nc::Announcement = nc::Announcement::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
