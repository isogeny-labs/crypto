#!/usr/bin/env bash
# The earlier studies' committed sessions, re-banded by the current ecbench
# (exact per-arm sums: charged_sum, native_sum) at this study's weights, into
# analysis/rebands/<study>--<session>.json. The certificate generator reads
# these and this study's own analysis/band-*.json. The sessions themselves
# are not touched.
set -euo pipefail
cd "$(dirname "$0")/.."
E=../../target/release/ecbench
mkdir -p analysis/rebands
for st in ic_planner_weight_rule_20261021 ic_planner_weight_rule_binkob_20261026 \
          ic_prime_residual_lp_20261026 ic_koblitz_descent_routing_20261026; do
  for d in ../$st/sessions/*/; do
    t=$(basename "$d")
    $E band --dir "$d" --weights 0,0.01,0.03,0.1,0.3,1 --json analysis/rebands/$st--$t.json \
      > analysis/rebands/$st--$t.txt
  done
done
