#!/usr/bin/env bash
# Run after source.tar.gz, manifest.json and allocation_sN.json are transferred.
set -uo pipefail

stratum=${1:?stratum 0 or 1 required}
bundle=${2:?absolute source bundle directory required}
allocation=${3:?absolute allocation receipt required}
case "$stratum" in 0|1) ;; *) exit 64 ;; esac
case "$bundle:$allocation" in /*:/*) ;; *) exit 64 ;; esac

base="/workspace/n83-crossfamily-s${stratum}"
repository=/workspace/crypto
mkdir -p "$base" "$repository"
export PYTHONDONTWRITEBYTECODE=1
export CARGO_TARGET_DIR=/workspace/n83-crossfamily-target
export CARGO_BUILD_JOBS=2
export RUST_TEST_THREADS=1

finish() {
  code=$?
  printf 'FINALIZING %s %s\n' "$code" "$(date -u +%FT%TZ)" > "$base/run.status"
  if [ -d "$base/results" ]; then
    tar -C "$base" -czf "$base/raw_results.tar.gz" results
    sha256sum "$base/raw_results.tar.gz" > "$base/raw_results.sha256"
  fi
  printf 'EXIT %s %s\n' "$code" "$(date -u +%FT%TZ)" > "$base/run.status"
}
trap finish EXIT
printf 'START %s\n' "$(date -u +%FT%TZ)" > "$base/run.status"

tar -C "$repository" -xzf "$bundle/source.tar.gz" || exit 65
cd "$repository" || exit 65
python3 -m research.n83_crossfamily_current_20261025.source_bundle \
  --verify "$bundle" --root "$repository" > "$base/source_check.json" || exit 66
python3 -m research.n83_crossfamily_current_20261025.run \
  --preflight --stratum "$stratum" > "$base/preflight.json" || exit 67

if ! command -v rustup >/dev/null; then
  apt-get update > "$base/apt.log" 2>&1 || exit 68
  DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential pkg-config libssl-dev curl >> "$base/apt.log" 2>&1 || exit 68
  curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs \
    | sh -s -- -y --profile minimal > "$base/rustup.log" 2>&1 || exit 68
fi
export PATH=/root/.cargo/bin:$PATH
rustup toolchain install 1.99.1 --profile minimal > "$base/toolchain.log" 2>&1 || exit 69
rustc +1.99.1 --version > "$base/rustc.txt" || exit 69
cargo +1.99.1 test --release --locked --example koblitz_orbit_dlp_fast \
  > "$base/ic_tests.log" 2>&1 || exit 70
cargo +1.99.1 test --release --locked --example n83_rho_optimization_eval \
  > "$base/rho_tests.log" 2>&1 || exit 71
cargo +1.99.1 build --release --locked \
  --example koblitz_orbit_dlp_fast --example n83_rho_optimization_eval \
  > "$base/build.log" 2>&1 || exit 72

cpus=$(python3 - <<'PY'
import os
ids = sorted(os.sched_getaffinity(0))[:4]
if len(ids) != 4:
    raise ValueError('four available CPU IDs required')
print(','.join(map(str, ids)))
PY
) || exit 73
printf '%s\n' "$cpus" > "$base/cpu_affinity_ids.txt"

timeout --signal=TERM --kill-after=30s 18300 \
  taskset -c "$cpus" \
  python3 -m research.n83_crossfamily_current_20261025.runpod_runner \
    --stratum "$stratum" \
    --binary "$CARGO_TARGET_DIR/release/examples/koblitz_orbit_dlp_fast" \
    --rho-binary "$CARGO_TARGET_DIR/release/examples/n83_rho_optimization_eval" \
    --provider-receipt "$allocation" --source-bundle "$bundle" \
    --output "$base/results" > "$base/producer.log" 2>&1
producer_code=$?
printf '%s\n' "$producer_code" > "$base/producer.exit"
if [ -d "$base/results" ]; then
  python3 -m research.n83_crossfamily_current_20261025.verify \
    "$base/results" --stratum "$stratum" \
    > "$base/replay.log" 2>&1
  replay_code=$?
  printf '%s\n' "$replay_code" > "$base/replay.exit"
else
  replay_code=98
fi
if [ "$producer_code" -ne 0 ] || [ "$replay_code" -gt 2 ]; then
  exit 74
fi
