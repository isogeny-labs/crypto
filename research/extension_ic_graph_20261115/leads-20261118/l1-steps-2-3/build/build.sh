#!/usr/bin/env bash
# Scoped build for the l1-steps-2-3 lane (no full-workspace build).
# Usage: SCRATCH=/dir OUT=/dir build.sh [cargo target args, default: all three bins]
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
: "${SCRATCH:?set SCRATCH}" "${OUT:?set OUT}"
export RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-1.98}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}
export CARGO_INCREMENTAL=0
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
bash "$here/fetch.sh"
python3 "$here/assemble.py" "$SCRATCH"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini23/Cargo.toml"
cp "$here/Cargo.lock" "$SCRATCH/mini23/Cargo.lock"
mkdir -p "$OUT"
cd "$SCRATCH/mini23"
cargo build --offline --release -j 1 --bins 2>&1 | grep -E "^(error|warning: unused)|^\s+-->|Finished|error\[" | head -80
for b in run50_deferred universe_count partial_match; do cp "$CARGO_TARGET_DIR/release/$b" "$OUT/$b"; done
sha256sum src/cryptanalysis/*.rs src/cryptanalysis/f4_fp/*.rs src/prime_hyperelliptic/*.rs src/run50/*.rs Cargo.toml Cargo.lock \
  "$lane"/*.rs "$OUT"/run50_deferred "$OUT"/universe_count "$OUT"/partial_match
rustc --version; cargo --version
