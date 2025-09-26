use crate::block_header::BlockHeader;
use crate::block_height::BlockHeight;
use crate::mining_status::MiningStatus;
use crate::native_currency_amount::NativeCurrencyAmount;
use crate::tx_proving_capability::TxProvingCapability;
use serde::Deserialize;
use serde::Serialize;
use twenty_first::tip5::digest::Digest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashBoardOverviewDataFromClient {
    pub tip_digest: Digest,
    pub tip_header: BlockHeader,
    pub syncing: bool,
    pub confirmed_available_balance: NativeCurrencyAmount,
    pub confirmed_total_balance: NativeCurrencyAmount,
    pub unconfirmed_available_balance: NativeCurrencyAmount,
    pub unconfirmed_total_balance: NativeCurrencyAmount,
    pub mempool_size: usize,
    pub mempool_total_tx_count: usize,
    pub mempool_own_tx_count: usize,

    // `None` symbolizes failure in getting peer count
    pub peer_count: Option<usize>,
    pub max_num_peers: usize,

    // `None` symbolizes failure to get mining status
    pub mining_status: Option<MiningStatus>,

    pub proving_capability: TxProvingCapability,

    // # of confirmations of the last wallet balance change.
    //
    // Starts at 1, as the block in which a tx is included is considered the 1st
    // confirmation.
    //
    // `None` indicates that wallet balance has never changed.
    pub confirmations: Option<BlockHeight>,

    /// CPU temperature in degrees Celsius
    pub cpu_temp: Option<f32>,
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
        pub use crate::dashboard_overview_data_from_client::DashBoardOverviewDataFromClient;
    }

    #[test]
    fn test_bincode_serialization_for_dashboardoverviewdatafromclient() {
        todo!()
        // let original_instance: DashBoardOverviewDataFromClient = DashBoardOverviewDataFromClient::default();
        // let nc_instance = nc::DashBoardOverviewDataFromClient::default();
        // test_bincode_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_serialization_for_dashboardoverviewdatafromclient() {
        todo!()
        // let original_instance: DashBoardOverviewDataFromClient = DashBoardOverviewDataFromClient::default();
        // let nc_instance = nc::DashBoardOverviewDataFromClient::default();
        // test_serde_json_serialization_for_type(original_instance, Some(nc_instance));
    }

    #[test]
    fn test_serde_json_wasm_serialization_for_dashboardoverviewdatafromclient() {
        todo!()
        // let original_instance: DashBoardOverviewDataFromClient = DashBoardOverviewDataFromClient::default();
        // let nc_instance = nc::DashBoardOverviewDataFromClient::default();
        // test_serde_json_wasm_serialization_for_type(original_instance, Some(nc_instance));
    }

}
