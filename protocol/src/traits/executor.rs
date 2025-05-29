pub use evm::backend::{ApplyBackend, Backend, MemoryBackend};

use crate::types::{
    Account, Address, Bytes, ExecResp, ExecutorContext, Log, MerkleRoot, SignedTransaction, TxResp,
    ValidatorExtend, U256, U64,
};

pub trait ExecutorReadOnlyAdapter: Backend {
    fn get(&self, key: &[u8]) -> Option<Bytes>;

    fn get_ctx(&self) -> ExecutorContext;

    fn get_account(&self, address: &Address) -> Account;
}

pub trait ExecutorAdapter: ExecutorReadOnlyAdapter + ApplyBackend {
    fn set_origin(&mut self, origin: Address);

    fn set_gas_price(&mut self, gas_price: U64);

    fn save_account(&mut self, address: &Address, account: &Account);

    fn commit(&mut self) -> MerkleRoot;

    fn take_logs(&mut self) -> Vec<Log>;
}

pub trait Executor: Send + Sync {
    fn call<B: Backend>(
        &self,
        backend: &B,
        gas_limit: u64,
        from: Option<Address>,
        to: Option<Address>,
        value: U256,
        data: Vec<u8>,
        estimate: bool,
    ) -> TxResp;

    fn exec<Adapter: ExecutorAdapter + ApplyBackend>(
        &self,
        adapter: &mut Adapter,
        txs: &[SignedTransaction],
        validators: &[ValidatorExtend],
    ) -> ExecResp;
}

/// This implementation is only used for test.
impl<'a> ExecutorReadOnlyAdapter for MemoryBackend<'a> {
    fn get(&self, _key: &[u8]) -> Option<Bytes> {
        unreachable!()
    }

    fn get_ctx(&self) -> ExecutorContext {
        unreachable!()
    }

    fn get_account(&self, _address: &Address) -> Account {
        unreachable!()
    }
}

impl<'a> ExecutorAdapter for MemoryBackend<'a> {
    fn set_origin(&mut self, _origin: Address) {
        unreachable!()
    }

    fn set_gas_price(&mut self, _gas_price: U64) {
        unreachable!()
    }

    fn take_logs(&mut self) -> Vec<Log> {
        unreachable!()
    }

    fn commit(&mut self) -> MerkleRoot {
        unreachable!()
    }

    fn save_account(&mut self, _address: &Address, _account: &Account) {
        unreachable!()
    }
}
