use crate::triton_vm::nondeterminism::NonDeterminism;
///# [cfg (any (test , feature = "arbitrary-impls"))]
#[cfg(any(all(test, feature = "original-tests"), feature = "arbitrary-impls"))]
use arbitrary::Arbitrary;
use get_size2::GetSize;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use triton_isa::instruction::LabelledInstruction;
use triton_isa::program::Program;
use triton_isa::triton_asm;
use triton_isa::triton_instr;
use triton_isa::triton_program;
use twenty_first::prelude::*;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, GetSize, BFieldCodec)]
pub struct LockScript {
    pub program: Program,
}

impl From<Vec<LabelledInstruction>> for LockScript {
    fn from(instrs: Vec<LabelledInstruction>) -> Self {
        Self {
            program: Program::new(&instrs),
        }
    }
}

impl From<&[LabelledInstruction]> for LockScript {
    fn from(instrs: &[LabelledInstruction]) -> Self {
        Self {
            program: Program::new(instrs),
        }
    }
}

impl LockScript {
    const BURN_ERROR: i128 = 1_000_300;
    pub fn new(program: Program) -> Self {
        Self { program }
    }
    pub fn anyone_can_spend() -> Self {
        Self {
            program: Program::new(&triton_asm!(read_io 5 halt)),
        }
    }
    pub fn hash(&self) -> Digest {
        self.program.hash()
    }

    /// Generate a lock script that verifies knowledge of a hash preimage, given
    /// the after-image. This type of lock script is called "standard hash
    /// lock".
    ///
    /// Satisfaction of this lock script establishes the UTXO owner's assent to
    /// the transaction.
    pub fn standard_hash_lock_from_after_image(after_image: Digest) -> LockScript {
        let push_spending_lock_digest_to_stack = after_image
            .values()
            .iter()
            .rev()
            .map(|elem| triton_instr!(push elem.value()))
            .collect_vec();

        let instructions = triton_asm!(
            divine 5
            hash
            {&push_spending_lock_digest_to_stack}
            assert_vector
            read_io 5
            halt
        );

        instructions.into()
    }

    /// A lock script that is guaranteed to fail
    pub fn burn() -> Self {
        Self {
            program: triton_program! {
                push 0 assert error_id { Self::BURN_ERROR }
            },
        }
    }
}
///# [cfg (any (test , feature = "arbitrary-impls"))]
#[cfg(any(all(test, feature = "original-tests"), feature = "arbitrary-impls"))]
impl<'a> Arbitrary<'a> for LockScript {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let program = Program::arbitrary(u)?;
        Ok(LockScript { program })
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, GetSize, BFieldCodec)]
pub struct LockScriptAndWitness {
    pub program: Program,
    nd_memory: Vec<(BFieldElement, BFieldElement)>,
    nd_tokens: Vec<BFieldElement>,
    nd_digests: Vec<Digest>,
}

impl From<LockScriptAndWitness> for LockScript {
    fn from(lock_script_and_witness: LockScriptAndWitness) -> Self {
        Self {
            program: lock_script_and_witness.program,
        }
    }
}

impl From<&LockScriptAndWitness> for LockScript {
    fn from(lock_script_and_witness: &LockScriptAndWitness) -> Self {
        Self {
            program: lock_script_and_witness.program.clone(),
        }
    }
}

impl LockScriptAndWitness {
    pub fn new_with_nondeterminism(program: Program, witness: NonDeterminism) -> Self {
        Self {
            program,
            nd_memory: witness.ram.into_iter().collect(),
            nd_tokens: witness.individual_tokens,
            nd_digests: witness.digests,
        }
    }

    /// Create a [`LockScriptAndWitness`] whose lock script is a standard hash
    /// lock, from the preimage.
    pub(crate) fn standard_hash_lock_from_preimage(preimage: Digest) -> LockScriptAndWitness {
        let after_image = preimage.hash();
        let lock_script = LockScript::standard_hash_lock_from_after_image(after_image);
        LockScriptAndWitness::new_with_nondeterminism(
            lock_script.program,
            NonDeterminism::new(preimage.reversed().values()),
        )
    }

    ///# [cfg (test)]
    #[cfg(all(test, feature = "original-tests"))]
    pub(crate) fn set_nd_tokens(&mut self, tokens: Vec<BFieldElement>) {
        self.nd_tokens = tokens;
    }
    pub fn new(program: Program) -> Self {
        Self {
            program,
            nd_memory: vec![],
            nd_tokens: vec![],
            nd_digests: vec![],
        }
    }
    pub fn new_with_tokens(program: Program, tokens: Vec<BFieldElement>) -> Self {
        Self {
            program,
            nd_memory: vec![],
            nd_tokens: tokens,
            nd_digests: vec![],
        }
    }
    pub fn nondeterminism(&self) -> NonDeterminism {
        NonDeterminism::new(self.nd_tokens.clone())
            .with_digests(self.nd_digests.clone())
            .with_ram(self.nd_memory.iter().copied().collect::<HashMap<_, _>>())
    }
}
///# [cfg (any (test , feature = "arbitrary-impls"))]
#[cfg(any(all(test, feature = "original-tests"), feature = "arbitrary-impls"))]
impl<'a> Arbitrary<'a> for LockScriptAndWitness {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let program = Program::arbitrary(u)?;
        let tokens = Digest::arbitrary(u)?.reversed().values().to_vec();
        Ok(LockScriptAndWitness::new_with_tokens(program, tokens))
    }
}
///# [cfg (test)]
#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use crate::models::blockchain::transaction::primitive_witness::PrimitiveWitness;
    use crate::models::blockchain::type_scripts::native_currency_amount::NativeCurrencyAmount;
    use num_traits::Zero;
    use proptest::prop_assert;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;
    #[proptest]
    fn lock_script_halts_gracefully_prop(
        #[strategy(arb::<Digest>())] txk_mast_hash: Digest,
        #[strategy(arb::<Digest>())] seed: Digest,
        #[strategy(NativeCurrencyAmount::arbitrary_non_negative())] amount: NativeCurrencyAmount,
    ) {
        let (_utxos, lock_scripts_and_witnesses) =
            PrimitiveWitness::transaction_inputs_from_address_seeds_and_amounts(&[seed], &[amount]);
        prop_assert!(lock_scripts_and_witnesses.into_iter().all(|lsaw| {
            lsaw.halts_gracefully(PublicInput::new(txk_mast_hash.reversed().values().to_vec()))
        }));
    }
    #[test]
    fn lock_script_halts_gracefully_unit() {
        let txk_mast_hash = Digest::default();
        let seed = Digest::default();
        let amount = NativeCurrencyAmount::zero();
        let (_utxos, lock_scripts_and_witnesses) =
            PrimitiveWitness::transaction_inputs_from_address_seeds_and_amounts(&[seed], &[amount]);
        assert!(lock_scripts_and_witnesses.into_iter().all(|lsaw| {
            lsaw.halts_gracefully(PublicInput::new(txk_mast_hash.reversed().values().to_vec()))
        }));
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
        pub use neptune_cash::protocol::consensus::transaction::lock_script::LockScript;
        pub use neptune_cash::protocol::consensus::transaction::lock_script::LockScriptAndWitness;
    }
    #[test]
    fn test_bincode_serialization_for_lock_script() {
        let original_instance: LockScript = todo!("Instantiate");
        let nc_instance: nc::LockScript = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_lock_script() {
        let original_instance: LockScript = todo!("Instantiate");
        let nc_instance: nc::LockScript = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_lock_script() {
        let original_instance: LockScript = todo!("Instantiate");
        let nc_instance: nc::LockScript = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_bincode_serialization_for_lock_script_and_witness() {
        let original_instance: LockScriptAndWitness = todo!("Instantiate");
        let nc_instance: nc::LockScriptAndWitness = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_lock_script_and_witness() {
        let original_instance: LockScriptAndWitness = todo!("Instantiate");
        let nc_instance: nc::LockScriptAndWitness = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_lock_script_and_witness() {
        let original_instance: LockScriptAndWitness = todo!("Instantiate");
        let nc_instance: nc::LockScriptAndWitness = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
