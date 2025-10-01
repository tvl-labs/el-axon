use bytes::Bytes;
pub use ethereum::{AccessList, AccessListItem, Account};
pub use evm::{backend::Log, Config, ExitError, ExitFatal, ExitReason, ExitRevert, ExitSucceed};
pub use hasher::HasherKeccak;

use rlp_derive::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::{
    codec::hex_encode,
    types::{Bloom, ExtraData, Hash, Hasher, Header, MerkleRoot, Proposal, H160, H256, U256, U64},
};

use super::Hex;

const BLOOM_BYTE_LENGTH: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecResp {
    pub state_root:   MerkleRoot,
    pub receipt_root: MerkleRoot,
    pub gas_used:     u64,
    pub tx_resp:      Vec<TxResp>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxResp {
    pub exit_reason:  ExitReason,
    pub ret:          Vec<u8>,
    pub gas_used:     u64,
    pub remain_gas:   u64,
    pub fee_cost:     U256,
    pub logs:         Vec<Log>,
    pub code_address: Option<Hash>,
    pub removed:      bool,
}

impl Default for TxResp {
    fn default() -> Self {
        TxResp {
            exit_reason:  ExitReason::Succeed(ExitSucceed::Stopped),
            gas_used:     u64::default(),
            remain_gas:   u64::default(),
            fee_cost:     U256::default(),
            removed:      false,
            ret:          vec![],
            logs:         vec![],
            code_address: None,
        }
    }
}

#[derive(RlpEncodable, RlpDecodable, Default, Clone, Debug, PartialEq, Eq)]
pub struct ExecutorContext {
    pub block_number:           U256,
    pub block_coinbase:         H160,
    pub block_timestamp:        U256,
    pub chain_id:               U256,
    pub origin:                 H160,
    pub gas_price:              U256,
    pub block_gas_limit:        U64,
    pub block_base_fee_per_gas: U64,
    pub extra_data:             Vec<ExtraData>,
}

impl From<Proposal> for ExecutorContext {
    fn from(h: Proposal) -> Self {
        ExecutorContext {
            block_number:           h.number.into(),
            block_coinbase:         h.proposer,
            block_timestamp:        h.timestamp.into(),
            chain_id:               h.chain_id.into(),
            origin:                 h.proposer,
            gas_price:              U256::one(),
            block_gas_limit:        h.gas_limit,
            block_base_fee_per_gas: h.base_fee_per_gas,
            extra_data:             h.extra_data,
        }
    }
}

impl From<&Header> for ExecutorContext {
    fn from(h: &Header) -> ExecutorContext {
        ExecutorContext {
            block_number:           h.number.into(),
            block_coinbase:         h.proposer,
            block_timestamp:        h.timestamp.into(),
            chain_id:               h.chain_id.into(),
            origin:                 h.proposer,
            gas_price:              U256::one(),
            block_gas_limit:        h.gas_limit,
            block_base_fee_per_gas: h.base_fee_per_gas,
            extra_data:             h.extra_data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CallType {
    Call,
    StaticCall,
    DelegateCall,
    CallCode,
    Create,
    Create2,
    SelfDestruct,
}

impl CallType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CallType::Call => "CALL",
            CallType::StaticCall => "STATICCALL",
            CallType::DelegateCall => "DELEGATECALL",
            CallType::CallCode => "CALLCODE",
            CallType::Create => "CREATE",
            CallType::Create2 => "CREATE2",
            CallType::SelfDestruct => "SELFDESTRUCT",
        }
    }
}

/// Represents a single call frame in the execution trace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CallFrame {
    /// Type of call: CALL, STATICCALL, DELEGATECALL, CALLCODE, CREATE, CREATE2,
    /// SELFDESTRUCT
    #[serde(rename = "type")]
    pub type_:         CallType,
    /// Caller address
    pub from:          H160,
    /// Callee address (None for CREATE before address is determined)
    pub to:            Option<H160>,
    /// Value transferred
    pub value:         U256,
    /// Gas provided for this call
    pub gas:           u64,
    /// Actual gas used by this call (updated on exit)
    pub gas_used:      Option<u64>,
    /// Input data
    pub input:         Bytes,
    /// Output data (updated on exit)
    pub output:        Bytes,
    /// Error message if call failed
    pub error:         Option<String>,
    /// Revert reason if available
    pub revert_reason: Option<String>,
    /// Nested calls made during this call
    pub calls:         Vec<CallFrame>,
    /// Logs emitted by this call
    pub logs:          Vec<Log>,
}

impl CallFrame {
    pub fn new(
        call_type: CallType,
        from: H160,
        to: Option<H160>,
        value: U256,
        gas: u64,
        input: Bytes,
    ) -> Self {
        Self {
            type_: call_type,
            from,
            to,
            value,
            gas,
            gas_used: None,
            input,
            output: Bytes::new(),
            error: None,
            revert_reason: None,
            calls: Vec::new(),
            logs: Vec::new(),
        }
    }

    /// Update the frame with exit information
    pub fn on_exit(&mut self, return_value: &[u8], reason: &ExitReason) {
        self.output = Bytes::from(return_value.to_vec());

        match reason {
            ExitReason::Succeed(_) => {
                self.error = None;
                self.revert_reason = None;
            }
            ExitReason::Revert(_) => {
                self.error = Some("revert".to_string());
                self.revert_reason = Some(decode_revert_msg(return_value));
            }
            _ => {
                self.error = Some(format!("{:?}", reason));
                self.revert_reason = None;
            }
        }
    }

    /// Add a child call
    pub fn add_call(&mut self, call: CallFrame) {
        self.calls.push(call);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EthAccountProof {
    pub address:       H160,
    pub balance:       U256,
    pub code_hash:     H256,
    pub nonce:         U256,
    pub storage_hash:  H256,
    pub account_proof: Vec<Hex>,
    pub storage_proof: Vec<EthStorageProof>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EthStorageProof {
    pub key:   U256,
    pub value: U256,
    pub proof: Vec<Hex>,
}

pub fn logs_bloom<'a, I>(logs: I) -> Bloom
where
    I: Iterator<Item = &'a Log>,
{
    let mut bloom = Bloom::zero();

    for log in logs {
        m3_2048(&mut bloom, log.address.as_bytes());
        for topic in log.topics.iter() {
            m3_2048(&mut bloom, topic.as_bytes());
        }
    }
    bloom
}

fn m3_2048(bloom: &mut Bloom, x: &[u8]) {
    let hash = Hasher::digest(x).0;
    for i in [0, 2, 4] {
        let bit = (hash[i + 1] as usize + ((hash[i] as usize) << 8)) & 0x7FF;
        bloom.0[BLOOM_BYTE_LENGTH - 1 - bit / 8] |= 1 << (bit % 8);
    }
}

fn decode_revert_msg(input: &[u8]) -> String {
    const FUNC_SELECTOR_LEN: usize = 4;
    const U256_BE_BYTES_LEN: usize = 32;
    const REVERT_MSG_LEN_OFFSET: usize = FUNC_SELECTOR_LEN + U256_BE_BYTES_LEN;
    const REVERT_EFFECT_MSG_OFFSET: usize = REVERT_MSG_LEN_OFFSET + U256_BE_BYTES_LEN;
    const EXEC_REVERT: &str = "execution reverted: ";

    if input.len() < REVERT_EFFECT_MSG_OFFSET {
        return EXEC_REVERT.to_string();
    }

    let decode_reason = |i: &[u8]| -> String {
        let reason = hex_encode(i);
        EXEC_REVERT.to_string() + &reason
    };

    decode_reason(&input[REVERT_EFFECT_MSG_OFFSET..])
}
