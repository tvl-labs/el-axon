use ethers::types::{Address, Bytes};
use ethers::{contract::EthEvent, signers::LocalWallet};
use ethers_providers::{Http, Middleware, Provider};

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::contract::axon_avs_task_manager::{AxonAVSTaskManager, NewTaskCreatedFilter};

pub struct Aggregator {
    pub eth_client:        Provider<Http>,
    pub task_manager_addr: Address,
    private_key:           String,
    tasks:                 BTreeMap<u32, NewTaskCreatedFilter>,
}

impl Aggregator {
    pub fn new(
        eth_client: Provider<Http>,
        task_manager_addr: Address,
        private_key: String,
    ) -> Self {
        Aggregator {
            eth_client,
            task_manager_addr,
            private_key,
            tasks: BTreeMap::new(),
        }
    }

    pub async fn send_new_task(
        &mut self,
        proof: Bytes,
        quorum_threshold_percentage: u32,
        quorum_numbers: Bytes,
    ) {
        let new_task = self
            .send_new_proof(proof, quorum_threshold_percentage, quorum_numbers)
            .await;
        self.tasks.insert(new_task.task_index, new_task.clone());
        let mut threshold = vec![0u32; new_task.task.quorum_numbers.len()];
        for i in 0..new_task.task.quorum_numbers.len() {
            threshold[i] = new_task.task.quorum_threshold_percentage;
        }
    }

    pub async fn send_new_proof(
        &self,
        proof: Bytes,
        quorum_threshold_percentage: u32,
        quorum_numbers: Bytes,
    ) -> NewTaskCreatedFilter {
        let wallet = LocalWallet::from_bytes(self.private_key.as_bytes()).unwrap();
        let task_manager =
            AxonAVSTaskManager::new(self.task_manager_addr, Arc::new(self.eth_client.clone()));
        let tx = task_manager
            .create_new_task(proof, quorum_threshold_percentage, quorum_numbers)
            .tx;
        let sig = wallet.sign_transaction_sync(&tx).unwrap();
        let receipt = self
            .eth_client
            .send_transaction(tx, None)
            .await
            .unwrap()
            .await
            .unwrap();
        NewTaskCreatedFilter::decode_log(&receipt.unwrap().logs[0].clone().into()).unwrap()
    }
}
