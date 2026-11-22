#!/usr/bin/env bash
# Phase 2 full run (PROTOCOL.md section 6). One core: BLAS threads pinned to 1.
set -euo pipefail
cd "$(dirname "$0")/.."
export OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 MKL_NUM_THREADS=1
mkdir -p data analysis
S=scripts
/usr/bin/time -v python3 $S/sumpoly.py --out data/sumpoly.json 2> data/sumpoly.log
/usr/bin/time -v python3 $S/census.py --K 2000 --pc1 --out data 2> data/census.log
/usr/bin/time -v python3 $S/macaulay.py --sumpoly data/sumpoly.json --out data/macaulay.json 2> data/macaulay.log
OUT=data bash $S/engine_calib.sh
sha256sum data/*.json data/*.npz > data/SHA256SUMS
python3 $S/analyse.py --data data --out analysis
