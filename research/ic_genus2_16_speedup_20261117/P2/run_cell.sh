#!/bin/bash
# One P1 cell: genus_ladder_g16 (fffa1b1243 + P1/instrument.patch), one curve.
# usage: run_cell.sh OUTDIR G P ARMS CURVES SEEDS MAXTRIALS [extra args]
set -u
OUT=$1; G=$2; P=$3; ARMS=$4; CURVES=$5; SEEDS=$6; MT=$7; shift 7
B=${GL_BIN:-/home/user/bin/genus_ladder_g16}
mkdir -p "$OUT"
F="$OUT/g${G}_p${P}_${ARMS//,/}.jsonl"
[ -s "$F" ] && grep -q '"kind":"run"' "$F" && { echo "skip $F"; exit 0; }
S=$(date +%s)
timeout ${CELL_TIMEOUT:-1000} "$B" run --primes "$P" --genera "$G" --curves "$CURVES" --seeds "$SEEDS" \
  --arms "$ARMS" --cc 2.0 --qc 3.0 --max-h 8 --max-trials "$MT" --seed-search 120 --threads 1 \
  --out "$F.tmp" "$@" > "$F.log" 2>&1
RC=$?
mv "$F.tmp" "$F" 2>/dev/null
echo "$(date -u +%FT%TZ) g=$G p=$P arms=$ARMS rc=$RC wall=$(( $(date +%s)-S ))s sha256=$(sha256sum "$B" | cut -c1-16)" >> "$OUT/CELLS.log"
