#!/usr/bin/env bash
# Frozen analysis of the C1 end-to-end stage (PROTOCOL.md "Decision rule").
# Usage: analyze.sh STD_SESSION TW_SESSION [ECBENCH]
#   STD_SESSION: the session of specs/c1-e2e-m<m>-std.json
#   TW_SESSION:  the session of specs/c1-e2e-m<m>-tw.json (same workloads and seeds)
# Band weights w = 0, 0.1, 1 GAE per uncharged native operation (AGENTS.md 5a).
set -euo pipefail
STD=${1:?std session}
TW=${2:?tw session}
E=${3:-./target/release/ecbench}
for D in "$STD" "$TW"; do
  "$E" verify --dir "$D" --replay-all --out "$D.audit.json"
done
for w in 0 0.1 1; do
  # C-tw-vs-std (decision): operations, cross-session, paired by (workload, round) seed.
  "$E" compare --dir "$STD" --a ic-std --b ic-tw-a --b-dir "$TW" --weight "$w" --save
  # C-gap-rho (distance remaining), each IC arm against both strong-rho arms.
  for ref in rho-strong-l1 rho-strong-l16; do
    "$E" compare --dir "$STD" --a "$ref" --b ic-std --weight "$w" --save
    "$E" compare --dir "$TW" --a "$ref" --b ic-tw-a --weight "$w" --save
  done
  "$E" compare --dir "$STD" --a rho-strong-l1 --b rho-strong-l16 --weight "$w" --save
  # C-aa-line (noise floor of the line draw).
  "$E" compare --dir "$TW" --a ic-tw-a --b ic-tw-b --weight "$w" --save
done
# Wall A/A per session (descriptive unless L2 and outside the A/A interval).
"$E" compare --dir "$TW" --a ic-tw-a --b ic-tw-a-aa --save
"$E" compare --dir "$STD" --a rho-strong-l1 --b rho-strong-l1-aa --save
"$E" table --dir "$STD"
"$E" table --dir "$TW"
