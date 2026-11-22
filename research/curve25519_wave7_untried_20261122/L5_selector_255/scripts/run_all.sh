#!/usr/bin/env bash
# Full L5 run (PROTOCOL.md "Instrument"). One core, sequential. Estimated ~1.6 h CPU, < 1 GB RAM.
# Usage: scripts/run_all.sh            (writes data/ and analysis/ in this lead directory)
set -euo pipefail
cd "$(dirname "$0")/.."
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 MKL_NUM_THREADS=1
mkdir -p data analysis
py() { python3 -I "$@"; }
py scripts/verify_curves.py > data/verify_curves.json                                    # facts, ~15 s
py scripts/pc_replay.py data/pc > data/pc_replay.stdout                                  # PC1, PC2, ~5 s
py scripts/pc3_pools.py data/pc3_pools.json 80 > data/pc3_pools.stdout                   # PC3 + calibration, ~30 s
py scripts/selector_scan.py --curve k20  --t 4 5 6 7 8 9 10 11 12 --N 4000 --tcov 9  --outdir data/frames > data/frames_k20.jsonl
py scripts/selector_scan.py --curve k24f --t 4 5 6 7 8 9 10 11 12 13 14 --N 4000 --tcov 10 --outdir data/frames > data/frames_k24f.jsonl
py scripts/selector_scan.py --curve k24r --t 4 5 6 7 8 9 10 11 12 13 14 --N 4000 --tcov 10 --outdir data/frames > data/frames_k24r.jsonl
bash scripts/run_iso.sh data/iso 1000 20:4 20:5 20:6 20:7 20:8 20:9 24:4 24:5 24:6 24:7 24:8 24:9 > data/iso.jsonl
py scripts/real25519_sample.py --r1 6:2000 8:1000 10:400 --r1c 12:400 14:200 16:50 \
   --tstar 128 --r2-frames 64 --r2-n 4096 --out data/real25519 > data/real25519.jsonl
py scripts/iso_pc.py data/iso > data/iso_pc.json                                          # iso instrument identity
py scripts/analyse.py --frames data/frames --iso data/iso --real data/real25519.npz --real-log data/real25519.jsonl \
   --pc data/pc/pc_replay.json --pools data/pc3_pools.json --frames-log data/frames_k20.jsonl data/frames_k24f.jsonl data/frames_k24r.jsonl \
   --out analysis --B 500 > analysis/verdicts.json
sha256sum data/*.json* data/frames/*.npz data/iso/*.jsonl data/real25519.npz analysis/* > data/SHA256SUMS
