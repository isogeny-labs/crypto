#!/bin/bash
# Validator determinism replay: one curve per invocation, single-threaded,
# run sequentially (one core).  usage: replay.sh OUTDIR G P CURVE_SEED [G P CS ...]
# Uses the producer's built binary unchanged (no rebuild).  Shows determinism only.
set -u
B=/home/user/bin/genus_ladder_g16d
OUT=$1; shift
mkdir -p "$OUT"
echo "binary sha256 $(sha256sum "$B" | cut -d' ' -f1)" >> "$OUT/REPLAY.log"
while [ $# -ge 3 ]; do
  G=$1; P=$2; S=$3; shift 3
  F="$OUT/g${G}_p${P}_c${S}_cDER.jsonl"
  T0=$(date +%s)
  "$B" run --primes "$P" --genera "$G" --curves 1 --curve-seeds "$P=$S" --seeds 2 \
     --arms c,D,E,R --cc 2.0 --qc 3.0 --max-h 8 --max-trials 5000000 --seed-search 1 \
     --threads 1 --out "$F" > "$F.log" 2>&1
  RC=$?
  echo "$(date -u +%FT%TZ) g=$G p=$P curve=$S rc=$RC wall=$(( $(date +%s)-T0 ))s" >> "$OUT/REPLAY.log"
done
