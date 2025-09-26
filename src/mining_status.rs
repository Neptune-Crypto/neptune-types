use std::fmt::Display;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use web_time::SystemTime;

#[cfg(not(target_arch = "wasm32"))]
use std::time::SystemTime;

use serde::Deserialize;
use serde::Serialize;

// use crate::models::blockchain::block::Block;
use crate::native_currency_amount::NativeCurrencyAmount;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GuessingWorkInfo {
    work_start: SystemTime,
    num_inputs: usize,
    num_outputs: usize,
    total_coinbase: NativeCurrencyAmount,
    pub(crate) total_guesser_fee: NativeCurrencyAmount,
}

/*
impl GuessingWorkInfo {
    pub(crate) fn new(work_start: SystemTime, block: &Block) -> Self {
        Self {
            work_start,
            num_inputs: block.body().transaction_kernel.inputs.len(),
            num_outputs: block.body().transaction_kernel.outputs.len(),
            total_coinbase: block.body().transaction_kernel.coinbase.unwrap_or_default(),
            total_guesser_fee: block.body().transaction_kernel.fee,
        }
    }
}
*/

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ComposingWorkInfo {
    // Only this info is available at the beginning of the composition work.
    // The rest of the information will have to be read from the log.
    work_start: SystemTime,
}

impl ComposingWorkInfo {
    pub fn new(work_start: SystemTime) -> Self {
        Self { work_start }
    }
}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, strum::EnumIs)]
pub enum MiningStatus {
    Guessing(GuessingWorkInfo),
    Composing(ComposingWorkInfo),

    #[default]
    Inactive,
}

impl Display for MiningStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed_time_exact = match self {
            MiningStatus::Guessing(guessing_work_info) => Some(
                guessing_work_info
                    .work_start
                    .elapsed()
                    .unwrap_or(Duration::ZERO),
            ),
            MiningStatus::Composing(composing_work_info) => Some(
                composing_work_info
                    .work_start
                    .elapsed()
                    .unwrap_or(Duration::ZERO),
            ),
            MiningStatus::Inactive => None,
        };
        // remove sub-second component, so humantime ends with seconds.
        let elapsed_time =
            elapsed_time_exact.map(|v| v - Duration::from_nanos(v.subsec_nanos().into()));
        let input_output_info = match self {
            MiningStatus::Guessing(info) => {
                format!(" {}/{}", info.num_inputs, info.num_outputs)
            }
            _ => String::default(),
        };

        let work_type_and_duration = match self {
            MiningStatus::Guessing(_) => {
                format!(
                    "guessing for {}",
                    humantime::format_duration(elapsed_time.unwrap())
                )
            }
            MiningStatus::Composing(_) => {
                format!(
                    "composing for {}",
                    humantime::format_duration(elapsed_time.unwrap())
                )
            }
            MiningStatus::Inactive => "inactive".to_owned(),
        };
        let reward = match self {
            MiningStatus::Guessing(block_work_info) => format!(
                "; total guesser reward: {}",
                block_work_info.total_guesser_fee
            ),
            _ => String::default(),
        };

        write!(f, "{work_type_and_duration}{input_output_info}{reward}",)
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
        pub use neptune_cash::api::export::GuessingWorkInfo;
        pub use neptune_cash::api::export::ComposingWorkInfo;
        pub use neptune_cash::api::export::MiningStatus;
    }

    #[test]
    fn test_bincode_serialization_for_guessingworkinfo() {
        let original_instance: GuessingWorkInfo = GuessingWorkInfo::default();
        let nc_instance: nc::GuessingWorkInfo = neptune_cash::api::export::GuessingWorkInfo::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_guessingworkinfo() {
        let original_instance: GuessingWorkInfo = GuessingWorkInfo::default();
        let nc_instance: nc::GuessingWorkInfo = neptune_cash::api::export::GuessingWorkInfo::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_guessingworkinfo() {
        let original_instance: GuessingWorkInfo = GuessingWorkInfo::default();
        let nc_instance: nc::GuessingWorkInfo = neptune_cash::api::export::GuessingWorkInfo::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_bincode_serialization_for_composingworkinfo() {
        let original_instance: ComposingWorkInfo = ComposingWorkInfo::default();
        let nc_instance: nc::ComposingWorkInfo = neptune_cash::api::export::ComposingWorkInfo::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_composingworkinfo() {
        let original_instance: ComposingWorkInfo = ComposingWorkInfo::default();
        let nc_instance: nc::ComposingWorkInfo = neptune_cash::api::export::ComposingWorkInfo::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_composingworkinfo() {
        let original_instance: ComposingWorkInfo = ComposingWorkInfo::default();
        let nc_instance: nc::ComposingWorkInfo = neptune_cash::api::export::ComposingWorkInfo::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_bincode_serialization_for_miningstatus() {
        let original_instance: MiningStatus = MiningStatus::default();
        let nc_instance: nc::MiningStatus = neptune_cash::api::export::MiningStatus::default();
        test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_miningstatus() {
        let original_instance: MiningStatus = MiningStatus::default();
        let nc_instance: nc::MiningStatus = neptune_cash::api::export::MiningStatus::default();
        test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_miningstatus() {
        let original_instance: MiningStatus = MiningStatus::default();
        let nc_instance: nc::MiningStatus = neptune_cash::api::export::MiningStatus::default();
        test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

}
