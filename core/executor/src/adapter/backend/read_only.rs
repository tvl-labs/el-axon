use std::sync::Arc;

use evm::backend::Basic;

use evm_types::BigEndianHash;
use protocol::traits::{Backend, Context, ExecutorReadOnlyAdapter, ReadOnlyStorage};
use protocol::trie::Trie as _;
use protocol::types::{
    Account, Address, Bytes, ExecutorContext, MerkleRoot, H256, NIL_DATA, RLP_NULL, U256,
};
use protocol::{codec::ProtocolCodec, trie, ProtocolResult};

use crate::system_contract::{
    HEADER_CELL_ROOT_KEY, IMAGE_CELL_CONTRACT_ADDRESS, METADATA_CONTRACT_ADDRESS, METADATA_ROOT_KEY,
};
use crate::{blocking_async, MPTTrie};

const GET_BLOCK_HASH_NUMBER_RANGE: u64 = 256;

pub struct AxonExecutorReadOnlyAdapter<S, DB: trie::DB> {
    pub(crate) exec_ctx: ExecutorContext,
    pub(crate) trie:     MPTTrie<DB>,
    pub(crate) storage:  Arc<S>,
    pub(crate) db:       Arc<DB>,
}

impl<S, DB> ExecutorReadOnlyAdapter for AxonExecutorReadOnlyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    fn get_ctx(&self) -> ExecutorContext {
        self.exec_ctx.clone()
    }

    fn get(&self, key: &[u8]) -> Option<Bytes> {
        self.trie.get(key).ok().flatten().map(Into::into)
    }

    fn get_account(&self, address: &Address) -> Account {
        if let Ok(Some(raw)) = self.trie.get(address.as_slice()) {
            return Account::decode(raw).unwrap();
        }

        Account {
            nonce:        0u64,
            balance:      U256::ZERO,
            storage_root: RLP_NULL,
            code_hash:    NIL_DATA,
        }
    }
}

impl<S, DB> Backend for AxonExecutorReadOnlyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    fn gas_price(&self) -> evm_types::U256 {
        evm_types::U256(self.exec_ctx.gas_price.into_limbs())
    }

    fn origin(&self) -> evm_types::H160 {
        evm_types::H160(self.exec_ctx.origin.into_array())
    }

    fn block_number(&self) -> evm_types::U256 {
        evm_types::U256(self.exec_ctx.block_number.into_limbs())
    }

    fn block_hash(&self, number: evm_types::U256) -> evm_types::H256 {
        let current_number = self.block_number();
        if number >= current_number {
            return Default::default();
        }

        if (current_number - number) > evm_types::U256::from(GET_BLOCK_HASH_NUMBER_RANGE) {
            return Default::default();
        }

        let number = number.low_u64();
        blocking_async!(self, get_storage, get_block, Context::new(), number)
            .map(|b| evm_types::H256(b.hash().0))
            .unwrap_or_default()
    }

    fn block_coinbase(&self) -> evm_types::H160 {
        evm_types::H160(self.exec_ctx.block_coinbase.into_array())
    }

    fn block_timestamp(&self) -> evm_types::U256 {
        evm_types::U256(self.exec_ctx.block_timestamp.into_limbs())
    }

    fn block_difficulty(&self) -> evm_types::U256 {
        evm_types::U256::one()
    }

    fn block_gas_limit(&self) -> evm_types::U256 {
        evm_types::U256::from(self.exec_ctx.block_gas_limit.to::<u64>())
    }

    fn block_base_fee_per_gas(&self) -> evm_types::U256 {
        evm_types::U256::from(self.exec_ctx.block_base_fee_per_gas.to::<u64>())
    }

    fn chain_id(&self) -> evm_types::U256 {
        evm_types::U256(self.exec_ctx.chain_id.into_limbs())
    }

    fn exists(&self, address: evm_types::H160) -> bool {
        self.trie.contains(address.as_bytes()).unwrap_or(false)
    }

    fn basic(&self, address: evm_types::H160) -> Basic {
        self.trie
            .get(address.as_bytes())
            .map(|raw| {
                if raw.is_none() {
                    return Basic::default();
                }
                Account::decode(raw.unwrap()).map_or_else(
                    |_| Default::default(),
                    |account| Basic {
                        balance: evm_types::U256(account.balance.into_limbs()),
                        nonce:   evm_types::U256::from(account.nonce),
                    },
                )
            })
            .unwrap_or_default()
    }

    fn code(&self, address: evm_types::H160) -> Vec<u8> {
        let code_hash = if let Some(bytes) = self.trie.get(address.as_bytes()).unwrap() {
            Account::decode(bytes).unwrap().code_hash
        } else {
            return Vec::new();
        };

        if code_hash == NIL_DATA {
            return Vec::new();
        }

        let res = blocking_async!(
            self,
            get_storage,
            get_code_by_hash,
            Context::new(),
            &code_hash
        );

        res.unwrap_or_default().to_vec()
    }

    // ### Notes
    //
    // - If a MPT tree is empty, the root should be `RLP_NULL`.
    // - In this function, when returns `H256::default()`, that means the tree is
    //   not initialized.
    fn storage(&self, address: evm_types::H160, index: evm_types::H256) -> evm_types::H256 {
        if let Ok(raw) = self.trie.get(address.as_bytes()) {
            if raw.is_none() {
                return Default::default();
            }

            Account::decode(raw.unwrap())
                .and_then(|account| {
                    let storage_root = account.storage_root;
                    if storage_root == RLP_NULL {
                        Ok(Default::default())
                    } else {
                        MPTTrie::from_root(storage_root, Arc::clone(&self.db)).map(
                            |trie| match trie.get(index.as_bytes()) {
                                Ok(Some(res)) => {
                                    let value: evm_types::U256 = rlp::decode(&res).unwrap();
                                    BigEndianHash::from_uint(&value)
                                }
                                _ => Default::default(),
                            },
                        )
                    }
                })
                .unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn original_storage(
        &self,
        address: evm_types::H160,
        index: evm_types::H256,
    ) -> Option<evm_types::H256> {
        // Fixme
        Some(self.storage(address, index))
    }
}

impl<S, DB> AxonExecutorReadOnlyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    pub fn new(db: Arc<DB>, storage: Arc<S>, exec_ctx: ExecutorContext) -> ProtocolResult<Self> {
        let trie = MPTTrie::new(Arc::clone(&db));
        Ok(AxonExecutorReadOnlyAdapter {
            trie,
            db,
            storage,
            exec_ctx,
        })
    }

    pub fn from_root(
        state_root: MerkleRoot,
        db: Arc<DB>,
        storage: Arc<S>,
        exec_ctx: ExecutorContext,
    ) -> ProtocolResult<Self> {
        let trie = MPTTrie::from_root(state_root, Arc::clone(&db))?;

        Ok(AxonExecutorReadOnlyAdapter {
            trie,
            db,
            storage,
            exec_ctx,
        })
    }

    pub fn get_metadata_root(&self) -> H256 {
        H256::new(
            self.storage(METADATA_CONTRACT_ADDRESS, *METADATA_ROOT_KEY)
                .0,
        )
    }

    pub fn get_image_cell_root(&self) -> H256 {
        H256::new(
            self.storage(IMAGE_CELL_CONTRACT_ADDRESS, *HEADER_CELL_ROOT_KEY)
                .0,
        )
    }

    fn get_storage(&self) -> Arc<S> {
        Arc::clone(&self.storage)
    }
}
