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
///# [cfg_attr (test , derive (arbitrary :: Arbitrary))]
#[cfg_attr(all(test, feature = "original-tests"), derive(arbitrary::Arbitrary))]
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
        pub use neptune_cash::models::state::wallet::utxo_notification::PrivateNotificationData;
        pub use neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium;
    }
    #[test]
    fn test_bincode_serialization_for_private_notification_data() {
        let original_instance: PrivateNotificationData = todo!("Instantiate");
        let nc_instance: nc::PrivateNotificationData = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_private_notification_data() {
        let original_instance: PrivateNotificationData = todo!("Instantiate");
        let nc_instance: nc::PrivateNotificationData = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_private_notification_data() {
        let original_instance: PrivateNotificationData = todo!("Instantiate");
        let nc_instance: nc::PrivateNotificationData = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
    #[test]
    fn test_bincode_serialization_for_utxo_notification_medium() {
        let original_instance: UtxoNotificationMedium = UtxoNotificationMedium::default();
        let nc_instance: nc::UtxoNotificationMedium = neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_utxo_notification_medium() {
        let original_instance: UtxoNotificationMedium = UtxoNotificationMedium::default();
        let nc_instance: nc::UtxoNotificationMedium = neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_utxo_notification_medium() {
        let original_instance: UtxoNotificationMedium = UtxoNotificationMedium::default();
        let nc_instance: nc::UtxoNotificationMedium = neptune_cash::models::state::wallet::utxo_notification::UtxoNotificationMedium::default();
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
    #[test]
    fn test_bincode_serialization_for_utxo_notify_method() {
        let original_instance: UtxoNotifyMethod = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, None::<UtxoNotifyMethod>);
    }
    #[test]
    fn test_serde_json_serialization_for_utxo_notify_method() {
        let original_instance: UtxoNotifyMethod = todo!("Instantiate");
        test_serde_json_serialization_for_type(
            original_instance,
            None::<UtxoNotifyMethod>,
        );
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_utxo_notify_method() {
        let original_instance: UtxoNotifyMethod = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            None::<UtxoNotifyMethod>,
        );
    }
}
