pub mod types;
use serde::de;

use std::{error, fmt, fs, io, path::Path};

/// Parse a config from reader.
pub fn parse_reader<R: io::Read, T: de::DeserializeOwned>(r: &mut R) -> Result<T, ParseError> {
    let mut buf = String::new();
    r.read_to_string(&mut buf)?;
    Ok(toml::from_str(&buf)?)
}

pub fn parse_json<R: io::Read, T: de::DeserializeOwned>(r: &mut R) -> Result<T, ParseError> {
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;
    Ok(serde_json::from_slice(&buf)?)
}

/// Parse a config from file.
///
/// Note: In most cases, function `parse` is better.
pub fn parse_file<T: de::DeserializeOwned>(
    name: impl AsRef<Path>,
    is_json: bool,
) -> Result<T, ParseError> {
    let mut f = fs::File::open(name)?;
    if is_json {
        parse_json(&mut f)
    } else {
        parse_reader(&mut f)
    }
}

#[derive(Debug)]
pub enum ParseError {
    IO(io::Error),
    Deserialize(toml::de::Error),
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::IO(e) => write!(f, "{}", e),
            ParseError::Deserialize(e) => write!(f, "{}", e),
            ParseError::Reqwest(e) => write!(f, "{}", e),
            ParseError::Json(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(error: io::Error) -> ParseError {
        ParseError::IO(error)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(error: serde_json::Error) -> ParseError {
        ParseError::Json(error)
    }
}

impl From<toml::de::Error> for ParseError {
    fn from(error: toml::de::Error) -> ParseError {
        ParseError::Deserialize(error)
    }
}

impl From<reqwest::Error> for ParseError {
    fn from(error: reqwest::Error) -> ParseError {
        ParseError::Reqwest(error)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        parse_file, parse_reader,
        types::{Config, ConfigRocksDB},
    };

    #[test]
    fn test_parse_config() {
        let file_path = "../../devtools/chain/config.toml";
        let config: Config = parse_file(file_path, false).unwrap();

        assert_eq!(config.executor.triedb_cache_size, 50_000);
        assert_eq!(config.rocksdb.block_cache_bytes, 1_073_741_824);
        assert_eq!(config.rocksdb.storage_cache_entries, 1_000);
        assert_eq!(config.rocksdb.max_open_files, 4096);
    }

    #[test]
    fn parse_rocksdb_cache_sizes_with_distinct_units() {
        let mut input = r#"
max_open_files = 2048
block_cache_bytes = 536870912
storage_cache_entries = 25000
options_file = "default.db-options"
"#
        .as_bytes();

        let config: ConfigRocksDB = parse_reader(&mut input).unwrap();

        assert_eq!(config.block_cache_bytes, 536_870_912);
        assert_eq!(config.storage_cache_entries, 25_000);
    }

    #[test]
    fn parse_default_storage_cache_entries() {
        let mut input = r#"
max_open_files = 2048
options_file = "default.db-options"
"#
        .as_bytes();

        let config: ConfigRocksDB = parse_reader(&mut input).unwrap();

        assert_eq!(config.storage_cache_entries, 1_000);
    }

    #[test]
    fn parse_legacy_cache_size_as_storage_cache_entries() {
        let mut input = r#"
max_open_files = 64
cache_size = 321
options_file = "default.db-options"
"#
        .as_bytes();

        let config: ConfigRocksDB = parse_reader(&mut input).unwrap();

        assert_eq!(config.block_cache_bytes, 1_073_741_824);
        assert_eq!(config.storage_cache_entries, 321);
    }
}
