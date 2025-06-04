use crate::types::{Block, SignedTransaction};

macro_rules! batch_msg_type {
    ($name: ident, $ty: ident) => {
        #[derive(alloy_rlp::RlpEncodable, alloy_rlp::RlpDecodable, Clone, Debug, PartialEq, Eq)]
        pub struct $name(pub Vec<$ty>);

        // impl crate::traits::MessageCodec for $name {
        //     fn encode_msg(&mut self) -> crate::ProtocolResult<Bytes> {
        //         let mut buf = bytes::BytesMut::new();
        //         <Self as alloy_rlp::Encodable>::encode(&self, &mut buf);
        //         Ok(buf.freeze())
        //     }

        //     fn decode_msg(bytes: Bytes) -> crate::ProtocolResult<Self> {
        //         let ret = <Self as alloy_rlp::Decodable>::decode(&mut bytes.as_ref())
        //             .map_err(crate::codec::error::CodecError::Rlp)?;
        //         Ok(ret)
        //     }
        // }

        impl $name {
            pub fn new(inner: Vec<$ty>) -> Self {
                Self(inner)
            }

            pub fn inner(self) -> Vec<$ty> {
                self.0
            }
        }
    };
}

batch_msg_type!(BatchSignedTxs, SignedTransaction);
batch_msg_type!(BatchBlocks, Block);

#[cfg(test)]
mod tests {
    use super::*;

    use alloy::primitives::TxKind;
    use rand7::rngs::OsRng;

    use common_crypto::{
        Crypto, PrivateKey, Secp256k1Recoverable, Secp256k1RecoverablePrivateKey, Signature,
    };

    use crate::codec::ProtocolCodec;
    use crate::types::{
        Eip1559Transaction, SignatureComponents, UnsignedTransaction, UnverifiedTransaction,
    };

    fn mock_sign_tx() -> SignedTransaction {
        let mut utx = UnverifiedTransaction {
            unsigned:  UnsignedTransaction::Eip1559(Eip1559Transaction {
                chain_id:                 0,
                nonce:                    Default::default(),
                max_priority_fee_per_gas: Default::default(),
                max_fee_per_gas:          Default::default(),
                gas_limit:                Default::default(),
                to:                       TxKind::Create,
                value:                    Default::default(),
                input:                    Default::default(),
                access_list:              Default::default(),
            }),
            signature: Some(SignatureComponents {
                standard_v: 4,
                r:          Default::default(),
                s:          Default::default(),
            }),
            hash:      Default::default(),
        }
        .calc_hash();

        let priv_key = Secp256k1RecoverablePrivateKey::generate(&mut OsRng);
        let signature = Secp256k1Recoverable::sign_message(
            utx.signature_hash(true).as_slice(),
            &priv_key.to_bytes(),
        )
        .unwrap()
        .to_bytes();
        utx.signature = Some(signature.into());

        SignedTransaction::from_unverified(utx).unwrap()
    }

    #[test]
    fn test_codec() {
        let stx = mock_sign_tx();
        let mut raw = alloy_rlp::encode(&stx);
        let decode = SignedTransaction::decode(&mut raw).unwrap();
        assert_eq!(stx, decode);
    }
}
