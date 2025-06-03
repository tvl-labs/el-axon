use alloy_rlp::{encode_list, Decodable, Encodable, Header};

use crate::types::{Address, BlockVersion, BufMut, ExtraData, Proof, Proposal, H256, U64};
use crate::{constants::BASE_FEE_PER_GAS, lazy::CHAIN_ID};

impl Encodable for BlockVersion {
    fn encode(&self, out: &mut dyn BufMut) {
        let enc: [&dyn Encodable; 1] = [&0u8];
        encode_list::<_, dyn Encodable>(&enc, out);
    }
}

impl Decodable for BlockVersion {
    fn decode(data: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let mut payload = alloy_rlp::Header::decode_bytes(data, true)?;
        match u8::decode(&mut payload)? {
            0 => Ok(BlockVersion::V0),
            _ => Err(alloy_rlp::Error::Custom("Invalid block version")),
        }
    }
}

impl Encodable for Proposal {
    fn encode(&self, out: &mut dyn BufMut) {
        let enc: [&dyn Encodable; 13] = [
            &self.version,
            &self.prev_hash,
            &self.proposer,
            &self.prev_state_root,
            &self.transactions_root,
            &self.signed_txs_hash,
            &self.timestamp,
            &self.number,
            &self.gas_limit.to::<u64>(),
            &self.extra_data,
            &self.proof,
            &self.call_system_script_count,
            &self.tx_hashes,
        ];
        encode_list::<_, dyn Encodable>(&enc, out);
    }
}

impl Decodable for Proposal {
    fn decode(data: &mut &[u8]) -> Result<Self, alloy_rlp::Error> {
        let mut payload = Header::decode_bytes(data, true)?;
        Ok(Proposal {
            version:                  BlockVersion::decode(&mut payload)?,
            prev_hash:                H256::decode(&mut payload)?,
            proposer:                 Address::decode(&mut payload)?,
            prev_state_root:          H256::decode(&mut payload)?,
            transactions_root:        H256::decode(&mut payload)?,
            signed_txs_hash:          H256::decode(&mut payload)?,
            timestamp:                u64::decode(&mut payload)?,
            number:                   u64::decode(&mut payload)?,
            gas_limit:                U64::decode(&mut payload)?,
            extra_data:               Vec::<ExtraData>::decode(&mut payload)?,
            base_fee_per_gas:         U64::from(BASE_FEE_PER_GAS),
            proof:                    Proof::decode(&mut payload)?,
            chain_id:                 **CHAIN_ID.load(),
            call_system_script_count: u32::decode(&mut payload)?,
            tx_hashes:                Vec::<H256>::decode(&mut payload)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::MessageCodec;
    use crate::types::{Block, Bytes, ExtraData, Header, Proof, H256};
    use rand::random;

    use super::*;

    fn rand_bytes(len: usize) -> Bytes {
        (0..len).map(|_| random::<u8>()).collect::<Vec<_>>().into()
    }

    #[test]
    fn test_version_codec() {
        let ver = BlockVersion::V0;
        let mut encoded = Vec::new();
        ver.encode(&mut encoded);

        assert_eq!(alloy_rlp::encode(&ver), encoded);

        let decode = BlockVersion::decode(&mut encoded.as_ref()).unwrap();
        assert_eq!(ver, decode);
    }

    #[test]
    fn test_block_codec() {
        let block = Block::default();
        let mut encoded = Vec::new();
        block.encode(&mut encoded);

        assert_eq!(alloy_rlp::encode(&block), encoded);

        let decode = Block::decode(&mut encoded.as_ref()).unwrap();
        assert_eq!(block, decode);
    }

    #[test]
    fn test_header_codec() {
        let header = Header::default();
        let mut encoded = Vec::new();
        header.encode(&mut encoded);

        assert_eq!(alloy_rlp::encode(&header), encoded);

        let decode = Header::decode(&mut encoded.as_ref()).unwrap();
        assert_eq!(header, decode);
    }

    #[test]
    fn test_proof_codec() {
        let mut proof = Proof::default();
        let bytes = proof.encode_msg().unwrap();
        let decode: Proof = Proof::decode_msg(bytes).unwrap();
        assert_eq!(proof, decode);
    }

    #[test]
    fn test_proposal_codec() {
        let mock_proof = Proof {
            number:     random(),
            round:      random(),
            block_hash: H256::random(),
            signature:  rand_bytes(65),
            bitmap:     rand_bytes(32),
        };
        let mut proposal = Proposal {
            version:                  BlockVersion::V0,
            prev_hash:                H256::random(),
            proposer:                 Address::random(),
            prev_state_root:          H256::random(),
            transactions_root:        H256::random(),
            signed_txs_hash:          H256::random(),
            timestamp:                random(),
            number:                   random(),
            gas_limit:                U64::from(30000000),
            extra_data:               vec![ExtraData {
                inner: H256::random().0.to_vec().into(),
            }],
            base_fee_per_gas:         U64::from(1337),
            proof:                    mock_proof,
            chain_id:                 0,
            call_system_script_count: random(),
            tx_hashes:                vec![H256::random()],
        };
        let bytes = proposal.encode_msg().unwrap();
        let decode: Proposal = Proposal::decode_msg(bytes).unwrap();
        assert_eq!(proposal, decode);
    }
}
