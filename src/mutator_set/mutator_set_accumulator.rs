use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
use twenty_first::util_types::mmr::mmr_accumulator::MmrAccumulator;
use super::active_window::ActiveWindow;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, GetSize, BFieldCodec)]
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
#[cfg_attr(feature = "tasm-lib", derive(tasm_lib::prelude::TasmObject))]
pub struct MutatorSetAccumulator {
    pub aocl: MmrAccumulator,
    pub swbf_inactive: MmrAccumulator,
    pub swbf_active: ActiveWindow,
}

impl Default for MutatorSetAccumulator {
    fn default() -> Self {
        Self {
            aocl: MmrAccumulator::new_from_leafs(vec![]),
            swbf_inactive: MmrAccumulator::new_from_leafs(vec![]),
            swbf_active: Default::default(),
        }
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
        pub use neptune_cash::util_types::mutator_set::mutator_set_accumulator::MutatorSetAccumulator;
    }
    #[test]
    fn test_bincode_serialization_for_mutator_set_accumulator() {
        let original_instance: MutatorSetAccumulator = todo!("Instantiate");
        let nc_instance: nc::MutatorSetAccumulator = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_mutator_set_accumulator() {
        let original_instance: MutatorSetAccumulator = todo!("Instantiate");
        let nc_instance: nc::MutatorSetAccumulator = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_mutator_set_accumulator() {
        let original_instance: MutatorSetAccumulator = todo!("Instantiate");
        let nc_instance: nc::MutatorSetAccumulator = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
}
