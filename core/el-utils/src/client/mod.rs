pub mod avs;
pub mod elcontract;
pub mod eth;

use crate::client::avs::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
    writer::AvsRegistryChainWriter,
};
use crate::client::elcontract::{reader::ELChainReader, writer::ELChainWriter};
use crate::client::eth::client::EthClient;
use crate::contract::bls_apk_registry::{BLSApkRegistryEvents, InitializedFilter};

use eigensdk_client_wallet::privatekey_wallet::PrivateKeyWallet;
use eigensdk_txmgr::SimpleTxManager;
use ethers::types::Address;
use ethers_providers::{Http, Provider};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ClientBuilder {
    pub eth_http_url:                 String,
    pub eth_ws_url:                   String,
    pub bls_apk_registry_addr:        Address,
    pub operator_registry_addr:       Address,
    pub registry_coordinator_addr:    Address,
    pub avs_name:                     String,
    pub from_metrics_ip_port_address: String,
    pub sender:                       Address,
    pub private_key:                  String,
}

impl ClientBuilder {
    pub async fn build(self) -> ElClient {
        let eth_client = Provider::<Http>::try_from(&self.eth_http_url).unwrap();
        let eth_http_client = EthClient::new(eth_client.clone(), "http");
        let eth_ws_client = EthClient::new(eth_client.clone(), "ws");

        // let avs_registry_chain_reader = AvsRegistryChainReader::new(
        //     self.registry_coordinator_addr,
        //     self.bls_apk_registry_addr,
        //     self.operator_registry_addr,
        //     self.stake_registry_addr,
        //     eth_client.clone(),
        // );

        let avs_registry_chain_subscriber =
            AvsRegistryChainSubscriber::new(eth_client.clone(), self.registry_coordinator_addr)
                .await;

        // let el_chain_reader = ELChainReader::build(
        //     self.delegation_manager_addr,
        //     self.avs_directory_addr,
        //     eth_client.clone(),
        // )
        // .await
        // .unwrap();

        // let avs_registry_chain_writer =
        // AvsRegistryChainWriter::new_avs_registry_chain_writer(
        //     self.service_manager_addr,
        //     self.registry_coordinator_addr,
        //     self.operator_registry_addr,
        //     self.stake_registry_addr,
        //     self.bls_apk_registry_addr,
        //     el_chain_reader.clone(),
        //     eth_client.clone(),
        //     Self::tx_mgr(eth_client.clone(), self.private_key.clone(), self.sender),
        // )
        // .await;

        // let el_chain_writer = ELChainWriter::new(
        //     self.delegation_manager_addr,
        //     self.strategy_manager_addr,
        //     el_chain_reader.clone(),
        //     eth_client.clone(),
        //     Self::tx_mgr(eth_client, self.private_key, self.sender),
        // );

        ElClient {
            avs_registry_chain_subscriber,
            eth_http_client,
        }
    }

    fn tx_mgr(eth_client: Provider<Http>, private_key: String, sender: Address) -> SimpleTxManager {
        let wallet = PrivateKeyWallet::new(eth_client.clone(), &private_key).unwrap();
        SimpleTxManager::new(wallet, eth_client, sender)
    }
}

pub struct ElClient {
    // avs_registry_chain_reader:     AvsRegistryChainReader,
    avs_registry_chain_subscriber: AvsRegistryChainSubscriber,
    // avs_registry_chain_writer:     AvsRegistryChainWriter,
    // el_chain_reader:               ELChainReader,
    // el_chain_writer:               ELChainWriter,
    eth_http_client:               EthClient,
    // eth_ws_client:                 EthClient,
}

impl ElClient {
    pub async fn run(self) {
        self.avs_registry_chain_subscriber.run().await;
    }
}
