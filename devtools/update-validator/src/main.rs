use std::fs::File;
use std::io::Read;

use clap::{Arg, Command};
use common_crypto::{
    HashValue, PrivateKey as _, PublicKey as _, Secp256k1RecoverablePrivateKey, Signature as _,
    ToPublicKey as _,
};
use core_executor::system_contract::{
    metadata::metadata_abi::{self, UpdateValidatorListCall, ValidatorList},
    METADATA_CONTRACT_ADDRESS,
};
use ethers::abi::AbiEncode as _;
use jsonrpsee::{core::client::ClientT as _, http_client::HttpClientBuilder, rpc_params};
use protocol::{
    constants::MAX_BLOCK_GAS_LIMIT,
    types::{
        Address, Hex, Key256Bits, LegacyTransaction, SignatureComponents, TransactionAction,
        UnsignedTransaction, UnverifiedTransaction, Validator, ValidatorExtend, H256, U256, U64,
    },
};
use rlp::Encodable as _;
use serde::{de::DeserializeOwned, Deserialize};

#[tokio::main]
async fn main() {
    let matches = Command::new("update-validator")
        .arg(
            Arg::new("validators")
                .short('v')
                .long("validators")
                .help("The new validators path."),
        )
        .arg(
            Arg::new("private_key")
                .short('p')
                .long("private_key")
                .help("The private key path."),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .help("The url of the node."),
        )
        .get_matches();

    let private_key = Secp256k1RecoverablePrivateKey::try_from(
        read_private_key(&matches.get_one::<String>("private_key").unwrap()).as_slice(),
    )
    .unwrap();
    let address = Address::from_pubkey_bytes(private_key.pub_key().to_bytes())
        .unwrap()
        .0;
    let validators =
        parse_reader::<ValidatorInfo>(&matches.get_one::<String>("validators").unwrap());

    let client = HttpClientBuilder::default()
        .build(matches.get_one::<String>("url").unwrap())
        .unwrap();
    let chain_id: U256 = client.request("eth_chainId", rpc_params![]).await.unwrap();
    let nonce: U256 = client
        .request("eth_getTransactionCount", rpc_params![address, "latest"])
        .await
        .unwrap();
    let tx = build_update_validator_tx(
        chain_id.low_u64(),
        nonce.low_u64().into(),
        validators.validators.into_iter().map(Into::into).collect(),
        private_key,
    );
    let raw_tx = Hex::encode(tx.rlp_bytes()).as_string();
    let hash: H256 = client
        .request("eth_sendRawTransaction", rpc_params![raw_tx])
        .await
        .unwrap();
    eprintln!("tx hash: {:?}", hash);
}

#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorInfo {
    pub validators: Vec<Validator>,
}

pub fn parse_reader<T: DeserializeOwned>(path: &str) -> T {
    let mut buf = String::new();
    File::open(path).unwrap().read_to_string(&mut buf).unwrap();
    toml::from_str(&buf).unwrap()
}

fn read_private_key(path: &str) -> Key256Bits {
    File::open(path)
        .and_then(|mut f| {
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).map(|_| buffer)
        })
        .and_then(|bytes| {
            const LEN: usize = 32;
            if bytes.len() == LEN {
                let mut v = [0u8; 32];
                v.copy_from_slice(&bytes);
                Ok(Key256Bits::from(v))
            } else {
                panic!("Invalid private key file");
            }
        })
        .unwrap()
}

fn build_update_validator_tx(
    chain_id: u64,
    nonce: U64,
    validators: Vec<ValidatorExtend>,
    private_key: Secp256k1RecoverablePrivateKey,
) -> UnverifiedTransaction {
    let unsigned = UnsignedTransaction::Legacy(LegacyTransaction {
        nonce:     nonce.into(),
        gas_price: 8u64.into(),
        gas_limit: MAX_BLOCK_GAS_LIMIT.into(),
        action:    TransactionAction::Call(METADATA_CONTRACT_ADDRESS),
        value:     U256::zero(),
        data:      metadata_abi::MetadataContractCalls::UpdateValidatorList(
            UpdateValidatorListCall {
                validators: ValidatorList {
                    verifier_list: validators.into_iter().map(Into::into).collect(),
                },
            },
        )
        .encode()
        .into(),
    });
    let mut tx = UnverifiedTransaction {
        unsigned,
        signature: None,
        chain_id: Some(chain_id),
        hash: Default::default(),
    };
    let sig = sign(private_key, tx.signature_hash(true));
    tx.signature = Some(sig);
    tx.calc_hash()
}

fn sign(private: Secp256k1RecoverablePrivateKey, hash: H256) -> SignatureComponents {
    let sig = private.sign_message(&HashValue::from_bytes_unchecked(hash.0));
    SignatureComponents::from(sig.to_bytes())
}
