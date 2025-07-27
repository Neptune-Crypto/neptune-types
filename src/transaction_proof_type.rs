use serde::Deserialize;
use serde::Serialize;

/// represents available types of transaction proofs
///
/// the types are ordered (asc) by proof-generation complexity.
#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, strum::Display, strum::EnumIs)]
#[repr(u8)]
pub enum TransactionProofType {
    /// a primitive-witness.  exposes secrets (keys).  this proof must not be shared.
    PrimitiveWitness = 1,
    /// a weak proof that does not expose secrets. can be shared with peers, but cannot be confirmed into a block.
    ProofCollection = 2,
    /// a strong proof.  required for confirming a transaction into a block.
    SingleProof = 3,
}

// impl From<&TransactionProof> for TransactionProofType {
//     fn from(proof: &TransactionProof) -> Self {
//         match *proof {
//             TransactionProof::Witness(_) => Self::PrimitiveWitness,
//             TransactionProof::ProofCollection(_) => Self::ProofCollection,
//             TransactionProof::SingleProof(_) => Self::SingleProof,
//         }
//     }
// }

impl TransactionProofType {
    /// indicates if the proof executes in triton-vm.
    pub fn executes_in_vm(&self) -> bool {
        matches!(self, Self::ProofCollection | Self::SingleProof)
    }
}
