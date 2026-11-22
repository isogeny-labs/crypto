#!/usr/bin/env bash
# SMOKE, NOT DATA: the full pipeline on tiny settings (<= 60 s CPU), output under smoke/.
set -euo pipefail
cd "$(dirname "$0")/.."
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1
rm -rf smoke; mkdir -p smoke
echo "SMOKE, NOT DATA (tiny N; nothing here is evidence)" > smoke/LABEL.txt
py() { python3 -I "$@"; }
py scripts/verify_curves.py k20 > smoke/verify_curves.json
py scripts/pc_replay.py smoke/pc > smoke/pc_replay.stdout
py scripts/selector_scan.py --curve k20 --t 4 5 6 7 8 9 10 11 12 --N 120 --tcov 9 --cost-sample 10 --outdir smoke/frames > smoke/frames_k20.jsonl
bash scripts/run_iso.sh smoke/iso 40 20:4 20:5 20:6 20:7 > smoke/iso.jsonl
py scripts/iso_pc.py smoke/iso > smoke/iso_pc.json
py scripts/real25519_sample.py --r1 6:40 8:12 --r1c 12:6 --tstar 128 --r2-frames 8 --r2-n 64 --out smoke/real25519 > smoke/real25519.jsonl
mkdir -p smoke/analysis
py scripts/analyse.py --frames smoke/frames --iso smoke/iso --real smoke/real25519.npz --real-log smoke/real25519.jsonl \
   --pc smoke/pc/pc_replay.json --frames-log smoke/frames_k20.jsonl --out smoke/analysis --B 40 --K 60 --Kiso 20 --Kreal 6 \
   --train-max 10 --iso-train-max 6 --smoke > smoke/analysis/verdicts.json
