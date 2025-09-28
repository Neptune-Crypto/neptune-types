use crate::sanction::Sanction;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::block_height::BlockHeight;
use twenty_first::tip5::Digest;

/// The reason for degrading a peer's standing
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NegativePeerSanction {
    InvalidBlock((BlockHeight, Digest)),
    DifferentGenesis,
    ForkResolutionError((BlockHeight, u16, Digest)),
    SynchronizationTimeout,

    InvalidSyncChallenge,
    InvalidSyncChallengeResponse,
    TimedOutSyncChallengeResponse,
    UnexpectedSyncChallengeResponse,
    FishyPowEvolutionChallengeResponse,
    FishyDifficultiesChallengeResponse,

    FloodPeerListResponse,
    BlockRequestUnknownHeight,

    // Be careful about using this too much as it's bad for log opportunities.
    InvalidMessage,
    NonMinedTransactionHasCoinbase,
    TooShortBlockBatch,
    ReceivedBatchBlocksOutsideOfSync,
    BatchBlocksInvalidStartHeight,
    BatchBlocksUnknownRequest,
    BatchBlocksRequestEmpty,
    BatchBlocksRequestTooManyDigests,

    InvalidTransaction,
    UnconfirmableTransaction,
    TransactionWithNegativeFee,
    DoubleSpendingTransaction,
    CannotApplyTransactionToMutatorSet,

    InvalidBlockMmrAuthentication,

    InvalidTransferBlock,

    BlockProposalNotFound,
    InvalidBlockProposal,
    NonFavorableBlockProposal,

    UnwantedMessage,

    NoStandingFoundMaybeCrash,
}

impl Display for NegativePeerSanction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            NegativePeerSanction::InvalidBlock(_) => "invalid block",
            NegativePeerSanction::DifferentGenesis => "different genesis",
            NegativePeerSanction::ForkResolutionError(_) => "fork resolution error",
            NegativePeerSanction::SynchronizationTimeout => "synchronization timeout",
            NegativePeerSanction::FloodPeerListResponse => "flood peer list response",
            NegativePeerSanction::BlockRequestUnknownHeight => "block request unknown height",
            NegativePeerSanction::InvalidMessage => "invalid message",
            NegativePeerSanction::TooShortBlockBatch => "too short block batch",
            NegativePeerSanction::ReceivedBatchBlocksOutsideOfSync => {
                "received block batch outside of sync"
            }
            NegativePeerSanction::BatchBlocksInvalidStartHeight => {
                "invalid start height of batch blocks"
            }
            NegativePeerSanction::BatchBlocksUnknownRequest => "batch blocks unknown request",
            NegativePeerSanction::InvalidTransaction => "invalid transaction",
            NegativePeerSanction::UnconfirmableTransaction => "unconfirmable transaction",
            NegativePeerSanction::TransactionWithNegativeFee => "negative-fee transaction",
            NegativePeerSanction::DoubleSpendingTransaction => "double-spending transaction",
            NegativePeerSanction::CannotApplyTransactionToMutatorSet => {
                "cannot apply tx to mutator set"
            }
            NegativePeerSanction::NonMinedTransactionHasCoinbase => {
                "non-mined transaction has coinbase"
            }
            NegativePeerSanction::NoStandingFoundMaybeCrash => {
                "No standing found in map. Did peer task crash?"
            }
            NegativePeerSanction::BlockProposalNotFound => "Block proposal not found",
            NegativePeerSanction::InvalidBlockProposal => "Invalid block proposal",
            NegativePeerSanction::UnwantedMessage => "unwanted message",
            NegativePeerSanction::NonFavorableBlockProposal => "non-favorable block proposal",
            NegativePeerSanction::BatchBlocksRequestEmpty => "batch block request empty",
            NegativePeerSanction::InvalidSyncChallenge => "invalid sync challenge",
            NegativePeerSanction::InvalidSyncChallengeResponse => "invalid sync challenge response",
            NegativePeerSanction::UnexpectedSyncChallengeResponse => {
                "unexpected sync challenge response"
            }
            NegativePeerSanction::InvalidTransferBlock => "invalid transfer block",
            NegativePeerSanction::TimedOutSyncChallengeResponse => {
                "timed-out sync challenge response"
            }
            NegativePeerSanction::InvalidBlockMmrAuthentication => {
                "invalid block mmr authentication"
            }
            NegativePeerSanction::BatchBlocksRequestTooManyDigests => {
                "too many digests in batch block request"
            }
            NegativePeerSanction::FishyPowEvolutionChallengeResponse => "fishy pow evolution",
            NegativePeerSanction::FishyDifficultiesChallengeResponse => "fishy difficulties",
        };
        write!(f, "{string}")
    }
}

impl Sanction for NegativePeerSanction {
    fn severity(self) -> i32 {
        match self {
            NegativePeerSanction::InvalidBlock(_) => -10,
            NegativePeerSanction::DifferentGenesis => i32::MIN,
            NegativePeerSanction::ForkResolutionError((_height, count, _digest)) => {
                i32::from(count).saturating_mul(-1)
            }
            NegativePeerSanction::SynchronizationTimeout => -5,
            NegativePeerSanction::FloodPeerListResponse => -2,
            NegativePeerSanction::InvalidMessage => -2,
            NegativePeerSanction::TooShortBlockBatch => -2,
            NegativePeerSanction::ReceivedBatchBlocksOutsideOfSync => -2,
            NegativePeerSanction::BatchBlocksInvalidStartHeight => -2,
            NegativePeerSanction::BatchBlocksUnknownRequest => -10,
            NegativePeerSanction::BlockRequestUnknownHeight => -1,
            NegativePeerSanction::InvalidTransaction => -10,
            NegativePeerSanction::UnconfirmableTransaction => -2,
            NegativePeerSanction::TransactionWithNegativeFee => -22,
            NegativePeerSanction::DoubleSpendingTransaction => -14,
            NegativePeerSanction::CannotApplyTransactionToMutatorSet => -3,
            NegativePeerSanction::NonMinedTransactionHasCoinbase => -10,
            NegativePeerSanction::NoStandingFoundMaybeCrash => -10,
            NegativePeerSanction::BlockProposalNotFound => -1,
            NegativePeerSanction::InvalidBlockProposal => -10,
            NegativePeerSanction::UnwantedMessage => -1,
            NegativePeerSanction::NonFavorableBlockProposal => -1,
            NegativePeerSanction::BatchBlocksRequestEmpty => -10,
            NegativePeerSanction::InvalidSyncChallenge => -50,
            NegativePeerSanction::InvalidSyncChallengeResponse => -500,
            NegativePeerSanction::UnexpectedSyncChallengeResponse => -1,
            NegativePeerSanction::InvalidTransferBlock => -50,
            NegativePeerSanction::TimedOutSyncChallengeResponse => -50,
            NegativePeerSanction::InvalidBlockMmrAuthentication => -4,
            NegativePeerSanction::BatchBlocksRequestTooManyDigests => -50,
            NegativePeerSanction::FishyPowEvolutionChallengeResponse => -51,
            NegativePeerSanction::FishyDifficultiesChallengeResponse => -51,
        }
    }
}
