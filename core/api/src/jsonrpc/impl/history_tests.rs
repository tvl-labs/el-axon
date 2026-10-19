use super::{test_adapter::MockAdapter, Web3RpcImpl};
use crate::jsonrpc::{web3_types::Web3Filter, Web3RpcServer};
use protocol::tokio;
use std::sync::{atomic::Ordering, Arc};
use std::time::Duration;

fn filter(from: u64, to: u64) -> Web3Filter {
    serde_json::from_value(
        serde_json::json!({"fromBlock": format!("0x{from:x}"), "toBlock": format!("0x{to:x}")}),
    )
    .unwrap()
}

#[tokio::test]
async fn historical_reads_do_not_block_runtime_timers() {
    let adapter = MockAdapter::new(100, 20);
    let rpc = Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000);
    let (result, timer_ran_during_scan) = tokio::join!(rpc.get_logs(filter(1, 8)), async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        adapter.reads.load(Ordering::SeqCst) < 8
    });
    assert!(result.is_ok());
    assert!(
        timer_ran_during_scan,
        "historical RPC blocked the runtime timer until all reads completed"
    );
}

#[tokio::test]
async fn oversized_log_query_returns_error_instead_of_partial_success() {
    let adapter = MockAdapter::new(100, 0);
    adapter.logs_per_block.store(10_001, Ordering::SeqCst);
    let rpc = Web3RpcImpl::new(adapter, 50_000_000, 25_000);
    assert!(rpc.get_logs(filter(1, 1)).await.is_err());
}

use super::{filter_module, history::HistoryExecutor};
use crate::jsonrpc::{
    web3_types::{FilterChanges, RawLoggerFilter},
    Web3FilterServer,
};

fn executor(limit: usize, timeout_ms: u64) -> HistoryExecutor {
    HistoryExecutor::new(limit, Duration::from_millis(timeout_ms), 16 * 1024 * 1024).unwrap()
}

async fn reads_started(adapter: &MockAdapter, count: usize) {
    tokio::time::timeout(Duration::from_secs(2), async {
        while adapter.reads.load(Ordering::SeqCst) < count {
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("storage read did not start");
}

async fn capacity_recovers(history: &HistoryExecutor) {
    tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            if let Ok(job) = history.start(|budget| async move { budget.check() }) {
                if job.wait().await.is_ok() {
                    break;
                }
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("historical RPC permit was not returned");
}

fn raw_filter() -> RawLoggerFilter {
    serde_json::from_value(serde_json::json!({})).unwrap()
}

#[test]
fn history_configuration_defaults_and_validation() {
    let config: common_config_parser::types::ConfigWeb3 =
        serde_json::from_value(serde_json::json!({})).unwrap();
    assert_eq!(config.history_query_concurrency, 2);
    assert_eq!(config.history_query_timeout_ms, 10_000);
    assert!(HistoryExecutor::new(0, Duration::from_secs(1), 1024).is_err());
    assert!(HistoryExecutor::new(1, Duration::ZERO, 1024).is_err());
    assert!(HistoryExecutor::new(usize::MAX, Duration::from_secs(1), 1024).is_err());
}

#[tokio::test]
async fn timeout_keeps_capacity_until_storage_exits_and_stops_scanning() {
    let adapter = MockAdapter::new(100, 0);
    let release = adapter.pause_next_read();
    let history = executor(1, 100);
    let rpc = Arc::new(
        Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000)
            .with_history_executor(history.clone()),
    );
    let task_rpc = Arc::clone(&rpc);
    let task = tokio::spawn(async move { task_rpc.get_logs(filter(1, 10)).await });
    reads_started(&adapter, 1).await;
    assert_eq!(task.await.unwrap().unwrap_err().code(), -40028);
    assert_eq!(rpc.get_logs(filter(1, 1)).await.unwrap_err().code(), -40027);
    release.send(()).unwrap();
    capacity_recovers(&history).await;
    assert_eq!(adapter.reads.load(Ordering::SeqCst), 1);
    assert!(rpc.get_logs(filter(1, 1)).await.is_ok());
}

#[tokio::test]
async fn dropped_query_keeps_capacity_until_storage_exits() {
    let adapter = MockAdapter::new(100, 0);
    let release = adapter.pause_next_read();
    let history = executor(1, 5_000);
    let rpc = Arc::new(
        Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000)
            .with_history_executor(history.clone()),
    );
    let task_rpc = Arc::clone(&rpc);
    let task = tokio::spawn(async move { task_rpc.get_logs(filter(1, 10)).await });
    reads_started(&adapter, 1).await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(rpc.get_logs(filter(1, 1)).await.unwrap_err().code(), -40027);
    release.send(()).unwrap();
    capacity_recovers(&history).await;
    assert_eq!(adapter.reads.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn missing_block_or_receipt_returns_error_without_partial_logs() {
    let adapter = MockAdapter::new(100, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    adapter.missing.store(2, Ordering::SeqCst);
    let rpc = Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000);
    assert_eq!(rpc.get_logs(filter(1, 2)).await.unwrap_err().code(), -40019);
    adapter.missing.store(u64::MAX, Ordering::SeqCst);
    adapter.missing_receipt.store(true, Ordering::SeqCst);
    assert!(rpc.get_logs(filter(1, 2)).await.is_err());
    adapter.missing_receipt.store(false, Ordering::SeqCst);
    assert_eq!(rpc.get_logs(filter(1, 2)).await.unwrap().len(), 2);
}

#[tokio::test]
async fn byte_budget_and_hash_queries_use_the_same_result_limit() {
    let adapter = MockAdapter::new(100, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    adapter.log_data_bytes.store(1024, Ordering::SeqCst);
    let history = HistoryExecutor::new(1, Duration::from_secs(1), 2048).unwrap();
    let rpc = Web3RpcImpl::new(adapter, 50_000_000, 25_000).with_history_executor(history);
    assert_eq!(rpc.get_logs(filter(1, 1)).await.unwrap_err().code(), -40030);
    let by_hash =
        serde_json::from_value(serde_json::json!({"blockHash": format!("0x{}", "00".repeat(32))}))
            .unwrap();
    assert_eq!(rpc.get_logs(by_hash).await.unwrap_err().code(), -40030);
}

#[tokio::test]
async fn exact_log_limit_is_complete_and_block_ranges_remain_compatible() {
    let adapter = MockAdapter::new(30_000, 0);
    adapter.logs_per_block.store(10_000, Ordering::SeqCst);
    let rpc = Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000)
        .with_history_executor(executor(1, 10_000));
    assert_eq!(rpc.get_logs(filter(1, 1)).await.unwrap().len(), 10_000);
    adapter.logs_per_block.store(0, Ordering::SeqCst);
    assert!(rpc.get_logs(filter(1, 5_000)).await.is_ok());
    assert!(rpc.get_logs(filter(0, 25_000)).await.is_ok());
    assert_eq!(
        rpc.get_logs(filter(0, 25_001)).await.unwrap_err().code(),
        -40016
    );
    assert!(rpc.get_logs(filter(30_000, 30_000)).await.is_ok());
}

#[tokio::test]
async fn failed_filter_scan_preserves_cursor_for_retry() {
    let adapter = MockAdapter::new(1, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    let filters = filter_module(Arc::clone(&adapter), 25_000, executor(1, 5_000));
    let id = filters.new_filter(raw_filter()).await.unwrap();
    adapter.head.store(4, Ordering::SeqCst);
    adapter.missing.store(3, Ordering::SeqCst);
    assert!(filters.get_filter_changes(id).await.is_err());
    adapter.missing.store(u64::MAX, Ordering::SeqCst);
    let FilterChanges::Logs(logs) = filters.get_filter_logs(id).await.unwrap() else {
        panic!("expected logs")
    };
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].block_number.unwrap().low_u64(), 2);
    assert_eq!(
        filters.get_filter_changes(id).await.unwrap(),
        FilterChanges::Logs(vec![])
    );
}

#[tokio::test]
async fn cancelled_filter_poll_preserves_cursor_and_excludes_overlapping_poll() {
    let adapter = MockAdapter::new(1, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    let history = executor(2, 5_000);
    let filters = Arc::new(filter_module(Arc::clone(&adapter), 25_000, history.clone()));
    let id = filters.new_filter(raw_filter()).await.unwrap();
    adapter.head.store(4, Ordering::SeqCst);
    let release = adapter.pause_next_read();
    let worker_filters = Arc::clone(&filters);
    let task = tokio::spawn(async move { worker_filters.get_filter_changes(id).await });
    reads_started(&adapter, 1).await;
    task.abort();
    assert!(task.await.unwrap_err().is_cancelled());
    assert_eq!(
        filters.get_filter_changes(id).await.unwrap_err().code(),
        -40027
    );
    release.send(()).unwrap();
    let result = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            match filters.get_filter_changes(id).await {
                Err(err) if err.code() == -40027 => tokio::task::yield_now().await,
                result => break result,
            }
        }
    })
    .await
    .unwrap()
    .unwrap();
    let FilterChanges::Logs(logs) = result else {
        panic!("expected logs")
    };
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].block_number.unwrap().low_u64(), 2);
}

#[tokio::test]
async fn uninstall_is_responsive_during_scan_and_cannot_be_undone_by_completion() {
    let adapter = MockAdapter::new(1, 0);
    let filters = Arc::new(filter_module(
        Arc::clone(&adapter),
        25_000,
        executor(1, 5_000),
    ));
    let id = filters.block_filter().await.unwrap();
    adapter.head.store(10, Ordering::SeqCst);
    let release = adapter.pause_next_read();
    let worker_filters = Arc::clone(&filters);
    let task = tokio::spawn(async move { worker_filters.get_filter_changes(id).await });
    reads_started(&adapter, 1).await;
    assert!(filters.uninstall_filter(id).await.unwrap());
    release.send(()).unwrap();
    assert!(task.await.unwrap().is_err());
    assert_eq!(
        filters.get_filter_changes(id).await.unwrap_err().code(),
        -40022
    );
    assert_eq!(adapter.reads.load(Ordering::SeqCst), 1);
    let next = filters.block_filter().await.unwrap();
    assert!(filters.uninstall_filter(next).await.unwrap());
}

#[tokio::test]
async fn oversized_filter_and_block_filter_fail_without_deleting_the_filter() {
    let adapter = MockAdapter::new(1, 0);
    let filters = filter_module(Arc::clone(&adapter), 2, executor(1, 5_000));
    let logs_id = filters.new_filter(raw_filter()).await.unwrap();
    let blocks_id = filters.block_filter().await.unwrap();
    adapter.head.store(3, Ordering::SeqCst);
    adapter.logs_per_block.store(10_001, Ordering::SeqCst);
    assert_eq!(
        filters.get_filter_logs(logs_id).await.unwrap_err().code(),
        -40030
    );
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    let FilterChanges::Logs(logs) = filters.get_filter_changes(logs_id).await.unwrap() else {
        panic!("expected logs")
    };
    assert_eq!(logs.len(), 2);
    adapter.head.store(4, Ordering::SeqCst);
    assert_eq!(
        filters
            .get_filter_changes(blocks_id)
            .await
            .unwrap_err()
            .code(),
        -40016
    );
    assert!(filters.uninstall_filter(blocks_id).await.unwrap());
}

#[tokio::test]
async fn http_ws_batch_and_filters_share_admission_while_light_rpc_stays_responsive() {
    use jsonrpsee::{
        core::{client::ClientT, Error},
        http_client::HttpClientBuilder,
        rpc_params,
        server::ServerBuilder,
        ws_client::WsClientBuilder,
    };
    let adapter = MockAdapter::new(1, 0);
    let history = executor(1, 5_000);
    let mut module = Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000)
        .with_history_executor(history.clone())
        .into_rpc();
    module
        .merge(filter_module(Arc::clone(&adapter), 25_000, history).into_rpc())
        .unwrap();
    let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().unwrap();
    let handle = server.start(module);
    let http = Arc::new(
        HttpClientBuilder::default()
            .build(format!("http://{addr}"))
            .unwrap(),
    );
    let ws = WsClientBuilder::default()
        .build(format!("ws://{addr}"))
        .await
        .unwrap();
    let id: protocol::types::U256 = http
        .request("eth_newBlockFilter", rpc_params![])
        .await
        .unwrap();
    adapter.head.store(100, Ordering::SeqCst);
    let release = adapter.pause_next_read();
    let client = Arc::clone(&http);
    let task = tokio::spawn(async move {
        client
            .request::<serde_json::Value, _>("eth_getLogs", rpc_params![
                serde_json::json!({"fromBlock":"0x1","toBlock":"0xa"})
            ])
            .await
    });
    reads_started(&adapter, 1).await;
    for method in ["eth_getLogs", "eth_getFilterChanges", "eth_getFilterLogs"] {
        let params = if method == "eth_getLogs" {
            rpc_params![serde_json::json!({"fromBlock":"0x1","toBlock":"0x1"})]
        } else {
            rpc_params![id]
        };
        let error = ws
            .request::<serde_json::Value, _>(method, params)
            .await
            .unwrap_err();
        assert!(
            matches!(error, Error::Call(ref err) if err.code() == -40027),
            "{error:?}"
        );
    }
    let body = serde_json::json!([
        {"jsonrpc":"2.0","id":1,"method":"eth_getLogs","params":[{"fromBlock":"0x1","toBlock":"0x1"}]},
        {"jsonrpc":"2.0","id":2,"method":"eth_blockNumber","params":[]}
    ]);
    let request = hyper::Request::post(format!("http://{addr}"))
        .header("content-type", "application/json")
        .body(hyper::Body::from(body.to_string()))
        .unwrap();
    let response = hyper::Client::new().request(request).await.unwrap();
    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let batch: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(batch[0]["error"]["code"], -40027);
    assert_eq!(batch[1]["result"], "0x64");
    let head: String = ws.request("eth_blockNumber", rpc_params![]).await.unwrap();
    assert_eq!(head, "0x64");
    release.send(()).unwrap();
    assert!(task.await.unwrap().is_ok());
    handle.stop().unwrap();
    handle.stopped().await;
}

#[tokio::test]
async fn timed_out_filter_can_retry_the_same_cursor_after_worker_exits() {
    let adapter = MockAdapter::new(1, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    let history = executor(1, 100);
    let filters = Arc::new(filter_module(Arc::clone(&adapter), 25_000, history.clone()));
    let id = filters.new_filter(raw_filter()).await.unwrap();
    adapter.head.store(4, Ordering::SeqCst);
    let release = adapter.pause_next_read();
    let worker_filters = Arc::clone(&filters);
    let task = tokio::spawn(async move { worker_filters.get_filter_changes(id).await });
    reads_started(&adapter, 1).await;
    assert_eq!(task.await.unwrap().unwrap_err().code(), -40028);
    assert_eq!(
        filters.get_filter_changes(id).await.unwrap_err().code(),
        -40027
    );
    release.send(()).unwrap();
    capacity_recovers(&history).await;
    let FilterChanges::Logs(logs) = filters.get_filter_changes(id).await.unwrap() else {
        panic!("expected logs")
    };
    assert_eq!(logs.len(), 3);
    assert_eq!(logs[0].block_number.unwrap().low_u64(), 2);
}

#[tokio::test]
async fn worker_panic_returns_error_and_releases_capacity() {
    let history = executor(1, 1_000);
    let failed = history
        .start::<(), _, _>(|_| async { panic!("simulated storage failure") })
        .unwrap()
        .wait()
        .await;
    assert_eq!(failed.unwrap_err().code(), -49999);
    capacity_recovers(&history).await;
}

#[tokio::test]
async fn nested_receipt_decode_tasks_stay_off_the_rpc_runtime() {
    let adapter = MockAdapter::new(100, 0);
    adapter.logs_per_block.store(1, Ordering::SeqCst);
    adapter
        .transactions_per_block
        .store(2_001, Ordering::SeqCst);
    adapter.decode_task_delay_ms.store(50, Ordering::SeqCst);
    let rpc = Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000);
    let (result, timer_ran_during_decode) = tokio::join!(rpc.get_logs(filter(1, 1)), async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        adapter.decode_tasks_done.load(Ordering::SeqCst) < 3
    });
    assert_eq!(result.unwrap().len(), 2_001);
    assert!(
        timer_ran_during_decode,
        "nested receipt decode tasks blocked the RPC runtime"
    );
    assert_eq!(adapter.decode_tasks_done.load(Ordering::SeqCst), 3);
}
