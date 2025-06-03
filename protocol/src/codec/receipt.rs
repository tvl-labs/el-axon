use alloy_rlp::{encode_list, Decodable, Encodable, Header};

use crate::types::{Address, Bloom, BufMut, ExitReason, Log, Receipt, H256, U64};

impl Encodable for Receipt {
    fn encode(&self, out: &mut dyn BufMut) {
        let raw_ret = bincode::serialize(&self.ret).unwrap();
        if let Some(code_address) = self.code_address {
            let enc: [&dyn Encodable; 14] = [
                &true,
                &self.tx_hash,
                &self.block_number,
                &self.block_hash,
                &self.tx_index,
                &self.state_root,
                &self.used_gas,
                &self.logs_bloom,
                &self.logs,
                &self.log_index,
                &code_address,
                &self.sender,
                &raw_ret,
                &self.removed,
            ];
            encode_list::<_, dyn Encodable>(&enc, out);
        } else {
            let enc: [&dyn Encodable; 13] = [
                &false,
                &self.tx_hash,
                &self.block_number,
                &self.block_hash,
                &self.tx_index,
                &self.state_root,
                &self.used_gas,
                &self.logs_bloom,
                &self.logs,
                &self.log_index,
                &self.sender,
                &raw_ret,
                &self.removed,
            ];
            encode_list::<_, dyn Encodable>(&enc, out);
        }
    }
}

impl Decodable for Receipt {
    fn decode(data: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let mut payload = Header::decode_bytes(data, true)?;
        let has_code_address = bool::decode(&mut payload)?;

        if has_code_address {
            Ok(Self {
                tx_hash:      H256::decode(&mut payload)?,
                block_number: u64::decode(&mut payload)?,
                block_hash:   H256::decode(&mut payload)?,
                tx_index:     u32::decode(&mut payload)?,
                state_root:   H256::decode(&mut payload)?,
                used_gas:     U64::decode(&mut payload)?,
                logs_bloom:   Bloom::decode(&mut payload)?,
                logs:         Vec::<Log>::decode(&mut payload)?,
                log_index:    u32::decode(&mut payload)?,
                code_address: Some(H256::decode(&mut payload)?),
                sender:       Address::decode(&mut payload)?,
                ret:          decode_exit_reason(&mut payload)?,
                removed:      bool::decode(&mut payload)?,
            })
        } else {
            Ok(Self {
                tx_hash:      H256::decode(&mut payload)?,
                block_number: u64::decode(&mut payload)?,
                block_hash:   H256::decode(&mut payload)?,
                tx_index:     u32::decode(&mut payload)?,
                state_root:   H256::decode(&mut payload)?,
                used_gas:     U64::decode(&mut payload)?,
                logs_bloom:   Bloom::decode(&mut payload)?,
                logs:         Vec::<Log>::decode(&mut payload)?,
                log_index:    u32::decode(&mut payload)?,
                code_address: None,
                sender:       Address::decode(&mut payload)?,
                ret:          decode_exit_reason(&mut payload)?,
                removed:      bool::decode(&mut payload)?,
            })
        }
    }
}

fn decode_exit_reason(data: &mut &[u8]) -> Result<ExitReason, alloy_rlp::Error> {
    let raw_exit_reason = Vec::<u8>::decode(data)?;
    let exit_reason = bincode::deserialize(&raw_exit_reason).unwrap();
    Ok(exit_reason)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receipt_codec() {
        let receipt = Receipt::default();
        let mut encoded = Vec::new();
        receipt.encode(&mut encoded);
        assert_eq!(alloy_rlp::encode(&receipt), encoded);

        let decode = Receipt::decode(&mut encoded.as_ref()).unwrap();
        assert_eq!(receipt, decode);
    }
}
