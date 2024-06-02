use crate::contract::{
    axon_avs_service_manager::AxonAVSServiceManager,
    axon_avs_task_manager::{AxonAVSTaskManager, TaskRespondedFilter},
    registry_coordinator::RegistryCoordinator,
};

use ethers_core::types::Address;
use ethers_providers::{Http, Provider, StreamExt};

use std::sync::Arc;
/// AvsRegistry Chain Subscriber struct
#[derive(Debug, Clone)]
pub struct AvsRegistryChainSubscriber {
    provider:                Provider<Http>,
    task_manager_address:    Address,
    service_manager_address: Address,
}

impl AvsRegistryChainSubscriber {
    pub async fn new(provider: Provider<Http>, registry_coordinator_address: Address) -> Self {
        let service_manager_address =
            RegistryCoordinator::new(registry_coordinator_address, Arc::new(provider.clone()))
                .service_manager()
                .call()
                .await
                .unwrap();
        let service_manager =
            AxonAVSServiceManager::new(service_manager_address, Arc::new(provider.clone()));
        let task_manager_address = service_manager
            .axon_avs_task_manager()
            .call()
            .await
            .unwrap();

        AvsRegistryChainSubscriber {
            provider,
            task_manager_address,
            service_manager_address,
        }
    }

    pub async fn run(self) {
        let event =
            AxonAVSTaskManager::new(self.task_manager_address, Arc::new(self.provider.clone()))
                .event_for_name::<TaskRespondedFilter>("TaskResponded")
                .unwrap();
        let mut event_stream = event.stream().await.unwrap();

        while let Some(Ok(evt)) = event_stream.next().await {
            let TaskRespondedFilter {
                task_response,
                task_response_metadata,
            } = evt;

            println!(
                "TaskResponded: task_response: {:?}, task_response_metadata: {:?}",
                task_response, task_response_metadata
            );

            println!(
                "TaskResponded: task_response: {:?}, task_response_metadata: {:?}",
                task_response, task_response_metadata
            );
        }
    }
}
