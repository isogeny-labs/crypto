#!/bin/sh
# Item (b): PROTOCOL.md Amendment 2 parameters. 1 core. Usage: sh run_item_b.sh (repository root)
set -eu
ROOT=$(git rev-parse --show-toplevel)
cd "$ROOT"
TS=$(date -u +%Y%m%dT%H%MZ)
OUT=research/tian_n3_quartic_route_20261024/results/b-ic-vs-rho-run-$TS
mkdir -p "$OUT"
BIN=/tmp/claude-0/work/quartic_ic_bin
rustc --edition 2021 -O examples/quartic_ic.rs -o "$BIN"
{
  echo "run: $TS (item b, PROTOCOL.md section 5 + Amendment 2)"
  echo "commit: $(git rev-parse HEAD) (working tree has the uncommitted study files)"
  echo "tool: examples/quartic_ic.rs sha256 $(sha256sum examples/quartic_ic.rs | cut -d' ' -f1)"
  echo "protocol: research/tian_n3_quartic_route_20261024/PROTOCOL.md sha256 $(sha256sum research/tian_n3_quartic_route_20261024/PROTOCOL.md | cut -d' ' -f1)"
  echo "rustc: $(rustc --version)"
  echo "host: $(uname -srm), $(nproc) cores shared, $(grep -m1 'model name' /proc/cpuinfo | cut -d: -f2 | sed 's/^ //')"
  echo "threads: 1"
  echo "commands: qic run OUT q n q 300 ic,rho[,rhoneg for q<=2039] 1000  for (q,n) = (127,12) (251,12) (509,10) (1021,10) (2039,8) (4093,6) (8191,4); then qic summarize OUT 127 251 509 1021 2039 4093 8191"
} > "$OUT/env.txt"
"$BIN" run "$OUT" 127 12 127 300 ic,rho,rhoneg 1000 2> "$OUT/log_q127.txt"
"$BIN" run "$OUT" 251 12 251 300 ic,rho,rhoneg 1000 2> "$OUT/log_q251.txt"
"$BIN" run "$OUT" 509 10 509 300 ic,rho,rhoneg 1000 2> "$OUT/log_q509.txt"
"$BIN" run "$OUT" 1021 10 1021 300 ic,rho,rhoneg 1000 2> "$OUT/log_q1021.txt"
"$BIN" run "$OUT" 2039 8 2039 300 ic,rho,rhoneg 1000 2> "$OUT/log_q2039.txt"
"$BIN" run "$OUT" 4093 6 4093 300 ic,rho 1000 2> "$OUT/log_q4093.txt"
"$BIN" run "$OUT" 8191 4 8191 300 ic,rho 1000 2> "$OUT/log_q8191.txt"
"$BIN" summarize "$OUT" 127 251 509 1021 2039 4093 8191 > "$OUT/summary.txt"
echo "done $(date -u +%Y%m%dT%H%MZ)" >> "$OUT/env.txt"
