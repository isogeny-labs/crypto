#!/bin/sh
# Build and run A7 (P1.6/Q018). Every computed result comes from the Rust binary.
# Writes results/{runs.csv,summary.json,controls.csv,controls_summary.json,fixtures.json}.
set -eu
cd "$(dirname "$0")"
N=${N:-100000}
cargo build --release --offline
mkdir -p results
./target/release/a7_xedni_q018 fixtures results
./target/release/a7_xedni_q018 controls results 2> results/controls_stderr.txt
./target/release/a7_xedni_q018 run "$N" results 2> results/run_stderr.txt
sha256sum results/runs.csv > results/runs.csv.sha256
# Optional port check against upstream Python (needs the math-st checkout and sympy):
#   python3 crosscheck_upstream.py /home/user/math-street/math-st
