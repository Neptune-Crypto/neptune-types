use serde::Deserialize;
use serde::Serialize;

use crate::address::ReceivingAddress;
use crate::utxo_notification_payload::UtxoNotificationPayload;

/// Enumerates the medium of exchange for UTXO-notifications.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum UtxoNotificationMedium {
    /// The UTXO notification should be sent on-chain
    #[default]
    OnChain,

    /// The UTXO notification should be sent off-chain
    OffChain,
}

/// enumerates how utxos and spending information is communicated, including how
/// to encrypt this information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(test, derive(arbitrary::Arbitrary))]
pub enum UtxoNotifyMethod {
    /// the utxo notification should be transferred to recipient encrypted on the blockchain
    OnChain(ReceivingAddress),

    /// the utxo notification should be transferred to recipient off the blockchain
    OffChain(ReceivingAddress),

    /// No UTXO notification is intended
    None,
}

impl UtxoNotifyMethod {
    pub fn new(medium: UtxoNotificationMedium, address: ReceivingAddress) -> Self {
        match medium {
            UtxoNotificationMedium::OnChain => Self::OnChain(address),
            UtxoNotificationMedium::OffChain => Self::OffChain(address),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivateNotificationData {
    pub cleartext: UtxoNotificationPayload,
    pub ciphertext: String,
    pub recipient_address: ReceivingAddress,

    /// Indicates if this client can unlock the UTXO
    pub owned: bool,
}
