#!/bin/sh
# Thin orchestration: build, test, fidelity gate, then the frozen timed modes.
# Usage: run_all.sh OUTPUT_DIR   (run from the study directory)
set -eu
out=$1
mkdir -p "$out"
M=../norm_one_torus_rust_mitm_20261126/public-precommit
cargo test --release --manifest-path harness/Cargo.toml
cargo build --release --manifest-path harness/Cargo.toml
B=harness/target/release/norm-one-torus-ablation
$B --manifest $M/manifest.json fidelity --results-csv $M/results.csv --certificates $M/certificates.json > "$out/fidelity.txt"
taskset -c 1 $B --manifest $M/manifest.json invbench > "$out/invbench.csv"
taskset -c 1 $B --manifest $M/manifest.json ablate --rounds 41 > "$out/ablate.csv"
taskset -c 1 $B --manifest $M/manifest.json xonly --rounds 41 --svg "$out/headline.svg" > "$out/xonly.csv"
$B --manifest $M/manifest.json recall > "$out/recall.txt"
