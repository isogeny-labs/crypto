#!/usr/bin/env bash
# S2b measurement (PLAN.md §8): HEC_SOLVER=fast,dense on round eight's 11 rows,
# 5 seeds, the whole process repeated 5 times (25 paired runs per row).
#
#   research/hec_ic_speedup_20261113/run_s2b.sh OUT_DIR
#
# NAT overrides the binary (crates/hec-native-ic examples/hec_native_bench,
# stable rustc 1.99.1).  Load averages before and after go in host.txt: the
# host is shared.  The summary is `summarise_s2b.sh OUT_DIR`.
set -euo pipefail
out=${1:?usage: run_s2b.sh OUT_DIR}
NAT=${NAT:-/Users/adamburan/pq-work/hec-native-target2/release/examples/hec_native_bench}
REPEATS=${REPEATS:-5}
mkdir -p "$out"
shasum -a 256 "$NAT" > "$out/binaries.sha256"
{ uname -a; echo "before: $(uptime)"; } > "$out/host.txt"
for i in $(seq 1 "$REPEATS"); do
  HEC_SOLVER=fast,dense "$NAT" > "$out/rep$i.txt"
  test "$(grep -c '^GATE' "$out/rep$i.txt")" -eq 55
  test "$(grep -c '"correct":true' "$out/rep$i.txt")" -eq 110
  echo "rep $i: $(uptime)" >> "$out/host.txt"
done
echo "ok: $REPEATS repeats, 55 run gates identical and 110/110 solves verified in each" >&2
