#!/usr/bin/env bash
# Frozen panel launcher for PROTOCOL.md section 5. NOT to be run on the shared dev box:
# it is the command for the isolab-eks / executor route. Deterministic: counted unit,
# seeds fixed by (MASTER_SEED, bits, idx, solver). Usage:
#   ./run_panel.sh BIN OUTDIR [JOBS]
# BIN = path to the release binary built from minors_gd/ (record its sha256).
set -euo pipefail
BIN=${1:?binary}; OUT=${2:?outdir}; JOBS=${3:-4}
mkdir -p "$OUT"
ARMS=rho,rho_aa,mgd.all2,mgd.gesc,mgd.apm2,gd,gd_half
CAPS=20:1e8,24:5e8,28:2e9,32:3e9,36:2e9,40:2e9
IDX0=10000          # panel instance indices 10000..10199 (disjoint from every development index)
BLOCK=25            # 8 blocks of 25 instances per size
{
  for b in 20 24 28 32 36 40; do
    for k in 0 1 2 3 4 5 6 7; do
      lo=$((IDX0 + k*BLOCK)); hi=$((lo + BLOCK))
      echo "$BIN panel --solvers $ARMS --bits $b --idx-from $lo --idx-to $hi --caps $CAPS --threads 1 --out $OUT/shard_b${b}_k${k}.jsonl"
    done
  done
} | xargs -P "$JOBS" -I{} bash -c '{}'
cat "$OUT"/shard_*.jsonl > "$OUT/records.jsonl"
sha256sum "$BIN" "$OUT/records.jsonl" > "$OUT/SHA256SUMS"
python3 -I "$(dirname "$0")/minors_gd_analyze.py" "$OUT/records.jsonl" --label panel --boot 5000 > "$OUT/analysis.json"
