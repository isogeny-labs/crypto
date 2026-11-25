#!/bin/sh
# Build and run A5 (CVP strong approximation) (all computed results come from the Rust binary).
set -eu
cd "$(dirname "$0")"
CSV=${CSV:-/home/user/math-street/math-st/problems/P3.3-ideal-to-isogeny-norms/data/measure_power_targets_p2203-245000047x18_t6_s33032028_20260713_raw.csv}
cargo build --release --offline
mkdir -p results
sha256sum "$CSV" > results/input.sha256
./target/release/a5_klpt_cvp "$CSV" results 5 20261123 > results/stdout.txt 2> results/stderr.txt
