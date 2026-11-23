#!/bin/bash
# Supplementary P1 cell split: one admitted curve seed per invocation.
# usage: run_curve.sh OUTDIR G P CURVE_SEED ARMS SEEDS MAXTRIALS
set -u
OUT=$1; G=$2; P=$3; CS=$4; ARMS=$5; SEEDS=$6; MT=$7
B=${GL_BIN:-/home/user/bin/genus_ladder_g16d}
mkdir -p "$OUT"; F="$OUT/g${G}_p${P}_c${CS}_${ARMS//,/}.jsonl"
[ -s "$F" ] && grep -q '"kind":"run"' "$F" && { echo "skip $F"; exit 0; }
S=$(date +%s)
timeout ${CELL_TIMEOUT:-3600} "$B" run --primes "$P" --genera "$G" --curves 1 --curve-seeds "$P=$CS" --seeds "$SEEDS" \
  --arms "$ARMS" --cc 2.0 --qc 3.0 --max-h 8 --max-trials "$MT" --seed-search 1 --threads 1 --out "$F.tmp" > "$F.log" 2>&1
RC=$?; mv "$F.tmp" "$F" 2>/dev/null
echo "$(date -u +%FT%TZ) g=$G p=$P curve=$CS arms=$ARMS rc=$RC wall=$(( $(date +%s)-S ))s sha256=$(sha256sum "$B" | cut -c1-16)" >> "$OUT/CELLS.log"
