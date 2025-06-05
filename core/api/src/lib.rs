pub mod adapter;
pub mod graphql;
pub mod jsonrpc;

pub use adapter::DefaultAPIAdapter;

use std::error::Error;

use protocol::{Display, ProtocolError, ProtocolErrorKind};

#[derive(Debug, Display)]
pub enum APIError {
    #[display("adapter error {_0}")]
    Adapter(String),

    #[display("http server error {_0}")]
    HttpServer(String),

    #[display("web socket server error {_0}")]
    WebSocketServer(String),

    #[display("storage error {_0}")]
    Storage(String),

    #[display("Invalid request payload {_0}")]
    RequestPayload(String),
}

impl Error for APIError {}

impl From<APIError> for ProtocolError {
    fn from(error: APIError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::API, Box::new(error))
    }
}
