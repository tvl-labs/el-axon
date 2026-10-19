# Historical RPC execution

`eth_getLogs`, log-filter polls and block-filter polls run their storage scans on
Tokio's blocking pool. Each admitted worker owns a current-thread Tokio runtime,
so nested `tokio::spawn` receipt-decoding tasks also stay off the main runtime.
Its internal blocking pool is capped at one thread. One admission budget is shared
by the node's HTTP and WebSocket servers, including requests inside batches. Filter creation also uses
this budget when reading the head. Other RPC methods keep their existing execution
paths.

The `[web3]` configuration supports:

```toml
history_query_concurrency = 2
history_query_timeout_ms = 10000
# Existing block-span setting; unchanged by historical-query isolation.
log_filter_max_block_range = 25000
```

Omitting the new settings uses two concurrent queries and a ten-second deadline.
Zero concurrency or timeout is rejected when starting the RPC server. Requests
that cannot obtain a permit fail immediately; there is no admission wait queue.
The existing log range check remains `to - from <= log_filter_max_block_range`,
so a 5,000-block client batch remains within the limit. Block filters also enforce
a maximum span from their last successfully returned block to the current head.

| JSON-RPC code | Meaning | Client action |
| --- | --- | --- |
| `-40027` | Global capacity exhausted or the same filter already has an active poll | Retry with backoff and jitter |
| `-40028` | Query deadline exceeded | Retry with a smaller range or narrower filter |
| `-40029` | Query cancelled, including filter removal | Stop the cancelled operation; recreate a removed filter if needed |
| `-40030` | Result exceeds 10,000 logs or the response budget | Split the range or narrow the filter |

Result-size checks use a conservative JSON upper bound against
`rpc.max_payload_size`, including envelope allowance, before encoding each log.
They can reject a response smaller than the transport's exact byte limit. Queries
return an error rather than a truncated successful result. Missing blocks or
receipts likewise fail the whole query.

A timeout or dropped handler future signals cancellation, but a synchronous
storage read already in progress cannot be interrupted. Its permit remains held
until the worker actually exits. The scanner checks cancellation and deadlines
between blocks, receipts and log construction. Transport disconnects only cancel
work early if the RPC framework drops the handler future; the deadline still
bounds subsequent scanning otherwise.

Filter state locks cover only snapshot and cursor publication. A lease excludes
concurrent polls of the same filter until both the caller and worker finish.
Failed, timed-out or cancelled handler futures leave the cursor unchanged;
successful polls publish it without an intervening await. Uninstall remains
available during a scan and prevents its completion from restoring the filter.
A weak-reference background task prunes idle filters every twenty seconds, with
the existing forty-second idle lifetime; it does not keep a dropped RPC module
alive. The existing behavior where both `eth_getFilterLogs` and `eth_getFilterChanges` poll the cursor is retained.

This limits historical-query scheduling pressure; it does not reserve CPU or disk
bandwidth for consensus, isolate other expensive RPC methods, or establish the
trigger of a previous production stall.

## Tests and simulated benchmark

```sh
cargo test --locked -p core-api -p common-config-parser
./devtools/benchmark-history-rpc.sh
```

The benchmark script exports baseline `f57c146068a790d459d17a45416036e45be368ae`
into a temporary directory, copies the same mock adapter, benchmark, dependency
manifest and lockfile there,
and runs the baseline and current real `eth_getLogs` handlers three times each
in release mode. Baseline and current builds use separate target directories to
avoid Cargo path-package cache collisions. It removes the temporary source on
exit. `RUNS` controls repetitions. It does not contact a running node.

Each run uses two Tokio runtime workers, two simultaneous successful queries and
a ten-millisecond heartbeat that records scheduling gaps:

- `block_reads`: each query scans 1,000 blocks with a one-millisecond synchronous
  sleep per historical block read. The fixture has no receipts or logs.
- `nested_receipt_decode`: each query scans one block with 2,001 receipts/logs.
  The adapter mirrors storage's `> 1,000` receipt branch by spawning three Tokio
  tasks, each with a fifty-millisecond synchronous delay to simulate decoding.

Every run checks completed read and log counts. These workloads measure scheduler
responsiveness under simulated blocking work, not RocksDB throughput, actual RLP
decoding speed, large-result serialization, network performance, or consensus
block production. The non-benchmark tests separately exercise real HTTP/WS/batch
requests with the mock adapter, log limits, missing data, deadlines, cancellation,
permit retention, nested receipt tasks and filter cursor/removal/expiry behavior.

Measured results are recorded below from local macOS arm64 release builds. Sleep
precision and machine load affect timings; the benchmark is intentionally ignored
in routine unit tests and has no timing-based CI pass/fail threshold.

Measured on 2026-10-19 with Rust 1.78, macOS 27.0.1 arm64:

| Simulated workload | Query elapsed, before / after (ms) | Heartbeat p99, before / after (ms) | Heartbeat max, before / after (ms) |
| --- | --- | --- | --- |
| `block_reads` | 1469 / 1466 | 1467 / 13 | 1467 / 13 |
| `nested_receipt_decode` | 177 / 175 | 169 / 13 | 169 / 13 |

Values are medians of the three per-run measurements for each version.

| Workload | Version | Run | Reads | Query elapsed (ms) | Heartbeat p99 (ms) | Heartbeat max (ms) |
| --- | --- | --- | --- | --- | --- | --- |
| block_reads | before | 1 | 2000 | 1467 | 1467 | 1467 |
| nested_receipt_decode | before | 1 | 2 | 177 | 169 | 169 |
| block_reads | before | 2 | 2000 | 1469 | 1466 | 1466 |
| nested_receipt_decode | before | 2 | 2 | 178 | 180 | 180 |
| block_reads | before | 3 | 2000 | 1490 | 1501 | 1501 |
| nested_receipt_decode | before | 3 | 2 | 167 | 168 | 168 |
| block_reads | after | 1 | 2000 | 1483 | 13 | 13 |
| nested_receipt_decode | after | 1 | 2 | 165 | 13 | 13 |
| block_reads | after | 2 | 2000 | 1461 | 13 | 13 |
| nested_receipt_decode | after | 2 | 2 | 175 | 13 | 13 |
| block_reads | after | 3 | 2000 | 1466 | 13 | 13 |
| nested_receipt_decode | after | 3 | 2 | 177 | 13 | 13 |
