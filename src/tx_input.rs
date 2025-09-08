//! provides an interface for working with transaction inputs
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::unlocked_utxo::UnlockedUtxo;
use crate::utxo::Utxo;
use serde::Deserialize;
use serde::Serialize;
use std::ops::Deref;
use std::ops::DerefMut;
/// represents a transaction input
///
/// this is a newtype wrapper around UnlockedUtxo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput(UnlockedUtxo);
impl From<UnlockedUtxo> for TxInput {
    fn from(unlocked_utxo: UnlockedUtxo) -> Self {
        Self(unlocked_utxo)
    }
}

impl From<TxInput> for UnlockedUtxo {
    fn from(tx_input: TxInput) -> Self {
        tx_input.0
    }
}

impl Deref for TxInput {
    type Target = UnlockedUtxo;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TxInput {
    /// retrieve native currency amount
    pub fn native_currency_amount(&self) -> NativeCurrencyAmount {
        self.utxo.get_native_currency_amount()
    }
}
/// Represents a list of [TxInput]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TxInputList(Vec<TxInput>);
impl Deref for TxInputList {
    type Target = Vec<TxInput>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TxInputList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<TxInput> for TxInputList {
    fn from(t: TxInput) -> Self {
        Self(vec![t])
    }
}

impl<I: Into<TxInput>, T: IntoIterator<Item = I>> From<T> for TxInputList {
    fn from(v: T) -> Self {
        Self(v.into_iter().map(|i| i.into()).collect())
    }
}

impl From<TxInputList> for Vec<TxInput> {
    fn from(list: TxInputList) -> Self {
        list.0
    }
}

impl From<TxInputList> for Vec<UnlockedUtxo> {
    fn from(list: TxInputList) -> Self {
        list.0.into_iter().map(|v| v.into()).collect()
    }
}

impl TxInputList {
    pub fn empty() -> Self {
        Self(vec![])
    }
    /// retrieves native currency sum(inputs)
    pub fn total_native_coins(&self) -> NativeCurrencyAmount {
        self.0
            .iter()
            .map(|u| u.utxo.get_native_currency_amount())
            .sum()
    }
    /// provides an iterator over input Utxo
    pub fn utxos_iter(&self) -> impl IntoIterator<Item = Utxo> + '_ {
        self.0.iter().map(|u| &u.utxo).cloned()
    }
    /// retrieves all Utxo
    pub fn utxos(&self) -> Vec<Utxo> {
        self.utxos_iter().into_iter().collect()
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
        pub use neptune_cash::api::export::TxInput;
        pub use neptune_cash::api::export::TxInputList;
    }
    #[test]
    fn test_bincode_serialization_for_tx_input() {
        let original_instance: TxInput = todo!("Instantiate");
        let nc_instance: nc::TxInput = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_tx_input() {
        let original_instance: TxInput = todo!("Instantiate");
        let nc_instance: nc::TxInput = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_tx_input() {
        let original_instance: TxInput = todo!("Instantiate");
        let nc_instance: nc::TxInput = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_bincode_serialization_for_tx_input_list() {
        let original_instance: TxInputList = TxInputList::default();
        let nc_instance: nc::TxInputList = neptune_cash::api::export::TxInputList::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_tx_input_list() {
        let original_instance: TxInputList = TxInputList::default();
        let nc_instance: nc::TxInputList = neptune_cash::api::export::TxInputList::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_tx_input_list() {
        let original_instance: TxInputList = TxInputList::default();
        let nc_instance: nc::TxInputList = neptune_cash::api::export::TxInputList::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
