#!/usr/bin/env bash
# Amendment 5 of PROTOCOL.md: the restricted low-first order on every other
# committed degree-4 refutation draw, plus the satisfiable controls.
# Usage: run_a5.sh OUTDIR   (from the repository root)
set -u
OUT=${1:?outdir}; mkdir -p "$OUT"
BIN=target/release/examples/pe_closure
DR=research/dreg_closure_driver_20261016/draws
export RAYON_NUM_THREADS=4 F4_F2_MAX_ROWS=50000000 F4_F2_MAX_COLS=50000000
log() { echo "$(date -u +%H:%M:%SZ) $*" >> "$OUT/queue-a5.log"; }
one() { # tag file draw
  local out="$OUT/$1-$2.d$3.jsonl"
  [ -s "$out" ] && return
  { time "$BIN" --draws "$DR/$2.jsonl" --draw "$3" --degree 4 --mode partial --batch 64 --order low --restrict > "$out"; } 2> "${out%.jsonl}.log"
  log "$1 $2 d$3 exit $?"
}
for spec in m4-11-5:67 m4-11-5:107 m4-11-5:143 m4-12-5:24 m4-12-5:26 m4-12-5:47 m4-12-5:52 \
            m3-22-6:1 m3-22-6:2 m3-22-6:3 m3-24-6:0 m3-24-6:1 m3-24-6:2 m3-24-6:3; do
  one a5r "${spec%:*}" "${spec#*:}"
done
for k in 0 1; do one a5ctl m4-11-5 "$k"; done
log DONE
