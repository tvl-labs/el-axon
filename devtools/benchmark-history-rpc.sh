#!/usr/bin/env bash
# Compare identical Criterion workloads against a pinned baseline and this checkout.
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
baseline=f57c146068a790d459d17a45416036e45be368ae
bench_source=$(mktemp -d "${TMPDIR:-/tmp}/axon-history-benchmark.XXXXXX")
trap 'rm -rf "$bench_source"' EXIT

# Only tracked baseline sources are copied. Production data and configuration are untouched.
git -C "$repo_root" archive "$baseline" | tar -x -C "$bench_source"
# Use identical dependency features and locked versions for both benchmark builds.
cp "$repo_root/core/api/Cargo.toml" "$bench_source/core/api/Cargo.toml"
cp "$repo_root/Cargo.lock" "$bench_source/Cargo.lock"
cp "$repo_root/core/api/src/jsonrpc/impl/test_adapter.rs" "$bench_source/core/api/src/jsonrpc/impl/test_adapter.rs"
mkdir -p "$bench_source/core/api/benches"
cp "$repo_root/core/api/benches/history_rpc.rs" "$bench_source/core/api/benches/history_rpc.rs"
cat >> "$bench_source/core/api/src/jsonrpc/mod.rs" <<'MODULES'

#[cfg(feature = "bench")]
#[doc(hidden)]
pub use r#impl::Web3RpcImpl;
MODULES

# Resolve relative paths before entering the temporary baseline workspace.
bench_target=${CARGO_TARGET_DIR:-"$repo_root/target"}
mkdir -p "$bench_target"
bench_target=$(cd "$bench_target" && pwd)
report_root=${CRITERION_HOME:-"$bench_target/criterion-history-rpc"}
mkdir -p "$report_root"
export CRITERION_HOME=$(cd "$report_root" && pwd)
for version in before after; do
    source_root=$repo_root
    export CARGO_TARGET_DIR=$bench_target
    criterion_args=(--baseline before)
    if [[ $version == before ]]; then
        source_root=$bench_source
        # Cargo's path-package fingerprints can collide across copied workspaces.
        # Never share baseline artifacts with the current checkout.
        export CARGO_TARGET_DIR="$bench_target/history-rpc-baseline"
        criterion_args=(--save-baseline before)
    fi
    printf 'HISTORY_BENCH_VERSION version=%s baseline=%s\n' "$version" "$baseline"
    (cd "$source_root" && cargo bench --locked -p core-api --features bench --bench history_rpc -- "${criterion_args[@]}" "$@")
done
printf 'Criterion reports: %s\n' "$CRITERION_HOME"
