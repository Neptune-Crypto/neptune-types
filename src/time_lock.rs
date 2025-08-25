use get_size2::GetSize;
use num_traits::Zero;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
use crate::timestamp::Timestamp;
use crate::utxo::Coin;
use crate::utxo::Utxo;
#[derive(
    Debug,
    Copy,
    Clone,
    Deserialize,
    Serialize,
    BFieldCodec,
    GetSize,
    PartialEq,
    Eq
)]
pub struct TimeLock;
impl TimeLock {
    /// Create a `TimeLock` type-script-and-state-pair that releases the coins at the
    /// given release date, which corresponds to the number of milliseconds that passed
    /// since the unix epoch started (00:00 am UTC on Jan 1 1970).
    pub fn until(date: Timestamp) -> Coin {
        Coin {
            type_script_hash: TimeLock.hash(),
            state: vec![date.0],
        }
    }
    /// Get the release date from a `Utxo`, if any. If there aren't any, return
    /// the null release date.
    pub fn extract_release_date(utxo: &Utxo) -> Timestamp {
        utxo.coins().iter().find_map(Coin::release_date).unwrap_or_else(Timestamp::zero)
    }
    pub fn hash(&self) -> Digest {
        Digest::try_from_hex(
                "4b4d251947a07f9f2c016c1c271c04ce41013ff50031bd42854919be6e0e4849ebf931e856b542ad",
            )
            .unwrap()
    }
}
///# [cfg (any (test , feature = "arbitrary-impls"))]
#[cfg(any(all(test, feature = "original-tests"), feature = "arbitrary-impls"))]
pub mod neptune_arbitrary {
    use num_traits::CheckedSub;
    use proptest::arbitrary::Arbitrary;
    use proptest::collection::vec;
    use proptest::strategy::BoxedStrategy;
    use proptest::strategy::Strategy;
    use proptest_arbitrary_interop::arb;
    use super::super::native_currency_amount::NativeCurrencyAmount;
    use super::*;
    use crate::models::blockchain::transaction::transaction_kernel::TransactionKernelModifier;
    use crate::models::blockchain::transaction::Announcement;
    impl Arbitrary for TimeLockWitness {
        /// Parameters are:
        ///  - release_dates : `Vec<u64>` One release date per input UTXO. 0 if the time lock
        ///    coin is absent.
        ///  - num_outputs : usize Number of outputs.
        ///  - num_public_announcements : usize Number of public announcements.
        ///  - transaction_timestamp: Timestamp determining when the transaction takes place.
        type Parameters = (Vec<Timestamp>, usize, usize, Timestamp);
        type Strategy = BoxedStrategy<Self>;
        fn arbitrary_with(parameters: Self::Parameters) -> Self::Strategy {
            let (
                release_dates,
                num_outputs,
                num_public_announcements,
                transaction_timestamp,
            ) = parameters;
            let num_inputs = release_dates.len();
            (
                vec(arb::<Digest>(), num_inputs),
                vec(NativeCurrencyAmount::arbitrary_non_negative(), num_inputs),
                vec(arb::<Digest>(), num_outputs),
                vec(NativeCurrencyAmount::arbitrary_non_negative(), num_outputs),
                vec(arb::<Announcement>(), num_public_announcements),
                NativeCurrencyAmount::arbitrary_coinbase(),
                NativeCurrencyAmount::arbitrary_non_negative(),
            )
                .prop_flat_map(move |
                    (
                        input_address_seeds,
                        input_amounts,
                        output_address_seeds,
                        mut output_amounts,
                        public_announcements,
                        maybe_coinbase,
                        mut fee,
                    )|
                {
                    let (mut input_utxos, input_lock_scripts_and_witnesses) = PrimitiveWitness::transaction_inputs_from_address_seeds_and_amounts(
                        &input_address_seeds,
                        &input_amounts,
                    );
                    let total_inputs = input_amounts
                        .into_iter()
                        .sum::<NativeCurrencyAmount>();
                    for (utxo, release_date) in input_utxos
                        .iter_mut()
                        .zip(release_dates.iter())
                    {
                        if !release_date.is_zero() {
                            let time_lock_coin = TimeLock::until(*release_date);
                            let mut coins = utxo.coins().to_vec();
                            coins.push(time_lock_coin);
                            *utxo = (utxo.lock_script_hash(), coins).into();
                        }
                    }
                    PrimitiveWitness::find_balanced_output_amounts_and_fee(
                        total_inputs,
                        maybe_coinbase,
                        &mut output_amounts,
                        &mut fee,
                    );
                    let output_utxos = PrimitiveWitness::valid_tx_outputs_from_amounts_and_address_seeds(
                        &output_amounts,
                        &output_address_seeds,
                        None,
                    );
                    PrimitiveWitness::arbitrary_primitive_witness_with(
                            &input_utxos,
                            &input_lock_scripts_and_witnesses,
                            &output_utxos,
                            &public_announcements,
                            NativeCurrencyAmount::zero(),
                            maybe_coinbase,
                        )
                        .prop_map(move |mut transaction_primitive_witness| {
                            let modified_kernel = TransactionKernelModifier::default()
                                .timestamp(transaction_timestamp)
                                .modify(transaction_primitive_witness.kernel);
                            transaction_primitive_witness.kernel = modified_kernel;
                            TimeLockWitness::from(transaction_primitive_witness)
                        })
                        .boxed()
                })
                .boxed()
        }
    }
    /// Generate a `Strategy` for a [`PrimitiveWitness`] with the given numbers of
    /// inputs, outputs, and public announcements, with active timelocks.
    ///
    /// The UTXOs are timelocked with a release date set between `now` and six
    /// months from `now`.
    ///
    #[doc(hidden)]
    pub fn arbitrary_primitive_witness_with_active_timelocks(
        num_inputs: usize,
        num_outputs: usize,
        num_announcements: usize,
        now: Timestamp,
    ) -> BoxedStrategy<PrimitiveWitness> {
        vec(
                Timestamp::arbitrary_between(now, now + Timestamp::months(6)),
                num_inputs + num_outputs,
            )
            .prop_flat_map(move |release_dates| {
                arbitrary_primitive_witness_with_timelocks(
                    num_inputs,
                    num_outputs,
                    num_announcements,
                    now,
                    release_dates,
                )
            })
            .boxed()
    }
    /// Generate a `Strategy` for a [`PrimitiveWitness`] with the given numbers of
    /// inputs, outputs, and public announcements, with expired timelocks.
    ///
    /// The UTXOs are timelocked with a release date set between six months in the
    /// past relative to `now` and `now`.
    ///
    #[doc(hidden)]
    pub fn arbitrary_primitive_witness_with_expired_timelocks(
        num_inputs: usize,
        num_outputs: usize,
        num_announcements: usize,
        now: Timestamp,
    ) -> BoxedStrategy<PrimitiveWitness> {
        vec(
                Timestamp::arbitrary_between(
                    now - Timestamp::months(6),
                    now - Timestamp::millis(1),
                ),
                num_inputs + num_outputs,
            )
            .prop_flat_map(move |release_dates| {
                arbitrary_primitive_witness_with_timelocks(
                    num_inputs,
                    num_outputs,
                    num_announcements,
                    now,
                    release_dates,
                )
            })
            .boxed()
    }
    #[expect(unused_variables, reason = "under development")]
    fn arbitrary_primitive_witness_with_timelocks(
        num_inputs: usize,
        num_outputs: usize,
        num_announcements: usize,
        now: Timestamp,
        release_dates: Vec<Timestamp>,
    ) -> BoxedStrategy<PrimitiveWitness> {
        (
            NativeCurrencyAmount::arbitrary_non_negative(),
            vec(arb::<Digest>(), num_inputs),
            vec(arb::<u64>(), num_inputs),
            vec(arb::<Digest>(), num_outputs),
            vec(arb::<u64>(), num_outputs),
            vec(arb::<Announcement>(), num_announcements),
            arb::<u64>(),
            arb::<Option<u64>>(),
        )
            .prop_flat_map(move |
                (
                    total_amount,
                    input_address_seeds,
                    input_dist,
                    output_address_seeds,
                    output_dist,
                    public_announcements,
                    fee_dist,
                    maybe_coinbase_dist,
                )|
            {
                let maybe_coinbase_dist = if num_inputs.is_zero() {
                    maybe_coinbase_dist
                } else {
                    None
                };
                let mut input_denominator = input_dist
                    .iter()
                    .map(|u| *u as f64)
                    .sum::<f64>();
                if let Some(d) = maybe_coinbase_dist {
                    input_denominator += d as f64;
                }
                let input_weights = input_dist
                    .into_iter()
                    .map(|u| (u as f64) / input_denominator)
                    .collect_vec();
                let mut input_amounts = input_weights
                    .into_iter()
                    .map(|w| total_amount.to_nau_f64() * w)
                    .map(|f| NativeCurrencyAmount::try_from(f).unwrap())
                    .collect_vec();
                let maybe_coinbase = if maybe_coinbase_dist.is_some()
                    || input_amounts.is_empty()
                {
                    Some(
                        total_amount
                            .checked_sub(
                                &input_amounts.iter().copied().sum::<NativeCurrencyAmount>(),
                            )
                            .unwrap(),
                    )
                } else {
                    let sum_of_all_but_last = input_amounts
                        .iter()
                        .rev()
                        .skip(1)
                        .copied()
                        .sum::<NativeCurrencyAmount>();
                    *input_amounts.last_mut().unwrap() = total_amount
                        .checked_sub(&sum_of_all_but_last)
                        .unwrap();
                    None
                };
                let output_denominator = output_dist
                    .iter()
                    .map(|u| *u as f64)
                    .sum::<f64>() + (fee_dist as f64);
                let output_weights = output_dist
                    .into_iter()
                    .map(|u| (u as f64) / output_denominator)
                    .collect_vec();
                let output_amounts = output_weights
                    .into_iter()
                    .map(|w| total_amount.to_nau_f64() * w)
                    .map(|f| NativeCurrencyAmount::try_from(f).unwrap())
                    .collect_vec();
                let total_outputs = output_amounts
                    .iter()
                    .copied()
                    .sum::<NativeCurrencyAmount>();
                let fee = total_amount.checked_sub(&total_outputs).unwrap();
                let (mut input_utxos, input_lock_scripts_and_witnesses) = PrimitiveWitness::transaction_inputs_from_address_seeds_and_amounts(
                    &input_address_seeds,
                    &input_amounts,
                );
                let total_inputs = input_amounts
                    .iter()
                    .copied()
                    .sum::<NativeCurrencyAmount>();
                assert_eq!(
                    total_inputs + maybe_coinbase
                    .unwrap_or(NativeCurrencyAmount::coins(0)), total_outputs + fee
                );
                let mut output_utxos = PrimitiveWitness::valid_tx_outputs_from_amounts_and_address_seeds(
                    &output_amounts,
                    &output_address_seeds,
                    None,
                );
                let mut counter = 0usize;
                for utxo in &mut input_utxos {
                    let release_date = release_dates[counter];
                    let time_lock = TimeLock::until(release_date);
                    let mut coins = utxo.coins().to_vec();
                    coins.push(time_lock);
                    *utxo = Utxo::from((utxo.lock_script_hash(), coins));
                    counter += 1;
                }
                for utxo in &mut output_utxos {
                    let mut coins = utxo.coins().to_vec();
                    coins.push(TimeLock::until(release_dates[counter]));
                    *utxo = Utxo::from((utxo.lock_script_hash(), coins));
                    counter += 1;
                }
                let release_dates = release_dates.clone();
                let merge_bit = false;
                PrimitiveWitness::arbitrary_primitive_witness_with_timestamp_and(
                        &input_utxos,
                        &input_lock_scripts_and_witnesses,
                        &output_utxos,
                        &public_announcements,
                        fee,
                        maybe_coinbase,
                        now,
                        merge_bit,
                    )
                    .prop_map(move |primitive_witness_template| {
                        let mut primitive_witness = primitive_witness_template.clone();
                        let modified_kernel = TransactionKernelModifier::default()
                            .timestamp(now)
                            .modify(primitive_witness.kernel);
                        primitive_witness.kernel = modified_kernel;
                        primitive_witness
                    })
            })
            .boxed()
    }
}
///# [cfg (test)]
#[cfg(all(test, feature = "original-tests"))]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::explicit_deref_methods)]
mod tests {
    use proptest::collection::vec;
    use proptest::prelude::Arbitrary;
    use proptest::prelude::Strategy;
    use proptest::prop_assert;
    use proptest::prop_assert_eq;
    use proptest::strategy::Just;
    use proptest::test_runner::TestRunner;
    use proptest_arbitrary_interop::arb;
    use tasm_lib::twenty_first::math::tip5::Tip5;
    use test_strategy::proptest;
    use super::neptune_arbitrary::arbitrary_primitive_witness_with_active_timelocks;
    use super::neptune_arbitrary::arbitrary_primitive_witness_with_expired_timelocks;
    use super::*;
    use crate::models::proof_abstractions::tasm::builtins as tasm;
    use crate::models::proof_abstractions::tasm::program::tests::test_program_snapshot;
    use crate::models::proof_abstractions::tasm::program::tests::ConsensusProgramSpecification;
    impl ConsensusProgramSpecification for TimeLock {
        #[expect(clippy::needless_return)]
        fn source(&self) {
            let self_digest: Digest = tasm::own_program_digest();
            let tx_kernel_digest: Digest = tasm::tasmlib_io_read_stdin___digest();
            let input_utxos_digest: Digest = tasm::tasmlib_io_read_stdin___digest();
            let _output_utxos_digest: Digest = tasm::tasmlib_io_read_stdin___digest();
            let leaf_index: u32 = 5;
            let timestamp: BFieldElement = tasm::tasmlib_io_read_secin___bfe();
            let leaf: Digest = Tip5::hash_varlen(&timestamp.encode());
            let tree_height: u32 = 3;
            tasm::tasmlib_hashing_merkle_verify(
                tx_kernel_digest,
                leaf_index,
                leaf,
                tree_height,
            );
            let input_utxos_pointer: u64 = tasm::tasmlib_io_read_secin___bfe().value();
            let _output_utxos_pointer: u64 = tasm::tasmlib_io_read_secin___bfe().value();
            let input_salted_utxos: SaltedUtxos = tasm::decode_from_memory(
                BFieldElement::new(input_utxos_pointer),
            );
            let input_salted_utxos_digest: Digest = Tip5::hash(&input_salted_utxos);
            assert_eq!(input_salted_utxos_digest, input_utxos_digest);
            let input_utxos = input_salted_utxos.utxos;
            let mut i = 0;
            while i < input_utxos.len() {
                let coins = input_utxos[i].coins();
                let mut j: usize = 0;
                while j < coins.len() {
                    let coin: &Coin = &coins[j];
                    if coin.type_script_hash == self_digest {
                        let state: &Vec<BFieldElement> = &coin.state;
                        assert!(state.len() == 1);
                        let release_date: BFieldElement = state[0];
                        assert!(release_date.value() < timestamp.value());
                    }
                    j += 1;
                }
                i += 1;
            }
            return;
        }
    }
    #[proptest(cases = 20)]
    fn test_unlocked(
        #[strategy(1usize..= 3)]
        _num_inputs: usize,
        #[strategy(1usize..= 3)]
        _num_outputs: usize,
        #[strategy(1usize..= 3)]
        _num_public_announcements: usize,
        #[strategy(vec(Just(Timestamp::zero()), #_num_inputs))]
        _release_dates: Vec<Timestamp>,
        #[strategy(Just::<Timestamp>(#_release_dates.iter().copied().min().unwrap()))]
        _transaction_timestamp: Timestamp,
        #[strategy(
            TimeLockWitness::arbitrary_with(
                (
                    #_release_dates,
                    #_num_outputs,
                    #_num_public_announcements,
                    #_transaction_timestamp,
                )
            )
        )]
        time_lock_witness: TimeLockWitness,
    ) {
        let rust_result = TimeLock
            .run_rust(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(rust_result.is_ok(), "time lock program did not halt gracefully");
        let tasm_result = TimeLock
            .run_tasm(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(tasm_result.is_ok(), "time lock program did not halt gracefully");
        prop_assert_eq!(rust_result.unwrap(), tasm_result.unwrap());
    }
    #[test]
    fn tx_timestamp_same_as_release_time_must_fail() {
        let release_date = Timestamp::now();
        let mut test_runner = TestRunner::deterministic();
        let time_lock_witness = TimeLockWitness::arbitrary_with((
                vec![release_date],
                1,
                0,
                release_date,
            ))
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        assert!(
            TimeLock {} .run_rust(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
        assert!(
            TimeLock {} .run_tasm(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
    }
    #[proptest(cases = 20)]
    fn test_locked(
        #[strategy(1usize..= 3)]
        _num_inputs: usize,
        #[strategy(1usize..= 3)]
        _num_outputs: usize,
        #[strategy(1usize..= 3)]
        _num_public_announcements: usize,
        #[strategy(
            vec(
                Timestamp::arbitrary_between(
                    Timestamp::now()-Timestamp::days(7),
                    Timestamp::now()-Timestamp::days(1),
                ),
                #_num_inputs,
            )
        )]
        _release_dates: Vec<Timestamp>,
        #[strategy(Just::<Timestamp>(#_release_dates.iter().copied().max().unwrap()))]
        _tx_timestamp: Timestamp,
        #[strategy(
            TimeLockWitness::arbitrary_with(
                (
                    #_release_dates,
                    #_num_outputs,
                    #_num_public_announcements,
                    #_tx_timestamp,
                )
            )
        )]
        time_lock_witness: TimeLockWitness,
    ) {
        println!("now: {}", Timestamp::now());
        prop_assert!(
            TimeLock {} .run_rust(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
        prop_assert!(
            TimeLock {} .run_tasm(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
    }
    #[proptest(cases = 20)]
    fn test_released(
        #[strategy(1usize..= 3)]
        _num_inputs: usize,
        #[strategy(1usize..= 3)]
        _num_outputs: usize,
        #[strategy(1usize..= 3)]
        _num_public_announcements: usize,
        #[strategy(
            vec(
                Timestamp::arbitrary_between(
                    Timestamp::now()-Timestamp::days(7),
                    Timestamp::now()-Timestamp::days(1),
                ),
                #_num_inputs,
            )
        )]
        _release_dates: Vec<Timestamp>,
        #[strategy(Just::<Timestamp>(#_release_dates.iter().copied().max().unwrap()))]
        _tx_timestamp: Timestamp,
        #[strategy(
            TimeLockWitness::arbitrary_with(
                (
                    #_release_dates,
                    #_num_outputs,
                    #_num_public_announcements,
                    #_tx_timestamp+Timestamp::days(1),
                )
            )
        )]
        time_lock_witness: TimeLockWitness,
    ) {
        println!("now: {}", Timestamp::now());
        let rust_result = TimeLock
            .run_rust(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(rust_result.is_ok(), "time lock program did not halt gracefully");
        let tasm_result = TimeLock
            .run_tasm(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(tasm_result.is_ok(), "time lock program did not halt gracefully");
        prop_assert_eq!(rust_result.unwrap(), tasm_result.unwrap());
    }
    #[proptest(cases = 5)]
    fn primitive_witness_with_active_timelocks_is_invalid(
        #[strategy(arb::<Timestamp>())]
        _now: Timestamp,
        #[strategy(arbitrary_primitive_witness_with_active_timelocks(2, 2, 2, #_now))]
        primitive_witness: PrimitiveWitness,
    ) {
        let rt = crate::tests::tokio_runtime();
        prop_assert!(rt.block_on(primitive_witness.validate()).is_err());
    }
    #[proptest(cases = 10)]
    fn arbitrary_primitive_witness_with_active_timelocks_fails(
        #[strategy(arb::<Timestamp>())]
        _now: Timestamp,
        #[strategy(arbitrary_primitive_witness_with_active_timelocks(2, 2, 2, #_now))]
        primitive_witness: PrimitiveWitness,
    ) {
        let time_lock_witness = TimeLockWitness::from(primitive_witness);
        prop_assert!(
            TimeLock {} .run_rust(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
        prop_assert!(
            TimeLock {} .run_tasm(& time_lock_witness.standard_input(), time_lock_witness
            .nondeterminism(),).is_err(), "time lock program failed to panic"
        );
    }
    #[proptest(cases = 10)]
    fn arbitrary_primitive_witness_with_expired_timelocks_passes(
        #[strategy(arb::<Timestamp>())]
        _now: Timestamp,
        #[strategy(arbitrary_primitive_witness_with_expired_timelocks(2, 2, 2, #_now))]
        primitive_witness: PrimitiveWitness,
    ) {
        let time_lock_witness = TimeLockWitness::from(primitive_witness);
        let rust_result = TimeLock
            .run_rust(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(rust_result.is_ok(), "time lock program did not halt gracefully");
        let tasm_result = TimeLock
            .run_tasm(
                &time_lock_witness.standard_input(),
                time_lock_witness.nondeterminism(),
            );
        prop_assert!(tasm_result.is_ok(), "time lock program did not halt gracefully");
        prop_assert_eq!(tasm_result.unwrap(), rust_result.unwrap());
    }
    test_program_snapshot!(
        TimeLock,
        "4b4d251947a07f9f2c016c1c271c04ce41013ff50031bd42854919be6e0e4849ebf931e856b542ad"
    );
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
        pub use neptune_cash::models::blockchain::type_scripts::time_lock::TimeLock;
    }
    #[test]
    fn test_bincode_serialization_for_time_lock() {
        let original_instance: TimeLock = todo!("Instantiate");
        let nc_instance: nc::TimeLock = todo!("Instantiate");
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_serialization_for_time_lock() {
        let original_instance: TimeLock = todo!("Instantiate");
        let nc_instance: nc::TimeLock = todo!("Instantiate");
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }
    #[test]
    fn test_serde_json_wasm_serialization_for_time_lock() {
        let original_instance: TimeLock = todo!("Instantiate");
        let nc_instance: nc::TimeLock = todo!("Instantiate");
        test_serde_json_wasm_serialization_for_type(
            original_instance,
            Some(nc_instance),
        );
    }
}
