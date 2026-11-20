#!/usr/bin/env bash
# Scoped build for the l1c1-cost protocol (frozen; PROTOCOL.md section 8).
# Reuses the l1-steps-2-3 fetch (read-only `git show` of fcda1e22 and 11c9a031,
# manifest-checked) and assembly unchanged, overlays the complete-accounting
# counted linear stage behind `--features counted` (overlay_counted.py, SHA-pinned),
# then builds, one job at a time:
#   OUT/l1c1_cost_frozen    frozen linear stage: prepare, audit, wall arms (HOOKS=0), PC1
#   OUT/l1c1_cost_counted   counted linear stage: counted arms (HOOKS=1)
#   OUT/fresh-scaling-gen   the hash-pinned generator, unchanged
#   OUT/l1c1_analyze        the std-only analysis program (overflow checks on)
# Usage: SCRATCH=/dir OUT=/dir build.sh
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/.." && pwd)
steps=$(cd "$lane/../l1-steps-2-3" && pwd)
gen=$(cd "$lane/../fresh-scaling/gen" && pwd)
: "${SCRATCH:?set SCRATCH}" "${OUT:?set OUT}"
export RUSTUP_TOOLCHAIN=${RUSTUP_TOOLCHAIN:-1.98}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$SCRATCH/cargo-target}
export CARGO_INCREMENTAL=0
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
SCRATCH="$SCRATCH" bash "$steps/build/fetch.sh"
python3 -I "$steps/build/assemble.py" "$SCRATCH"
python3 -I "$here/overlay_counted.py" "$SCRATCH"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini23/Cargo.toml"
cp "$steps/build/Cargo.lock" "$SCRATCH/mini23/Cargo.lock"
mkdir -p "$OUT"
(cd "$SCRATCH/mini23" && cargo build --offline --locked --release -j 1 --bins 2>&1 | grep -E "^(error|warning)|^\s+-->|Finished|error\[" | head -80)
cp "$CARGO_TARGET_DIR/release/l1c1_cost" "$OUT/l1c1_cost_frozen"
(cd "$SCRATCH/mini23" && cargo build --offline --locked --release -j 1 --bins --features counted 2>&1 | grep -E "^(error|warning)|^\s+-->|Finished|error\[" | head -80)
cp "$CARGO_TARGET_DIR/release/l1c1_cost" "$OUT/l1c1_cost_counted"
cmp -s "$OUT/l1c1_cost_frozen" "$OUT/l1c1_cost_counted" && { echo "frozen and counted binaries are identical: overlay not applied"; exit 4; }
cargo build --offline --locked --release -j 1 --manifest-path "$gen/Cargo.toml" 2>&1 | grep -E "^(error|warning)|Finished" | head -20
cp "$CARGO_TARGET_DIR/release/fresh-scaling-gen" "$OUT/fresh-scaling-gen"
rustc --edition 2021 -O -C overflow-checks=on -o "$OUT/l1c1_analyze" "$lane/l1c1_analyze.rs"
cd "$SCRATCH/mini23"
sha256sum src/cryptanalysis/*.rs src/cryptanalysis/f4_fp/*.rs src/prime_hyperelliptic/*.rs src/run50/*.rs src/lib.rs Cargo.toml Cargo.lock
sha256sum "$lane"/l1c1_cost.rs "$lane"/l1c1_analyze.rs "$steps"/ffield.rs "$gen"/src/*.rs "$gen"/Cargo.toml "$gen"/Cargo.lock
sha256sum "$OUT"/l1c1_cost_frozen "$OUT"/l1c1_cost_counted "$OUT"/fresh-scaling-gen "$OUT"/l1c1_analyze
rustc --version; cargo --version
