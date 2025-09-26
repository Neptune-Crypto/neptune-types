use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::Digest;
// use tasm_lib::prelude::TasmObject;
use twenty_first::math::bfield_codec::BFieldCodec;

#[derive(
    // Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, BFieldCodec, TasmObject, GetSize,
    Copy,
    Clone,
    Debug,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    BFieldCodec,
    GetSize,
)]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary, Default)
)]
pub struct GuesserReceiverData {
    pub receiver_digest: Digest,
    pub lock_script_hash: Digest,
}

/* private in neptune-cash

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
        pub use neptune_cash::models::blockchain::block::guesser_receiver_data::GuesserReceiverData;
    }

    #[test]
    fn test_bincode_serialization_for_guesserreceiverdata() {
        let original_instance: GuesserReceiverData = GuesserReceiverData::default();
        let nc_instance = nc::GuesserReceiverData::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_guesserreceiverdata() {
        let original_instance: GuesserReceiverData = GuesserReceiverData::default();
        let nc_instance = nc::GuesserReceiverData::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_guesserreceiverdata() {
        let original_instance: GuesserReceiverData = GuesserReceiverData::default();
        let nc_instance = nc::GuesserReceiverData::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

}
*/