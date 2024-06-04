use ethers::middleware::{MiddlewareBuilder, NonceManagerMiddleware};
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Address, Bytes, Filter, Log, TransactionReceipt, H160};
use ethers::{contract::EthEvent, signers::LocalWallet};
use ethers_providers::{Http, Middleware, Provider, SubscriptionStream, Ws};
use protocol::types::Hex;

use std::collections::BTreeMap;
use std::sync::Arc;

use crate::contract::axon_avs_task_manager::{
    AxonAVSTaskManager, NewTaskCreatedFilter, NonSignerStakesAndSignature, Task,
    TaskRespondedFilter, TaskResponse,
};

pub struct Aggregator {
    pub eth_client:        Provider<Http>,
    pub ws_client:         Provider<Ws>,
    pub task_manager_addr: Address,
    private_key:           Hex,
    nonce_manager:         NonceManagerMiddleware<Provider<Http>>,
    tasks:                 BTreeMap<u32, NewTaskCreatedFilter>,
}

impl Aggregator {
    pub async fn new(
        http_url: String,
        ws_url: String,
        task_manager_addr: Address,
        private_key: Hex,
        address: H160,
    ) -> Self {
        let provider = Provider::<Http>::try_from(http_url)
            .unwrap()
            .with_sender(address);
        let ws_provider = Provider::connect(ws_url).await.unwrap();
        let provider_clone = provider.clone();

        Aggregator {
            nonce_manager: provider_clone.nonce_manager(address),
            eth_client: provider,
            ws_client: ws_provider,
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

    pub async fn subscribe_new_task(&self) -> SubscriptionStream<Ws, Log> {
        let filter = Filter::new()
            .address(self.task_manager_addr)
            .topic0(NewTaskCreatedFilter::signature());
        self.ws_client.subscribe_logs(&filter).await.unwrap()
    }

    pub async fn send_resp_proof(
        &self,
        task: Task,
        task_response: u32,
        sig: NonSignerStakesAndSignature,
    ) -> TaskRespondedFilter {
        let task_manager =
            AxonAVSTaskManager::new(self.task_manager_addr, Arc::new(self.eth_client.clone()));
        let resp = TaskResponse {
            reference_task_index: task_response,
        };
        let mut tx = task_manager.respond_to_task(task, resp, sig).tx;
        let receipt = self.fill_and_send_tx(&mut tx).await;
        TaskRespondedFilter::decode_log(&receipt.logs[0].clone().into()).unwrap()
    }

    pub async fn send_new_proof(
        &self,
        proof: Bytes,
        quorum_threshold_percentage: u32,
        quorum_numbers: Bytes,
    ) -> NewTaskCreatedFilter {
        let task_manager =
            AxonAVSTaskManager::new(self.task_manager_addr, Arc::new(self.eth_client.clone()));
        let mut tx = task_manager
            .create_new_task(proof, quorum_threshold_percentage, quorum_numbers)
            .tx;

        let receipt = self.fill_and_send_tx(&mut tx).await;
        NewTaskCreatedFilter::decode_log(&receipt.logs[0].clone().into()).unwrap()
    }

    async fn fill_and_send_tx(&self, tx: &mut TypedTransaction) -> TransactionReceipt {
        tx.set_gas(400_000);
        tx.set_chain_id(31337);
        self.nonce_manager.fill_transaction(tx, None).await.unwrap();
        self.eth_client.fill_transaction(tx, None).await.unwrap();

        let wallet = LocalWallet::from_bytes(&self.private_key.as_bytes()).unwrap();
        let sig = wallet.sign_transaction_sync(tx).unwrap();

        println!("tx: {:?}", tx);

        let raw = tx.rlp_signed(&sig);

        let receipt = self
            .eth_client
            .send_raw_transaction(raw)
            .await
            .unwrap()
            .await
            .unwrap();
        println!("{:?}", receipt);
        receipt.unwrap()
    }
}
