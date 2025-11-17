use crate::utxo::Utxo;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
/// The payload of a UTXO notification, containing all information necessary
/// to claim it, provided access to the associated spending key.
///
/// future work:
/// we should consider adding functionality that would facilitate passing
/// these payloads from sender to receiver off-chain for lower-fee transfers
/// between trusted parties or eg wallets owned by the same person/org.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UtxoNotificationPayload {
    pub(crate) utxo: Utxo,
    pub(crate) sender_randomness: Digest,
}

impl UtxoNotificationPayload {
    pub fn new(utxo: Utxo, sender_randomness: Digest) -> Self {
        Self {
            utxo,
            sender_randomness,
        }
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
        pub use neptune_cash::state::wallet::utxo_notification::UtxoNotificationPayload;
    }
    #[test]
    fn test_bincode_serialization_for_utxo_notification_payload() {
        let original_instance: UtxoNotificationPayload = todo!("Instantiate");
        let nc_instance: nc::UtxoNotificationPayload = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_utxo_notification_payload() {
        let original_instance: UtxoNotificationPayload = todo!("Instantiate");
        let nc_instance: nc::UtxoNotificationPayload = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_utxo_notification_payload() {
        let original_instance: UtxoNotificationPayload = todo!("Instantiate");
        let nc_instance: nc::UtxoNotificationPayload = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
