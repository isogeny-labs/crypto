#!/usr/bin/env bash
# NC-1 Stage 0 controls. BSGS-class boundary result, outside the IC ratchet. Zero group operations.
# Run from the repository root. Seeds are derived inside the binary from (r, seed index).
# D3 stage (train/held-out hill climb) is gated on the D0 control and was NOT run: see RESULTS.md.
set -euo pipefail
OUT=research/curve25519_autolab_20261104/nc1/raw
cargo build --release --bin nc1_stage0
BIN=target/release/nc1_stage0
$BIN control --k 20 --seeds 8 > $OUT/control-k20.txt
$BIN control --k 24 --seeds 4 > $OUT/control-k24.txt
$BIN control --k 28 --seeds 2 > $OUT/control-k28.txt
# Gated, not run (D0 control at k=20 is outside [0.85, 0.92]):
# $BIN d3 --k 20 --c 2,3,4,8 --budget 1000 --restarts 4 --seed 1 --seeds 8 > $OUT/d3-k20.txt
# $BIN d3 --k 24 --c 2,3,4,8 --budget 300  --restarts 4 --seed 1 --seeds 4 > $OUT/d3-k24.txt
# $BIN d3 --k 28 --c 2,3,4,8 --budget 50   --restarts 4 --seed 1 --seeds 2 > $OUT/d3-k28.txt
cargo test --release --bin nc1_stage0 2>&1 | tail -12 > $OUT/cargo-test.txt
