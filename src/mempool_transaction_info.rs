use serde::Deserialize;
use serde::Serialize;

use crate::native_currency_amount::NativeCurrencyAmount;
use crate::transaction_kernel_id::TransactionKernelId;
use crate::transaction_proof_type::TransactionProofType;

#[derive(Clone, Debug, Copy, Serialize, Deserialize)]
pub struct MempoolTransactionInfo {
    pub id: TransactionKernelId,
    pub proof_type: TransactionProofType,
    pub num_inputs: usize,
    pub num_outputs: usize,
    pub positive_balance_effect: NativeCurrencyAmount,
    pub negative_balance_effect: NativeCurrencyAmount,
    pub fee: NativeCurrencyAmount,
    pub synced: bool,
}

// impl From<&Transaction> for MempoolTransactionInfo {
//     fn from(mptx: &Transaction) -> Self {
//         MempoolTransactionInfo {
//             id: mptx.kernel.txid(),
//             proof_type: match mptx.proof {
//                 TransactionProof::Witness(_) => TransactionProofType::PrimitiveWitness,
//                 TransactionProof::SingleProof(_) => TransactionProofType::SingleProof,
//                 TransactionProof::ProofCollection(_) => TransactionProofType::ProofCollection,
//             },
//             num_inputs: mptx.kernel.inputs.len(),
//             num_outputs: mptx.kernel.outputs.len(),
//             positive_balance_effect: NativeCurrencyAmount::zero(),
//             negative_balance_effect: NativeCurrencyAmount::zero(),
//             fee: mptx.kernel.fee,
//             synced: false,
//         }
//     }
// }

impl MempoolTransactionInfo {
    pub fn with_positive_effect_on_balance(
        mut self,
        positive_balance_effect: NativeCurrencyAmount,
    ) -> Self {
        self.positive_balance_effect = positive_balance_effect;
        self
    }

    pub fn with_negative_effect_on_balance(
        mut self,
        negative_balance_effect: NativeCurrencyAmount,
    ) -> Self {
        self.negative_balance_effect = negative_balance_effect;
        self
    }

    pub fn synced(mut self) -> Self {
        self.synced = true;
        self
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
        pub use neptune_cash::api::export::MempoolTransactionInfo;
    }

    #[test]
    fn test_bincode_serialization_for_mempooltransactioninfo() {
        let original_instance: MempoolTransactionInfo = MempoolTransactionInfo::default();
        let nc_instance: nc::MempoolTransactionInfo = neptune_cash::api::export::MempoolTransactionInfo::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_mempooltransactioninfo() {
        let original_instance: MempoolTransactionInfo = MempoolTransactionInfo::default();
        let nc_instance: nc::MempoolTransactionInfo = neptune_cash::api::export::MempoolTransactionInfo::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_mempooltransactioninfo() {
        let original_instance: MempoolTransactionInfo = MempoolTransactionInfo::default();
        let nc_instance: nc::MempoolTransactionInfo = neptune_cash::api::export::MempoolTransactionInfo::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

}
