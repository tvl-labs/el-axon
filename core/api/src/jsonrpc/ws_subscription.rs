use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use jsonrpsee::server::{
    IdProvider, IntoSubscriptionCloseResponse, PendingSubscriptionSink, RpcModule,
    SubscriptionMessage, SubscriptionSink,
};
use jsonrpsee::types::{error::ErrorCode, params::Params, ErrorObjectOwned, SubscriptionId};
use serde::{Deserialize, Serialize};

use core_consensus::SYNC_STATUS;
use protocol::tokio::sync::mpsc::{channel, Receiver, Sender};
use protocol::tokio::{self, select, time::interval};
use protocol::traits::{APIAdapter, Context};
use protocol::types::{BigEndianHash, Hash, Hex, H160, H256, U256};

use crate::jsonrpc::{
    r#impl::from_receipt_to_web3_log,
    web3_types::{MultiNestType, MultiType, Web3Header, Web3SyncStatus},
};

pub async fn ws_subscription_module<Adapter>(adapter: Arc<Adapter>) -> RpcModule<Sender<RawHub>>
where
    Adapter: APIAdapter + 'static,
{
    let (tx, rx) = channel(128);

    let inner = Subscription::new(adapter, rx).await;

    tokio::spawn(inner.run());

    let mut rpc = RpcModule::new(tx);
    rpc.register_subscription(
        "eth_subscribe",
        "eth_subscription",
        "eth_unsubscribe",
        subscription_callback,
    )
    .unwrap();
    rpc
}

async fn subscription_callback(
    params: Params<'static>,
    sink: PendingSubscriptionSink,
    ctx: Arc<Sender<RawHub>>,
) -> impl IntoSubscriptionCloseResponse {
    match Type::try_from(params) {
        Ok(type_) => {
            let raw_hub = RawHub {
                typ:  type_,
                sink: sink.accept().await?,
            };

            tokio::spawn(async move {
                let _ignore = ctx.send(raw_hub).await;
            });
            Ok(())
        }
        Err(e) => {
            sink.reject(e).await;
            Ok(())
        }
    }
}

pub struct Subscription<Adapter> {
    log_hubs:       Vec<Hub<LoggerFilter>>,
    header_hubs:    Vec<Hub<()>>,
    sync_hubs:      Vec<Hub<()>>,
    adapter:        Arc<Adapter>,
    current_number: u64,
    recv:           Receiver<RawHub>,
}

impl<Adapter> Subscription<Adapter>
where
    Adapter: APIAdapter + 'static,
{
    pub async fn new(adapter: Arc<Adapter>, recv: Receiver<RawHub>) -> Self {
        let latest = adapter
            .get_block_header_by_number(Context::new(), None)
            .await
            .unwrap()
            .unwrap();

        Self {
            log_hubs: Vec::new(),
            header_hubs: Vec::new(),
            sync_hubs: Vec::new(),
            adapter,
            current_number: latest.number,
            recv,
        }
    }

    async fn notify(&mut self) {
        self.header_hubs.retain(|hub| !hub.sink.is_closed());
        self.sync_hubs.retain(|hub| !hub.sink.is_closed());
        self.log_hubs.retain(|hub| !hub.sink.is_closed());

        let latest_block = self
            .adapter
            .get_block_by_number(Context::new(), None)
            .await
            .unwrap()
            .unwrap();

        if self.current_number == latest_block.header.number {
            return;
        }

        let mut log_vec = Vec::new();
        let latest_header_number = latest_block.header.number;

        // Send all header
        if !self.header_hubs.is_empty() {
            for number in self.current_number + 1..latest_block.header.number {
                let block = self
                    .adapter
                    .get_block_by_number(Context::new(), Some(number))
                    .await
                    .unwrap()
                    .unwrap();

                log_vec.push((block.header.number, block.tx_hashes));

                let web3_header = Web3Header::from(block.header);
                let msg = SubscriptionMessage::from_json(&web3_header).unwrap();

                for hub in self.header_hubs.iter_mut() {
                    let _ignore = hub.sink.send(msg.clone()).await;
                }
            }

            let latest_web3_header = Web3Header::from(latest_block.header);
            let msg = SubscriptionMessage::from_json(&latest_web3_header).unwrap();

            for hub in self.header_hubs.iter_mut() {
                let _ignore = hub.sink.send(msg.clone()).await;
            }
        }

        // Send all sync status
        if !self.sync_hubs.is_empty() {
            let web3_sync_state: Web3SyncStatus = { SYNC_STATUS.read().clone().into() };
            let msg = SubscriptionMessage::from_json(&web3_sync_state).unwrap();

            for hub in self.sync_hubs.iter_mut() {
                // unbound sender can ignore it's return
                let _ignore = hub.sink.send(msg.clone()).await;
            }
        }

        // Send all logs
        if !self.log_hubs.is_empty() {
            // May not has header_hub
            if log_vec.is_empty() {
                for number in self.current_number + 1..latest_header_number {
                    let block = self
                        .adapter
                        .get_block_by_number(Context::new(), Some(number))
                        .await
                        .unwrap()
                        .unwrap();

                    log_vec.push((block.header.number, block.tx_hashes));
                }
            }

            log_vec.push((latest_header_number, latest_block.tx_hashes));

            for (number, tx_hashes) in log_vec {
                let receipts = self
                    .adapter
                    .get_receipts_by_hashes(Context::new(), number, &tx_hashes)
                    .await
                    .unwrap();

                let mut index = 0;
                let mut logs = Vec::new();
                for receipt in receipts.into_iter().flatten() {
                    let log_len = receipt.logs.len();
                    for hub in self.log_hubs.iter_mut() {
                        from_receipt_to_web3_log(
                            index,
                            hub.filter.topics.as_slice(),
                            hub.filter.address.as_ref().unwrap_or(&Vec::new()),
                            &receipt,
                            &mut logs,
                        );

                        for log in logs.drain(..) {
                            let msg = SubscriptionMessage::from_json(&log).unwrap();
                            // unbound sender can ignore it's return
                            let _ignore = hub.sink.send(msg).await;
                        }
                    }
                    index += log_len;
                }
            }
        }

        self.current_number = latest_header_number;
    }

    pub async fn run(mut self) {
        let mut time_internal = interval(Duration::from_secs(3));
        loop {
            select! {
                event = self.recv.recv() => {
                    match event {
                        Some(hub) => {
                            match hub.typ {
                                Type::NewHeads => self.header_hubs.push(Hub{filter: (), sink: hub.sink}),
                                Type::Logs(filter) => self.log_hubs.push(Hub{filter, sink: hub.sink}),
                                Type::Syncing => self.sync_hubs.push(Hub{filter: (), sink: hub.sink})
                            }
                        },
                        None => {
                            break
                        }
                    }
                }
                _ = time_internal.tick() => {
                    self.notify().await;
                }
                else => {
                    break
                }
            }
        }
    }
}

enum Type {
    NewHeads,
    Logs(LoggerFilter),
    Syncing,
}

impl<'a> TryFrom<Params<'a>> for Type {
    type Error = ErrorObjectOwned;

    fn try_from(value: Params<'a>) -> Result<Self, Self::Error> {
        let mut iter = value.sequence();

        let method: &str = iter.next()?;

        match method {
            "newHeads" => Ok(Type::NewHeads),
            "syncing" => Ok(Type::Syncing),
            "logs" => {
                let filter: RawLoggerFilter = iter.next()?;
                Ok(Type::Logs(filter.into()))
            }
            _ => Err(ErrorCode::MethodNotFound.into()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct RawLoggerFilter {
    #[serde(default)]
    address: MultiType<H160>,
    topics:  Option<Vec<MultiNestType<Hash>>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct LoggerFilter {
    address: Option<Vec<H160>>,
    topics:  Vec<Option<Vec<Option<Hash>>>>,
}

impl From<RawLoggerFilter> for LoggerFilter {
    fn from(src: RawLoggerFilter) -> Self {
        LoggerFilter {
            address: src.address.into(),
            topics:  src
                .topics
                .unwrap_or_default()
                .into_iter()
                .take(4)
                .map(Into::<Option<Vec<Option<H256>>>>::into)
                .collect(),
        }
    }
}

pub struct RawHub {
    typ:  Type,
    sink: SubscriptionSink,
}

struct Hub<T> {
    filter: T,
    sink:   SubscriptionSink,
}

#[derive(Debug)]
pub struct HexIdProvider {
    id: AtomicU64,
}

impl Default for HexIdProvider {
    fn default() -> Self {
        Self {
            id: AtomicU64::new(0),
        }
    }
}

impl IdProvider for HexIdProvider {
    fn next_id(&self) -> SubscriptionId<'static> {
        let id = self.id.fetch_add(1, Ordering::Acquire);
        let hash: Hash = BigEndianHash::from_uint(&U256::from(id));

        SubscriptionId::Str(beef::Cow::owned(Hex::encode(hash.as_bytes()).as_string()))
    }
}
