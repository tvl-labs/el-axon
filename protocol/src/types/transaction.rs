use std::fmt::Debug;

pub use alloy::consensus::{
    Transaction, TxEip1559 as Eip1559Transaction, TxEip2930 as Eip2930Transaction,
    TxLegacy as LegacyTransaction,
};
pub use alloy::primitives::TxKind;
pub use alloy::rpc::types::{AccessList, AccessListItem};

use alloy::consensus::Receipt;
use alloy_rlp::Encodable;
use serde::{Deserialize, Serialize};

use common_crypto::secp256k1_recover;

use crate::codec::hex_encode;
use crate::codec::transaction::rlp_encode_legacy_tx;
use crate::types::{
    Address, Bytes, BytesMut, CellDepWithPubKey, Hash, Hasher, Public, TxResp, TypesError, H256,
    H512, U256,
};
use crate::ProtocolResult;

pub const MAX_PRIORITY_FEE_PER_GAS: u64 = 1_337;

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnsignedTransaction {
    Legacy(LegacyTransaction),
    Eip2930(Eip2930Transaction),
    Eip1559(Eip1559Transaction),
}

impl UnsignedTransaction {
    pub fn type_(&self) -> u64 {
        match self {
            UnsignedTransaction::Legacy(_) => 0x00,
            UnsignedTransaction::Eip2930(_) => 0x01,
            UnsignedTransaction::Eip1559(_) => 0x02,
        }
    }

    pub fn chain_id(&self) -> Option<u64> {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.chain_id,
            UnsignedTransaction::Eip2930(tx) => Some(tx.chain_id),
            UnsignedTransaction::Eip1559(tx) => Some(tx.chain_id),
        }
    }

    pub fn is_eip155(&self) -> bool {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.chain_id.is_some(),
            UnsignedTransaction::Eip2930(_) => true,
            UnsignedTransaction::Eip1559(_) => true,
        }
    }

    pub fn may_cost(&self) -> ProtocolResult<U256> {
        if let Some(res) = U256::from(self.gas_price()).checked_mul(U256::from(*self.gas_limit())) {
            return Ok(res.checked_add(*self.value()).unwrap_or(U256::MAX));
        }

        Err(TypesError::PrepayGasIsTooLarge.into())
    }

    pub fn is_legacy(&self) -> bool {
        matches!(self, UnsignedTransaction::Legacy(_))
    }

    pub fn is_eip1559(&self) -> bool {
        matches!(self, UnsignedTransaction::Eip1559(_))
    }

    pub fn data(&self) -> &[u8] {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.input.as_ref(),
            UnsignedTransaction::Eip2930(tx) => tx.input.as_ref(),
            UnsignedTransaction::Eip1559(tx) => tx.input.as_ref(),
        }
    }

    pub fn set_action(&mut self, action: TxKind) {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.to = action,
            UnsignedTransaction::Eip2930(tx) => tx.to = action,
            UnsignedTransaction::Eip1559(tx) => tx.to = action,
        }
    }

    pub fn set_data(&mut self, data: Bytes) {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.input = data.into(),
            UnsignedTransaction::Eip2930(tx) => tx.input = data.into(),
            UnsignedTransaction::Eip1559(tx) => tx.input = data.into(),
        }
    }

    pub fn gas_price(&self) -> u128 {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.gas_price,
            UnsignedTransaction::Eip2930(tx) => tx.gas_price,
            UnsignedTransaction::Eip1559(tx) => tx.max_fee_per_gas.max(tx.max_priority_fee_per_gas),
        }
    }

    pub fn max_priority_fee_per_gas(&self) -> &u128 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.gas_price,
            UnsignedTransaction::Eip2930(tx) => &tx.gas_price,
            UnsignedTransaction::Eip1559(tx) => &tx.max_priority_fee_per_gas,
        }
    }

    pub fn get_legacy(&self) -> Option<LegacyTransaction> {
        match self {
            UnsignedTransaction::Legacy(tx) => Some(tx.clone()),
            _ => None,
        }
    }

    pub fn as_u8(&self) -> u8 {
        match self {
            UnsignedTransaction::Legacy(_) => unreachable!(),
            UnsignedTransaction::Eip2930(_) => 1u8,
            UnsignedTransaction::Eip1559(_) => 2u8,
        }
    }

    pub fn encode(&self, signature: Option<SignatureComponents>) -> Bytes {
        let mut buf = BytesMut::new();
        let utx = UnverifiedTransaction {
            unsigned: self.clone(),
            signature,
            hash: Default::default(),
        };
        Encodable::encode(&utx, &mut buf);

        buf.freeze()
    }

    pub fn to(&self) -> Option<Address> {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.kind().into_to(),
            UnsignedTransaction::Eip2930(tx) => tx.to(),
            UnsignedTransaction::Eip1559(tx) => tx.to(),
        }
    }

    pub fn value(&self) -> &U256 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.value,
            UnsignedTransaction::Eip2930(tx) => &tx.value,
            UnsignedTransaction::Eip1559(tx) => &tx.value,
        }
    }

    pub fn gas_limit(&self) -> &u64 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.gas_limit,
            UnsignedTransaction::Eip2930(tx) => &tx.gas_limit,
            UnsignedTransaction::Eip1559(tx) => &tx.gas_limit,
        }
    }

    pub fn nonce(&self) -> &u64 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.nonce,
            UnsignedTransaction::Eip2930(tx) => &tx.nonce,
            UnsignedTransaction::Eip1559(tx) => &tx.nonce,
        }
    }

    pub fn action(&self) -> TxKind {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.kind(),
            UnsignedTransaction::Eip2930(tx) => tx.kind(),
            UnsignedTransaction::Eip1559(tx) => tx.kind(),
        }
    }

    pub fn access_list(&self) -> AccessList {
        match self {
            UnsignedTransaction::Legacy(_) => AccessList(vec![]),
            UnsignedTransaction::Eip2930(tx) => tx.access_list.clone(),
            UnsignedTransaction::Eip1559(tx) => tx.access_list.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnverifiedTransaction {
    pub unsigned:  UnsignedTransaction,
    pub signature: Option<SignatureComponents>,
    pub hash:      H256,
}

impl UnverifiedTransaction {
    pub fn calc_hash(mut self) -> Self {
        debug_assert!(self.signature.is_some());
        let hash = self.get_hash();
        self.hash = hash;
        self
    }

    pub fn get_hash(&self) -> H256 {
        let buf = self.unsigned.encode(self.signature.clone());
        Hasher::digest(&buf)
    }

    pub fn check_hash(&self) -> ProtocolResult<()> {
        let calc_hash = self.get_hash();
        if self.hash != calc_hash {
            return Err(TypesError::TxHashMismatch {
                origin: self.hash,
                calc:   calc_hash,
            }
            .into());
        }

        Ok(())
    }

    pub fn signature_hash(&self, with_chain_id: bool) -> Hash {
        if !with_chain_id {
            let legacy_tx = self.unsigned.get_legacy().unwrap();
            let mut buf = BytesMut::new();
            rlp_encode_legacy_tx(&legacy_tx, None, &mut buf, false);
            return Hasher::digest(buf.freeze());
        }

        Hasher::digest(self.unsigned.encode(None))
    }

    pub fn recover_public(&self, with_chain_id: bool) -> ProtocolResult<H512> {
        Ok(H512::from_slice(
            &secp256k1_recover(
                self.signature_hash(with_chain_id).as_slice(),
                self.signature
                    .as_ref()
                    .ok_or(TypesError::MissingSignature)?
                    .as_bytes()
                    .as_ref(),
            )
            .map_err(TypesError::Crypto)?
            .serialize_uncompressed()[1..65],
        ))
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq)]
pub struct SignatureComponents {
    pub r:          Bytes,
    pub s:          Bytes,
    pub standard_v: u8,
}

impl Debug for SignatureComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SignatureComponents {{ r: {}, s: {}, standard_v: {} }}",
            hex_encode(&self.r),
            hex_encode(&self.s),
            self.standard_v
        )
    }
}

/// This is only use for test.
impl From<Bytes> for SignatureComponents {
    // assume that all the bytes data are in Ethereum-like format
    fn from(bytes: Bytes) -> Self {
        debug_assert!(bytes.len() == 65);
        SignatureComponents {
            r:          Bytes::from(bytes[0..32].to_vec()),
            s:          Bytes::from(bytes[32..64].to_vec()),
            standard_v: bytes[64],
        }
    }
}

impl From<SignatureComponents> for Bytes {
    fn from(sc: SignatureComponents) -> Self {
        let mut bytes = BytesMut::from(sc.r.as_ref());
        bytes.extend_from_slice(sc.s.as_ref());
        bytes.extend_from_slice(&[sc.standard_v]);
        bytes.freeze()
    }
}

impl SignatureComponents {
    pub const SECP256K1_SIGNATURE_LEN: usize = 65;

    pub fn as_bytes(&self) -> Bytes {
        self.clone().into()
    }

    pub fn is_eth_sig(&self) -> bool {
        self.len() == Self::SECP256K1_SIGNATURE_LEN
    }

    pub fn add_chain_replay_protection(&self, chain_id: Option<u64>) -> u64 {
        (self.standard_v as u64) + chain_id.map(|i| i * 2 + 35).unwrap_or(27)
    }

    pub fn extract_standard_v(v: u64) -> Option<u8> {
        match v {
            v if v == 27 => Some(0),
            v if v == 28 => Some(1),
            v if v >= 35 => Some(((v - 1) % 2) as u8),
            _ => None,
        }
    }

    pub fn extract_chain_id(v: u64) -> Option<u64> {
        if v >= 35 {
            Some((v - 35) / 2u64)
        } else {
            None
        }
    }

    pub(crate) fn extract_interoperation_tx_sender(&self) -> ProtocolResult<Address> {
        // Only call CKB-VM mode is supported now
        if self.r[0] == 0 {
            let r = alloy_rlp::decode_exact::<CellDepWithPubKey>(&self.r[1..])
                .map_err(TypesError::DecodeInteroperationSigR)?;

            return Ok(Address::from_slice(&Hasher::digest(&r.pub_key).0[12..]));
        }

        Err(TypesError::InvalidSignatureRType.into())
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.r.len() + self.s.len() + 1
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SignedTransaction {
    pub transaction: UnverifiedTransaction,
    pub sender:      Address,
    pub public:      Option<Public>,
}

impl SignedTransaction {
    pub fn from_unverified(utx: UnverifiedTransaction) -> ProtocolResult<Self> {
        if utx.signature.is_none() {
            return Err(TypesError::Unsigned.into());
        }

        let hash = utx.signature_hash(true);
        let sig = utx.signature.as_ref().unwrap();

        if sig.is_eth_sig() {
            let pubkey = secp256k1_recover(hash.as_slice(), sig.as_bytes().as_ref())
                .map_err(TypesError::Crypto)?
                .serialize_uncompressed();
            let public = Public::from_slice(&pubkey[1..65]);

            return Ok(SignedTransaction {
                transaction: utx.calc_hash(),
                sender:      public_to_address(&public),
                public:      Some(public),
            });
        }

        // Otherwise it is an interoperation transaction
        Ok(SignedTransaction {
            sender:      sig.extract_interoperation_tx_sender()?,
            public:      Some(H512::default()),
            transaction: utx.calc_hash(),
        })
    }

    pub fn type_(&self) -> u64 {
        self.transaction.unsigned.type_()
    }

    pub fn get_to(&self) -> Option<Address> {
        self.transaction.unsigned.to()
    }

    pub fn is_eip155(&self) -> bool {
        self.transaction.unsigned.is_eip155()
    }

    /// Encode a transaction receipt into bytes.
    ///
    /// According to [`EIP-2718`]:
    /// - `Receipt` is either `TransactionType || ReceiptPayload` or
    ///   `LegacyReceipt`.
    /// - `LegacyReceipt` is kept to be RLP encoded bytes; it is `rlp([status,
    ///   cumulativeGasUsed, logsBloom, logs])`.
    /// - `ReceiptPayload` is an opaque byte array whose interpretation is
    ///   dependent on the `TransactionType` and defined in future EIPs.
    ///   - As [`EIP-2930`] defined: if `TransactionType` is `1`,
    ///     `ReceiptPayload` is `rlp([status, cumulativeGasUsed, logsBloom,
    ///     logs])`.
    ///   - As [`EIP-1559`] defined: if `TransactionType` is `2`,
    ///     `ReceiptPayload` is `rlp([status, cumulative_transaction_gas_used,
    ///     logs_bloom, logs])`.
    ///
    /// [`EIP-2718`]: https://eips.ethereum.org/EIPS/eip-2718#receipts
    /// [`EIP-2930`]: https://eips.ethereum.org/EIPS/eip-2930#parameters
    /// [`EIP-1559`]: https://eips.ethereum.org/EIPS/eip-1559#specification
    pub fn encode_receipt(&self, r: &TxResp) -> Bytes {
        let mut buf = BytesMut::new();
        let receipt = Receipt {
            status:              r.exit_reason.is_succeed().into(),
            cumulative_gas_used: r.gas_used,
            logs:                r.logs.clone(),
        };
        let bloom = receipt.bloom_slow();
        receipt.rlp_encode_fields_with_bloom(&bloom, &mut buf);
        buf.freeze()
    }
}

pub fn public_to_address(public: &Public) -> Address {
    Address::from_slice(&Hasher::digest(public)[12..])
}
