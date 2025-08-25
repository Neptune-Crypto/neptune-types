use std::fmt::Display;
use itertools::Itertools;
use num_traits::CheckedSub;
use num_traits::Zero;
use serde::Deserialize;
use serde::Serialize;
use crate::announcement::Announcement;
use crate::network::Network;
use crate::mutator_set::mutator_set_accumulator::MutatorSetAccumulator;
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::timestamp::Timestamp;
use crate::tx_input::TxInputList;
use crate::tx_output::TxOutputList;
/// contains the unblinded data that a [Transaction](crate::models::blockchain::transaction::Transaction) is generated from,
/// minus the [TransactionProof](crate::models::blockchain::transaction::TransactionProof).
///
/// conceptually, `TransactionDetails` + `TransactionProof` --> `Transaction`.
///
/// or in more detail:
///
/// ```text
/// TransactionDetails -> (TransactionKernel, PrimitiveWitness)
/// (TransactionKernel, PrimitiveWitness) -> (TransactionKernel, ProofCollection)
/// (TransactionKernel, ProofCollection) -> (TransactionKernel, SingleProof)
/// (TransactionKernel, SingleProof) -> (TransactionKernel, SingleProof)
/// TransactionProof = PrimitiveWitness | ProofCollection | SingleProof
/// Transaction = TransactionKernel + TransactionProof
/// ```
///
/// security: This type contains secrets (keys) and should never be shared.
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct TransactionDetails {
    pub tx_inputs: TxInputList,
    pub tx_outputs: TxOutputList,

    /// announcements *excluding* encrypted UTXO notifications.
    extra_announcements: Vec<Announcement>,
    pub fee: NativeCurrencyAmount,
    pub coinbase: Option<NativeCurrencyAmount>,
    pub timestamp: Timestamp,
    pub mutator_set_accumulator: MutatorSetAccumulator,
    pub network: Network,
}


impl Display for TransactionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"TransactionDetails:
    timestamp: {},
    spend_amount: {},
    inputs_amount: {},
    outputs_amount: {},
    fee: {},
    coinbase: {},
    inputs: {},
    outputs: {},
    change_outputs: {},
    owned_outputs: {},
    network: {},
    extra announcements:\n[{}],    
"#,
            self.timestamp.standard_format(), self.spend_amount(), self.tx_inputs
            .total_native_coins(), self.tx_outputs.total_native_coins(), self.fee, self
            .coinbase.unwrap_or_else(NativeCurrencyAmount::zero), self.tx_inputs.iter()
            .map(| o | o.native_currency_amount()).join(", "), self.tx_outputs.iter()
            .map(| o | o.native_currency_amount()).join(", "), self.tx_outputs
            .change_iter().map(| o | o.native_currency_amount()).join(", "), self
            .tx_outputs.owned_iter().map(| o | o.native_currency_amount()).join(", "),            
            self.network,
            self.extra_announcements.iter().map(|pa| format!("{pa}")).join(",\n"),            
        )
    }
}

impl TransactionDetails {
    /// Construct a [`TransactionDetails`] instance with coinbase from state
    /// information.
    ///
    /// Does sanity checks on:
    /// - amounts, must be balanced
    /// - mutator set membership proofs, must be valid wrt. supplied mutator set
    ///
    /// See also: [Self::new_without_coinbase].
    pub fn new_with_coinbase(
        tx_inputs: impl Into<TxInputList>,
        tx_outputs: impl Into<TxOutputList>,
        coinbase: NativeCurrencyAmount,
        fee: NativeCurrencyAmount,
        timestamp: Timestamp,
        mutator_set_accumulator: MutatorSetAccumulator,
        network: Network,
    ) -> Self {
        Self::new(
            tx_inputs,
            tx_outputs,
            fee,
            Some(coinbase),
            timestamp,
            mutator_set_accumulator,
            network,
        )
    }
    /// Construct a [`TransactionDetails`] instance without coinbase from state
    /// information.
    ///
    /// Does sanity checks on:
    /// - amounts, must be balanced
    /// - mutator set membership proofs, must be valid wrt. supplied mutator set
    ///
    /// See also: [Self::new_with_coinbase].
    pub fn new_without_coinbase(
        tx_inputs: impl Into<TxInputList>,
        tx_outputs: impl Into<TxOutputList>,
        fee: NativeCurrencyAmount,
        timestamp: Timestamp,
        mutator_set_accumulator: MutatorSetAccumulator,
        network: Network,
    ) -> Self {
        Self::new(
            tx_inputs,
            tx_outputs,
            fee,
            None,
            timestamp,
            mutator_set_accumulator,
            network,
        )
    }
    /// Constructor for TransactionDetails with some sanity checks.
    ///
    /// This fn does not perform any validation.  use validate() instead.
    pub(crate) fn new(
        tx_inputs: impl Into<TxInputList>,
        tx_outputs: impl Into<TxOutputList>,
        fee: NativeCurrencyAmount,
        coinbase: Option<NativeCurrencyAmount>,
        timestamp: Timestamp,
        mutator_set_accumulator: MutatorSetAccumulator,
        network: Network,
    ) -> Self {
        Self {
            tx_inputs: tx_inputs.into(),
            tx_outputs: tx_outputs.into(),
            extra_announcements: vec![],            
            fee,
            coinbase,
            timestamp,
            mutator_set_accumulator,
            network,
        }
    }

    /// Extend the [`TransactionDetails`] object with announcements.
    ///
    /// Use this method for announcements that are *not* encrypted UTXO
    /// notifications.
    ///
    /// Announcements are not part of the main constructor [`Self::new`]
    /// because in the common case they are not necessary. If there are
    /// encrypted UTXO notifications, these are computed on the fly from the
    /// transaction outputs. This function should only be used for
    /// announcements that are not encrypted UTXO notifications, which is an
    /// exceptional case.
    pub fn with_announcements<Iter: IntoIterator<Item = Announcement>>(
        mut self,
        announcements: Iter,
    ) -> Self {
        self.extra_announcements = self
            .extra_announcements
            .into_iter()
            .chain(announcements)
            .collect_vec();
        self
    }

    /// amount spent (excludes change and fee)
    ///
    /// ie: sum(inputs) - (change + fee)
    pub fn spend_amount(&self) -> NativeCurrencyAmount {
        let not_spend = self.tx_outputs.change_amount() + self.fee;
        self.tx_inputs
            .total_native_coins()
            .checked_sub(&not_spend)
            .unwrap_or_else(NativeCurrencyAmount::zero)
    }

/* neptune-type todo:
    /// verifies the transaction details are valid.
    ///
    /// specifically, a [PrimitiveWitness] is built from these
    /// details and validated.
    pub async fn validate(&self) -> Result<(), WitnessValidationError> {
        PrimitiveWitness::from_transaction_details(self)
            .validate()
            .await
    }
 
    /// Produce the list of announcements, including the UTXO
    /// notifications.
    pub fn announcements(&self) -> Vec<Announcement> {
        [
            self.extra_announcements.clone(),
            self.tx_outputs.announcements(),
        ]
        .concat()
    }


    pub fn primitive_witness(&self) -> PrimitiveWitness {
        self.into()
    }

    /// Assemble the transaction kernel corresponding to this
    /// [`TransactionDetails`] object.
    pub fn transaction_kernel(&self) -> TransactionKernel {
        let removal_records = self
            .tx_inputs
            .iter()
            .map(|txi| txi.removal_record(&self.mutator_set_accumulator))
            .collect_vec();
        TransactionKernelProxy {
            inputs: removal_records,
            outputs: self.tx_outputs.addition_records(),
            announcements: self.announcements(),
            fee: self.fee,
            coinbase: self.coinbase,
            timestamp: self.timestamp,
            mutator_set_hash: self.mutator_set_accumulator.hash(),
            merge_bit: false,
        }
        .into_kernel()
    }
*/
}
///# [cfg (test)]
#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;
    use super::*;
    #[proptest]
    fn test_fee_gobbler_properties(
        #[strategy(NativeCurrencyAmount::arbitrary_non_negative())]
        gobbled_fee: NativeCurrencyAmount,
        #[strategy(arb())]
        sender_randomness: Digest,
        #[strategy(arb())]
        mutator_set_accumulator: MutatorSetAccumulator,
        #[strategy(arb())]
        now: Timestamp,
        #[filter(#notification_method!= UtxoNotifyMethod::None)]
        #[strategy(arb())]
        notification_method: UtxoNotifyMethod,
    ) {
        let fee_gobbler = TransactionDetails::fee_gobbler(
            gobbled_fee,
            sender_randomness,
            mutator_set_accumulator,
            now,
            notification_method,
            Network::Main,
        );
        assert!(fee_gobbler.tx_inputs.is_empty(), "fee gobbler must have no inputs");
        assert_eq!(
            NativeCurrencyAmount::zero(), fee_gobbler.tx_outputs.iter().map(| txo | txo
            .utxo().get_native_currency_amount()).sum::< NativeCurrencyAmount > () +
            fee_gobbler.fee, "total transaction amount must be zero for fee gobbler"
        );
        assert!(
            fee_gobbler.fee.is_negative() || fee_gobbler.fee.is_zero(),
            "fee must be negative or zero; got {}", fee_gobbler.fee
        );
        let mut half_of_fee = fee_gobbler.fee;
        half_of_fee.div_two();
        let time_locked_amount = fee_gobbler
            .tx_outputs
            .iter()
            .map(|txo| txo.utxo())
            .filter(|utxo| match utxo.release_date() {
                Some(date) => {
                    date >= fee_gobbler.timestamp + MINING_REWARD_TIME_LOCK_PERIOD
                }
                None => false,
            })
            .map(|utxo| utxo.get_native_currency_amount())
            .sum::<NativeCurrencyAmount>();
        assert!(
            - half_of_fee <= time_locked_amount,
            "at least half of negative-fee must be time-locked\nhalf of negative fee: {}\ntime-locked amount: {}",
            - half_of_fee, time_locked_amount,
        );
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
        pub use neptune_cash::api::export::TransactionDetails;
    }
    #[test]
    fn test_bincode_serialization_for_transaction_details() {
        let original_instance: TransactionDetails = todo!("Instantiate");
        let nc_instance: nc::TransactionDetails = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_transaction_details() {
        let original_instance: TransactionDetails = todo!("Instantiate");
        let nc_instance: nc::TransactionDetails = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_transaction_details() {
        let original_instance: TransactionDetails = todo!("Instantiate");
        let nc_instance: nc::TransactionDetails = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
}
