pub use alloy::consensus::Account;
pub use alloy::eips::eip2930::{AccessList, AccessListItem};
use alloy::primitives::logs_bloom;
pub use alloy::primitives::Log;
pub use evm::{Config, ExitError, ExitFatal, ExitReason, ExitRevert, ExitSucceed};
pub use hasher::HasherKeccak;

use alloy::consensus::Receipt;
use alloy_rlp::{RlpDecodable, RlpEncodable};
use serde::{Deserialize, Serialize};

use crate::types::{
    Address, Bloom, ExtraData, Hash, Header, MerkleRoot, Proposal, H256, U256, U64,
};

use super::Hex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExecResp {
    pub state_root:   MerkleRoot,
    pub receipt_root: MerkleRoot,
    pub gas_used:     u64,
    pub tx_resp:      Vec<TxResp>,
}

impl ExecResp {
    pub fn bloom(&self) -> Bloom {
        logs_bloom(self.tx_resp.iter().map(|tx| tx.logs.iter()).flatten())
    }
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

impl TxResp {
    pub fn receipt_bloom(&self) -> Bloom {
        Receipt {
            status:              self.exit_reason.is_succeed().into(),
            cumulative_gas_used: self.gas_used,
            logs:                self.logs.clone(),
        }
        .bloom_slow()
    }
}

#[derive(RlpEncodable, RlpDecodable, Default, Clone, Debug, PartialEq, Eq)]
pub struct ExecutorContext {
    pub block_number:           U256,
    pub block_coinbase:         Address,
    pub block_timestamp:        U256,
    pub chain_id:               U256,
    pub origin:                 Address,
    pub gas_price:              U256,
    pub block_gas_limit:        U64,
    pub block_base_fee_per_gas: U64,
    pub extra_data:             Vec<ExtraData>,
}

impl From<Proposal> for ExecutorContext {
    fn from(h: Proposal) -> Self {
        ExecutorContext {
            block_number:           U256::from(h.number),
            block_coinbase:         h.proposer,
            block_timestamp:        U256::from(h.timestamp),
            chain_id:               U256::from(h.chain_id),
            origin:                 h.proposer,
            gas_price:              U256::ONE,
            block_gas_limit:        h.gas_limit,
            block_base_fee_per_gas: h.base_fee_per_gas,
            extra_data:             h.extra_data,
        }
    }
}

impl From<&Header> for ExecutorContext {
    fn from(h: &Header) -> ExecutorContext {
        ExecutorContext {
            block_number:           U256::from(h.number),
            block_coinbase:         h.proposer,
            block_timestamp:        U256::from(h.timestamp),
            chain_id:               U256::from(h.chain_id),
            origin:                 h.proposer,
            gas_price:              U256::ONE,
            block_gas_limit:        h.gas_limit,
            block_base_fee_per_gas: h.base_fee_per_gas,
            extra_data:             h.extra_data.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EthAccountProof {
    pub address:       Address,
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
