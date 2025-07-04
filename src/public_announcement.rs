use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
/// represents arbitrary data that can be stored in a transaction on the public blockchain
///
/// initially these are used for transmitting encrypted secrets necessary
/// for a utxo recipient to identify and claim it.
///
/// See [Transaction]
#[derive(
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    GetSize,
    BFieldCodec,
    Default
)]
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
pub struct PublicAnnouncement {
    pub message: Vec<BFieldElement>,
}

impl PublicAnnouncement {
    pub fn new(message: Vec<BFieldElement>) -> Self {
        Self { message }
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
    use serde::{Serialize, Deserialize};
    pub mod nc {
        pub use neptune_cash::models::blockchain::transaction::PublicAnnouncement;
    }
    #[test]
    fn test_bincode_serialization_for_public_announcement() {
        let original_instance: PublicAnnouncement = PublicAnnouncement::default();
        let nc_instance: nc::PublicAnnouncement = neptune_cash::models::blockchain::transaction::PublicAnnouncement::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_public_announcement() {
        let original_instance: PublicAnnouncement = PublicAnnouncement::default();
        let nc_instance: nc::PublicAnnouncement = neptune_cash::models::blockchain::transaction::PublicAnnouncement::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_public_announcement() {
        let original_instance: PublicAnnouncement = PublicAnnouncement::default();
        let nc_instance: nc::PublicAnnouncement = neptune_cash::models::blockchain::transaction::PublicAnnouncement::default();
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
}
