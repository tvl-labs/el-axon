mod abi;
mod store;

pub use abi::image_cell_abi;
pub use store::{CellInfo, CellKey};

use std::sync::atomic::{AtomicBool, Ordering};

use ethers::abi::AbiDecode;
use evm_types::U64;

use protocol::traits::{ApplyBackend, ExecutorAdapter};
use protocol::types::{SignedTransaction, TxResp, H256};
use protocol::ProtocolResult;

use crate::system_contract::image_cell::store::ImageCellStore;
use crate::system_contract::utils::{succeed_resp, update_states};
use crate::system_contract::{system_contract_address, SystemContract};
use crate::{exec_try, system_contract_struct, MPTTrie, CURRENT_HEADER_CELL_ROOT};

pub const IMAGE_CELL_CONTRACT_ADDRESS: evm_types::H160 = system_contract_address(0x3);
static ALLOW_READ: AtomicBool = AtomicBool::new(false);

system_contract_struct!(ImageCellContract);

impl<Adapter: ExecutorAdapter + ApplyBackend> SystemContract<Adapter>
    for ImageCellContract<Adapter>
{
    const ADDRESS: evm_types::H160 = IMAGE_CELL_CONTRACT_ADDRESS;

    fn exec_(&self, adapter: &mut Adapter, tx: &SignedTransaction) -> TxResp {
        let sender = tx.sender;
        let tx = &tx.transaction.unsigned;
        let tx_data = tx.data();
        let gas_limit = U64::from(*tx.gas_limit());

        let root = H256::new(CURRENT_HEADER_CELL_ROOT.with(|r| *r.borrow()).0);
        let mut store = exec_try!(
            ImageCellStore::new(root),
            gas_limit,
            "[image cell] init image cell mpt"
        );

        let call_abi = exec_try!(
            image_cell_abi::ImageCellContractCalls::decode(tx_data),
            gas_limit,
            "[image cell] invalid tx data"
        );

        match call_abi {
            image_cell_abi::ImageCellContractCalls::SetState(data) => {
                ALLOW_READ.store(data.allow_read, Ordering::Relaxed);
            }
            image_cell_abi::ImageCellContractCalls::Update(data) => {
                exec_try!(store.update(data), gas_limit, "[image cell] update error:");
            }
            image_cell_abi::ImageCellContractCalls::Rollback(data) => {
                exec_try!(
                    store.rollback(data),
                    gas_limit,
                    "[image cell] rollback error:"
                );
            }
        }

        let sender = evm_types::H160(sender.into_array());
        update_states(adapter, sender, Self::ADDRESS);
        succeed_resp(gas_limit)
    }
}

#[derive(Default)]
pub(crate) struct ImageCellReader;

/// These methods are provide for interoperation module to get CKB cells.
impl ImageCellReader {
    pub fn get_cell(&self, root: H256, key: &CellKey) -> ProtocolResult<Option<CellInfo>> {
        ImageCellStore::new(root)?.get_cell(key)
    }

    #[cfg(test)]
    pub fn allow_read(&self) -> bool {
        ALLOW_READ.load(Ordering::Relaxed)
    }
}
