use crate::address::hash_lock_key::HashLockKey;
use crate::lock_script::LockScript;
use crate::native_currency::NativeCurrency;
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::time_lock::TimeLock;
use crate::timestamp::Timestamp;
use get_size2::GetSize;
use itertools::Itertools;
use num_traits::Zero;
use rand::rngs::StdRng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::hash::Hash as StdHash;
use std::hash::Hasher as StdHasher;
use twenty_first::prelude::*;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, BFieldCodec)]
///# [cfg_attr (any (test , feature = "arbitrary-impls") , derive (arbitrary :: Arbitrary))]
#[cfg_attr(
    any(all(test, feature = "original-tests"), feature = "arbitrary-impls"),
    derive(arbitrary::Arbitrary)
)]
pub struct Coin {
    pub type_script_hash: Digest,
    pub state: Vec<BFieldElement>,
}

impl Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = if self.type_script_hash == NativeCurrency.hash() {
            let amount = match NativeCurrencyAmount::decode(&self.state) {
                Ok(boxed_amount) => boxed_amount.to_string(),
                Err(_) => "Error: Unable to decode amount".to_owned(),
            };
            format!("Native currency: {amount}")
        } else if self.type_script_hash == TimeLock.hash() {
            let release_date = self.release_date().unwrap();
            format!("Timelock until: {release_date}")
        } else {
            "Unknown type script hash".to_owned()
        };
        write!(f, "{}", output)
    }
}

impl Coin {
    pub fn release_date(&self) -> Option<Timestamp> {
        if self.type_script_hash == TimeLock.hash() {
            Timestamp::decode(&self.state).ok().map(|b| *b)
        } else {
            None
        }
    }
    pub fn new_native_currency(amount: NativeCurrencyAmount) -> Self {
        Self {
            type_script_hash: NativeCurrency.hash(),
            state: amount.encode(),
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, BFieldCodec)]
pub struct Utxo {
    lock_script_hash: Digest,
    coins: Vec<Coin>,
}

impl Display for Utxo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.coins
                .iter()
                .enumerate()
                .map(|(i, coin)| format!("coin {i}: {coin}"))
                .join("; ")
        )
    }
}

impl GetSize for Utxo {
    fn get_stack_size() -> usize {
        size_of::<Self>()
    }

    fn get_heap_size(&self) -> usize {
        let mut total = self.lock_script_hash().get_heap_size();
        for v in &self.coins {
            total += size_of::<Digest>();
            total += v.state.len() * size_of::<BFieldElement>();
        }
        total
    }
}

impl From<(Digest, Vec<Coin>)> for Utxo {
    fn from((lock_script_hash, coins): (Digest, Vec<Coin>)) -> Self {
        Self {
            lock_script_hash,
            coins,
        }
    }
}

impl Utxo {
    pub fn new(lock_script: LockScript, coins: Vec<Coin>) -> Self {
        (lock_script.hash(), coins).into()
    }
    pub fn coins(&self) -> &[Coin] {
        &self.coins
    }
    pub fn lock_script_hash(&self) -> Digest {
        self.lock_script_hash
    }
    /// Returns true iff this UTXO is a lock script with the preimage provided
    /// as input argument.
    pub fn is_lockscript_with_preimage(&self, preimage: Digest) -> bool {
        self.lock_script_hash == HashLockKey::from_preimage(preimage).lock_script_hash()
    }
    pub fn new_native_currency(lock_script: LockScript, amount: NativeCurrencyAmount) -> Self {
        Self::new(lock_script, vec![Coin::new_native_currency(amount)])
    }
    pub fn has_native_currency(&self) -> bool {
        self.coins
            .iter()
            .any(|coin| coin.type_script_hash == NativeCurrency.hash())
    }
    /// Get the amount of Neptune coins that are encapsulated in this UTXO,
    /// regardless of which other coins are present. (Even if that makes the
    /// Neptune coins unspendable.)
    pub fn get_native_currency_amount(&self) -> NativeCurrencyAmount {
        self.coins
            .iter()
            .filter(|coin| coin.type_script_hash == NativeCurrency.hash())
            .map(|coin| match NativeCurrencyAmount::decode(&coin.state) {
                Ok(boxed_amount) => *boxed_amount,
                Err(_) => NativeCurrencyAmount::zero(),
            })
            .sum()
    }
    /// If the UTXO has a timelock, find out what the release date is.
    pub fn release_date(&self) -> Option<Timestamp> {
        self.coins.iter().find_map(Coin::release_date)
    }
    /// Test the coins for state validity, relative to known type scripts.
    /// Adds a time-lock coin, if necessary.
    ///
    /// Does nothing if there is a time lock present already whose release date
    /// is later than the argument.
    pub fn with_time_lock(self, release_date: Timestamp) -> Self {
        if self.release_date().is_some_and(|x| x >= release_date) {
            self
        } else {
            let mut coins = self
                .coins
                .into_iter()
                .filter(|c| c.type_script_hash != TimeLock.hash())
                .collect_vec();
            coins.push(TimeLock::until(release_date));
            Self {
                lock_script_hash: self.lock_script_hash,
                coins,
            }
        }
    }
    /// Determine whether there is a time-lock, with any release date, on the
    /// UTXO.
    ///# [cfg (test)]
    #[cfg(all(test, feature = "original-tests"))]
    pub(crate) fn is_timelocked(&self) -> bool {
        self.coins
            .iter()
            .filter_map(Coin::release_date)
            .any(|_| true)
    }
}
/// Make `Utxo` hashable with `StdHash` for using it in `HashMap`.
///
/// The Clippy warning is safe to suppress, because we do not violate the invariant: k1 == k2 => hash(k1) == hash(k2).
impl StdHash for Utxo {
    fn hash<H: StdHasher>(&self, state: &mut H) {
        StdHash::hash(&self.encode(), state);
    }
}
/// Generate a UTXO pseudorandomly, for testing purposes
pub fn pseudorandom_utxo(seed: [u8; 32]) -> Utxo {
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    Utxo::from((
        rng.random(),
        NativeCurrencyAmount::coins(rng.next_u32() % 42000000).to_native_coins(),
    ))
}
///# [cfg (any (test , feature = "arbitrary-impls"))]
#[cfg(any(all(test, feature = "original-tests"), feature = "arbitrary-impls"))]
pub mod neptune_arbitrary {
    use super::*;
    impl<'a> Arbitrary<'a> for Utxo {
        /// Produce a strategy for "arbitrary" UTXOs where "arbitrary" means:
        ///  - lock script corresponding to an arbitrary generation address
        ///  - one coin of type NativeCurrency and arbitrary, non-negative amount.
        fn arbitrary(u: &mut ::arbitrary::Unstructured<'a>) -> ::arbitrary::Result<Self> {
            let lock_script_hash: Digest = Digest::arbitrary(u)?;
            let type_script_hash = NativeCurrency.hash();
            let amount = NativeCurrencyAmount::arbitrary(u)?.abs();
            let coins = vec![Coin {
                type_script_hash,
                state: amount.encode(),
            }];
            Ok((lock_script_hash, coins).into())
        }
    }
}
///# [cfg (test)]
#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::explicit_deref_methods)]
mod tests {
    use super::*;
    use crate::triton_vm::prelude::*;
    use proptest::prelude::*;
    use proptest_arbitrary_interop::arb;
    use test_strategy::proptest;
    use tracing_test::traced_test;
    impl Utxo {
        pub(crate) fn with_coin(mut self, coin: Coin) -> Self {
            self.coins.push(coin);
            self
        }
        pub(crate) fn append_to_coin_state(
            mut self,
            coin_index: usize,
            new_element: BFieldElement,
        ) -> Self {
            self.coins[coin_index].state.push(new_element);
            self
        }
    }
    proptest::proptest! {
        #[test] fn hash_utxo_test(output in arb::< Utxo > ()) { let _digest = crate
        ::Hash::hash(& output); }
    }
    #[traced_test]
    #[proptest]
    fn serialization_test(#[strategy(arb::<Utxo>())] utxo: Utxo) {
        let serialized: String = serde_json::to_string(&utxo).unwrap();
        let utxo_again: Utxo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(utxo, utxo_again);
    }
    #[proptest]
    fn utxo_timelock_test(
        #[strategy(0_u64..1<<63)]
        #[map(|t|Timestamp(bfe!(t)))]
        release_date: Timestamp,
        #[strategy(0_u64..1<<63)]
        #[map(|t|Timestamp(bfe!(t)))]
        #[filter(Timestamp::zero()<#delta&&#delta<= #release_date)]
        delta: Timestamp,
    ) {
        let no_lock = LockScript::new(triton_program!(halt));
        let mut coins = NativeCurrencyAmount::coins(1).to_native_coins();
        coins.push(TimeLock::until(release_date));
        let utxo = Utxo::new(no_lock, coins);
        prop_assert!(!utxo.can_spend_at(release_date - delta));
        prop_assert!(utxo.is_timelocked());
        let epsilon = Timestamp::millis(1);
        prop_assert!(!utxo.can_spend_at(release_date - epsilon));
        prop_assert!(!utxo.can_spend_at(release_date));
        prop_assert!(utxo.can_spend_at(release_date + epsilon));
        prop_assert!(utxo.can_spend_at(release_date + delta));
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
        pub use neptune_cash::models::blockchain::transaction::utxo::Coin;
        pub use neptune_cash::models::blockchain::transaction::utxo::Utxo;
    }

    impl Default for Coin {
        fn default() -> Self {
            Coin {
                type_script_hash: Digest::default(),
                state: Vec::default(),
            }
        }
    }
    fn nc_coin_default() -> nc::Coin {
        nc::Coin {
            type_script_hash: dg(Digest::default()),
            state: Vec::default(),
        }
    }

    impl Default for Utxo {
        fn default() -> Self {
            Utxo {
                lock_script_hash: Digest::default(),
                coins: Vec::default(),
            }
        }
    }
    fn nc_utxo_default() -> nc::Utxo {
        (dg(Digest::default()), Vec::<nc::Coin>::default()).into()
    }

    #[test]
    fn test_bincode_serialization_for_coin() {
        let original_instance = Coin::default();
        let nc_instance: nc::Coin = nc_coin_default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_coin() {
        let original_instance = Coin::default();
        let nc_instance: nc::Coin = nc_coin_default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_coin() {
        let original_instance = Coin::default();
        let nc_instance: nc::Coin = nc_coin_default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_bincode_serialization_for_utxo() {
        let original_instance = Utxo::default();
        let nc_instance: nc::Utxo = nc_utxo_default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_utxo() {
        let original_instance = Utxo::default();
        let nc_instance: nc::Utxo = nc_utxo_default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_utxo() {
        let original_instance = Utxo::default();
        let nc_instance: nc::Utxo = nc_utxo_default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }
}
