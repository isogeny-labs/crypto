#!/usr/bin/env bash
# Validator's own agreement sample (task check 3), on GLS curves outside E25's ten
# (p = 101..397, seeds 1, 2) and outside the unit test's four (E25 p = 101 s1, 149 s2;
# E24 p = 101 s2, 53 s1):
#   (a) e31_stage_a on (229, 3), (353, 3), (113, 4): 1,000 calls each, all three solvers on the
#       same calls, solutions / degree / outcome / decomposition set and M_P2 / charpoly compared;
#   (b) quartic_pipeline_gls end to end on (113, 4) with --solver macaulay, chain, chain-sparsest
#       (RAYON_NUM_THREADS=4, E25's arguments), compared leaf by leaf by agreement_compare.py.
#
#   STAGE_A_BIN=... PIPE_BIN=... benchmarks/20261024-e31-chain-solve/replay/agreement.sh
set -u
here="$(cd "$(dirname "$0")" && pwd)"
out="$here/agreement"
mkdir -p "$out"
: "${STAGE_A_BIN:?}" "${PIPE_BIN:?}"
for ps in "229 3" "353 3" "113 4"; do
  set -- $ps
  f="$out/stage_a_p$1s$2.json"
  [ -s "$f" ] || RAYON_NUM_THREADS=1 "$STAGE_A_BIN" --sizes "$1" --seeds "$2" --targets 1000 \
    --json "$f" 2>>"$out/stage_a_new_curves.log"
done
for solver in macaulay chain chain-sparsest; do
  f="$out/e2e_${solver}_p113s4.json"
  [ -s "$f" ] || RAYON_NUM_THREADS=4 "$PIPE_BIN" --sizes 113 --first-seed 4 --seeds 1 --rho-runs 16 \
    --max-residuals 5000000 --solver "$solver" --json "$f.part" >>"$out/e2e_p113s4.log" 2>&1 && mv "$f.part" "$f"
done
echo "finished $(date -u +%FT%TZ)" >>"$out/finished.log"
