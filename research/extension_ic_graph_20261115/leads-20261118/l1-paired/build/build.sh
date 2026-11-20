#!/usr/bin/env bash
# Scoped build for the l1-paired protocol (frozen; see ../PROTOCOL.md "Build").
# Reuses the l1-steps-2-3 fetch (read-only `git show` of fcda1e22 and 11c9a031,
# manifest-checked) and assembly unchanged, then builds only l1_paired, the
# fresh-scaling generator and the std-only analysis program.
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
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini23/Cargo.toml"
cp "$steps/build/Cargo.lock" "$SCRATCH/mini23/Cargo.lock"
mkdir -p "$OUT"
(cd "$SCRATCH/mini23" && cargo build --offline --locked --release -j 1 --bins 2>&1 | grep -E "^(error|warning)|^\s+-->|Finished|error\[" | head -80)
cp "$CARGO_TARGET_DIR/release/l1_paired" "$OUT/l1_paired"
cargo build --offline --locked --release -j 1 --manifest-path "$gen/Cargo.toml" 2>&1 | grep -E "^(error|warning)|Finished" | head -20
cp "$CARGO_TARGET_DIR/release/fresh-scaling-gen" "$OUT/fresh-scaling-gen"
rustc --edition 2021 -O -C overflow-checks=on -o "$OUT/l1p_analyze" "$lane/l1p_analyze.rs"
cd "$SCRATCH/mini23"
sha256sum src/cryptanalysis/*.rs src/cryptanalysis/f4_fp/*.rs src/prime_hyperelliptic/*.rs src/run50/*.rs src/lib.rs Cargo.toml Cargo.lock
sha256sum "$lane"/l1_paired.rs "$lane"/l1p_analyze.rs "$steps"/ffield.rs "$gen"/src/*.rs "$gen"/Cargo.toml "$gen"/Cargo.lock
sha256sum "$OUT"/l1_paired "$OUT"/fresh-scaling-gen "$OUT"/l1p_analyze
rustc --version; cargo --version
