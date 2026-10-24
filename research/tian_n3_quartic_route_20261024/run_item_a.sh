#!/bin/sh
# Item (a): run every command of PROTOCOL.md section 4 in one run directory.
# Usage: sh run_item_a.sh   (from the repository root; 1 core; about 40 minutes)
set -eu
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
TS=$(date -u +%Y%m%dT%H%MZ)
OUT=research/tian_n3_quartic_route_20261024/results/a-census-run-$TS
mkdir -p "$OUT"
BIN=/tmp/tian_quartic_census_bin
rustc --edition 2021 -O examples/tian_quartic_census.rs -o "$BIN"
CSV=research/weil_restriction_isogeny_census_20261018/results/run-20261018T1605Z/curves.csv
{
  echo "run: $TS (item a, PROTOCOL.md section 4)"
  echo "commit: $(git rev-parse HEAD) (working tree has the uncommitted study files)"
  echo "tool: examples/tian_quartic_census.rs sha256 $(sha256sum examples/tian_quartic_census.rs | cut -d' ' -f1)"
  echo "protocol: research/tian_n3_quartic_route_20261024/PROTOCOL.md sha256 $(sha256sum research/tian_n3_quartic_route_20261024/PROTOCOL.md | cut -d' ' -f1)"
  echo "rustc: $(rustc --version)"
  echo "host: $(uname -srm), $(nproc) cores shared, $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^ //')"
  echo "threads: 1"
  echo "commands (in order):"
  echo "  tqc list OUT $CSV"
  for q in 7 11 13 17 19; do echo "  tqc smooth OUT $q 20000 $((3000+q))"; done
  echo "  tqc search OUT 7 3000000 1007 $CSV ; 11 3000000 1011 ; 13 3000000 1013 ; 17 2500000 1017 ; 19 2500000 1019"
  echo "  tqc search OUT/aa 7 3000000 2007 $CSV ; tqc aa OUT/aa OUT/census_q7.csv OUT/aa/census_q7.csv"
  echo "  tqc summarize OUT 7 11 13 17 19"
  echo "  tqc recount OUT q 4000+q 10   (q = 7 11 13)"
  echo "  tqc control OUT q 1000000 2000+q   (q = 7 11 13)"
} > "$OUT/env.txt"
"$BIN" list "$OUT" "$CSV"
for q in 7 11 13 17 19; do "$BIN" smooth "$OUT" $q 20000 $((3000+q)); done
"$BIN" search "$OUT" 7 3000000 1007 "$CSV"
"$BIN" search "$OUT" 11 3000000 1011 "$CSV"
"$BIN" search "$OUT" 13 3000000 1013 "$CSV"
"$BIN" search "$OUT" 17 2500000 1017 "$CSV"
"$BIN" search "$OUT" 19 2500000 1019 "$CSV"
"$BIN" search "$OUT/aa" 7 3000000 2007 "$CSV"
"$BIN" aa "$OUT/aa" "$OUT/census_q7.csv" "$OUT/aa/census_q7.csv"
"$BIN" summarize "$OUT" 7 11 13 17 19
for q in 7 11 13; do "$BIN" recount "$OUT" $q $((4000+q)) 10; done
for q in 7 11 13; do "$BIN" control "$OUT" $q 1000000 $((2000+q)); done
echo "done $(date -u +%Y%m%dT%H%MZ)" >> "$OUT/env.txt"
