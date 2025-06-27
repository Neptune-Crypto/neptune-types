use get_size2::GetSize;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::prelude::*;
// use tasm_lib::data_type::DataType;
// use tasm_lib::field;
// use tasm_lib::field_with_size;
// use tasm_lib::hashing::algebraic_hasher::hash_static_size::HashStaticSize;
// use tasm_lib::hashing::algebraic_hasher::hash_varlen::HashVarlen;
// use tasm_lib::memory::encode_to_memory;
// use tasm_lib::memory::FIRST_NON_DETERMINISTICALLY_INITIALIZED_MEMORY_ADDRESS;
// use tasm_lib::prelude::Digest;
// use tasm_lib::prelude::Library;
// use tasm_lib::prelude::TasmObject;
// use tasm_lib::structure::tasm_object::DEFAULT_MAX_DYN_FIELD_SIZE;
// use tasm_lib::structure::verify_nd_si_integrity::VerifyNdSiIntegrity;
// use tasm_lib::triton_vm::prelude::*;
// use tasm_lib::twenty_first::math::b_field_element::BFieldElement;

// use super::native_currency_amount::NativeCurrencyAmount;
// use super::TypeScript;
// use super::TypeScriptWitness;
// use crate::models::blockchain::block::MINING_REWARD_TIME_LOCK_PERIOD;
// use crate::models::blockchain::transaction::primitive_witness::PrimitiveWitness;
// use crate::models::blockchain::transaction::primitive_witness::SaltedUtxos;
// use crate::models::blockchain::transaction::transaction_kernel::TransactionKernel;
// use crate::models::blockchain::transaction::transaction_kernel::TransactionKernelField;
// use crate::models::blockchain::transaction::utxo::Coin;
// use crate::models::blockchain::transaction::utxo::Utxo;
// use crate::models::blockchain::transaction::validity::tasm::coinbase_amount::CoinbaseAmount;
// use crate::models::blockchain::type_scripts::BFieldCodec;
// use crate::models::blockchain::type_scripts::TypeScriptAndWitness;
// use crate::models::proof_abstractions::mast_hash::MastHash;
// use crate::models::proof_abstractions::tasm::program::ConsensusProgram;
// use crate::timestamp::Timestamp;
// use crate::models::proof_abstractions::SecretWitness;

// const BAD_COINBASE_SIZE_ERROR: i128 = 1_000_030;
// const BAD_SALTED_UTXOS_ERROR: i128 = 1_000_031;
// const NO_INFLATION_VIOLATION: i128 = 1_000_032;
// const BAD_STATE_SIZE_ERROR: i128 = 1_000_033;
// const COINBASE_TIMELOCK_INSUFFICIENT: i128 = 1_000_034;
// const UTXO_SIZE_TOO_LARGE_ERROR: i128 = 1_000_035;
// const TOO_BIG_COIN_FIELD_SIZE_ERROR: i128 = 1_000_036;
// const STATE_LENGTH_FOR_TIME_LOCK_NOT_ONE_ERROR: i128 = 1_000_037;
// const FEE_EXCEEDS_MAX: i128 = 1_000_038;
// const FEE_EXCEEDS_MIN: i128 = 1_000_039;
// const SUM_OF_OUTPUTS_EXCEEDS_MAX: i128 = 1_000_040;
// const SUM_OF_OUTPUTS_IS_NEGATIVE: i128 = 1_000_041;
// const COINBASE_IS_SET_AND_FEE_IS_NEGATIVE: i128 = 1_000_042;
// const INVALID_COIN_AMOUNT: i128 = 1_000_043;
// const INVALID_COINBASE_DISCRIMINANT: i128 = 1_000_044;

/// `NativeCurrency` is the type script that governs Neptune's native currency,
/// Neptune coins.
///
/// The arithmetic for amounts is defined by the struct `NativeCurrencyAmount`.
/// This type script is responsible for checking that transactions that transfer
/// Neptune are balanced, *i.e.*,
///
///  sum inputs  +  (optional: coinbase)  ==  sum outputs  +  fee .
///
/// Transactions that are not balanced in this way are invalid. Furthermore, the
/// type script checks that no overflow occurs while computing the sums.
///
/// Lastly, if the coinbase is set then at least half of this amount must be
/// time-locked for 3 years.
#[derive(Debug, Copy, Clone, Serialize, Deserialize, BFieldCodec, GetSize, PartialEq, Eq)]
pub struct NativeCurrency;

impl NativeCurrency {
    // pub(crate) const TIME_LOCK_HASH: Digest = Digest([
    //     BFieldElement::new(11493081001297792331),
    //     BFieldElement::new(14845021226026139948),
    //     BFieldElement::new(4809053857285865793),
    //     BFieldElement::new(5280486431890426245),
    //     BFieldElement::new(12484740501891840491),
    // ]);

    pub const fn hash(&self) -> Digest {
        // note: this is a hash of self.program() which never changes
        //       so the hash can be retrieved from neptune-core and hard-coded.
        todo!()
    }
}


/*
impl TypeScript for NativeCurrency {
    type State = NativeCurrencyAmount;
}

#[derive(Debug, Clone, Deserialize, Serialize, BFieldCodec, GetSize, PartialEq, Eq, TasmObject)]
pub struct NativeCurrencyWitness {
    pub salted_input_utxos: SaltedUtxos,
    pub salted_output_utxos: SaltedUtxos,
    pub kernel: TransactionKernel,
}

impl From<PrimitiveWitness> for NativeCurrencyWitness {
    fn from(primitive_witness: PrimitiveWitness) -> Self {
        NativeCurrencyWitness {
            salted_input_utxos: primitive_witness.input_utxos,
            salted_output_utxos: primitive_witness.output_utxos,
            kernel: primitive_witness.kernel,
        }
    }
}

/// The part of witness data that is read from memory
///
/// Factored out since this makes auditing the preloaded data much cheaper as
/// we avoid having to audit the [TransactionKernel].
// #[derive(Debug, Clone, BFieldCodec)]
#[derive(Debug, Clone, BFieldCodec, TasmObject)]
struct NativeCurrencyWitnessMemory {
    salted_input_utxos: SaltedUtxos,
    salted_output_utxos: SaltedUtxos,
    coinbase: Option<NativeCurrencyAmount>,
    fee: NativeCurrencyAmount,
    timestamp: Timestamp,
}

impl From<&NativeCurrencyWitness> for NativeCurrencyWitnessMemory {
    fn from(value: &NativeCurrencyWitness) -> Self {
        Self {
            salted_input_utxos: value.salted_input_utxos.clone(),
            salted_output_utxos: value.salted_output_utxos.clone(),
            coinbase: value.kernel.coinbase,
            fee: value.kernel.fee,
            timestamp: value.kernel.timestamp,
        }
    }
}

impl TypeScriptWitness for NativeCurrencyWitness {
    fn new(
        transaction_kernel: TransactionKernel,
        salted_input_utxos: SaltedUtxos,
        salted_output_utxos: SaltedUtxos,
    ) -> Self {
        Self {
            salted_input_utxos,
            salted_output_utxos,
            kernel: transaction_kernel,
        }
    }

    fn transaction_kernel(&self) -> TransactionKernel {
        self.kernel.clone()
    }

    fn salted_input_utxos(&self) -> SaltedUtxos {
        self.salted_input_utxos.clone()
    }

    fn salted_output_utxos(&self) -> SaltedUtxos {
        self.salted_output_utxos.clone()
    }

    fn type_script_and_witness(&self) -> TypeScriptAndWitness {
        TypeScriptAndWitness::new_with_nondeterminism(
            NativeCurrency.program(),
            self.nondeterminism(),
        )
    }
}

impl SecretWitness for NativeCurrencyWitness {
    fn program(&self) -> Program {
        NativeCurrency.program()
    }

    fn standard_input(&self) -> PublicInput {
        self.type_script_standard_input()
    }

    fn nondeterminism(&self) -> NonDeterminism {
        // set memory
        let mut memory = HashMap::default();
        let memory_part_of_witness: NativeCurrencyWitnessMemory = self.into();
        encode_to_memory(
            &mut memory,
            FIRST_NON_DETERMINISTICALLY_INITIALIZED_MEMORY_ADDRESS,
            &memory_part_of_witness,
        );

        // individual tokens
        let individual_tokens = vec![];

        // digests
        let mast_paths = [
            self.kernel.mast_path(TransactionKernelField::Coinbase),
            self.kernel.mast_path(TransactionKernelField::Fee),
            self.kernel.mast_path(TransactionKernelField::Timestamp),
        ]
        .concat();

        // put everything together
        NonDeterminism::new(individual_tokens)
            .with_digests(mast_paths)
            .with_ram(memory)
    }
}
*/

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
#[allow(clippy::explicit_deref_methods)] // suppress clippy's bad autosuggestion
pub mod tests {
    use std::panic;

    use macro_rules_attr::apply;
    use num_traits::CheckedAdd;
    use num_traits::Zero;
    use proptest::collection::vec;
    use proptest::prelude::*;
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    use proptest_arbitrary_interop::arb;
    use tasm_lib::triton_vm::proof::Claim;
    use test_strategy::proptest;

    use super::*;
    use crate::config_models::network::Network;
    use crate::models::blockchain::shared::Hash;
    use crate::models::blockchain::transaction::lock_script::LockScriptAndWitness;
    use crate::models::blockchain::transaction::transaction_kernel::TransactionKernelModifier;
    use crate::models::blockchain::transaction::PublicAnnouncement;
    use crate::models::blockchain::type_scripts::native_currency_amount::tests::invalid_positive_amount;
    use crate::models::blockchain::type_scripts::time_lock::neptune_arbitrary::arbitrary_primitive_witness_with_active_timelocks;
    use crate::models::blockchain::type_scripts::time_lock::TimeLock;
    use crate::models::proof_abstractions::tasm::builtins as tasm;
    use crate::models::proof_abstractions::tasm::program::tests::test_program_snapshot;
    use crate::models::proof_abstractions::tasm::program::tests::ConsensusProgramSpecification;
    use crate::models::proof_abstractions::tasm::program::ConsensusError;
    use crate::models::proof_abstractions::timestamp::Timestamp;
    use crate::models::proof_abstractions::verifier::verify;
    use crate::tests::shared_tokio_runtime;
    use crate::triton_vm_job_queue::TritonVmJobPriority;
    use crate::triton_vm_job_queue::TritonVmJobQueue;

    impl ConsensusProgramSpecification for NativeCurrency {
        fn source(&self) {
            // get in the current program's hash digest
            let self_digest: Digest = tasm::own_program_digest();

            // read standard input:
            //  - transaction kernel mast hash
            //  - input salted utxos digest
            //  - output salted utxos digest
            // (All type scripts take this triple as input.)
            let tx_kernel_digest: Digest = tasm::tasmlib_io_read_stdin___digest();
            let input_utxos_digest: Digest = tasm::tasmlib_io_read_stdin___digest();
            let output_utxos_digest: Digest = tasm::tasmlib_io_read_stdin___digest();

            // divine witness from memory
            let start_address: BFieldElement =
                FIRST_NON_DETERMINISTICALLY_INITIALIZED_MEMORY_ADDRESS;
            let native_currency_witness_mem: NativeCurrencyWitnessMemory =
                tasm::decode_from_memory(start_address);
            let coinbase: Option<NativeCurrencyAmount> = native_currency_witness_mem.coinbase;
            let fee: NativeCurrencyAmount = native_currency_witness_mem.fee;
            let input_salted_utxos: SaltedUtxos = native_currency_witness_mem.salted_input_utxos;
            let output_salted_utxos: SaltedUtxos = native_currency_witness_mem.salted_output_utxos;
            let timestamp = native_currency_witness_mem.timestamp;

            // authenticate coinbase against kernel mast hash
            let coinbase_leaf_index: u32 = 4;
            let coinbase_leaf: Digest = Hash::hash(&coinbase);
            let kernel_tree_height: u32 = 3;
            tasm::tasmlib_hashing_merkle_verify(
                tx_kernel_digest,
                coinbase_leaf_index,
                coinbase_leaf,
                kernel_tree_height,
            );

            // unpack coinbase
            let some_coinbase: NativeCurrencyAmount = match coinbase {
                Some(coins) => coins,
                None => NativeCurrencyAmount::coins(0),
            };
            assert!(!some_coinbase.is_negative());

            // authenticate fee against kernel mast hash
            let fee_leaf_index: u32 = 3;
            let fee_leaf: Digest = Hash::hash(&fee);
            tasm::tasmlib_hashing_merkle_verify(
                tx_kernel_digest,
                fee_leaf_index,
                fee_leaf,
                kernel_tree_height,
            );

            assert!(coinbase.is_none() || !fee.is_negative());

            let timestamp_leaf_index = TransactionKernelField::Timestamp as u32;
            let timestamp_leaf = Tip5::hash(&timestamp);
            tasm::tasmlib_hashing_merkle_verify(
                tx_kernel_digest,
                timestamp_leaf_index,
                timestamp_leaf,
                kernel_tree_height,
            );

            // authenticate inputs against salted commitment
            assert_eq!(input_utxos_digest, Hash::hash(&input_salted_utxos));

            // authenticate outputs against salted commitment
            assert_eq!(output_utxos_digest, Hash::hash(&output_salted_utxos));

            // get total input amount from inputs
            let mut total_input = NativeCurrencyAmount::coins(0);
            let mut i: u32 = 0;
            let num_inputs: u32 = input_salted_utxos.utxos.len() as u32;
            while i < num_inputs {
                let utxo_i = &input_salted_utxos.utxos[i as usize];
                let num_coins: u32 = utxo_i.coins().len() as u32;
                let mut j = 0;
                while j < num_coins {
                    if utxo_i.coins()[j as usize].type_script_hash == self_digest {
                        // decode state to get amount
                        let amount: NativeCurrencyAmount =
                            *NativeCurrencyAmount::decode(&utxo_i.coins()[j as usize].state)
                                .unwrap();

                        // make sure amount is positive (or zero)
                        assert!(!amount.is_negative());

                        // safely add to total
                        total_input = total_input.checked_add(&amount).unwrap();
                    }
                    j += 1;
                }
                i += 1;
            }

            // get total output amount from outputs
            let mut total_output = NativeCurrencyAmount::coins(0);
            let mut total_timelocked_output = NativeCurrencyAmount::coins(0);

            i = 0;
            let num_outputs: u32 = output_salted_utxos.utxos.len() as u32;
            while i < num_outputs {
                let utxo_i = output_salted_utxos.utxos[i as usize].clone();
                let num_coins: u32 = utxo_i.coins().len() as u32;
                let mut total_amount_for_utxo = NativeCurrencyAmount::coins(0);
                let mut time_locked = false;
                let mut j = 0;
                while j < num_coins {
                    let coin_j = utxo_i.coins()[j as usize].clone();
                    if coin_j.type_script_hash == self_digest {
                        // decode state to get amount
                        let amount: NativeCurrencyAmount =
                            *NativeCurrencyAmount::decode(&coin_j.state).unwrap();

                        // make sure amount is positive (or zero)
                        assert!(!amount.is_negative());

                        // safely add to total
                        total_amount_for_utxo = total_amount_for_utxo.checked_add(&amount).unwrap();
                    } else if coin_j.type_script_hash == Self::TIME_LOCK_HASH {
                        // decode state to get release date
                        let release_date = *Timestamp::decode(&coin_j.state).unwrap();
                        if release_date >= timestamp + MINING_REWARD_TIME_LOCK_PERIOD {
                            time_locked = true;
                        }
                    }
                    j += 1;
                }
                total_output = total_output.checked_add(&total_amount_for_utxo).unwrap();
                if time_locked {
                    total_timelocked_output = total_timelocked_output
                        .checked_add(&total_amount_for_utxo)
                        .unwrap();
                }
                i += 1;
            }

            assert!(
                fee >= NativeCurrencyAmount::min(),
                "fee exceeds amount lower bound"
            );
            assert!(
                fee <= NativeCurrencyAmount::max(),
                "fee exceeds amount upper bound"
            );

            // if coinbase is set, verify that half of it is time-locked
            let mut half_of_coinbase = some_coinbase;
            half_of_coinbase.div_two();
            let mut half_of_fee = fee;
            half_of_fee.div_two();
            assert!(some_coinbase.is_zero() || half_of_coinbase <= total_timelocked_output + half_of_fee,
                    "not enough funds timelocked -- half of coinbase == {} > total_timelocked_output + half_of_fee == {} whereas total output == {}",
                    half_of_coinbase,
                    total_timelocked_output + half_of_fee,
                    total_output,);

            // test no-inflation equation
            let total_input_plus_coinbase: NativeCurrencyAmount =
                total_input.checked_add(&some_coinbase).unwrap();
            let total_output_plus_fee: NativeCurrencyAmount =
                total_output.checked_add_negative(&fee).unwrap();
            assert_eq!(total_input_plus_coinbase, total_output_plus_fee);
        }
    }

    fn assert_both_rust_and_tasm_halt_gracefully(
        native_currency_witness: NativeCurrencyWitness,
    ) -> Result<(), TestCaseError> {
        let rust_result = NativeCurrency
            .run_rust(
                &native_currency_witness.standard_input(),
                native_currency_witness.nondeterminism(),
            )
            .expect("rust run should pass");
        prop_assert!(rust_result.is_empty());

        let tasm_result = match NativeCurrency.run_tasm(
            &native_currency_witness.standard_input(),
            native_currency_witness.nondeterminism(),
        ) {
            Ok(r) => r,
            Err(e) => match e {
                ConsensusError::RustShadowPanic(rsp) => {
                    panic!("Tasm run failed due to rust shadow panic (?): {rsp}");
                }
                ConsensusError::TritonVMPanic(err, instruction_error) => {
                    panic!("Tasm run failed due to VM panic: {instruction_error}:\n{err}");
                }
            },
        };

        prop_assert!(tasm_result.is_empty());

        Ok(())
    }

    fn assert_both_rust_and_tasm_fail(
        native_currency_witness: NativeCurrencyWitness,
        expected_error_ids: &[i128],
    ) {
        let stdin = native_currency_witness.standard_input();
        let nd = native_currency_witness.nondeterminism();
        let test_result = NativeCurrency.test_assertion_failure(stdin, nd, expected_error_ids);
        test_result.unwrap();
    }

    #[test]
    fn native_currency_derived_witness_generates_accepting_tasm_program_empty_tx() {
        // Generate a tx with coinbase input, no outputs, fee-size is the same
        // as the coinbase, so tx is valid.
        let mut test_runner = TestRunner::deterministic();
        let primitive_witness = PrimitiveWitness::arbitrary_with_size_numbers(Some(0), 0, 0)
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness).unwrap();
    }

    #[test]
    fn native_currency_derived_witness_generates_accepting_tasm_program_unittest() {
        let mut test_runner = TestRunner::deterministic();
        let primitive_witness = PrimitiveWitness::arbitrary_with_size_numbers(Some(2), 2, 2)
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness).unwrap();
    }

    #[proptest(cases = 50)]
    fn balanced_transaction_is_valid(
        #[strategy(0usize..=3)] _num_inputs: usize,
        #[strategy(0usize..=3)] _num_outputs: usize,
        #[strategy(0usize..=1)] _num_public_announcements: usize,
        #[strategy(PrimitiveWitness::arbitrary_with_size_numbers(Some(#_num_inputs), #_num_outputs, #_num_public_announcements))]
        primitive_witness: PrimitiveWitness,
    ) {
        // PrimitiveWitness::arbitrary_with already ensures the transaction is balanced
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness)?;
    }

    #[proptest(cases = 50)]
    fn native_currency_is_valid_for_primitive_witness_with_timelock(
        #[strategy(1usize..=3)] _num_inputs: usize,
        #[strategy(0usize..=3)] _num_outputs: usize,
        #[strategy(0usize..=1)] _num_public_announcements: usize,
        #[strategy(arb::<Timestamp>())] _now: Timestamp,
        #[strategy(arbitrary_primitive_witness_with_active_timelocks(
            #_num_inputs,
            #_num_outputs,
            #_num_public_announcements,
            #_now,
        ))]
        primitive_witness: PrimitiveWitness,
    ) {
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);

        // there are inputs so there can be no coinbase and we are testing a
        // regular transaction
        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness)?;
    }

    #[test]
    fn native_currency_is_valid_for_primitive_witness_with_timelock_deterministic() {
        let mut test_runner = TestRunner::deterministic();
        let now = arb::<Timestamp>()
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        let primitive_witness = arbitrary_primitive_witness_with_active_timelocks(2, 2, 3, now)
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);

        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness).unwrap();
    }

    #[proptest(cases = 50)]
    fn unbalanced_transaction_without_coinbase_is_invalid_prop(
        #[strategy(1usize..=3)] _num_inputs: usize,
        #[strategy(1usize..=3)] _num_outputs: usize,
        #[strategy(0usize..=3)] _num_public_announcements: usize,
        #[strategy(vec(arb::<Utxo>(), #_num_inputs))] _input_utxos: Vec<Utxo>,
        #[strategy(vec(arb::<LockScriptAndWitness>(), #_num_inputs))]
        _input_lock_scripts_and_witnesses: Vec<LockScriptAndWitness>,
        #[strategy(vec(arb::<Utxo>(), #_num_outputs))] _output_utxos: Vec<Utxo>,
        #[strategy(vec(arb(), #_num_public_announcements))] _public_announcements: Vec<
            PublicAnnouncement,
        >,
        #[strategy(arb())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_primitive_witness_with(
            &#_input_utxos,
            &#_input_lock_scripts_and_witnesses,
            &#_output_utxos,
            &#_public_announcements,
            #_fee,
            None,
        ))]
        primitive_witness: PrimitiveWitness,
    ) {
        // with high probability the amounts (which are random) do not add up
        let witness = NativeCurrencyWitness::from(primitive_witness);

        NativeCurrency.test_assertion_failure(
            witness.standard_input(),
            witness.nondeterminism(),
            &[NO_INFLATION_VIOLATION],
        )?;
    }

    #[proptest(cases = 50)]
    fn unbalanced_transaction_with_coinbase_is_invalid(
        #[strategy(1usize..=3)] _num_inputs: usize,
        #[strategy(1usize..=3)] _num_outputs: usize,
        #[strategy(1usize..=3)] _num_public_announcements: usize,
        #[strategy(NativeCurrencyAmount::arbitrary_non_negative())] _coinbase: NativeCurrencyAmount,
        #[strategy(vec(arb::<Utxo>(), #_num_inputs))] _input_utxos: Vec<Utxo>,
        #[strategy(vec(arb::<LockScriptAndWitness>(), #_num_inputs))]
        _input_lock_scripts_and_witnesses: Vec<LockScriptAndWitness>,
        #[strategy(vec(arb::<Utxo>(), #_num_outputs))] _output_utxos: Vec<Utxo>,
        #[strategy(vec(arb(), #_num_public_announcements))] _public_announcements: Vec<
            PublicAnnouncement,
        >,
        #[strategy(arb())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_primitive_witness_with(
            &#_input_utxos,
            &#_input_lock_scripts_and_witnesses,
            &#_output_utxos,
            &#_public_announcements,
            #_fee,
            Some(#_coinbase),
        ))]
        primitive_witness: PrimitiveWitness,
    ) {
        // with high probability the amounts (which are random) do not add up
        // and since the coinbase is set, the coinbase-timelock test might fail
        // before the no-inflation test.
        let witness = NativeCurrencyWitness::from(primitive_witness);
        assert!(witness.kernel.coinbase.is_some(), "coinbase is none");
        NativeCurrency.test_assertion_failure(
            witness.standard_input(),
            witness.nondeterminism(),
            &[
                NO_INFLATION_VIOLATION,
                COINBASE_TIMELOCK_INSUFFICIENT,
                COINBASE_IS_SET_AND_FEE_IS_NEGATIVE,
            ],
        )?;
    }

    #[test]
    fn tx_with_negative_fee_with_coinbase_deterministic() {
        let mut test_runner = TestRunner::deterministic();
        let mut primitive_witness =
            PrimitiveWitness::arbitrary_with_fee(-NativeCurrencyAmount::coins(1))
                .new_tree(&mut test_runner)
                .unwrap()
                .current();
        let good_native_currency_witness = NativeCurrencyWitness::from(primitive_witness.clone());
        assert_both_rust_and_tasm_halt_gracefully(good_native_currency_witness).unwrap();

        let kernel_modifier =
            TransactionKernelModifier::default().coinbase(Some(NativeCurrencyAmount::coins(1)));
        primitive_witness.kernel = kernel_modifier.modify(primitive_witness.kernel);
        let bad_native_currency_witness = NativeCurrencyWitness::from(primitive_witness.clone());
        NativeCurrency
            .test_assertion_failure(
                bad_native_currency_witness.standard_input(),
                bad_native_currency_witness.nondeterminism(),
                &[COINBASE_IS_SET_AND_FEE_IS_NEGATIVE],
            )
            .unwrap();
    }

    #[proptest]
    fn tx_with_negative_fee_with_coinbase(
        #[strategy(PrimitiveWitness::arbitrary_with_fee(-NativeCurrencyAmount::coins(1)))]
        mut primitive_witness: PrimitiveWitness,
    ) {
        let good_native_currency_witness = NativeCurrencyWitness::from(primitive_witness.clone());
        assert_both_rust_and_tasm_halt_gracefully(good_native_currency_witness).unwrap();

        let kernel_modifier =
            TransactionKernelModifier::default().coinbase(Some(NativeCurrencyAmount::coins(1)));
        primitive_witness.kernel = kernel_modifier.modify(primitive_witness.kernel);
        let bad_native_currency_witness = NativeCurrencyWitness::from(primitive_witness.clone());
        NativeCurrency
            .test_assertion_failure(
                bad_native_currency_witness.standard_input(),
                bad_native_currency_witness.nondeterminism(),
                &[COINBASE_IS_SET_AND_FEE_IS_NEGATIVE],
            )
            .unwrap();
    }

    #[apply(shared_tokio_runtime)]
    async fn native_currency_failing_proof() {
        let network = Network::Main;
        let mut test_runner = TestRunner::deterministic();
        let primitive_witness = PrimitiveWitness::arbitrary_with_size_numbers(Some(2), 2, 2)
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        let txk_mast_hash = primitive_witness.kernel.mast_hash();
        let salted_input_utxos_hash = Tip5::hash(&primitive_witness.input_utxos);
        let salted_output_utxos_hash = Tip5::hash(&primitive_witness.output_utxos);

        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        let type_script_and_witness = TypeScriptAndWitness::new_with_nondeterminism(
            NativeCurrency.program(),
            native_currency_witness.nondeterminism(),
        );
        let tasm_halts = type_script_and_witness.halts_gracefully(
            txk_mast_hash,
            salted_input_utxos_hash,
            salted_output_utxos_hash,
        );

        assert!(tasm_halts);

        let claim = Claim::new(NativeCurrency.program().hash())
            .with_input(native_currency_witness.standard_input().individual_tokens);
        let proof = type_script_and_witness
            .prove(
                txk_mast_hash,
                salted_input_utxos_hash,
                salted_output_utxos_hash,
                TritonVmJobQueue::get_instance(),
                TritonVmJobPriority::default().into(),
            )
            .await
            .unwrap();
        assert!(verify(claim, proof, network).await, "proof fails");
    }

    #[proptest]
    fn transaction_with_timelocked_coinbase_is_valid(
        #[strategy(PrimitiveWitness::arbitrary_with_size_numbers(None, 2, 2))]
        #[filter(!#primitive_witness.kernel.fee.is_negative())]
        primitive_witness: PrimitiveWitness,
    ) {
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_halt_gracefully(native_currency_witness).unwrap();
    }

    #[test]
    fn transaction_with_timelocked_coinbase_is_valid_deterministic() {
        let mut test_runner = TestRunner::deterministic();
        let mut primitive_witness = PrimitiveWitness::arbitrary_with_size_numbers(None, 2, 2)
            .new_tree(&mut test_runner)
            .unwrap()
            .current();
        while primitive_witness.kernel.fee.is_negative() {
            primitive_witness = PrimitiveWitness::arbitrary_with_size_numbers(None, 2, 2)
                .new_tree(&mut test_runner)
                .unwrap()
                .current();
        }
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        let mut fee = native_currency_witness.kernel.fee;
        fee.div_two();

        assert!(assert_both_rust_and_tasm_halt_gracefully(native_currency_witness).is_ok());
    }

    #[test]
    fn unbalanced_transaction_without_coinbase_is_invalid_deterministic() {
        fn sample<T: Clone, S: Strategy<Value = T>>(
            strategy: S,
            test_runner: &mut TestRunner,
        ) -> T {
            strategy.new_tree(test_runner).unwrap().current().clone()
        }

        let mut tr = TestRunner::deterministic();

        for _ in 0..10 {
            let input_utxos = sample(vec(arb::<Utxo>(), 3), &mut tr);
            let input_lock_scripts_and_witnesses =
                sample(vec(arb::<LockScriptAndWitness>(), 3), &mut tr);
            let output_utxos = sample(vec(arb::<Utxo>(), 3), &mut tr);
            let public_announcements = sample(vec(arb(), 3), &mut tr);
            let fee = sample(NativeCurrencyAmount::arbitrary_non_negative(), &mut tr);
            let primitive_witness = PrimitiveWitness::arbitrary_primitive_witness_with(
                &input_utxos,
                &input_lock_scripts_and_witnesses,
                &output_utxos,
                &public_announcements,
                fee,
                None,
            )
            .new_tree(&mut tr)
            .unwrap()
            .current()
            .clone();
            // with high probability the amounts (which are random) do not add up
            let witness = NativeCurrencyWitness::from(primitive_witness);
            let result = NativeCurrency.test_assertion_failure(
                witness.standard_input(),
                witness.nondeterminism(),
                &[NO_INFLATION_VIOLATION],
            );
            assert!(result.is_ok());
        }
    }

    #[proptest]
    fn coinbase_transaction_with_not_enough_funds_timelocked_is_invalid(
        #[strategy(1usize..=3)] _num_outputs: usize,
        #[strategy(1usize..=3)] _num_public_announcements: usize,
        #[strategy(PrimitiveWitness::arbitrary_with_size_numbers(
            None,
            #_num_outputs,
            #_num_public_announcements,
        ))]
        mut primitive_witness: PrimitiveWitness,
        #[strategy(arb())]
        #[filter(NativeCurrencyAmount::zero() < #delta)]
        delta: NativeCurrencyAmount,
    ) {
        // Modify the kernel so as to increase the coinbase but not the fee. The
        // resulting transaction is imbalanced but since the timelocked coinbase
        // amount is checked prior to the input/output balancing check, we know
        // which assert will be hit.
        let coinbase = primitive_witness.kernel.coinbase.unwrap();
        let kernel_modifier = TransactionKernelModifier::default().coinbase(Some(coinbase + delta));
        primitive_witness.kernel = kernel_modifier.modify(primitive_witness.kernel);
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_fail(native_currency_witness, &[COINBASE_TIMELOCK_INSUFFICIENT]);
    }

    #[proptest]
    fn coinbase_transaction_with_too_early_release_is_invalid_fixed_delta(
        #[strategy(1usize..=3)] _num_outputs: usize,
        #[strategy(1usize..=3)] _num_public_announcements: usize,
        #[strategy(PrimitiveWitness::arbitrary_with_size_numbers(
            None,
            #_num_outputs,
            #_num_public_announcements,
        ))]
        mut primitive_witness: PrimitiveWitness,
    ) {
        // Modify the kernel's timestamp to push it later in time. As a result,
        // the time-locks embedded in the coinbase UTXOs are less than the
        // coinbase time-lock time.
        let delta = Timestamp::days(1);
        let kernel_modifier = TransactionKernelModifier::default()
            .timestamp(primitive_witness.kernel.timestamp + delta);
        primitive_witness.kernel = kernel_modifier.modify(primitive_witness.kernel);
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_fail(native_currency_witness, &[COINBASE_TIMELOCK_INSUFFICIENT]);
    }

    #[proptest(cases = 50)]
    fn coinbase_transaction_with_too_early_release_is_invalid_prop_delta(
        #[strategy(1usize..=3)] _num_outputs: usize,
        #[strategy(1usize..=3)] _num_public_announcements: usize,
        #[strategy(PrimitiveWitness::arbitrary_with_size_numbers(
            None,
            #_num_outputs,
            #_num_public_announcements,
        ))]
        mut primitive_witness: PrimitiveWitness,
        #[strategy(arb())]
        #[filter(Timestamp::zero() < #delta)]
        delta: Timestamp,
    ) {
        // Modify the kernel's timestamp to push it later in time. As a result,
        // the time-locks embedded in the coinbase UTXOs are less than the
        // coinbase time-lock time.
        // Skip test-cases that wrap around on the timestamp value, as this
        // represents an earlier timestamp.
        prop_assume!(
            primitive_witness.kernel.timestamp + delta >= primitive_witness.kernel.timestamp
        );
        let kernel_modifier = TransactionKernelModifier::default()
            .timestamp(primitive_witness.kernel.timestamp + delta);
        primitive_witness.kernel = kernel_modifier.modify(primitive_witness.kernel);
        let native_currency_witness = NativeCurrencyWitness::from(primitive_witness);
        assert_both_rust_and_tasm_fail(native_currency_witness, &[COINBASE_TIMELOCK_INSUFFICIENT]);
    }

    #[test]
    fn hardcoded_time_lock_hash_matches_hash_of_time_lock_program() {
        let calculated = TimeLock.hash();
        assert_eq!(
            NativeCurrency::TIME_LOCK_HASH,
            calculated,
            "Timelock.hash():\n{}",
            calculated
        );
    }

    #[proptest(cases = 1)]
    fn assertion_failure_is_caught_gracefully() {
        // This test is supposed to catch wrong compilation flags causing
        // causing asserts not to be caught by catch_unwind.
        let result = panic::catch_unwind(|| {
            let f = false;
            assert!(f, "This assertion will fail");
        });
        prop_assert!(result.is_err());
    }

    #[test]
    fn fee_can_be_positive_deterministic() {
        let mut test_runner = TestRunner::deterministic();
        for _ in 0..10 {
            let fee = NativeCurrencyAmount::arbitrary_non_negative()
                .new_tree(&mut test_runner)
                .unwrap()
                .current();
            let pw = PrimitiveWitness::arbitrary_with_fee(fee)
                .new_tree(&mut test_runner)
                .unwrap()
                .current();
            assert_both_rust_and_tasm_halt_gracefully(NativeCurrencyWitness::from(pw)).unwrap();
        }
    }

    #[proptest]
    fn fee_can_be_positive(
        #[strategy(NativeCurrencyAmount::arbitrary_non_negative())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_with_fee(#_fee))]
        primitive_witness: PrimitiveWitness,
    ) {
        assert_both_rust_and_tasm_halt_gracefully(NativeCurrencyWitness::from(primitive_witness))?;
    }

    #[proptest]
    fn fee_can_be_negative(
        #[strategy(NativeCurrencyAmount::arbitrary_non_negative())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_with_fee(-#_fee))]
        primitive_witness: PrimitiveWitness,
    ) {
        assert_both_rust_and_tasm_halt_gracefully(NativeCurrencyWitness::from(primitive_witness))?;
    }

    #[proptest]
    fn positive_fee_cannot_exceed_max_nau(
        #[strategy(invalid_positive_amount())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_with_fee(#_fee))]
        primitive_witness: PrimitiveWitness,
    ) {
        // Why INVALID_COIN_AMOUNT and not FEE_EXCEEDS_MAX?
        // It's because an invalid fee needs to come from invalid inputs; so
        // the INVALID_COIN_AMOUNT assert is triggered when computing the sum
        // of all inputs.
        assert_both_rust_and_tasm_fail(
            NativeCurrencyWitness::from(primitive_witness),
            &[INVALID_COIN_AMOUNT],
        );
    }

    #[ignore]
    #[proptest]
    fn negative_fee_cannot_exceed_min_nau(
        #[strategy(invalid_positive_amount())] _fee: NativeCurrencyAmount,
        #[strategy(PrimitiveWitness::arbitrary_with_fee(-#_fee))]
        primitive_witness: PrimitiveWitness,
    ) {
        assert_both_rust_and_tasm_fail(
            NativeCurrencyWitness::from(primitive_witness),
            &[FEE_EXCEEDS_MIN],
        );

        // It is actually impossible to trigger this assert error id -- or is it?
        // I'm not convinced.
    }

    test_program_snapshot!(
        NativeCurrency,
        // snapshot taken from master on 2025-04-11 e2a712efc34f78c6a28801544418e7051127d284
        "f1d74e829aa26ab4ca51bd237e3da0e7f459c2a2eed8b3f7fe0e35e21a4f12a7a2e193fff80dc524"
    );
}
