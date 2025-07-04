//! provides an abstraction over addressable spending keys.
//!
//! The types in this module can be considered sub-types of
//! the types in [super::base_key].
//!
//! In particular these enums only abstract over spending key
//! types that have a corresponding address.  This specialization
//! enables the type system to enforce correct-by-construction
//! semantics wherever the type is used.
use anyhow::bail;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
use super::base_key::BaseKeyType;
use super::base_key::BaseSpendingKey;
use super::common;
use super::generation_address;
use super::receiving_address::ReceivingAddress;
use super::symmetric_key;
use crate::lock_script::LockScript;
use crate::lock_script::LockScriptAndWitness;
use crate::incoming_utxo::IncomingUtxo;
use crate::network::Network;
use crate::public_announcement::PublicAnnouncement;
use crate::utxo::Utxo;
/// Enumerates key types with corresponding addresses
///
/// `AddressableKey` enumerates the sub-set of [BaseKeyType]
/// variants that are represented in [ReceivingAddress].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, strum::EnumIs)]
#[repr(u8)]
pub enum AddressableKeyType {
    /// [generation_address] built on [crate::prelude::twenty_first::math::lattice::kem]
    ///
    /// wraps a symmetric key built on aes-256-gcm
    Generation = generation_address::GENERATION_FLAG_U8,
    /// [symmetric_key] built on aes-256-gcm
    Symmetric = symmetric_key::SYMMETRIC_KEY_FLAG_U8,
}

impl From<AddressableKeyType> for BaseKeyType {
    fn from(t: AddressableKeyType) -> Self {
        match t {
            AddressableKeyType::Generation => Self::Generation,
            AddressableKeyType::Symmetric => Self::Symmetric,
        }
    }
}

impl std::fmt::Display for AddressableKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generation => write!(f, "Generation"),
            Self::Symmetric => write!(f, "Symmetric"),
        }
    }
}

impl From<&ReceivingAddress> for AddressableKeyType {
    fn from(addr: &ReceivingAddress) -> Self {
        match addr {
            ReceivingAddress::Generation(_) => Self::Generation,
            ReceivingAddress::Symmetric(_) => Self::Symmetric,
        }
    }
}

impl From<&AddressableKey> for AddressableKeyType {
    fn from(addr: &AddressableKey) -> Self {
        match addr {
            AddressableKey::Generation(_) => Self::Generation,
            AddressableKey::Symmetric(_) => Self::Symmetric,
        }
    }
}

impl From<AddressableKeyType> for BFieldElement {
    fn from(key_type: AddressableKeyType) -> Self {
        (key_type as u8).into()
    }
}

impl TryFrom<&PublicAnnouncement> for AddressableKeyType {
    type Error = anyhow::Error;
    fn try_from(pa: &PublicAnnouncement) -> Result<Self> {
        match common::key_type_from_public_announcement(pa) {
            Ok(kt) if kt == Self::Generation.into() => Ok(Self::Generation),
            Ok(kt) if kt == Self::Symmetric.into() => Ok(Self::Symmetric),
            _ => bail!("encountered PublicAnnouncement of unknown type"),
        }
    }
}

impl AddressableKeyType {
    /// returns all available `AddressableKeyType`
    pub fn all_types() -> Vec<AddressableKeyType> {
        vec![Self::Generation, Self::Symmetric]
    }
    /// returns human readable prefix of an address
    pub fn get_hrp(&self, network: Network) -> String {
        match self {
            Self::Generation => {
                generation_address::GenerationReceivingAddress::get_hrp(network)
            }
            Self::Symmetric => symmetric_key::SymmetricKey::get_hrp(network),
        }
    }
}
/// Represents cryptographic data necessary for spending funds (or, more
/// specifically, for unlocking UTXOs).
///
/// This enum provides an abstraction API for spending key types, so that a
/// method or struct may simply accept a `AddressableKey` and be
/// forward-compatible with new types of spending key as they are implemented.
///
/// `AddressableKey` enumerates the sub-set of [BaseSpendingKey]
/// variants that are represented in [ReceivingAddress].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AddressableKey {
    /// a key from [generation_address]
    Generation(generation_address::GenerationSpendingKey),
    /// a [symmetric_key]
    Symmetric(symmetric_key::SymmetricKey),
}

impl From<AddressableKey> for BaseSpendingKey {
    fn from(t: AddressableKey) -> Self {
        match t {
            AddressableKey::Generation(k) => Self::Generation(k),
            AddressableKey::Symmetric(k) => Self::Symmetric(k),
        }
    }
}

impl std::hash::Hash for AddressableKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.privacy_preimage(), state)
    }
}

impl From<generation_address::GenerationSpendingKey> for AddressableKey {
    fn from(key: generation_address::GenerationSpendingKey) -> Self {
        Self::Generation(key)
    }
}

impl From<symmetric_key::SymmetricKey> for AddressableKey {
    fn from(key: symmetric_key::SymmetricKey) -> Self {
        Self::Symmetric(key)
    }
}

impl AddressableKey {
    /// returns the address that corresponds to this spending key.
    pub fn to_address(self) -> ReceivingAddress {
        match self {
            Self::Generation(k) => k.to_address().into(),
            Self::Symmetric(k) => k.into(),
        }
    }
    /// Return the lock script and its witness
    pub fn lock_script_and_witness(&self) -> LockScriptAndWitness {
        match self {
            AddressableKey::Generation(generation_spending_key) => {
                generation_spending_key.lock_script_and_witness()
            }
            AddressableKey::Symmetric(symmetric_key) => {
                symmetric_key.lock_script_and_witness()
            }
        }
    }
    pub fn lock_script(&self) -> LockScript {
        LockScript {
            program: self.lock_script_and_witness().program,
        }
    }
    pub fn lock_script_hash(&self) -> Digest {
        self.lock_script().hash()
    }
    /// Return the privacy preimage if this spending key has a corresponding
    /// receiving address.
    ///
    /// note: The hash of the preimage is available in the receiving address
    /// as the privacy_digest
    pub fn privacy_preimage(&self) -> Digest {
        match self {
            Self::Generation(k) => k.privacy_preimage(),
            Self::Symmetric(k) => k.privacy_preimage(),
        }
    }
    /// Return the receiver_identifier if this spending key has a corresponding
    /// receiving address.
    ///
    /// The receiver identifier is a public (=readably by anyone) fingerprint of
    /// the beneficiary's receiving address. It is used to efficiently scan
    /// incoming blocks for new UTXOs that are destined for this key.
    ///
    /// However, the fingerprint can *also* be used to link different payments
    /// to the same address as payments to the same person. Users who want to
    /// avoid this linkability must generate a new address. Down the line we
    /// expect to support address formats that do not come with fingerprints,
    /// and users can enable them for better privacy in exchange for the
    /// increased workload associated with detecting incoming UTXOs.
    pub fn receiver_identifier(&self) -> BFieldElement {
        match self {
            Self::Generation(k) => k.receiver_identifier(),
            Self::Symmetric(k) => k.receiver_identifier(),
        }
    }
    /// Decrypt a slice of BFieldElement into a [Utxo] and [Digest] representing
    /// `sender_randomness`, if this spending key has a corresponding receiving
    /// address.
    ///
    /// # Return Value
    ///
    ///  - `None` if this spending key has no associated receiving address.
    ///  - `Some(Err(..))` if decryption failed.
    ///  - `Some(Ok(..))` if decryption succeeds.
    pub fn decrypt(&self, ciphertext_bfes: &[BFieldElement]) -> Result<(Utxo, Digest)> {
        match self {
            Self::Generation(k) => k.decrypt(ciphertext_bfes),
            Self::Symmetric(k) => k.decrypt(ciphertext_bfes).map_err(anyhow::Error::new),
        }
    }
    /// Scans all public announcements in a `Transaction` and return all
    /// UTXOs that are recognized by this spending key.
    ///
    /// Note that a single `Transaction` may represent an entire block.
    ///
    /// # Side Effects
    ///
    ///  - Logs a warning for any announcement targeted at this key that cannot
    ///    be decrypted.
    pub fn scan_for_announced_utxos<'a>(
        &self,
        public_announcements: impl Iterator<Item = &'a PublicAnnouncement>,
    ) -> Vec<IncomingUtxo> {
        let receiver_identifier = self.receiver_identifier();
        let receiver_preimage = self.privacy_preimage();
        public_announcements
            .filter(|pa| self.matches_public_announcement_key_type(pa))
            .filter(move |pa| {
                matches!(
                    common::receiver_identifier_from_public_announcement(pa), Ok(r) if r
                    == receiver_identifier
                )
            })
            .filter_map(|pa| {
                self.ok_warn(common::ciphertext_from_public_announcement(pa))
            })
            .filter_map(|c| self.ok_warn(self.decrypt(&c)))
            .map(move |(utxo, sender_randomness)| {
                IncomingUtxo {
                    utxo,
                    sender_randomness,
                    receiver_preimage,
                }
            })
            .collect()
    }
    /// converts a result into an Option and logs a warning on any error
    fn ok_warn<T>(&self, result: Result<T>) -> Option<T> {
        match result {
            Ok(v) => Some(v),
            Err(_e) => None,
        }
    }
    /// returns true if the [PublicAnnouncement] has a type-flag that matches the type of this key
    pub(super) fn matches_public_announcement_key_type(
        &self,
        pa: &PublicAnnouncement,
    ) -> bool {
        matches!(
            AddressableKeyType::try_from(pa), Ok(kt) if kt ==
            AddressableKeyType::from(self)
        )
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
        pub use neptune_cash::models::state::wallet::address::AddressableKey;
        pub use neptune_cash::models::state::wallet::address::AddressableKeyType;
    }
    #[test]
    fn test_bincode_serialization_for_addressable_key() {
        let original_instance: AddressableKey = todo!("Instantiate");
        let nc_instance: nc::AddressableKey = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_addressable_key() {
        let original_instance: AddressableKey = todo!("Instantiate");
        let nc_instance: nc::AddressableKey = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_addressable_key() {
        let original_instance: AddressableKey = todo!("Instantiate");
        let nc_instance: nc::AddressableKey = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
    #[test]
    fn test_bincode_serialization_for_addressable_key_type() {
        let original_instance: AddressableKeyType = todo!("Instantiate");
        let nc_instance: nc::AddressableKeyType = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_addressable_key_type() {
        let original_instance: AddressableKeyType = todo!("Instantiate");
        let nc_instance: nc::AddressableKeyType = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_addressable_key_type() {
        let original_instance: AddressableKeyType = todo!("Instantiate");
        let nc_instance: nc::AddressableKeyType = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
}
