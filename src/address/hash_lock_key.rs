use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
// use tasm_lib::prelude::Digest;
// use tasm_lib::triton_vm::isa::triton_asm;
// use tasm_lib::triton_vm::isa::triton_instr;
// use tasm_lib::triton_vm::prelude::BFieldElement;
// use tasm_lib::triton_vm::vm::NonDeterminism;
use triton_isa::triton_asm;
use triton_isa::triton_instr;

use crate::lock_script::LockScript;
use crate::lock_script::LockScriptAndWitness;
use crate::triton_vm::nondeterminism::NonDeterminism;

// pub(crate) const RAW_HASH_LOCK_KEY_FLAG_U8: u8 = 0u8;
// pub(crate) const RAW_HASH_LOCK_KEY_FLAG: BFieldElement =
//     BFieldElement::new(RAW_HASH_LOCK_KEY_FLAG_U8 as u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashLockKey {
    preimage: Digest,
}

impl HashLockKey {
    pub(crate) fn after_image(&self) -> Digest {
        self.preimage.hash()
    }

    pub fn from_preimage(preimage: Digest) -> Self {
        Self { preimage }
    }

    /// Generate a lock script for this hash lock.
    pub fn lock_script(&self) -> LockScript {
        Self::lock_script_from_after_image(self.after_image())
    }

    pub fn lock_script_hash(&self) -> Digest {
        self.lock_script().hash()
    }

    pub fn lock_script_and_witness(&self) -> LockScriptAndWitness {
        let lock_script = Self::lock_script_from_after_image(self.after_image());
        LockScriptAndWitness::new_with_nondeterminism(
            lock_script.program,
            NonDeterminism::new(self.preimage.reversed().values()),
        )
    }

    /// Generate a lock script that verifies knowledge of a hash preimage, given
    /// the after-image.
    ///
    /// Satisfaction of this lock script establishes the UTXO owner's assent to
    /// the transaction.
    pub fn lock_script_from_after_image(after_image: Digest) -> LockScript {
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

    #[cfg(test)]
    pub(crate) fn preimage(&self) -> Digest {
        self.preimage
    }
}
