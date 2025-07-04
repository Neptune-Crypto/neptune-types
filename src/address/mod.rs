//! implements wallet keys and addresses.
//!
//! naming: it would make more sense for this module to be named 'key' or 'keys'
//! and it will probably be renamed in a future commit.
//!
//! (especially since we now have a key type with no corresponding address)
mod addressable_key;
mod base_key;
mod common;
pub mod encrypted_utxo_notification;
pub mod generation_address;
pub mod hash_lock_key;
pub mod receiving_address;
pub mod symmetric_key;
pub use addressable_key::AddressableKey;
pub use addressable_key::AddressableKeyType;
pub use base_key::BaseKeyType;
pub use base_key::BaseSpendingKey;
pub type KeyType = AddressableKeyType;
pub type SpendingKey = AddressableKey;
pub use receiving_address::ReceivingAddress;
