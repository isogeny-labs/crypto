#!/usr/bin/env bash
# Smoke test (smoke, not data): k = 16, t = 4 5 6, K = 100 frames per arm, all arms; desk checks at k = 16.
set -euo pipefail
cd "$(dirname "$0")"
mkdir -p ../smoke/data ../smoke/analysis
python3 desk_checks.py --k 16 --out ../smoke/desk_k16.json
python3 census.py --k 16 --t 4 5 6 --K 100 --outdir ../smoke/data/k16 2> ../smoke/census_k16.log
python3 analyse.py --data ../smoke/data --out ../smoke/analysis --smoke
