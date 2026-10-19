//! Synthetic scheduler benchmark using the real eth_getLogs handler and
//! synchronous mock reads.
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::time::{Duration, Instant};

use protocol::tokio;

use super::{test_adapter::MockAdapter, Web3RpcImpl};
use crate::jsonrpc::{web3_types::Web3Filter, Web3RpcServer};

#[test]
#[ignore = "simulated latency benchmark; run in release mode with --ignored --nocapture"]
fn historical_scan_benchmark() {
    for workload in [
        Workload {
            name:      "block_reads",
            blocks:    1_000,
            read_ms:   1,
            receipts:  0,
            decode_ms: 0,
        },
        Workload {
            name:      "nested_receipt_decode",
            blocks:    1,
            read_ms:   0,
            receipts:  2_001,
            decode_ms: 50,
        },
    ] {
        run_benchmark(workload);
    }
}

struct Workload {
    name:      &'static str,
    blocks:    u64,
    read_ms:   u64,
    receipts:  usize,
    decode_ms: u64,
}

fn run_benchmark(workload: Workload) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let adapter = MockAdapter::new(10_000, workload.read_ms);
        adapter.transactions_per_block.store(workload.receipts, Ordering::SeqCst);
        adapter.logs_per_block.store(usize::from(workload.receipts > 0), Ordering::SeqCst);
        adapter.decode_task_delay_ms.store(workload.decode_ms, Ordering::SeqCst);
        let rpc = Arc::new(Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000));
        let ticks = Arc::new(Mutex::new(Vec::new()));
        let times = Arc::clone(&ticks);
        let heartbeat = tokio::spawn(async move {
            let mut last = Instant::now();
            loop {
                tokio::time::sleep(Duration::from_millis(10)).await;
                let now = Instant::now();
                times.lock().unwrap().push(now.duration_since(last).as_millis());
                last = now;
            }
        });
        tokio::time::sleep(Duration::from_millis(100)).await;
        let started = Instant::now();
        let mut jobs = Vec::new();
        let blocks = workload.blocks;
        let expected_logs = workload.receipts;
        for _ in 0..2 {
            let rpc = Arc::clone(&rpc);
            jobs.push(tokio::spawn(async move {
                let filter: Web3Filter = serde_json::from_value(
                    serde_json::json!({"fromBlock": "0x1", "toBlock": format!("0x{blocks:x}")}),
                ).unwrap();
                rpc.get_logs(filter).await
            }));
        }
        for job in jobs {
            assert_eq!(job.await.unwrap().unwrap().len(), expected_logs);
        }
        let elapsed = started.elapsed();
        tokio::time::sleep(Duration::from_millis(50)).await;
        heartbeat.abort();
        let mut gaps = ticks.lock().unwrap().clone();
        gaps.sort_unstable();
        println!(
            "HISTORY_BENCH workload={} workers=2 queries=2 blocks_per_query={} simulated_read_ms={} receipts_per_block={} simulated_decode_chunk_ms={} reads={} elapsed_ms={} heartbeat_p99_ms={} heartbeat_max_ms={}",
            workload.name,
            workload.blocks,
            workload.read_ms,
            workload.receipts,
            workload.decode_ms,
            adapter.reads.load(Ordering::SeqCst),
            elapsed.as_millis(),
            gaps[gaps.len() * 99 / 100],
            gaps.last().unwrap(),
        );
        assert_eq!(adapter.reads.load(Ordering::SeqCst), (2 * workload.blocks) as usize);
    });
}
