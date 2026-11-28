#!/bin/sh
# Build and run A6: the A5 CVP KLPT with e1 rule ell^e1 >= kappa p N^3, kappa in {1,2,4,8,16}.
set -eu
cd "$(dirname "$0")"
CSV=${CSV:-/home/user/math-street/math-st/problems/P3.3-ideal-to-isogeny-norms/data/measure_power_targets_p2203-245000047x18_t6_s33032028_20260713_raw.csv}
cargo build --release --offline
mkdir -p results
sha256sum "$CSV" > results/input.sha256
for k in 1 2 4 8 16; do
  mkdir -p results/k$k
  ./target/release/a6_klpt_kappa "$CSV" results/k$k 5 20261123 $k > results/k$k/stdout.txt 2> results/k$k/stderr.txt
  ./target/release/pair ../a5_klpt_cvp/results/runs.csv results/k$k/runs.csv > results/k$k/paired_vs_a5.csv
done
