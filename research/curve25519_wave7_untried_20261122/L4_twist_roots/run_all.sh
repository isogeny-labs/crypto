#!/usr/bin/env bash
# L4 full run (Phase 2).  NOT executed in Phase 1.  One core, < 1 GB, estimated ~15 min CPU.
# Usage: bash run_all.sh        (run from this directory; outputs to data/)
set -euo pipefail
cd "$(dirname "$0")"
OUT=data
TAG=main
mkdir -p "$OUT"
# 1. controls first (positive, negative, identities, Z predictor)
python3 controls.py --k 16 --n 20000 --n3 1000000 --tag $TAG --out $OUT --predict
python3 controls.py --k 20 --n 20000 --n3 1000000 --tag $TAG --out $OUT --predict
# 2. confirmatory cells
python3 run_l4.py exhaustive --k 16 --tag $TAG --out $OUT                  # all 2.14e9 unordered pairs
python3 run_l4.py sample     --k 20 --n 200000000 --tag $TAG --out $OUT    # 2e8 pairs per stratum
# 3. secondary cells (same predictions, reported separately)
python3 run_l4.py sample     --k 16 --variant given --n 50000000 --tag $TAG --out $OUT
python3 run_l4.py sample     --k 20 --variant given --n 50000000 --tag $TAG --out $OUT
# 4. supplementary: faithful k = 24 member (brief), sampled
python3 controls.py --k 24 --n 5000 --n3 1000000 --tag $TAG --out $OUT --predict
python3 run_l4.py sample     --k 24 --n 50000000 --tag $TAG --out $OUT
# 5. analysis
python3 analyze_l4.py --dir $OUT --tag $TAG --ks 16,20,24
