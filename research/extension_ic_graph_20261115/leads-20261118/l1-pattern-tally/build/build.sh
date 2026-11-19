#!/usr/bin/env bash
# Scoped build for the l1-pattern-tally lane (no full-workspace build).
# Same boundary as descent-walltime/build (copied unchanged: fetch_frozen.sh,
# assemble.py, Cargo.lock): frozen fcda1e22 Nagao descent closure, the
# nagao-e2e jv_cover overlay, byte-copied shims. Frozen linear stage only.
# Usage: SCRATCH=/dir OUT=/dir build.sh
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
: "${SCRATCH:?set SCRATCH}" "${OUT:?set OUT}"
export RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-1.98}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}
export CARGO_INCREMENTAL=0
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
bash "$here/fetch_frozen.sh"
python3 "$here/assemble.py" "$SCRATCH"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini4/Cargo.toml"
cp "$here/Cargo.lock" "$SCRATCH/mini4/Cargo.lock"
mkdir -p "$OUT"
cd "$SCRATCH/mini4"
cargo build --offline --release -j 1 --bin pattern_tally 2>&1 | tail -3
cp "$CARGO_TARGET_DIR/release/pattern_tally" "$OUT/pattern_tally"
sha256sum src/cryptanalysis/*.rs src/cryptanalysis/f4_fp/*.rs src/prime_hyperelliptic/*.rs Cargo.toml Cargo.lock \
  "$lane/pattern_tally.rs" "$OUT/pattern_tally"
rustc --version; cargo --version
