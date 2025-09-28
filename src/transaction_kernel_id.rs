use std::fmt::Display;

use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
use thiserror::Error;
use twenty_first::prelude::*;

/// A unique identifier of a transaction whose value is unaffected by a
/// transaction update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, GetSize, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct TransactionKernelId(Digest);

impl Display for TransactionKernelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}

impl From<Digest> for TransactionKernelId {
    fn from(d: Digest) -> Self {
        Self(d)
    }
}

#[derive(Debug, Error)]
pub enum ParseTxIdError {
    #[error("Invalid transaction ID format: {0}")]
    InvalidFormat(String),
}

impl FromStr for TransactionKernelId {
    type Err = ParseTxIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Digest likely has a constructor from a hex string. Adjust if necessary.
        let digest =
            Digest::try_from_hex(s).map_err(|e| ParseTxIdError::InvalidFormat(e.to_string()))?;
        Ok(TransactionKernelId(digest))
    }
}

#[cfg(test)]
mod generated_tests {

    use super::*;
    use crate::test_shared::*;
    use neptune_cash::api::export;

    #[test]
    fn test_bincode_serialization_for_transaction_kernel_id() {
        test_bincode_serialization_for_type(
            TransactionKernelId::default(),
            None::<export::TransactionKernelId>,
        );
    }

    #[test]
    fn test_serde_json_serialization_for_transaction_kernel_id() {
        test_serde_json_serialization_for_type(
            TransactionKernelId::default(),
            None::<export::TransactionKernelId>,
        );
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_transaction_kernel_id() {
        test_serde_json_wasm_serialization_for_type(
            TransactionKernelId::default(),
            None::<export::TransactionKernelId>,
        );
    }
}
