#!/usr/bin/env bash
# Validator replay of L1 and L3 cells (independent of run_cells.py): rebuilds each command from the frozen
# protocols (seed = 20261108 + 1000*m + D, 6 targets, cap 1800 s, ulimit -v 6000000, RAYON_NUM_THREADS=1),
# runs it with the new binary and writes the last stdout line (the engine's JSON) to <outdir>/<tag>.json.
# Usage: l3_l1_replay.sh <outdir>
set -u
OUT=${1:?outdir}; mkdir -p "$OUT"
BIN=/home/user/ctarget-f1/release/curve25519_family
WANT=5abc70ba47b1192df71a5e93d697f095e59a0e5f0132cf47db1d0d34d34e3746
GOT=$(sha256sum "$BIN" | cut -d' ' -f1)
[ "$GOT" = "$WANT" ] || { echo "binary hash mismatch: $GOT"; exit 1; }
echo "binary $BIN $GOT" > "$OUT/replay.log"
run() {  # mini kind m D
  local mini=$1 kind=$2 m=$3 d=$4; local seed=$((20261108 + 1000 * m + d))
  local tag; tag="$(echo "$mini" | tr -d '=' | tr ',' '_')_${kind}_m${m}_sym_d${d}"
  local t0; t0=$(date +%s.%N)
  bash -c "ulimit -v 6000000; RAYON_NUM_THREADS=1 $BIN f4sub $mini $kind $d 0 mont $m sym 6 $seed 1800" \
    > "$OUT/$tag.stdout" 2> "$OUT/$tag.stderr"; local rc=$?
  local t1; t1=$(date +%s.%N)
  tail -n 1 "$OUT/$tag.stdout" > "$OUT/$tag.json"
  echo "$(date -u +%H:%M:%SZ) $tag exit $rc wall $(echo "$t1 - $t0" | bc)" >> "$OUT/replay.log"
}
# L3 chain controls (P5) with the new binary
run 20 sub 3 9; run 20 sub 2 57; run 20 sub 3 21
# L3 flat cells under two minutes
for d in 9 21 27 57 63; do run 20 subflat 2 $d; done
for d in 9 12 14 18 21 27 28; do run 20 subflat 3 $d; done
# L1 faithful member: all m = 2 cells and m = 3 up to D = 15
for d in 3 4 5 6 10 12 15 20 30 60; do run p=16777141,A=7842 sub 2 $d; done
for d in 3 4 5 6 10 12 15; do run p=16777141,A=7842 sub 3 $d; done
echo REPLAY_DONE >> "$OUT/replay.log"
