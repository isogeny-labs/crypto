#!/bin/sh
# Recomputes the headline medians straight from sessions/*/records.jsonl with jq field extraction,
# sort and awk (a different code path from analysis/summarise.sh, which does the quantile in jq).
# Prints "name<TAB>value". Same author as the study: NOT an independent validation (see REPORT.md).
set -eu
S=research/ecbench_allocator_confound_20261023/sessions
vals() { # arm field sessions...
  arm=$1; field=$2; shift 2
  for s in "$@"; do jq -r --arg arm "$arm" "select(.warmup == false and .arm == \$arm and .outcome.status == \"verified\") | $field" $S/$s/records.jsonl; done | sort -n |
    awk '{v[NR]=$1} END {if (NR%2) print v[(NR+1)/2]; else printf "%.10g\n", (v[NR/2]+v[NR/2+1])/2}'
}
SOLVE=.time.solve_wall_ns; ONLINE=.online.wall_ns
for arm in bsgs rho-neg kangaroo; do
  echo "smoke_${arm}_solve_median_base_ns	$(vals $arm $SOLVE base-smoke base-smoke-rev)"
  echo "smoke_${arm}_solve_median_fix_ns	$(vals $arm $SOLVE fix-smoke fix-smoke-rev)"
done
echo "claim_ic_online_median_base_ns	$(vals ic $ONLINE base-claim base-claim-rev)"
echo "claim_ic_online_median_fix_ns	$(vals ic $ONLINE fix-claim fix-claim-rev)"
echo "claim_rho-strong_solve_median_base_ns	$(vals rho-strong $SOLVE base-claim base-claim-rev)"
echo "claim_rho-strong_solve_median_fix_ns	$(vals rho-strong $SOLVE fix-claim fix-claim-rev)"
