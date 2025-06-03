use alloy_rlp::{encode_list, Decodable, Encodable, Header};

use crate::types::{BufMut, Log, TxResp, H256, U256};

impl Encodable for TxResp {
    fn encode(&self, out: &mut dyn BufMut) {
        let raw_exit_reason = bincode::serialize(&self.exit_reason).unwrap();
        if let Some(code_address) = self.code_address {
            let enc: [&dyn Encodable; 9] = [
                &true,
                &raw_exit_reason,
                &self.ret,
                &self.gas_used,
                &self.remain_gas,
                &self.fee_cost,
                &self.logs,
                &code_address,
                &self.removed,
            ];
            encode_list::<_, dyn Encodable>(&enc, out);
        } else {
            let enc: [&dyn Encodable; 8] = [
                &false,
                &raw_exit_reason,
                &self.ret,
                &self.gas_used,
                &self.remain_gas,
                &self.fee_cost,
                &self.logs,
                &self.removed,
            ];
            encode_list::<_, dyn Encodable>(&enc, out);
        }
    }
}

impl Decodable for TxResp {
    fn decode(data: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let mut payload = Header::decode_bytes(data, true)?;
        let raw_exit_reason = Vec::<u8>::decode(&mut payload)?;
        let exit_reason = bincode::deserialize(&raw_exit_reason).unwrap();

        let has_code_address = bool::decode(&mut payload)?;
        if has_code_address {
            Ok(Self {
                exit_reason,
                ret: Vec::<u8>::decode(&mut payload)?,
                gas_used: u64::decode(&mut payload)?,
                remain_gas: u64::decode(&mut payload)?,
                fee_cost: U256::decode(&mut payload)?,
                logs: Vec::<Log>::decode(&mut payload)?,
                code_address: Some(H256::decode(&mut payload)?),
                removed: bool::decode(&mut payload)?,
            })
        } else {
            Ok(Self {
                exit_reason,
                ret: Vec::<u8>::decode(&mut payload)?,
                gas_used: u64::decode(&mut payload)?,
                remain_gas: u64::decode(&mut payload)?,
                fee_cost: U256::decode(&mut payload)?,
                logs: Vec::<Log>::decode(&mut payload)?,
                code_address: None,
                removed: bool::decode(&mut payload)?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ExecutorContext;

    #[test]
    fn test_exec_ctx_codec() {
        let exec_ctx = ExecutorContext::default();
        let mut encoded = Vec::new();
        exec_ctx.encode(&mut encoded);
        assert_eq!(alloy_rlp::encode(&exec_ctx), encoded);

        let decode = ExecutorContext::decode(&mut encoded.as_ref()).unwrap();
        assert_eq!(exec_ctx, decode);
    }
}
