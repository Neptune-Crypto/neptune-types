use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
use crate::lock_script::LockScriptAndWitness;
use crate::utxo::Utxo;
use crate::mutator_set::ms_membership_proof::MsMembershipProof;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockedUtxo {
    pub utxo: Utxo,
    lock_script_and_witness: LockScriptAndWitness,
    membership_proof: MsMembershipProof,
}

impl UnlockedUtxo {
    pub fn unlock(
        utxo: Utxo,
        lock_script_and_witness: LockScriptAndWitness,
        membership_proof: MsMembershipProof,
    ) -> Self {
        Self {
            utxo,
            lock_script_and_witness,
            membership_proof,
        }
    }
    /// Return the `item` from the perspective of the mutator set
    pub fn mutator_set_item(&self) -> Digest {
        Tip5::hash(&self.utxo)
    }
    pub fn mutator_set_mp(&self) -> &MsMembershipProof {
        &self.membership_proof
    }
    pub fn lock_script_and_witness(&self) -> &LockScriptAndWitness {
        &self.lock_script_and_witness
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
    pub mod nc {}
    #[test]
    fn test_bincode_serialization_for_unlocked_utxo() {
        let original_instance: UnlockedUtxo = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, None::<UnlockedUtxo>);
    }
    #[test]
    fn test_serde_json_serialization_for_unlocked_utxo() {
        let original_instance: UnlockedUtxo = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, None::<UnlockedUtxo>);
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_unlocked_utxo() {
        let original_instance: UnlockedUtxo = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            None::<UnlockedUtxo>,
        );
    }
}
