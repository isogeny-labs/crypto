#!/usr/bin/env bash
# Scoped build for the descent-walltime lane (no full-workspace build).
# Assembles the frozen fcda1e22 Nagao descent closure as descent-finder's
# mini-build does (nagao-e2e jv_cover overlay, byte-copied shims), plus the
# complete-accounting lane's counted zero_dim_la.rs (SHA-256 pinned) behind
# `--features counted`, and builds descent_walltime twice:
#   OUT/descent_walltime          frozen linear stage (all timing runs)
#   OUT/descent_walltime_counted  counted linear stage (exact counts only)
# Usage: SCRATCH=/dir OUT=/dir build.sh
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
: "${SCRATCH:?set SCRATCH}" "${OUT:?set OUT}"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
bash "$here/fetch_frozen.sh"
python3 "$here/assemble.py" "$SCRATCH"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini4/Cargo.toml"
mkdir -p "$OUT"
cd "$SCRATCH/mini4"
cargo build --offline --release --bin descent_walltime 2>&1 | tail -3
cp "$CARGO_TARGET_DIR/release/descent_walltime" "$OUT/descent_walltime"
cargo build --offline --release --features counted --bin descent_walltime 2>&1 | tail -3
cp "$CARGO_TARGET_DIR/release/descent_walltime" "$OUT/descent_walltime_counted"
sha256sum src/cryptanalysis/*.rs src/cryptanalysis/f4_fp/*.rs src/prime_hyperelliptic/*.rs Cargo.toml Cargo.lock \
  "$lane/descent_walltime.rs" "$OUT/descent_walltime" "$OUT/descent_walltime_counted"
rustc --version; cargo --version
