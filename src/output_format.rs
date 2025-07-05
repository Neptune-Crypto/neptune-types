//! provides a builder and related type(s) for generating [TxOutputList], ie a list of
//! transaction outputs ([TxOutput]).
//!
//! outputs may be specified in several ways via the [OutputFormat] enum.
//!
//! see [builder](super) for examples of using the builders together.
use crate::address::ReceivingAddress;
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::tx_output::TxOutput;
use crate::utxo::Utxo;
use crate::utxo_notification::UtxoNotificationMedium;
use serde::Deserialize;
use serde::Serialize;
/// enumerates various ways to specify a transaction output as a simple tuple.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    /// specify receiving address and amount
    AddressAndAmount(ReceivingAddress, NativeCurrencyAmount),
    /// specify receiving address, amount, and a utxo-notification-medium
    AddressAndAmountAndMedium(
        ReceivingAddress,
        NativeCurrencyAmount,
        UtxoNotificationMedium,
    ),
    /// specify utxo and receiving address
    AddressAndUtxo(ReceivingAddress, Utxo),
    /// specify utxo, receiving address, and a utxo-notification-medium
    AddressAndUtxoAndMedium(ReceivingAddress, Utxo, UtxoNotificationMedium),
    /// specify a [TxOutput]
    TxOutput(TxOutput),
}

impl OutputFormat {
    /// returns the native currency amount
    pub fn native_currency_amount(&self) -> NativeCurrencyAmount {
        match self {
            Self::AddressAndAmount(_, amt) => *amt,
            Self::AddressAndAmountAndMedium(_, amt, _) => *amt,
            Self::AddressAndUtxo(_, u) => u.get_native_currency_amount(),
            Self::AddressAndUtxoAndMedium(_, u, _) => u.get_native_currency_amount(),
            Self::TxOutput(to) => to.native_currency_amount(),
        }
    }
}

impl From<(ReceivingAddress, NativeCurrencyAmount)> for OutputFormat {
    fn from(v: (ReceivingAddress, NativeCurrencyAmount)) -> Self {
        Self::AddressAndAmount(v.0, v.1)
    }
}

impl
    From<(
        ReceivingAddress,
        NativeCurrencyAmount,
        UtxoNotificationMedium,
    )> for OutputFormat
{
    fn from(
        v: (
            ReceivingAddress,
            NativeCurrencyAmount,
            UtxoNotificationMedium,
        ),
    ) -> Self {
        Self::AddressAndAmountAndMedium(v.0, v.1, v.2)
    }
}

impl From<(ReceivingAddress, Utxo)> for OutputFormat {
    fn from(v: (ReceivingAddress, Utxo)) -> Self {
        Self::AddressAndUtxo(v.0, v.1)
    }
}

impl From<(ReceivingAddress, Utxo, UtxoNotificationMedium)> for OutputFormat {
    fn from(v: (ReceivingAddress, Utxo, UtxoNotificationMedium)) -> Self {
        Self::AddressAndUtxoAndMedium(v.0, v.1, v.2)
    }
}

impl From<TxOutput> for OutputFormat {
    fn from(v: TxOutput) -> Self {
        Self::TxOutput(v)
    }
}
#[cfg(test)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unreachable_code)]
#[allow(non_snake_case)]
mod generated_tests {
    use super::*;
    use crate::address::symmetric_key;
    use crate::address::ReceivingAddress;
    use crate::test_shared::*;
    use bincode;
    use serde::{Deserialize, Serialize};
    use twenty_first::prelude::*;
    pub mod nc {
        pub use neptune_cash::api::export::NativeCurrencyAmount;
        pub use neptune_cash::api::export::OutputFormat;
        pub use neptune_cash::api::export::ReceivingAddress;
        pub use neptune_cash::api::export::SymmetricKey;
    }
    #[test]
    fn test_bincode_serialization_for_output_format() {
        let seed: Digest = rand::random();
        let address = ReceivingAddress::from(symmetric_key::SymmetricKey::from_seed(seed));
        let original_instance: OutputFormat = (
            address,
            NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        let nc_address = nc::ReceivingAddress::from(nc::SymmetricKey::from_seed(dg(seed)));
        let nc_instance: nc::OutputFormat = (
            nc_address,
            nc::NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_output_format() {
        let seed: Digest = rand::random();
        let address = ReceivingAddress::from(symmetric_key::SymmetricKey::from_seed(seed));
        let original_instance: OutputFormat = (
            address,
            NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        let nc_address = nc::ReceivingAddress::from(nc::SymmetricKey::from_seed(dg(seed)));
        let nc_instance: nc::OutputFormat = (
            nc_address,
            nc::NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_output_format() {
        let seed: Digest = rand::random();
        let address = ReceivingAddress::from(symmetric_key::SymmetricKey::from_seed(seed));
        let original_instance: OutputFormat = (
            address,
            NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        let nc_address = nc::ReceivingAddress::from(nc::SymmetricKey::from_seed(dg(seed)));
        let nc_instance: nc::OutputFormat = (
            nc_address,
            nc::NativeCurrencyAmount::coins_from_str("125.12345").unwrap(),
        )
            .into();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
