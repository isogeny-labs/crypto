#!/usr/bin/env bash
# Scoped build of the counted linear stage (complete-accounting lane).
# Assembles the frozen fcda1e22 descent closure (as descent-finder/mini-build),
# the nagao-e2e jv_cover overlay, the lane's counted zero_dim_la.rs and the
# frozen original as zero_dim_la_orig.rs, then builds/tests only that crate.
# Usage: SCRATCH=/some/dir build.sh   (CARGO_TARGET_DIR defaults to the shared
# /home/user/cargo-target-round2)
set -euo pipefail
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
lane=$(cd "$here/../counted-linear-stage" && pwd)
: "${SCRATCH:?set SCRATCH}"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/home/user/cargo-target-round2}
avail=$(df --output=avail -BG / | tail -1 | tr -dc 0-9)
if [ "$avail" -lt 5 ]; then echo "refusing to build: ${avail}G free on /"; exit 3; fi
bash "$here/fetch_frozen.sh"
python3 "$here/assemble.py" "$SCRATCH"
sed "s#LANE#$lane#g" "$here/Cargo.toml.in" > "$SCRATCH/mini3/Cargo.toml"
cd "$SCRATCH/mini3"
cargo test --offline --release --lib zero_dim_la::tests 2>&1 | tail -25
cargo test --offline --release --test conservation -- --nocapture 2>&1 | tail -15
cargo build --offline --release --bin linear_stage_recount 2>&1 | tail -3
sha256sum src/cryptanalysis/*.rs Cargo.toml Cargo.lock "$CARGO_TARGET_DIR/release/linear_stage_recount"
