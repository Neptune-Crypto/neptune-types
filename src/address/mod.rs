//! implements wallet keys and addresses.
//!
//! naming: it would make more sense for this module to be named 'key' or 'keys'
//! and it will probably be renamed in a future commit.
//!
//! (especially since we now have a key type with no corresponding address)
mod addressable_key;
mod common;
pub mod encrypted_utxo_notification;
pub mod generation_address;
pub mod receiving_address;
pub mod symmetric_key;
pub use addressable_key::SpendingKey;
pub use addressable_key::KeyType;
pub use receiving_address::ReceivingAddress;
