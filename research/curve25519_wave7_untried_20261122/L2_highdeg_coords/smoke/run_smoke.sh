#!/usr/bin/env bash
# SMOKE, NOT DATA. Smallest case (k = 16), tiny K / target counts, <= 60 s CPU total.
set -euo pipefail
cd "$(dirname "$0")/.."
export OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1
O=smoke
{ time python3 scripts/sumpoly.py --k 16 --funcs x ye xe v q xeye --verify 200 --out $O/sumpoly_k16.json; } 2> $O/sumpoly.log
{ time python3 scripts/census.py --k 16 --t 4 --K 20 --out $O; } 2> $O/census.log
{ time python3 scripts/macaulay.py --sumpoly $O/sumpoly_k16.json --k 16 --funcs x q xeye --T 4 5 --m2T 4 5 --targets 1 --covered 1 --out $O/macaulay_k16.json; } 2> $O/macaulay.log
{ time env OUT=$O KS=16 TS=4 NT=2 TMO=60 bash scripts/engine_calib.sh; } 2> $O/engine_time.log
python3 scripts/analyse.py --data $O --out $O > $O/analyse.stdout 2> $O/analyse.stderr || echo "analyse exit $?" >> $O/analyse.stderr
