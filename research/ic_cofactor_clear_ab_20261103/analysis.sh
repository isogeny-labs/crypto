#!/usr/bin/env bash
# Frozen analysis for research/ic_cofactor_clear_ab_20261103 (protocol-v2.json,
# protocol-h1-control.json).  Usage: analysis.sh <session-dir> [out-dir]
# Every command is fixed here before any data exists; it runs on every arm alike.
#
# `ecbench compare --a A --b B` reports B / A = sum S_B / sum S_A over matched
# (workload, round) pairs (src/cryptanalysis/ecbench/compare.rs), workloads as
# bootstrap clusters.  So `--a ic-clear --b ic-raw` is the gain
#   G_w = sum S_w(ic-raw) / sum S_w(ic-clear).
set -euo pipefail
S="${1:?usage: analysis.sh <session-dir> [out-dir]}"
OUT="${2:-$S/analysis}"
E="${ECBENCH:-./target/release/ecbench}"
mkdir -p "$OUT"

for w in 0 0.1 1; do
  # C-main: the decisive comparison, at each unpriced-operation price.
  "$E" compare --dir "$S" --a ic-clear --b ic-raw --weight "$w" --json > "$OUT/main_w$w.json"
  # C-gap: distance remaining to the matched rho (ratio ic / rho-neg).
  "$E" compare --dir "$S" --a rho-neg --b ic-raw   --weight "$w" --json > "$OUT/gap_raw_w$w.json"
  "$E" compare --dir "$S" --a rho-neg --b ic-clear --weight "$w" --json > "$OUT/gap_clear_w$w.json"
done
# C-aa: noise floor (ratio of the A/A arm to its twin).
"$E" compare --dir "$S" --a ic-raw --b ic-raw-aa --json > "$OUT/aa.json"
# C-positive: a known counted effect (rho-plain vs rho-neg).
"$E" compare --dir "$S" --a rho-neg --b rho-plain --json > "$OUT/positive.json"
# Pricing band and the one-unit table.
"$E" band  --dir "$S" --ranks --json "$OUT/band.json" > "$OUT/band.txt"
"$E" table --dir "$S" > "$OUT/table.txt"
# The sealed audit (exact replays).
"$E" verify --dir "$S" --replay-all > "$OUT/verify.txt" 2>&1
