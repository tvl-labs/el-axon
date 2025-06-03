pub mod block;
pub mod error;
pub mod executor;
pub mod receipt;
pub mod transaction;

pub use transaction::truncate_slice;

use std::fmt::Debug;

use alloy_rlp::{Decodable, Encodable};
use serde::{Deserialize as _, Deserializer, Serializer};

use crate::types::{Bytes, BytesMut, DBBytes, Key256Bits, TypesError, U256};
use crate::{codec::error::CodecError, ProtocolResult};

static CHARS: &[u8] = b"0123456789abcdef";

pub trait ProtocolCodec: Sized + Send {
    fn encode(&self) -> ProtocolResult<Bytes>;

    fn decode<B: AsRef<[u8]>>(bytes: B) -> ProtocolResult<Self>;
}

impl<T: Encodable + Decodable + Send> ProtocolCodec for T {
    fn encode(&self) -> ProtocolResult<Bytes> {
        let mut buf = BytesMut::new();
        <Self as Encodable>::encode(self, &mut buf);
        Ok(buf.freeze())
    }

    fn decode<B: AsRef<[u8]>>(bytes: B) -> ProtocolResult<Self> {
        Ok(<Self as Decodable>::decode(&mut bytes.as_ref()).map_err(CodecError::Rlp)?)
    }
}

impl ProtocolCodec for DBBytes {
    fn encode(&self) -> ProtocolResult<Bytes> {
        Ok(self.0.clone())
    }

    fn decode<B: AsRef<[u8]>>(bytes: B) -> ProtocolResult<Self> {
        let inner = Bytes::copy_from_slice(bytes.as_ref());
        Ok(Self(inner))
    }
}

pub fn hex_encode<T: AsRef<[u8]>>(src: T) -> String {
    faster_hex::hex_string(src.as_ref())
}

pub fn hex_decode(src: &str) -> ProtocolResult<Vec<u8>> {
    if src.is_empty() {
        return Ok(Vec::new());
    }

    let src = if src.starts_with("0x") {
        src.split_at(2).1
    } else {
        src
    };

    let src = src.as_bytes();
    let mut ret = vec![0u8; src.len() / 2];
    faster_hex::hex_decode(src, &mut ret).map_err(TypesError::FromHex)?;

    Ok(ret)
}

pub fn serialize_uint<S, U>(val: &U, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    U: TryInto<U256> + Copy + Debug,
    <U as TryInto<U256>>::Error: Debug,
{
    let val: U256 = (*val).try_into().unwrap();
    let mut slice = [0u8; 2 + 64];
    let bytes: [u8; 32] = val.to_be_bytes();
    let non_zero = bytes.iter().take_while(|b| **b == 0).count();
    let bytes = &bytes[non_zero..];

    if bytes.is_empty() {
        s.serialize_str("0x0")
    } else {
        s.serialize_str(to_hex_raw(&mut slice, bytes, true))
    }
}

pub fn decode_256bits_key(s: &str) -> Result<Key256Bits, String> {
    const LEN: usize = 66;
    if s.starts_with("0x") || s.starts_with("0X") {
        if s.len() == LEN {
            let slice = &s.as_bytes()[2..];
            let mut v = [0u8; 32];
            faster_hex::hex_decode(slice, &mut v)
                .map(|_| Key256Bits::from(v))
                .map_err(|err| format!("failed to parse the 256 bits key since {err}."))
        } else {
            let err = format!(
                "failed to parse the 256 bits key since its length is {} but expect {LEN}.",
                s.len()
            );
            Err(err)
        }
    } else {
        let err = "failed to parse the 256 bits key since it's not 0x-prefixed.";
        Err(err.to_owned())
    }
}

pub fn deserialize_256bits_key<'de, D>(deserializer: D) -> Result<Key256Bits, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)
        .and_then(|s| decode_256bits_key(&s).map_err(serde::de::Error::custom))
}

fn to_hex_raw<'a>(v: &'a mut [u8], bytes: &[u8], skip_leading_zero: bool) -> &'a str {
    debug_assert!(v.len() > 1 + bytes.len() * 2);

    v[0] = b'0';
    v[1] = b'x';

    let mut idx = 2;
    let first_nibble = bytes[0] >> 4;
    if first_nibble != 0 || !skip_leading_zero {
        v[idx] = CHARS[first_nibble as usize];
        idx += 1;
    }
    v[idx] = CHARS[(bytes[0] & 0xf) as usize];
    idx += 1;

    for &byte in bytes.iter().skip(1) {
        v[idx] = CHARS[(byte >> 4) as usize];
        v[idx + 1] = CHARS[(byte & 0xf) as usize];
        idx += 2;
    }

    // SAFETY: all characters come either from CHARS or "0x", therefore valid UTF8
    unsafe { std::str::from_utf8_unchecked(&v[0..idx]) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Hex;
    use rand::random;

    impl Hex {
        fn random() -> Self {
            let data = (0..128).map(|_| random()).collect::<Vec<u8>>();
            Hex::encode(data)
        }
    }

    #[test]
    fn test_hex_codec() {
        let data = (0..128).map(|_| random()).collect::<Vec<u8>>();
        let data = data.to_vec();

        assert_eq!(hex_encode(&data), hex::encode(data.clone()));
        assert_eq!(
            hex_decode(&hex_encode(&data)).unwrap(),
            hex::decode(hex::encode(data)).unwrap()
        );
        assert!(hex_decode(String::new().as_str()).unwrap().is_empty());
    }

    #[test]
    fn test_hex_rlp() {
        let origin = Hex::random();
        let mut encoded = Vec::new();
        <Hex as Encodable>::encode(&origin, &mut encoded);
        let decode = <Hex as Decodable>::decode(&mut encoded.as_ref()).unwrap();

        assert_eq!(origin, decode);
    }
}
