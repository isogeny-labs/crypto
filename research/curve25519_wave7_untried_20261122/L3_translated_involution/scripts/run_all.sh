#!/usr/bin/env bash
# Wave-7 L3 full run (PROTOCOL.md). One core, sequential. Do not run before the protocol is frozen.
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p ../data ../analysis
for k in 16 18 20; do
  python3 desk_checks.py --k $k --out ../data/desk_k$k.json
  python3 census.py --k $k --t 4 5 6 7 8 --K 2000 --outdir ../data/k$k 2> ../data/census_k$k.log
done
python3 model.py > ../analysis/model_table.txt
python3 analyse.py --data ../data --out ../analysis
sha256sum ../data/k*/*.npz ../data/k*/meta.json ../data/desk_k*.json > ../data/SHA256SUMS
