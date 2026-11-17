#!/usr/bin/env bash
# PROTOCOL.md (frozen 012d945af9): five family-rule miniatures, seed 20261104+k.
set -euo pipefail
cd "$(dirname "$0")/.."
run() { python3 scripts/torsion_census.py --k $1 --p $2 --A $3 --n $4 --out data/k$1.json 2> data/k$1.log; }
run 16 65437 1058 65672 &
run 18 262133 78 263096 &
run 20 1048573 926 1049624 &
run 22 4194301 43186 4197848 &
wait
run 24 16777213 318 16778792
sha256sum data/*.json > data/SHA256SUMS
