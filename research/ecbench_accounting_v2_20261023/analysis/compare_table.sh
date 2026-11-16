#!/bin/bash
# One row per saved comparison: B/A ratio of total S over matched (workload, round) pairs, its interval, pairs, status.
cd "$(dirname "$0")/compare"
echo -e "comparison\tA\tB\tratio_B_over_A\tci95_lo\tci95_hi\tci_method\tpairs\tstatus\tsame_seeds\tbounded"
for f in *.json; do
  jq -r --arg f "${f%.json}" '[$f, .a.arm, .b.arm, (.ops.ratio_b_over_a|tostring), (.ops.ci95[0]|tostring), (.ops.ci95[1]|tostring), .ops.ci_method, (.ops.pairs|tostring), .ops.status, (.ops.same_seeds|tostring), (.ops.bounded|tostring)]|@tsv' "$f"
done
