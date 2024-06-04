use std::str::FromStr;

use bls_on_arkworks::types::{G1AffinePoint, G2AffinePoint};
use bls_on_arkworks::{keygen, pubkey_to_point, sign, signature_to_point, sk_to_pk, DST_ETHEREUM};
use el_utils::aggregator::Aggregator;
use el_utils::contract::axon_avs_task_manager::NewTaskCreatedFilter;
use ethers::abi::{AbiEncode, RawLog};
use ethers::contract::EthLogDecode;
use ethers::types::H256;
use futures_util::stream::StreamExt;
use protocol::tokio;
use protocol::types::{Address, Hasher, Hex};

const SIGNER_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const SIGNER_ADDR: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const TASK_MANAGER_ADDR: &str = "0x9E545E3C0baAB3E08CdfD552C960A1050f373042";
const HTTP_URL: &str = "http://127.0.0.1:8545";
const WS_URL: &str = "ws://127.0.0.1:8545";

#[tokio::main]
async fn main() {
    let signer_key = Hex::from_str(SIGNER_KEY).unwrap();
    let signer_addr = Address::from_str(SIGNER_ADDR).unwrap();
    let task_manager_addr = Address::from_str(TASK_MANAGER_ADDR).unwrap();
    let aggregator = Aggregator::new(
        HTTP_URL.to_string(),
        WS_URL.to_string(),
        task_manager_addr.0,
        signer_key,
        signer_addr.0,
    )
    .await;
    let mut subscriber = aggregator.subscribe_new_task().await;

    while let Some(log) = subscriber.next().await {
        let new_task = NewTaskCreatedFilter::decode_log(&RawLog::from(log)).unwrap();
        let _ = sign_proof(Hasher::digest(AbiEncode::encode(new_task.task.clone())));

        aggregator
            .send_resp_proof(new_task.task, new_task.task_index, Default::default())
            .await;
    }
}

pub fn sign_proof(hash: H256) -> (G1AffinePoint, G2AffinePoint) {
    let sign_key = keygen(&DST_ETHEREUM.as_bytes().to_vec());
    let pubkey = sk_to_pk(sign_key.clone());
    let signature = sign(
        sign_key,
        &hash.as_bytes().to_vec(),
        &DST_ETHEREUM.as_bytes().to_vec(),
    )
    .unwrap();

    (
        pubkey_to_point(&pubkey).unwrap(),
        signature_to_point(&signature).unwrap(),
    )
}
