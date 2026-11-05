#!/usr/bin/env bash
# E1 fix: the protocol names family-rule curves. The rule (curve25519_family family)
# gives k=18 A=78 and k=22 A=43186; the first run used off-rule A=54 / A=894
# (kept in confirm_offrule/). Same frozen instrument, seeds 20261103+k, K=2000.
set -euo pipefail
cd "$(dirname "$0")/.."
run() { python3 scripts/frame_census.py --p $2 --A $3 --n $4 --t 4 5 6 7 8 --K 2000 --seed $((20261103+$1)) --out confirm/k$1.json 2> confirm/k$1.log; }
run 18 262133 78 263096 &
run 22 4194301 43186 4197848 &
wait
sha256sum confirm/*.json > confirm/SHA256SUMS
