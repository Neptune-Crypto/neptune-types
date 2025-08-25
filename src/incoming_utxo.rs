use twenty_first::prelude::*;
use crate::utxo::Utxo;
use crate::utxo_notification_payload::UtxoNotificationPayload;
/// A [`Utxo`] along with associated data necessary for a recipient to claim it.
///
/// This struct does not store:
///  - Membership proofs -- the recipient must produce them on their own,
///    possibly by running an archival node.
///  - Unlock keys -- cryptographic data necessary for unlocking the UTXO.
///    (There is one exception to this rule: for guesser fee UTXOs, the unlock
///    key coincides with the receiver preimage.)
///
/// See [UtxoNotificationPayload], [ExpectedUtxo]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
pub struct IncomingUtxo {
    pub(crate) utxo: Utxo,
    pub(crate) sender_randomness: Digest,
    pub(crate) receiver_preimage: Digest,
}

impl IncomingUtxo {
    pub fn from_utxo_notification_payload(
        payload: UtxoNotificationPayload,
        receiver_preimage: Digest,
    ) -> Self {
        Self {
            utxo: payload.utxo,
            sender_randomness: payload.sender_randomness,
            receiver_preimage,
        }
    }
}
///# [cfg (test)]
#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;
    use super::*;
    #[proptest]
    fn consistent_conversion(
        #[strategy(arb())]
        incoming_utxo: IncomingUtxo,
        #[strategy(arb())]
        notifier: UtxoNotifier,
    ) {
        let as_expected_utxo = incoming_utxo.clone().into_expected_utxo(notifier);
        prop_assert_eq!(
            incoming_utxo.addition_record(), as_expected_utxo.addition_record
        );
        let back_again: IncomingUtxo = (&as_expected_utxo).into();
        prop_assert_eq!(incoming_utxo, back_again);
    }
}
