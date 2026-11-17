#!/bin/bash
# Runs the frozen cells of PROTOCOL.md sequentially (one process at a time).
# Each process is held to 3.5 GB of address space (ulimit -v); a cell that
# exceeds it, or the 900 s hard timeout, is recorded as {"failed":true}.
# usage: run_cells.sh [append]   (default truncates analysis/f4cost_m3.jsonl;
# `append` skips the three isogeny chain cells t = 3,4,5 already recorded)
BIN=${BIN:-/home/user/agent-target/release/curve25519_family}
OUT=$(dirname "$0")/analysis/f4cost_m3.jsonl
[ "$1" = append ] || : > "$OUT"
ulimit -v 3500000
run() { # form t G mode
  seed=$((2026111300 + $2))
  echo "# $*" >&2
  line=$(timeout 900 "$BIN" f4cost 20 "$1" "$2" "$3" 100 "$seed" 600 3 "$4" 2>/dev/null)
  if [ -n "$line" ]; then echo "$line" >> "$OUT"; else
    echo "{\"form\":\"$1\",\"t\":$2,\"guess\":$3,\"m\":3,\"mode\":\"$4\",\"failed\":true}" >> "$OUT"; fi
}
if [ "$1" != append ]; then for t in 3 4 5; do run isogeny $t 0 chain; done; fi
run isogeny 6 0 chain
for t in 4 5 6; do for g in 0 3; do run bits $t $g chain; done; done
for t in 3 4 5 6; do run isogeny $t 0 full; done
for t in 4 5 6; do for g in 0 3; do run bits $t $g full; done; done
