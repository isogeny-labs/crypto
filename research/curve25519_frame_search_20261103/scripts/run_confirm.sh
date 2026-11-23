#!/usr/bin/env bash
# Confirmation census (PROTOCOL.md): family miniatures, seed 20261103+k, K=2000.
set -euo pipefail
cd "$(dirname "$0")/.."
run() { python3 scripts/frame_census.py --p $2 --A $3 --n $4 --t 4 5 6 7 8 --K 2000 --seed $((20261103+$1)) --out confirm/k$1.json 2> confirm/k$1.log; }
run 16 65437 1058 65672 &
run 18 262133 54 261752 &
run 20 1048573 926 1049624 &
run 22 4194301 894 4192376 &
wait
run 24 16777213 318 16778792
sha256sum confirm/*.json > confirm/SHA256SUMS
