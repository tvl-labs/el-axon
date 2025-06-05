pub use batch::*;
pub use block::*;
pub use bytes::{Buf, BufMut, Bytes, BytesMut};
pub use ckb_client::*;
pub use evm::{
    backend::{Apply, ApplyBackend, Backend, Basic, MemoryAccount, MemoryBackend},
    ExitError, ExitRevert, ExitSucceed,
};
pub use executor::{
    AccessList, AccessListItem, Account, Config, EthAccountProof, EthStorageProof, ExecResp,
    ExecutorContext, ExitReason, HasherKeccak, Log, TxResp,
};
pub use interoperation::*;
pub use primitive::*;
pub use receipt::*;
pub use transaction::*;

pub mod batch;
pub mod block;
pub mod ckb_client;
pub mod executor;
pub mod interoperation;
pub mod primitive;
pub mod receipt;
pub mod transaction;

use std::error::Error;

use derive_more::{Display, From};

use common_crypto::Error as CryptoError;

use crate::{ProtocolError, ProtocolErrorKind};

#[derive(Debug, Display, From)]
pub enum TypesError {
    #[display("Expect {}, get {}.", expect, real)]
    LengthMismatch { expect: usize, real: usize },

    #[display(
        "Eip1559Transaction hash mismatch origin {:#x}, computed {:#x}",
        origin,
        calc
    )]
    TxHashMismatch { origin: H256, calc: H256 },

    #[display("{:?}", _0)]
    FromHex(faster_hex::Error),

    #[display("{:?} is an invalid address", _0)]
    InvalidAddress(String),

    #[display("Hex should start with 0x")]
    HexPrefix,

    #[display("Invalid public key")]
    InvalidPublicKey,

    #[display("Invalid check sum")]
    InvalidCheckSum,

    #[display("Unsigned")]
    Unsigned,

    #[display("Crypto error {:?}", _0)]
    Crypto(CryptoError),

    #[display("Missing signature")]
    MissingSignature,

    #[display("Invalid crosschain direction")]
    InvalidDirection,

    #[display("Signature R is empty")]
    SignatureRIsEmpty,

    #[display("Invalid signature R type")]
    InvalidSignatureRType,

    #[display("Invalid address source type")]
    InvalidAddressSourceType,

    #[display("Missing interoperation sender")]
    MissingInteroperationSender,

    #[display("InvalidBlockVersion {}", _0)]
    InvalidBlockVersion(u8),

    #[display("Decode interoperation signature R error {:?}", _0)]
    DecodeInteroperationSigR(alloy_rlp::Error),

    #[display("Prepay gas is too large")]
    PrepayGasIsTooLarge,
}

impl Error for TypesError {}

impl From<TypesError> for ProtocolError {
    fn from(error: TypesError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Types, Box::new(error))
    }
}
