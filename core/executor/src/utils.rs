use protocol::codec::hex_encode;
use protocol::types::{Address, Hasher, U256};

use crate::FeeAllocate;

const FUNC_SELECTOR_LEN: usize = 4;
const U256_BE_BYTES_LEN: usize = 32;
const REVERT_MSG_LEN_OFFSET: usize = FUNC_SELECTOR_LEN + U256_BE_BYTES_LEN;
const REVERT_EFFECT_MSG_OFFSET: usize = REVERT_MSG_LEN_OFFSET + U256_BE_BYTES_LEN;
const EXEC_REVERT: &str = "execution reverted: ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeeInlet {
    pub address: Address,
    pub amount:  U256,
}

#[derive(Default, Clone, Debug)]
pub struct DefaultFeeAllocator;

impl FeeAllocate for DefaultFeeAllocator {
    fn allocate(
        &self,
        block_number: U256,
        fee_collect: U256,
        _proposer: Address,
        validators: &[protocol::types::ValidatorExtend],
    ) -> Vec<FeeInlet> {
        if fee_collect.is_zero() || block_number.is_zero() {
            return Vec::new();
        }

        let weight_sum = U256::from(validators.iter().map(|v| v.vote_weight).sum::<u32>());

        validators
            .iter()
            .map(|v| FeeInlet {
                address: v.address,
                amount:  (fee_collect / weight_sum) * U256::from(v.vote_weight),
            })
            .collect()
    }
}

pub fn code_address(sender: &evm_types::H160, nonce: &evm_types::U256) -> evm_types::H256 {
    let mut stream = rlp::RlpStream::new_list(2);
    stream.append(sender);
    stream.append(nonce);
    evm_types::H256(Hasher::digest(&stream.out()).0)
}

pub fn decode_revert_msg(input: &[u8]) -> String {
    if input.len() < REVERT_EFFECT_MSG_OFFSET {
        return EXEC_REVERT.to_string();
    }

    let decode_reason = |i: &[u8]| -> String {
        let reason = hex_encode(i);
        EXEC_REVERT.to_string() + &reason
    };

    decode_reason(&input[REVERT_EFFECT_MSG_OFFSET..])
}

#[cfg(test)]
mod tests {
    use protocol::codec::{hex_decode, hex_encode};

    use super::*;

    #[test]
    fn test_code_address() {
        let sender = evm_types::H160::from_slice(
            hex_decode("8ab0cf264df99d83525e9e11c7e4db01558ae1b1")
                .unwrap()
                .as_ref(),
        );
        let nonce = evm_types::U256::zero();
        let addr: evm_types::H160 = code_address(&sender, &nonce).into();
        assert_eq!(
            hex_encode(addr.0).as_str(),
            "a13763691970d9373d4fab7cc323d7ba06fa9986"
        );

        let sender = evm_types::H160::from_slice(
            hex_decode("6ac7ea33f8831ea9dcc53393aaa88b25a785dbf0")
                .unwrap()
                .as_ref(),
        );
        let addr: evm_types::H160 = code_address(&sender, &nonce).into();
        assert_eq!(
            hex_encode(addr.0).as_str(),
            "cd234a471b72ba2f1ccf0a70fcaba648a5eecd8d"
        )
    }
}
