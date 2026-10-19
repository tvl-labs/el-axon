#!/usr/bin/env bash
# Compare the same synthetic workload against a baseline and the current checkout.
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
baseline=f57c146068a790d459d17a45416036e45be368ae
bench_runs=${RUNS:-3}
bench_source=$(mktemp -d "${TMPDIR:-/tmp}/axon-history-benchmark.XXXXXX")
trap 'rm -rf "$bench_source"' EXIT

# Only tracked baseline sources are copied. Production data and configuration are untouched.
git -C "$repo_root" archive "$baseline" | tar -x -C "$bench_source"
# Use identical dependency features and locked versions for both benchmark builds.
cp "$repo_root/core/api/Cargo.toml" "$bench_source/core/api/Cargo.toml"
cp "$repo_root/Cargo.lock" "$bench_source/Cargo.lock"
for source in test_adapter.rs history_benchmark.rs; do
    cp "$repo_root/core/api/src/jsonrpc/impl/$source" "$bench_source/core/api/src/jsonrpc/impl/$source"
done
cat >> "$bench_source/core/api/src/jsonrpc/impl/mod.rs" <<'MODULES'

#[cfg(test)]
mod test_adapter;
#[cfg(test)]
mod history_benchmark;
MODULES

bench_target=${CARGO_TARGET_DIR:-"$repo_root/target"}
for version in before after; do
    source_root=$repo_root
    export CARGO_TARGET_DIR=$bench_target
    if [[ $version == before ]]; then
        source_root=$bench_source
        # Cargo's path-package fingerprints can collide across copied workspaces.
        # Never share baseline artifacts with the current checkout.
        export CARGO_TARGET_DIR="$bench_target/history-rpc-baseline"
    fi
    for ((run = 1; run <= bench_runs; run++)); do
        printf 'HISTORY_BENCH_VERSION version=%s run=%s baseline=%s\n' "$version" "$run" "$baseline"
        (cd "$source_root" && cargo test --locked --release -p core-api historical_scan_benchmark -- --ignored --nocapture)
    done
done
