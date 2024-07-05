use std::str::FromStr;

use alloy_primitives::Address;
use ark_bn254::{Fq, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInt, BigInteger, PrimeField};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_client_avsregistry::subscriber::AvsRegistryChainSubscriber;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use el_utils::aggregator::Aggregator;
use el_utils::contract::axon_avs_task_manager::{
    G1Point, G2Point, NewTaskCreatedFilter, NonSignerStakesAndSignature,
};
use ethers::abi::{AbiEncode, RawLog};
use ethers::contract::EthLogDecode;
use ethers::types::{H160, H256, U256};
use futures_util::stream::StreamExt;
use num_bigint::BigUint;
use protocol::tokio;
use protocol::types::{Hasher, Hex};
use rust_bls_bn254::sign;

const SIGNER_KEY: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const SIGNER_ADDR: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const TASK_MANAGER_ADDR: &str = "0x9E545E3C0baAB3E08CdfD552C960A1050f373042";
const COORDINATOR_ADDR: &str = "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9";
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
        H160(task_manager_addr.into_array()),
        signer_key,
        H160(signer_addr.into_array()),
    )
    .await;
    let mut subscriber = aggregator.subscribe_new_task().await;
    let registry_caller = new_avs_registry_caller().await;
    let sk = Fr::new(BigInt::one());

    while let Some(log) = subscriber.next().await {
        let new_task = NewTaskCreatedFilter::decode_log(&RawLog::from(log)).unwrap();
        let quorum_numbers = new_task.task.quorum_numbers.to_vec();
        let task_created_block = new_task.task.task_created_block;
        let hash_msg = Hasher::digest(AbiEncode::encode(new_task.task.clone()));
        let (sig, pk) = sign_proof(sk, hash_msg);
        let indices = registry_caller
            .get_avs_registry()
            .get_check_signatures_indices(
                new_task.task.task_created_block,
                quorum_numbers.clone(),
                vec![],
            )
            .await
            .unwrap();
        let quorums_avs_stake = registry_caller
            .get_quorums_avs_state_at_block(quorum_numbers.clone().into(), task_created_block)
            .await;
        let mut quorum_apks_g1: Vec<G1Point> = vec![];
        for quorum_number in quorum_numbers.iter() {
            if let Some(val) = quorums_avs_stake.get(quorum_number) {
                quorum_apks_g1.push(G1Point {
                    x: U256(val.agg_pub_key_g1.X.into_limbs()),
                    y: U256(val.agg_pub_key_g1.Y.into_limbs()),
                });
            }
        }

        let sig = NonSignerStakesAndSignature {
            non_signer_quorum_bitmap_indices: indices.nonSignerQuorumBitmapIndices,
            non_signer_pubkeys: vec![],
            quorum_apks: quorum_apks_g1,
            apk_g2: EthConvert::to_g2(pk).unwrap(),
            sigma: EthConvert::to_g1(sig).unwrap(),
            quorum_apk_indices: indices.quorumApkIndices,
            total_stake_indices: indices.totalStakeIndices,
            non_signer_stake_indices: indices.nonSignerStakeIndices,
        };

        aggregator
            .send_resp_proof(new_task.task, new_task.task_index, sig)
            .await;
    }
}

pub fn sign_proof(sk: Fr, hash: H256) -> (G1Affine, G2Affine) {
    let sig = sign(sk, hash.as_bytes()).unwrap();
    let pk = sk_to_pk(sk);
    (sig, pk)
}

fn sk_to_pk(sk: Fr) -> G2Affine {
    (G2Affine::generator() * sk).into_affine()
}

pub struct EthConvert;

impl EthConvert {
    pub fn to_u256(p: &Fq) -> protocol::types::U256 {
        protocol::types::U256::from_little_endian(&p.into_bigint().to_bytes_le()[..])
    }

    pub fn to_g1(xy: G1Affine) -> Option<G1Point> {
        xy.xy().map(|(x, y)| G1Point {
            x: EthConvert::to_u256(x),
            y: EthConvert::to_u256(y),
        })
    }

    pub fn to_g2(xy: G2Affine) -> Option<G2Point> {
        xy.xy().map(|(x, y)| G2Point {
            x: [EthConvert::to_u256(&x.c1), EthConvert::to_u256(&x.c0)],
            y: [EthConvert::to_u256(&y.c1), EthConvert::to_u256(&y.c0)],
        })
    }

    pub fn from_g1(xy: G1Point) -> G1Affine {
        let mut buf = vec![];
        xy.x.to_little_endian(&mut buf);
        let x = BigUint::from_bytes_le(&buf);

        let mut buf = vec![];
        xy.y.to_little_endian(&mut buf);
        let y = BigUint::from_bytes_le(&buf);

        G1Affine::new(x.into(), y.into())
    }
}

async fn new_avs_registry_caller() -> AvsRegistryServiceChainCaller {
    let chain_reader = AvsRegistryChainReader::new(
        Address::parse_checksummed(COORDINATOR_ADDR, None).unwrap(),
        Address::default(),
        HTTP_URL.to_string(),
    )
    .await
    .unwrap();
    let chain_subscriber = AvsRegistryChainSubscriber::new(WS_URL.to_string());
    AvsRegistryServiceChainCaller::new(
        chain_reader.clone(),
        OperatorInfoServiceInMemory::new(chain_subscriber, chain_reader, WS_URL.to_string()).await,
    )
}
