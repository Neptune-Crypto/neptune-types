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
    pub(crate) fn new(utxo: Utxo, sender_randomness: Digest) -> Self {
        Self {
            utxo,
            sender_randomness,
        }
    }
}
