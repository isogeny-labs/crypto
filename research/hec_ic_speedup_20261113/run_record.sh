#!/usr/bin/env bash
# Recorded rerun of slices S1 and S2 (PLAN.md §6, §7) in one process tree:
# the BigUint engine (round eight, frozen) and the native engine with both
# solvers, on the eleven round-eight rows, five seeds each.
#
#   research/hec_ic_speedup_20261113/run_record.sh OUT_DIR
#
# BIG and NAT override the binaries.  BIG is
# `examples/hyperelliptic_ic_large_prime` built from main 8839290e5f; NAT is
# `crates/hec-native-ic` `examples/hec_native_bench` built from this branch.
# Both are built with stable rustc 1.99.1 (see S1-RESULTS.md).
# The summary is `summarise.sh OUT_DIR`.
set -euo pipefail
out=${1:?usage: run_record.sh OUT_DIR}
BIG=${BIG:-/Users/adamburan/pq-work/hec-target/release/examples/hyperelliptic_ic_large_prime}
NAT=${NAT:-/Users/adamburan/pq-work/hec-native-target2/release/examples/hec_native_bench}
mkdir -p "$out"
shasum -a 256 "$BIG" "$NAT" > "$out/binaries.sha256"
uname -a > "$out/host.txt"
"$BIG" > "$out/biguint.txt"
HEC_SOLVER=both "$NAT" > "$out/native_both.txt"
test "$(grep -c '^GATE' "$out/native_both.txt")" -eq 55
test "$(grep -c '"correct":true' "$out/native_both.txt")" -eq 110
echo "ok: 55 gates identical, 110/110 native solves verified" >&2
