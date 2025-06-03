use std::error::Error;

use derive_more::Display;

use crate::{ProtocolError, ProtocolErrorKind};

#[derive(Debug, Display)]
pub enum CodecError {
    #[display("rlp: {}", _0)]
    Rlp(alloy_rlp::Error),
}

impl Error for CodecError {}

impl From<CodecError> for ProtocolError {
    fn from(err: CodecError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Codec, Box::new(err))
    }
}
