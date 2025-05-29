use alloy::consensus::SignableTransaction;
pub use alloy::consensus::{
    Transaction, TxEip1559 as Eip1559Transaction, TxEip2930 as Eip2930Transaction,
    TxLegacy as LegacyTransaction,
};
pub use alloy::primitives::TxKind;
pub use alloy::rpc::types::{AccessList, AccessListItem};

use alloy::consensus::crypto::secp256k1::{public_key_to_address, recover_signer};
use serde::{Deserialize, Serialize};

use common_crypto::secp256k1_recover;

use crate::types::{
    Address, Bloom, Bytes, BytesMut, CellDepWithPubKey, ExitReason, Hash, Hasher, TxResp,
    TypesError, H256, H512, U256, U64,
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

    pub fn may_cost(&self) -> ProtocolResult<U256> {
        if let Some(res) = U256::from(self.gas_price().low_u64())
            .checked_mul(U256::from(self.gas_limit().low_u64()))
        {
            return Ok(res
                .checked_add(*self.value())
                .unwrap_or_else(U256::max_value));
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
            UnsignedTransaction::Legacy(tx) => tx.kind = action,
            UnsignedTransaction::Eip2930(tx) => tx.kind = action,
            UnsignedTransaction::Eip1559(tx) => tx.kind = action,
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

    pub fn encode_for_signing(&self) -> Bytes {
        let mut buf = Vec::new();
        match self {
            UnsignedTransaction::Legacy(tx) => tx.encode_for_signing(&mut buf),
            UnsignedTransaction::Eip2930(tx) => tx.encode_for_signing(&mut buf),
            UnsignedTransaction::Eip1559(tx) => tx.encode_for_signing(&mut buf),
        }
        buf.into()
    }

    pub fn encode(&self, signature: Option<SignatureComponents>) -> BytesMut {
        UnverifiedTransaction {
            unsigned: self.clone(),
            signature,
            hash: Default::default(),
        }
        .rlp_bytes()
    }

    pub fn to(&self) -> Option<Address> {
        match self {
            UnsignedTransaction::Legacy(tx) => tx.kind().into_to(),
            UnsignedTransaction::Eip2930(tx) => tx.get_to(),
            UnsignedTransaction::Eip1559(tx) => tx.get_to(),
        }
    }

    pub fn value(&self) -> &U256 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.value,
            UnsignedTransaction::Eip2930(tx) => &tx.value,
            UnsignedTransaction::Eip1559(tx) => &tx.value,
        }
    }

    pub fn gas_limit(&self) -> &U64 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.gas_limit,
            UnsignedTransaction::Eip2930(tx) => &tx.gas_limit,
            UnsignedTransaction::Eip1559(tx) => &tx.gas_limit,
        }
    }

    pub fn nonce(&self) -> &U64 {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.nonce,
            UnsignedTransaction::Eip2930(tx) => &tx.nonce,
            UnsignedTransaction::Eip1559(tx) => &tx.nonce,
        }
    }

    pub fn action(&self) -> &TxKind {
        match self {
            UnsignedTransaction::Legacy(tx) => &tx.action,
            UnsignedTransaction::Eip2930(tx) => &tx.action,
            UnsignedTransaction::Eip1559(tx) => &tx.action,
        }
    }

    pub fn access_list(&self) -> AccessList {
        match self {
            UnsignedTransaction::Legacy(_) => Vec::new(),
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
        Hasher::digest(&self.unsigned.encode(self.signature.clone()))
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

    pub fn signature_hash(&self) -> Hash {
        Hasher::digest(self.unsigned.encode_for_signing())
    }

    pub fn recover_public(&self) -> ProtocolResult<H512> {
        Ok(H512::from_slice(
            &secp256k1_recover(
                self.signature_hash().as_bytes(),
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

#[derive(Serialize, Deserialize, Default, Clone, Debug, Hash, PartialEq, Eq)]
pub struct SignatureComponents {
    pub r:          Bytes,
    pub s:          Bytes,
    pub standard_v: u8,
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

            return Ok(Hasher::digest(&r.pub_key).into());
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
    pub public:      Option<H512>,
}

impl SignedTransaction {
    pub fn from_unverified(utx: UnverifiedTransaction) -> ProtocolResult<Self> {
        if utx.signature.is_none() {
            return Err(TypesError::Unsigned.into());
        }

        let hash = utx.signature_hash();
        let sig = utx.signature.as_ref().unwrap();

        if sig.is_eth_sig() {
            let public = utx.recover_public()?;
            let signer = Hasher::digest(&public).into();

            return Ok(SignedTransaction {
                transaction: utx.calc_hash(),
                sender:      signer,
                public:      Some(public),
            });
        }

        // Otherwise it is an interoperation transaction
        Ok(SignedTransaction {
            sender:      sig.extract_interoperation_tx_sender()?,
            public:      Some(H512::zero()),
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
        self.transaction.chain_id.is_some()
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
    pub fn encode_receipt(&self, r: &TxResp, logs_bloom: Bloom) -> Bytes {
        // Status: either 1 (success) or 0 (failure).
        // Only present after activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658)
        let status: u64 = if matches!(r.exit_reason, ExitReason::Succeed(_)) {
            1
        } else {
            0
        };
        let used_gas = U256::from(r.gas_used);
        let legacy_receipt = {
            let mut rlp = RlpStream::new();
            rlp.begin_list(4);
            rlp.append(&status);
            rlp.append(&used_gas);
            rlp.append(&logs_bloom);
            rlp.append_list(&r.logs);
            rlp.out().freeze()
        };
        match self.type_() {
            x if x == 0x01 || x == 0x02 => [&x.to_be_bytes()[7..], &legacy_receipt].concat().into(),
            _ => legacy_receipt, // legacy (0x00) or undefined type
        }
    }
}
