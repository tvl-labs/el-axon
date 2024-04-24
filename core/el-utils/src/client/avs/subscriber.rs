use crate::client::avs::NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE;
use crate::contract::incredible_squaring_task_manager::{
    IncredibleSquaringTaskManager, TaskRespondedFilter,
};

use ethers_core::types::{Address, Filter};
use ethers_providers::{
    Http, Middleware, Provider, ProviderError, StreamExt, SubscriptionStream, Ws,
};
use futures::Future;

use std::pin::Pin;
use std::sync::Arc;

type MyStream<'a, T> =
    Pin<Box<dyn Future<Output = Result<SubscriptionStream<'static, Ws, T>, ProviderError>> + Send>>;

/// AvsRegistry Chain Subscriber struct
#[derive(Debug, Clone)]
pub struct AvsRegistryChainSubscriber {
    provider:                Provider<Http>,
    task_manager_address:    Address,
    service_manager_address: Address,
}

impl AvsRegistryChainSubscriber {
    pub fn new(
        provider: Provider<Http>,
        task_manager_address: Address,
        service_manager_address: Address,
    ) -> Self {
        AvsRegistryChainSubscriber {
            provider,
            task_manager_address,
            service_manager_address,
        }
    }

    pub async fn run(self) {
        let event = IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            Arc::new(self.provider.inner()),
        )
        .event_for_name::<TaskRespondedFilter>("TaskResponded")
        .unwrap();
        let mut event_stream = event.stream().await.unwrap();

        while let Some(Ok(evt)) = event_stream.next().await {
            let TaskRespondedFilter {
                task_response,
                task_response_metadata,
            } = evt;
        }
    }
}
