use std::sync::Arc;

use evm::backend::{Apply, Basic};

use protocol::traits::{
    ApplyBackend, Backend, Context, ExecutorAdapter, ExecutorReadOnlyAdapter, ReadOnlyStorage,
    Storage,
};
use protocol::trie::Trie;
use protocol::types::{
    Account, Address, Bytes, ExecutorContext, Hasher, Log, MerkleRoot, H256, NIL_DATA, RLP_NULL,
    U256, U64,
};
use protocol::{codec::ProtocolCodec, trie, ProtocolResult};

use crate::blocking_async;
use crate::system_contract::{METADATA_CONTRACT_ADDRESS, METADATA_ROOT_KEY};
use crate::{adapter::AxonExecutorReadOnlyAdapter, MPTTrie};

pub struct AxonExecutorApplyAdapter<S, DB: trie::DB> {
    inner: AxonExecutorReadOnlyAdapter<S, DB>,
    logs:  Vec<Log>,
}

impl<S, DB> ExecutorReadOnlyAdapter for AxonExecutorApplyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    fn get_ctx(&self) -> ExecutorContext {
        self.inner.get_ctx()
    }

    fn get(&self, key: &[u8]) -> Option<Bytes> {
        self.inner.get(key)
    }

    fn get_account(&self, address: &Address) -> Account {
        self.inner.get_account(address)
    }
}

impl<S, DB> Backend for AxonExecutorApplyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    fn gas_price(&self) -> evm_types::U256 {
        self.inner.gas_price()
    }

    fn origin(&self) -> evm_types::H160 {
        self.inner.origin()
    }

    fn block_hash(&self, number: evm_types::U256) -> evm_types::H256 {
        self.inner.block_hash(number)
    }

    fn block_number(&self) -> evm_types::U256 {
        self.inner.block_number()
    }

    fn block_coinbase(&self) -> evm_types::H160 {
        self.inner.block_coinbase()
    }

    fn block_timestamp(&self) -> evm_types::U256 {
        self.inner.block_timestamp()
    }

    fn block_difficulty(&self) -> evm_types::U256 {
        self.inner.block_difficulty()
    }

    fn block_gas_limit(&self) -> evm_types::U256 {
        self.inner.block_gas_limit()
    }

    fn block_base_fee_per_gas(&self) -> evm_types::U256 {
        self.inner.block_base_fee_per_gas()
    }

    fn chain_id(&self) -> evm_types::U256 {
        self.inner.chain_id()
    }

    fn exists(&self, address: evm_types::H160) -> bool {
        self.inner.exists(address)
    }

    fn basic(&self, address: evm_types::H160) -> Basic {
        self.inner.basic(address)
    }

    fn code(&self, address: evm_types::H160) -> Vec<u8> {
        self.inner.code(address)
    }

    fn storage(&self, address: evm_types::H160, key: evm_types::H256) -> evm_types::H256 {
        self.inner.storage(address, key)
    }

    fn original_storage(
        &self,
        address: evm_types::H160,
        key: evm_types::H256,
    ) -> Option<evm_types::H256> {
        self.inner.original_storage(address, key)
    }
}

impl<S, DB> ExecutorAdapter for AxonExecutorApplyAdapter<S, DB>
where
    S: Storage + 'static,
    DB: trie::DB + 'static,
{
    fn set_origin(&mut self, origin: Address) {
        self.inner.exec_ctx.origin = origin;
    }

    fn set_gas_price(&mut self, gas_price: U64) {
        self.inner.exec_ctx.gas_price = U256::from_limbs([gas_price.into_limbs()[0], 0, 0, 0]);
    }

    fn take_logs(&mut self) -> Vec<Log> {
        let mut ret = Vec::new();
        ret.append(&mut self.logs);
        ret
    }

    fn commit(&mut self) -> MerkleRoot {
        self.inner.trie.commit().unwrap()
    }

    fn save_account(&mut self, address: &Address, account: &Account) {
        self.inner
            .trie
            .insert(
                address.as_slice().to_vec(),
                account.encode().unwrap().to_vec(),
            )
            .unwrap();
    }
}

impl<S, DB> AxonExecutorApplyAdapter<S, DB>
where
    S: Storage + 'static,
    DB: trie::DB + 'static,
{
    fn apply<I: IntoIterator<Item = (evm_types::H256, evm_types::H256)>>(
        &mut self,
        address: evm_types::H160,
        basic: Basic,
        code: Option<Vec<u8>>,
        storage: I,
        reset_storage: bool,
    ) -> bool {
        let old_account = match self.inner.trie.get(address.as_bytes()) {
            Ok(Some(raw)) => Account::decode(raw).unwrap(),
            _ => Account {
                nonce:        0u64,
                balance:      U256::ZERO,
                storage_root: RLP_NULL,
                code_hash:    NIL_DATA,
            },
        };

        let storage_root = if reset_storage {
            RLP_NULL
        } else {
            old_account.storage_root
        };

        let mut storage_trie = if storage_root == RLP_NULL {
            MPTTrie::new(Arc::clone(&self.inner.db))
        } else {
            MPTTrie::from_root(old_account.storage_root, Arc::clone(&self.inner.db)).unwrap()
        };

        storage.into_iter().for_each(|(k, v)| {
            // https://github.com/ethereum/go-ethereum/blob/ad16f11f841ab3a5fdedc8ddfc602f0717a34dd0/core/state/state_object.go#L306-L311
            // if value is zero, delete it's key
            if v.is_zero() {
                storage_trie
                    .remove(k.as_bytes())
                    .expect("Failed to remove entry with zero value from storage trie");
            } else {
                storage_trie
                    .insert(
                        k.as_bytes().to_vec(),
                        // https://github.com/ethereum/go-ethereum/blob/ad16f11f841ab3a5fdedc8ddfc602f0717a34dd0/core/state/state_object.go#L314
                        // Trim left zeroes and then rlp
                        rlp::encode(&evm_types::U256::from_big_endian(v.as_bytes())).to_vec(),
                    )
                    .expect("trie tree insert fail");
            }
        });

        let storage_root = storage_trie
            .commit()
            .unwrap_or_else(|err| panic!("failed to update the trie storage since {err}"));

        let mut new_account = Account {
            nonce: basic.nonce.low_u64(),
            balance: U256::from_limbs(basic.balance.0),
            code_hash: old_account.code_hash,
            storage_root,
        };

        if let Some(c) = code {
            let new_code_hash = Hasher::digest(&c);
            if new_code_hash != old_account.code_hash {
                let address: evm_types::H256 = address.into();
                blocking_async!(
                    self,
                    get_storage,
                    insert_code,
                    Context::new(),
                    H256::new(address.0),
                    new_code_hash,
                    c.into()
                );

                new_account.code_hash = new_code_hash;
            }
        }

        let bytes = new_account.encode().unwrap();

        {
            self.inner
                .trie
                .insert(address.as_bytes().to_vec(), bytes.to_vec())
                .unwrap();
        }

        new_account.balance == U256::ZERO
            && new_account.nonce == 0u64
            && new_account.code_hash.is_zero()
    }
}

impl<S, DB> ApplyBackend for AxonExecutorApplyAdapter<S, DB>
where
    S: Storage + 'static,
    DB: trie::DB + 'static,
{
    fn apply<A, I, L>(&mut self, values: A, logs: L, delete_empty: bool)
    where
        A: IntoIterator<Item = Apply<I>>,
        I: IntoIterator<Item = (evm_types::H256, evm_types::H256)>,
        L: IntoIterator<Item = evm::backend::Log>,
    {
        for apply in values.into_iter() {
            match apply {
                Apply::Modify {
                    address,
                    basic,
                    code,
                    storage,
                    reset_storage,
                } => {
                    let is_empty = self.apply(address, basic, code, storage, reset_storage);
                    if is_empty && delete_empty {
                        self.inner.trie.remove(address.as_bytes()).unwrap();
                    }
                }
                Apply::Delete { address } => {
                    let _ = self.inner.trie.remove(address.as_bytes());
                }
            }
        }

        self.logs = logs
            .into_iter()
            .map(|i| {
                let topics = i.topics.iter().map(|i| H256::new(i.0)).collect::<Vec<_>>();
                Log::new_unchecked(Address::new(i.address.0), topics, i.data.into())
            })
            .collect::<Vec<_>>();
    }
}

impl<S, DB> AxonExecutorApplyAdapter<S, DB>
where
    S: ReadOnlyStorage + 'static,
    DB: trie::DB + 'static,
{
    pub fn new(db: Arc<DB>, storage: Arc<S>, exec_ctx: ExecutorContext) -> ProtocolResult<Self> {
        Ok(AxonExecutorApplyAdapter {
            inner: AxonExecutorReadOnlyAdapter::new(db, storage, exec_ctx)?,
            logs:  Vec::new(),
        })
    }

    pub fn from_root(
        state_root: MerkleRoot,
        db: Arc<DB>,
        storage: Arc<S>,
        exec_ctx: ExecutorContext,
    ) -> ProtocolResult<Self> {
        Ok(AxonExecutorApplyAdapter {
            inner: AxonExecutorReadOnlyAdapter::from_root(state_root, db, storage, exec_ctx)?,
            logs:  Vec::new(),
        })
    }

    pub fn get_metadata_root(&self) -> evm_types::H256 {
        self.storage(METADATA_CONTRACT_ADDRESS, *METADATA_ROOT_KEY)
    }

    fn get_storage(&self) -> Arc<S> {
        Arc::clone(&self.inner.storage)
    }
}
