use std::error::Error;
use std::io;

use molecule::error::VerificationError;
use rlp::DecoderError;

use protocol::Display;

#[derive(Debug, Display)]
pub enum ImageCellError {
    #[display(fmt = "Create DB path {}", _0)]
    CreateDB(io::Error),

    #[display(fmt = "rocksdb {}", _0)]
    RocksDB(rocksdb::Error),

    #[display(fmt = "Invalid block number: {:?}", _0)]
    InvalidBlockNumber(u64),

    #[display(fmt = "Update block number error: {:?}, number: {:?}", e, number)]
    UpdateBlockNumber { e: String, number: u64 },

    #[display(fmt = "Get block number error: {:?}", _0)]
    GetBlockNumber(String),

    #[display(fmt = "Decode block number failed: {:?}", _0)]
    RlpDecodeBlockNumber(DecoderError),

    #[display(fmt = "Restore MPT error: {:?}", _0)]
    RestoreMpt(String),

    #[display(fmt = "Insert cell error: {:?}", _0)]
    InsertCell(String),

    #[display(fmt = "Remove cell error: {:?}", _0)]
    RemoveCell(String),

    #[display(fmt = "Get cell error: {:?}", _0)]
    GetCell(String),

    #[display(fmt = "Decode cell failed: {:?}", _0)]
    RlpDecodeCell(DecoderError),

    #[display(fmt = "Insert header error: {:?}", _0)]
    InsertHeader(String),

    #[display(fmt = "Remove header error: {:?}", _0)]
    RemoveHeader(String),

    #[display(fmt = "Get header error: {:?}", _0)]
    GetHeader(String),

    #[display(fmt = "Commit error: {:?}", _0)]
    CommitError(String),

    #[display(fmt = "Molecule verification error: {:?}", _0)]
    MoleculeVerification(VerificationError),

    #[display(fmt = "TrieDB has not been initialized")]
    TrieDbNotInit,
}

impl Error for ImageCellError {}

pub type ImageCellResult<T> = Result<T, ImageCellError>;