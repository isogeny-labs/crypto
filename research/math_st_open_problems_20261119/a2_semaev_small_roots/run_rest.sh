#!/bin/sh
# Continuation after run_all.sh was stopped during S3 p32 m5 (see REPORT.md,
# "Deviations"): m = 5 uses grid_down (from alpha = 0.20, up to 3 zeros, down to
# 3 consecutive 20/20 cells); everything else as in run_all.sh.
set -eu
cd "$(dirname "$0")"
cargo build --release
B=./target/run-bin
cp ./target/release/a2_semaev_small_roots $B
$B grid_down S3 32 5 20
$B cell S4 32 1 5
$B grid S4 32 1 34 3
for bits in 48 64; do
  for m in 1 2 3 4; do
    $B cell S3 $bits $m 5
    $B grid S3 $bits $m 50 3
  done
  $B cell S4 $bits 1 5
  $B grid S4 $bits 1 34 3
  $B grid_down S3 $bits 5 20
done
echo DONE
