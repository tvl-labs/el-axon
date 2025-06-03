use std::{error::Error, num::ParseIntError};

use derive_more::Display;
use tentacle::{
    bytes::Bytes,
    multiaddr::Multiaddr,
    secio::{PeerId, PublicKey},
    ProtocolId, SessionId,
};

use protocol::{types::Address, ProtocolError, ProtocolErrorKind};

use crate::common::ConnectedAddr;

#[derive(Debug, Display)]
pub enum ErrorKind {
    #[display("{} offline", _0)]
    Offline(&'static str),

    #[display("protocol {} missing", _0)]
    MissingProtocol(&'static str),

    #[display("kind: bad protocl logic code")]
    BadProtocolHandle {
        proto_id: ProtocolId,
        cause:    Box<dyn Error + Send>,
    },

    #[display("kind: given string isn't an id: {}", _0)]
    NotIdString(ParseIntError),

    #[display("kind: unable to encode or decode: {}", _0)]
    BadMessage(Box<dyn Error + Send>),

    #[display("kind: unknown rid {} from session {}", rid, sid)]
    UnknownRpc { sid: SessionId, rid: u64 },

    #[display("kind: unexpected rpc sender, wrong type")]
    UnexpectedRpcSender,

    #[display("kind: more than one arc rpc sender, cannot unwrap it")]
    MoreArcRpcSender,

    #[display("kind: session id not found in context")]
    NoSessionId,

    #[display("kind: remote peer id not found in context")]
    NoRemotePeerId,

    #[display("kind: rpc id not found in context")]
    NoRpcId,

    #[display("kind: rpc future dropped {:?}", _0)]
    RpcDropped(Option<ConnectedAddr>),

    #[display("kind: rpc timeout {:?}", _0)]
    RpcTimeout(Option<ConnectedAddr>),

    #[display("kind: not reactor register for {}", _0)]
    NoReactor(String),

    #[display("kind: cannot create chain address from bytes {:#x} {}", pubkey, cause)]
    NoChainAddress {
        pubkey: Bytes,
        cause:  Box<dyn Error + Send>,
    },

    #[display("kind: public key {:?} not match {:?}", pubkey, id)]
    PublicKeyNotMatchId { pubkey: PublicKey, id: PeerId },

    #[display("kind: untaggable {}", _0)]
    Untaggable(String),

    #[display("kind: PeerStore Eviction Failed")]
    PeerStoreEvictionFailed,

    #[display("kind: PeerStore Serde Failed")]
    PeerStoreSerde,

    #[display("kind: internal {}", _0)]
    Internal(String),
}

impl Error for ErrorKind {}

#[derive(Debug, Display)]
#[display("peer id not found in {}", _0)]
pub struct PeerIdNotFound(pub(crate) Multiaddr);

impl Error for PeerIdNotFound {}

#[derive(Debug, Display)]
pub enum NetworkError {
    #[display("io error: {}", _0)]
    IoError(std::io::Error),

    #[display("temporary unavailable, try again later")]
    Busy,

    #[display("send incompletely, blocked {:?}, other {:?}", blocked, other)]
    Send {
        blocked: Option<Vec<SessionId>>,
        other:   Option<Box<dyn Error + Send>>,
    },

    #[display("send incompletely, unconnected {:?}, other {:?}", unconnected, other)]
    MultiCast {
        unconnected: Option<Vec<PeerId>>,
        other:       Option<Box<dyn Error + Send>>,
    },

    #[display("shutdown")]
    Shutdown,

    #[display("unexected error: {}", _0)]
    UnexpectedError(Box<dyn Error + Send>),

    #[display("cannot decode public key bytes")]
    InvalidPublicKey,

    #[display("cannot decode private key bytes")]
    InvalidPrivateKey,

    #[display("cannot decode peer id")]
    InvalidPeerId,

    #[display("unsupported peer address {}", _0)]
    UnexpectedPeerAddr(String),

    #[display("unknown endpoint scheme {}", _0)]
    UnexpectedScheme(String),

    #[display("cannot serde encode or decode: {}", _0)]
    SerdeError(Box<dyn Error + Send>),

    #[display("malformat or exceed maximum length, /[scheme]/[name]/[method] etc")]
    NotEndpoint,

    #[display("{:?} account addrs aren't connecting, try connect them", miss)]
    PartialRouteMessage { miss: Vec<Address> },

    #[display("remote response {}", _0)]
    RemoteResponse(String),

    #[display("trust max history should be longer than {} secs", _0)]
    SmallTrustMaxHistory(u64),

    #[display("transport {}", _0)]
    Transport(tentacle::error::TransportErrorKind),

    #[display("inbound connection limit is equal or smaller than max connections")]
    InboundLimitEqualOrSmallerThanMaxConn,

    #[display("internal error: {}", _0)]
    Internal(Box<dyn Error + Send>),
}

impl Error for NetworkError {}

impl From<PeerIdNotFound> for NetworkError {
    fn from(err: PeerIdNotFound) -> NetworkError {
        NetworkError::Internal(Box::new(err))
    }
}

impl From<ErrorKind> for NetworkError {
    fn from(kind: ErrorKind) -> NetworkError {
        NetworkError::Internal(Box::new(kind))
    }
}

impl From<NetworkError> for ProtocolError {
    fn from(err: NetworkError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Network, Box::new(err))
    }
}

impl From<std::io::Error> for NetworkError {
    fn from(err: std::io::Error) -> NetworkError {
        NetworkError::IoError(err)
    }
}

impl From<tentacle::error::TransportErrorKind> for NetworkError {
    fn from(err: tentacle::error::TransportErrorKind) -> NetworkError {
        NetworkError::Transport(err)
    }
}

impl From<NetworkError> for Box<dyn Error + Send> {
    fn from(err: NetworkError) -> Box<dyn Error + Send> {
        err.boxed()
    }
}

impl NetworkError {
    pub fn boxed(self) -> Box<dyn Error + Send> {
        Box::new(self) as Box<dyn Error + Send>
    }
}
