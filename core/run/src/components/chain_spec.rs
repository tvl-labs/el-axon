use common_config_parser::types::spec::ChainSpec;

use protocol::types::{Block, RichBlock};

pub(crate) trait ChainSpecExt {
    //! Generate the genesis block.
    fn generate_genesis_block(&self) -> RichBlock;
}

impl ChainSpecExt for ChainSpec {
    fn generate_genesis_block(&self) -> RichBlock {
        let txs = vec![];
        let block = Block {
            header:    self.genesis.build_header(),
            tx_hashes: vec![],
        };

        RichBlock { block, txs }
    }
}
