#!/bin/sh
# Thin orchestration of the A2 grid (all computation in the Rust binary).
# Grid: alpha = 0.02, 0.04, ... (S3 up to 0.50, S4 up to 0.34), 20 instances per
# cell; a (poly, p, m) row stops after 3 consecutive 0/20 cells.
# Control (2) cells at alpha = 0.05 are run first for every (poly, p, m).
set -eu
cd "$(dirname "$0")"
cargo build --release
B=./target/run-bin
cp ./target/release/a2_semaev_small_roots $B
$B controls
for bits in 32 48 64; do
  for m in 1 2 3 4 5; do
    $B cell S3 $bits $m 5
    $B grid S3 $bits $m 50 3
  done
  $B cell S4 $bits 1 5
  $B grid S4 $bits 1 34 3
done
echo DONE
