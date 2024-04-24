use crate::client::avs::error::AvsRegistryError;
use crate::client::avs::NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE;
use crate::contract::{
    bls_apk_registry::{bls_apk_registry, NewPubkeyRegistrationFilter},
    operator_state_retriever::{operator_state_retriever, CheckSignaturesIndices, Operator},
    registry_coordinator::registry_coordinator,
    stake_registry::stake_registry,
};

use eigensdk_crypto_bls::attestation::{G1Point, G2Point};
use eigensdk_crypto_bn254::utils::u256_to_bigint256;
use eigensdk_types::operator::{bitmap_to_quorum_ids, OperatorPubKeys};
use ethers::providers::Middleware;
use ethers_core::types::{
    Address, BlockNumber, Bytes, Filter, FilterBlockOption, Topic, ValueOrArray, H256, U256,
};
use ethers_providers::{Http, Provider};
use num_bigint::BigInt;
use std::fmt::Debug;
use tracing::debug;

use std::collections::HashMap;

#[derive(Debug)]
pub struct AvsRegistryChainReader {
    bls_apk_registry_addr:     Address,
    registry_coordinator_addr: Address,
    operator_state_retriever:  Address,
    stake_registry_addr:       Address,
    eth_client:                Provider<Http>,
}

trait AvsRegistryReader {
    fn get_quorum_count() -> Result<u8, String>;
}

impl AvsRegistryChainReader {
    pub fn new(
        registry_coordinator_addr: Address,
        bls_apk_registry_addr: Address,
        operator_state_retriever: Address,
        stake_registry_addr: Address,
        eth_client: Provider<Http>,
    ) -> Self {
        AvsRegistryChainReader {
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever,
            stake_registry_addr,
            eth_client,
        }
    }

    pub async fn build_avs_registry_chain_reader(
        &self,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
    ) -> Result<AvsRegistryChainReader, AvsRegistryError> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let bls_apk_registry_addr = contract_registry_coordinator
            .bls_apk_registry()
            .call()
            .await;

        match bls_apk_registry_addr {
            Ok(address) => Ok(AvsRegistryChainReader {
                bls_apk_registry_addr: address,
                registry_coordinator_addr,
                operator_state_retriever: operator_state_retriever_addr,
                stake_registry_addr,
                eth_client: self.eth_client.clone(),
            }),

            Err(_) => Err(AvsRegistryError::GetBlsApkRegistry),
        }
    }

    pub async fn get_quorum_count(&self) -> Result<u8, AvsRegistryError> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let quorum_count = contract_registry_coordinator.quorum_count().call().await;

        match quorum_count {
            Ok(quorum) => {
                return Ok(quorum);
            }

            Err(_) => Err(AvsRegistryError::GetQuorumCount),
        }
    }

    pub async fn get_operators_stake_in_quorums_at_block(
        &self,
        block_number: u32,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<Operator>>, AvsRegistryError> {
        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                self.operator_state_retriever,
                self.eth_client.clone().into(),
            );
        let operator_state_result = contract_operator_state_retriever
            .get_operator_state(self.registry_coordinator_addr, quorum_numbers, block_number)
            .call()
            .await;

        match operator_state_result {
            Ok(operator_state) => {
                return Ok(operator_state);
            }
            Err(_) => {
                return Err(AvsRegistryError::GetOperatorState);
            }
        }
    }

    pub async fn get_operators_stake_in_quorums_at_block_operator_id(
        &self,
        block_number: u32,
        operator_id: H256,
    ) -> Result<(U256, Vec<Vec<Operator>>), AvsRegistryError> {
        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                self.operator_state_retriever,
                self.eth_client.clone().into(),
            );
        let operator_state_with_registry_coordinator_and_oeprator_id_result =
            contract_operator_state_retriever
                .get_operator_state_with_registry_coordinator_and_operator_id(
                    self.registry_coordinator_addr,
                    operator_id.into(),
                    block_number,
                )
                .call()
                .await;

        match operator_state_with_registry_coordinator_and_oeprator_id_result {
            Ok(operator_state_with_registry_coordinator_and_oeprator_id) => {
                return Ok(operator_state_with_registry_coordinator_and_oeprator_id);
            }
            Err(_) => {
                return Err(AvsRegistryError::GetOperatorStateWithRegistryCoordinatorAndOperatorId);
            }
        }
    }

    pub async fn get_operators_stake_in_quorums_at_current_block(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<Operator>>, AvsRegistryError> {
        let current_block_number_result = self.eth_client.get_block_number().await;

        match current_block_number_result {
            Ok(current_block_number) => {
                if current_block_number > u32::MAX.into() {
                    return Err(AvsRegistryError::BlockNumberOverflow);
                }

                let operators_stake_in_quorums_at_block_result = self
                    .get_operators_stake_in_quorums_at_block(
                        current_block_number.as_u64() as u32,
                        quorum_numbers,
                    )
                    .await;

                match operators_stake_in_quorums_at_block_result {
                    Ok(operators_stake_in_quorums_at_block) => {
                        Ok(operators_stake_in_quorums_at_block)
                    }
                    Err(_) => return Err(AvsRegistryError::GetOperatorStakeInQuorumAtBlockNumber),
                }
            }
            Err(_) => return Err(AvsRegistryError::GetBlockNumber),
        }
    }

    pub async fn get_operators_stake_in_quorums_of_operator_at_block(
        &self,
        operator_id: H256,
        block_number: u32,
    ) -> Result<(Vec<u8>, Vec<Vec<Operator>>), AvsRegistryError> {
        let result_ = self
            .get_operators_stake_in_quorums_at_block_operator_id(block_number, operator_id)
            .await;

        match result_ {
            Ok((quorum_bitmaps, operator_stakes)) => {
                let quorums = bitmap_to_quorum_ids(quorum_bitmaps);

                let s = (quorums, operator_stakes);
                return Ok(s);
            }
            Err(_) => return Err(AvsRegistryError::GetOperatorStakeInQuorumAtBlockOperatorId),
        }
    }

    pub async fn get_operators_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: H256,
    ) -> Result<(Vec<u8>, Vec<Vec<Operator>>), AvsRegistryError> {
        let current_block_number_result = self.eth_client.get_block_number().await;

        match current_block_number_result {
            Ok(current_block_number) => {
                if current_block_number > u32::MAX.into() {
                    return Err(AvsRegistryError::BlockNumberOverflow);
                }

                let operator_stake_in_quorum_of_operaotr_at_block_result = self
                    .get_operators_stake_in_quorums_of_operator_at_block(
                        operator_id,
                        current_block_number.as_u64() as u32,
                    )
                    .await;

                match operator_stake_in_quorum_of_operaotr_at_block_result {
                    Ok(operator_stake_in_quorum_of_operaotr_at_block) => {
                        Ok(operator_stake_in_quorum_of_operaotr_at_block)
                    }
                    Err(_) => {
                        return Err(AvsRegistryError::GetOperatorStakeInQuorumAtCurrentBlockNumber)
                    }
                }
            }
            Err(_) => return Err(AvsRegistryError::GetBlockNumber),
        }
    }

    pub async fn get_operator_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: H256,
    ) -> Result<HashMap<u8, BigInt>, AvsRegistryError> {
        let registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let quorum_bitmap_result = registry_coordinator
            .get_current_quorum_bitmap(operator_id.into())
            .call()
            .await;

        match quorum_bitmap_result {
            Ok(quorum_bitmap) => {
                let quorums = bitmap_to_quorum_ids(quorum_bitmap);

                let mut quorum_stakes: HashMap<u8, BigInt> = HashMap::new();
                let stake_registry = stake_registry::StakeRegistry::new(
                    self.stake_registry_addr,
                    self.eth_client.clone().into(),
                );
                for quorum in quorums.iter() {
                    let stakes_result = stake_registry
                        .get_current_stake(operator_id.into(), *quorum)
                        .call()
                        .await;

                    match stakes_result {
                        Ok(current_stake) => {
                            quorum_stakes.insert(*quorum, current_stake.into());
                        }
                        Err(_) => return Err(AvsRegistryError::GetCurrentStake),
                    }
                }
                Ok(quorum_stakes)
            }
            Err(_) => return Err(AvsRegistryError::GetCurrentQuorumBitmap),
        }
    }

    pub async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<[u8; 32]>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError> {
        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                self.operator_state_retriever,
                self.eth_client.clone().into(),
            );

        let check_signature_indices_result = contract_operator_state_retriever
            .get_check_signatures_indices(
                self.registry_coordinator_addr,
                reference_block_number,
                quorum_numbers.into(),
                non_signer_operator_ids,
            )
            .call()
            .await;

        match check_signature_indices_result {
            Ok(check_signature_indices) => Ok(check_signature_indices),
            Err(_) => return Err(AvsRegistryError::CheckSignatureIndices),
        }
    }

    pub async fn get_operator_id(
        &self,
        operator_address: Address,
    ) -> Result<[u8; 32], AvsRegistryError> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let operator_id_result = contract_registry_coordinator
            .get_operator_id(operator_address)
            .call()
            .await;

        match operator_id_result {
            Ok(operator_id) => {
                return Ok(operator_id);
            }
            Err(_) => return Err(AvsRegistryError::GetOperatorId),
        }
    }

    pub async fn get_operator_from_id(
        &self,
        operator_id: H256,
    ) -> Result<Address, AvsRegistryError> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let operator_address_result = contract_registry_coordinator
            .get_operator_from_id(operator_id.into())
            .call()
            .await;

        match operator_address_result {
            Ok(operator_address) => Ok(operator_address),
            Err(_) => return Err(AvsRegistryError::GetOperatorFromId),
        }
    }

    pub async fn is_operator_registered(
        &self,
        operator_address: Address,
    ) -> Result<bool, AvsRegistryError> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let operator_status_result = contract_registry_coordinator
            .get_operator_status(operator_address)
            .call()
            .await;

        match operator_status_result {
            Ok(operator_status) => Ok(operator_status == 1),
            Err(_) => return Err(AvsRegistryError::GetOperatorStatus),
        }
    }

    pub async fn query_existing_registered_operator_pub_keys(
        &self,
        start_block: Option<BlockNumber>,
        stop_block: Option<BlockNumber>,
    ) -> Result<(Vec<Address>, Vec<OperatorPubKeys>), AvsRegistryError> {
        let mut block_option: FilterBlockOption = FilterBlockOption::Range {
            from_block: (start_block),
            to_block:   (stop_block),
        };

        if stop_block.is_none() {
            let current_block_number_result = self.eth_client.get_block_number().await;

            match current_block_number_result {
                Ok(current_block_number) => {
                    block_option =
                        block_option.set_to_block(BlockNumber::Number(current_block_number));
                }
                Err(_) => return Err(AvsRegistryError::GetBlockNumber),
            }
        }

        let query = Filter {
            block_option,
            address: Some(ValueOrArray::Value(self.bls_apk_registry_addr)),
            topics: [
                Some(Topic::Value(Some(NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE))),
                None,
                None,
                None,
            ],
        };

        let contract_bls_apk_registry = bls_apk_registry::BLSApkRegistry::new(
            self.bls_apk_registry_addr,
            self.eth_client.clone().into(),
        );
        let logs_result = self.eth_client.get_logs(&query).await;

        match logs_result {
            Ok(logs) => {
                debug!(transactionLogs = ?logs, "avsRegistryChainReader.QueryExistingRegisteredOperatorPubKeys");
                let mut operator_addresses: Vec<Address> = vec![];
                let mut operator_pub_keys: Vec<OperatorPubKeys> = vec![];

                for (i, v_log) in logs.iter().enumerate() {
                    let operator_addr = Address::from_slice(&v_log.topics[i].as_bytes()[12..]);
                    operator_addresses.push(operator_addr);

                    let decoded_event_result = contract_bls_apk_registry
                        .decode_event::<NewPubkeyRegistrationFilter>(
                            "NewPubkeyRegistration",
                            v_log.topics.clone(),
                            v_log.data.clone(),
                        );

                    match decoded_event_result {
                        Ok(decoded_event) => {
                            let g1_pub_key = decoded_event.pubkey_g1;
                            let g2_pub_key = decoded_event.pubkey_g2;

                            let operator_pub_key = OperatorPubKeys {
                                g1_pub_key: G1Point::new(
                                    u256_to_bigint256(g1_pub_key.x),
                                    u256_to_bigint256(g1_pub_key.y),
                                ),
                                g2_pub_key: G2Point::new(
                                    (
                                        u256_to_bigint256(g2_pub_key.x[0]),
                                        u256_to_bigint256(g2_pub_key.x[1]),
                                    ),
                                    (
                                        u256_to_bigint256(g2_pub_key.y[0]),
                                        u256_to_bigint256(g2_pub_key.y[1]),
                                    ),
                                ),
                            };

                            operator_pub_keys.push(operator_pub_key);
                        }
                        Err(_) => {
                            return Err(AvsRegistryError::DecodeEventNewPubkeyRegistrationFilter)
                        }
                    }
                }

                Ok((operator_addresses, operator_pub_keys))
            }
            Err(_) => return Err(AvsRegistryError::GetEthLogs),
        }
    }
}
