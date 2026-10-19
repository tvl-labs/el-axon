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
# Run only the current checkout, or pass a benchmark filter:
cargo bench --locked -p core-api --features bench --bench history_rpc
./devtools/benchmark-history-rpc.sh nested_receipt_decode
```

The standalone `history_rpc` target uses Criterion 0.5.1 with `harness = false`.
The opt-in `bench` feature only exposes the real handler to that harness; it does
not change RPC execution. The benchmark reuses the regression tests' mock adapter.
It is separate from routine unit tests and has no timing-based CI pass/fail gate.

The comparison script exports baseline
`f57c146068a790d459d17a45416036e45be368ae` into a temporary directory and copies
the same adapter, Criterion harness, dependency manifest and lockfile there.
Baseline and current builds use separate target directories to avoid Cargo
path-package cache collisions. The temporary source is removed on exit. No
running node is contacted.

Both builds share a report directory: `target/criterion-history-rpc` by default,
or `CRITERION_HOME` when set. Criterion saves the old implementation with
`--save-baseline before`, then compares the current implementation with
`--baseline before`. This produces estimates, raw samples, comparison statistics
and HTML reports in the same directory. Script arguments are forwarded to both
Criterion runs, for example `--sample-size 30` or `--measurement-time 60`.
The former `RUNS` environment variable is no longer used.

Each workload has two measurements, collected independently with Criterion's
`iter_custom` timing loop:

- `query_elapsed`: time spent releasing two concurrent queries through a barrier
  and waiting until both complete successfully. Runtime/fixture setup and heartbeat grace
  periods are excluded.
- `heartbeat_max`: the largest gap between consecutive heartbeat ticks in one
  workload iteration. Criterion estimates the mean of these per-iteration
  maxima; this is **not** a p99, an overall worst-case bound or query elapsed time.
  Each timing-loop batch returns the sum of its iteration maxima, so Criterion
  can normalize by the iteration count.

A two-worker Tokio runtime is created outside sample collection and reused. Each
iteration creates a fresh handler and adapter. A synchronous start barrier places
the queries on distinct workers before releasing them; its third participant is
the separate thread calling `Runtime::block_on`. This prevents an async barrier
from rescheduling both query tasks onto one worker. A ten-millisecond heartbeat runs
for 100 ms before the queries and 50 ms afterward so a delayed tick is recorded.
Both grace periods are included in the heartbeat observation window. Each
measurement defaults to 20 samples, a two-second warmup and a thirty-second
measurement target with flat sampling. Criterion calibrates iteration counts
using actual wall time, including the grace periods, even for `heartbeat_max`.
Slow workloads can exceed the target to collect all 20 samples.

- `block_reads`: each query scans 1,000 blocks with a one-millisecond synchronous
  sleep per historical block read. The fixture has no receipts or logs.
- `nested_receipt_decode`: each query scans one block with 2,001 receipts/logs.
  The adapter mirrors storage's `> 1,000` receipt branch by spawning three Tokio
  tasks, each with a fifty-millisecond synchronous delay to simulate decoding.

Every iteration checks completed read, log and nested decode task counts. These
workloads measure scheduler responsiveness under simulated blocking work, not
RocksDB throughput, actual RLP decoding speed, large-result serialization, network
performance, or consensus block production. The regression tests separately
exercise HTTP/WS/batch requests, log limits, missing data, deadlines, cancellation,
permit retention, nested receipt tasks and filter cursor/removal/expiry behavior.

### Recorded results

Measured on 2026-10-19 with Rust 1.78.0, Criterion 0.5.1 and macOS 27.0.1
arm64, using the default configuration above. Every measurement contains 20
samples: one iteration per sample for `block_reads`, five per sample for
`nested_receipt_decode`. All read/log/decode assertions passed in both versions.

| Workload | Measurement | Before, mean [95% CI] (ms) | After, mean [95% CI] (ms) | Mean change |
| --- | --- | --- | --- | --- |
| `block_reads` | `query_elapsed` | 1473.02 [1470.45, 1475.60] | 1472.01 [1468.91, 1474.96] | -0.07% |
| `block_reads` | `heartbeat_max` | 1468.21 [1464.65, 1471.84] | 13.62 [13.51, 13.74] | -99.07% |
| `nested_receipt_decode` | `query_elapsed` | 170.79 [169.82, 171.72] | 171.55 [170.89, 172.28] | +0.45% |
| `nested_receipt_decode` | `heartbeat_max` | 165.02 [162.72, 167.13] | 13.25 [13.21, 13.28] | -91.97% |

Criterion detected no significant change in query elapsed time (`p = 0.64` for
block reads and `p = 0.23` for nested receipt decoding). Mean per-iteration maximum
heartbeat gaps decreased by 99.07% and 91.97%, respectively (`p < 0.05`). These
estimates describe the synthetic workloads above; sleep precision and machine
load affect timings.

Raw `sample.json` and `estimates.json` files are under
`target/criterion-history-rpc/<workload>/<measurement>/{before,new}/`, with change
estimates in `change/` and HTML reports in `report/index.html`. They are generated
build artifacts; rerun the script to produce them locally.
