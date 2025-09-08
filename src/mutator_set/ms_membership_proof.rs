use super::chunk_dictionary::ChunkDictionary;
use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use twenty_first::prelude::*;
impl Error for MembershipProofError {}

impl fmt::Display for MembershipProofError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MembershipProofError {
    AlreadyExistingChunk(u64),
    MissingChunkOnUpdateFromAdd(u64),
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, GetSize, BFieldCodec)]
#[cfg_attr(feature = "tasm-lib", derive(tasm_lib::prelude::TasmObject))]
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
pub struct MsMembershipProof {
    pub sender_randomness: Digest,
    pub receiver_preimage: Digest,
    pub auth_path_aocl: MmrMembershipProof,
    pub aocl_leaf_index: u64,
    pub target_chunks: ChunkDictionary,
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
        pub use neptune_cash::util_types::mutator_set::ms_membership_proof::MsMembershipProof;
    }
    #[test]
    fn test_bincode_serialization_for_ms_membership_proof() {
        let original_instance: MsMembershipProof = todo!("Instantiate");
        let nc_instance: nc::MsMembershipProof = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_ms_membership_proof() {
        let original_instance: MsMembershipProof = todo!("Instantiate");
        let nc_instance: nc::MsMembershipProof = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_ms_membership_proof() {
        let original_instance: MsMembershipProof = todo!("Instantiate");
        let nc_instance: nc::MsMembershipProof = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
