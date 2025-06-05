use protocol::{Display, From, ProtocolError, ProtocolErrorKind};

#[derive(Debug, Display, From)]
pub enum MainError {
    #[display("The axon configuration read failed {:?}", _0)]
    ConfigParse(common_config_parser::ParseError),

    #[display("{:?}", _0)]
    Io(std::io::Error),

    #[display("Toml fails to parse genesis {:?}", _0)]
    GenesisTomlDe(toml::de::Error),

    #[display("crypto error {:?}", _0)]
    Crypto(common_crypto::Error),

    #[display("{:?}", _0)]
    Utf8(std::string::FromUtf8Error),

    #[display("{:?}", _0)]
    JSONParse(serde_json::error::Error),

    #[display("other error {:?}", _0)]
    Other(String),
}

impl std::error::Error for MainError {}

impl From<MainError> for ProtocolError {
    fn from(error: MainError) -> ProtocolError {
        ProtocolError::new(ProtocolErrorKind::Main, Box::new(error))
    }
}
