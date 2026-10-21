//! Synthetic scheduling measurements using the real eth_getLogs handler.
use std::sync::{atomic::Ordering, Arc, Mutex};
use std::time::{Duration, Instant};

use core_api::jsonrpc::{web3_types::Web3Filter, Web3RpcImpl, Web3RpcServer};
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use protocol::tokio;

// Share the regression-test fixture, including its nested receipt tasks.
#[allow(dead_code)]
#[path = "../src/jsonrpc/impl/test_adapter.rs"]
mod test_adapter;
use test_adapter::MockAdapter;

struct Workload {
    name:      &'static str,
    blocks:    u64,
    read_ms:   u64,
    receipts:  usize,
    decode_ms: u64,
}

struct Observation {
    query_elapsed: Duration,
    heartbeat_max: Duration,
}

fn historical_queries(c: &mut Criterion) {
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
        // Runtime startup is outside both the timing loop and sample
        // collection.
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();
        let mut group = c.benchmark_group(workload.name);
        group.sampling_mode(SamplingMode::Flat);
        group.bench_function("query_elapsed", |b| {
            b.iter_custom(|iterations| {
                (0..iterations)
                    .map(|_| rt.block_on(observe(&workload)).query_elapsed)
                    .sum()
            });
        });
        group.bench_function("heartbeat_max", |b| {
            b.iter_custom(|iterations| {
                (0..iterations)
                    .map(|_| rt.block_on(observe(&workload)).heartbeat_max)
                    .sum()
            });
        });
        group.finish();
    }
}

async fn observe(workload: &Workload) -> Observation {
    let adapter = MockAdapter::new(10_000, workload.read_ms);
    adapter
        .transactions_per_block
        .store(workload.receipts, Ordering::SeqCst);
    adapter
        .logs_per_block
        .store(usize::from(workload.receipts > 0), Ordering::SeqCst);
    adapter
        .decode_task_delay_ms
        .store(workload.decode_ms, Ordering::SeqCst);
    let rpc = Arc::new(Web3RpcImpl::new(Arc::clone(&adapter), 50_000_000, 25_000));
    let ticks = Arc::new(Mutex::new(Vec::new()));
    let times = Arc::clone(&ticks);
    let heartbeat = tokio::spawn(async move {
        let mut last = Instant::now();
        loop {
            tokio::time::sleep(Duration::from_millis(10)).await;
            let now = Instant::now();
            times.lock().unwrap().push(now.duration_since(last));
            last = now;
        }
    });
    // Establish a heartbeat before the queries and allow its delayed tick to be
    // recorded after they finish. Neither grace period is query elapsed time.
    tokio::time::sleep(Duration::from_millis(100)).await;
    // Deliberately occupy both runtime workers at the start. An async
    // rendezvous can reschedule both queries onto the same worker. The caller
    // of Runtime::block_on runs on a separate thread and releases both workers.
    let barrier = Arc::new(std::sync::Barrier::new(3));
    let mut jobs = Vec::new();
    let blocks = workload.blocks;
    for _ in 0..2 {
        let rpc = Arc::clone(&rpc);
        let barrier = Arc::clone(&barrier);
        jobs.push(tokio::spawn(async move {
            let filter: Web3Filter = serde_json::from_value(
                serde_json::json!({"fromBlock": "0x1", "toBlock": format!("0x{blocks:x}")}),
            )
            .unwrap();
            barrier.wait();
            rpc.get_logs(filter).await
        }));
    }
    let started = Instant::now();
    barrier.wait();
    for job in jobs {
        assert_eq!(job.await.unwrap().unwrap().len(), workload.receipts);
    }
    let query_elapsed = started.elapsed();
    tokio::time::sleep(Duration::from_millis(50)).await;
    heartbeat.abort();
    assert!(heartbeat.await.unwrap_err().is_cancelled());
    assert_eq!(
        adapter.reads.load(Ordering::SeqCst),
        (2 * workload.blocks) as usize
    );
    let expected_decode_tasks = if workload.receipts > 1_000 && workload.decode_ms > 0 {
        2 * workload.receipts.div_ceil(1_000)
    } else {
        0
    };
    assert_eq!(
        adapter.decode_tasks_done.load(Ordering::SeqCst),
        expected_decode_tasks
    );
    let heartbeat_max = *ticks
        .lock()
        .unwrap()
        .iter()
        .max()
        .expect("heartbeat did not run");
    Observation {
        query_elapsed,
        heartbeat_max,
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(20)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(30));
    targets = historical_queries
}
criterion_main!(benches);
