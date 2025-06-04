pub mod adapter;
pub mod consensus;
pub mod engine;
pub mod message;
pub mod status;
pub mod synchronization;

#[cfg(test)]
mod tests;

pub mod stop_signal;
pub mod types;
pub mod util;
pub mod wal;

use std::error::Error;

use common_crypto::Error as CryptoError;

use protocol::types::{ExitReason, Hash, MerkleRoot};
use protocol::{Display, ProtocolError, ProtocolErrorKind};

pub use crate::adapter::OverlordConsensusAdapter;
pub use crate::consensus::OverlordConsensus;
pub use crate::synchronization::{OverlordSynchronization, SyncStatus, SYNC_STATUS};
pub use crate::wal::{ConsensusWal, SignedTxsWAL};
pub use overlord::{types::Node, DurationConfig};

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub enum ConsensusType {
    #[display("Signed Proposal")]
    SignedProposal,

    #[display("Signed Vote")]
    SignedVote,

    #[display("Aggregated Vote")]
    AggregateVote,

    #[display("Rich Height")]
    RichHeight,

    #[display("Rpc Pull Blocks")]
    RpcPullBlocks,

    #[display("Rpc Pull Transactions")]
    RpcPullTxs,

    #[display("Signed Choke")]
    SignedChoke,

    #[display("WAL Signed Transactions")]
    WALSignedTxs,
}

/// Consensus errors defines here.
#[derive(Debug, Display)]
pub enum ConsensusError {
    /// Check block error.
    #[display("Check invalid prev_hash, expect {:#x} get {:#x}", expect, actual)]
    InvalidPrevhash { expect: Hash, actual: Hash },

    #[display("Check invalid order root, expect {:#x} get {:#x}", expect, actual)]
    InvalidOrderRoot {
        expect: MerkleRoot,
        actual: MerkleRoot,
    },

    #[display("Check invalid state root, expect {:#x} get {:#x}", expect, actual)]
    InvalidStateRoot {
        expect: MerkleRoot,
        actual: MerkleRoot,
    },

    #[display("Check invalid receipts root, expect {:#x} get {:#x}", expect, actual)]
    InvalidReceiptsRoot {
        expect: MerkleRoot,
        actual: MerkleRoot,
    },

    #[display(
        "Check invalid order signed transactions hash, expect {:#x} get {:#x}",
        expect,
        actual
    )]
    InvalidOrderSignedTransactionsHash { expect: Hash, actual: Hash },

    #[display("Check invalid status vec")]
    InvalidStatusVec,

    /// Decode consensus message error.
    #[display("Decode {:?} message failed", _0)]
    DecodeErr(ConsensusType),

    /// Encode consensus message error.
    #[display("Encode {:?} message failed", _0)]
    EncodeErr(ConsensusType),

    /// Overlord consensus protocol error.
    #[display("Overlord error {:?}", _0)]
    OverlordErr(Box<dyn Error + Send>),

    /// Consensus missed last block proof.
    #[display("Invalid proof block number, expect {}, actual {}", expect, actual)]
    InvalidProof { expect: u64, actual: u64 },

    /// Consensus missed the pill.
    #[display("Consensus missed pill cooresponding {:#x}", _0)]
    MissingPill(Hash),

    /// Invalid timestamp
    #[display("Consensus invalid timestamp")]
    InvalidTimestamp,

    /// Consensus missed the block header.
    #[display("Consensus missed block header of {} block", _0)]
    MissingBlockHeader(u64),

    /// This boxed error should be a `CryptoError`.
    #[display("Crypto error {:?}", _0)]
    CryptoErr(Box<CryptoError>),

    #[display("Synchronization {} block error", _0)]
    VerifyTransaction(u64),

    #[display("Synchronization/Consensus {} block error : {}", _0, _1)]
    VerifyBlockHeader(u64, BlockHeaderField),

    #[display("Synchronization/Consensus {} block error : {}", _0, _1)]
    VerifyProof(u64, BlockProofField),

    ///
    #[display("Execute transactions error {:?}", _0)]
    ExecuteErr(String),

    ///
    WALErr(std::io::Error),

    #[display("Call EVM error {:?}", _0)]
    CallEvm(ExitReason),

    #[display("Storage item not found")]
    StorageItemNotFound,

    #[display("Lock in sync")]
    LockInSync,

    #[display("Wal transactions mismatch, height {}", _0)]
    WalTxsMismatch(u64),

    #[display(
        "Commit an outdated block, block_height {}, last_committed_height {}",
        _0,
        _1
    )]
    OutdatedCommit(u64, u64),

    /// Other error used for very few errors.
    #[display("{:?}", _0)]
    Other(String),

    #[display("{:?}", _0)]
    SystemTime(std::time::SystemTimeError),

    #[display("parse file name into timestamp error")]
    FileNameTimestamp,

    #[display("consensus wal dir doesn't exist")]
    ConsensusWalDirNotExist,

    #[display("no consensus wal file available")]
    ConsensusWalNoWalFile,

    #[display("Confused metadata range [{}, {})!", _0, _1)]
    ConfusedMetadata(u64, u64),

    #[display("Build trie merkle tree error {}", _0)]
    BuildMerkle(String),

    #[display("Proposal hardfork info error {}", _0)]
    Hardfork(String),
}

#[derive(Debug, Display)]
pub enum BlockHeaderField {
    #[display("The prev_hash mismatch the previous block")]
    PreviousBlockHash,

    #[display("The prev_hash mismatch the hash in the proof field")]
    ProofHash,

    #[display("The proposer is not in the committee")]
    Proposer,

    #[display("There is at least one validator not in the committee")]
    Validator,

    #[display("There is at least one validator's weight mismatch")]
    Weight,

    #[display("The block version")]
    Version,
}

#[derive(Debug, Display)]
pub enum BlockProofField {
    #[display("The bit_map has error with committer, can't get signed voters")]
    BitMap,

    #[display("The proof signature is fraud or error")]
    Signature,

    #[display("Heights of block and proof diverse, block {}, proof {}", _0, _1)]
    HeightMismatch(u64, u64),

    #[display("Hash of block and proof diverse")]
    HashMismatch,

    #[display("There is at least one validator not in the committee")]
    Validator,

    #[display("There is at least one validator's weight mismatch")]
    Weight,

    #[display("There is at least one validator's weight missing")]
    WeightNotFound,
}

impl Error for ConsensusError {}

impl From<ConsensusError> for ProtocolError {
    fn from(err: ConsensusError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Consensus, Box::new(err))
    }
}
