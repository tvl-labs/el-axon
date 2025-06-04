use protocol::traits::{ApplyBackend, ExecutorAdapter};
use protocol::types::{Apply, Basic, SignedTransaction, TxResp};

use crate::system_contract::utils::{revert_resp, succeed_resp};
use crate::system_contract::{system_contract_address, SystemContract};
use crate::system_contract_struct;

pub const NATIVE_TOKEN_CONTRACT_ADDRESS: evm_types::H160 = system_contract_address(0x0);

system_contract_struct!(NativeTokenContract);

impl<Adapter: ExecutorAdapter + ApplyBackend> SystemContract<Adapter>
    for NativeTokenContract<Adapter>
{
    const ADDRESS: evm_types::H160 = NATIVE_TOKEN_CONTRACT_ADDRESS;

    fn exec_(&self, backend: &mut Adapter, tx: &SignedTransaction) -> TxResp {
        let tx = &tx.transaction.unsigned;
        let tx_data = tx.data();
        let tx_value = evm_types::U256(tx.value().into_limbs());
        let gas_limit = *tx.gas_limit();

        if tx_data.len() < 21 || tx_data[0] > 1 {
            return revert_resp(gas_limit.into());
        }

        let direction = tx_data[0] == 0u8;
        let l2_addr = evm_types::H160::from_slice(&tx_data[1..21]);
        let mut account = backend.basic(l2_addr);

        if direction {
            account.balance += tx_value;
        } else {
            if account.balance < tx_value {
                return revert_resp(gas_limit.into());
            }

            account.balance -= tx_value;
        }

        backend.apply(
            vec![Apply::Modify {
                address:       l2_addr,
                basic:         Basic {
                    balance: account.balance,
                    nonce:   account.nonce + 1u64,
                },
                code:          None,
                storage:       vec![],
                reset_storage: false,
            }],
            vec![],
            false,
        );

        succeed_resp(gas_limit.into())
    }
}
