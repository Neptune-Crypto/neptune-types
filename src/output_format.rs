//! provides a builder and related type(s) for generating [TxOutputList], ie a list of
//! transaction outputs ([TxOutput]).
//!
//! outputs may be specified in several ways via the [OutputFormat] enum.
//!
//! see [builder](super) for examples of using the builders together.
use serde::Deserialize;
use serde::Serialize;

use crate::address::ReceivingAddress;
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::tx_output::TxOutput;
use crate::utxo::Utxo;
use crate::utxo_notification::UtxoNotificationMedium;

// ##multicoin## :
//  1. The *AndUtxo variants enable basic multi-coin support.
//  2. maybe there should be some variant like AddressAndCoinAndAmount(ReceivingAddress, Coin, CoinAmount)
//     but this requires a new amount type.

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

    // ##multicoin## : maybe something like
    // pub fn amount(&self, coint: Coin) -> CoinAmount;
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
