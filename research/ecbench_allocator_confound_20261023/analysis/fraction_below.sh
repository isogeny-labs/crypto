#!/bin/sh
# For H2: the share of B's verified measured values lying below A's median.
# Usage: fraction_below.sh METRIC(solve|online) ARM A=dir[,dir] B=dir[,dir]
set -eu
metric=$1; arm=$2; a=${3#*=}; b=${4#*=}
vals() { for d in $(echo "$1" | tr ',' ' '); do
  jq -c --arg arm "$arm" --arg m "$metric" 'select(.warmup == false and .arm == $arm and .outcome.status == "verified")
    | if $m == "online" then .online.wall_ns else .time.solve_wall_ns end' "$d/records.jsonl"; done; }
va=$(vals "$a" | jq -s 'sort'); vb=$(vals "$b" | jq -s 'sort')
jq -n --argjson a "$va" --argjson b "$vb" --arg arm "$arm" --arg m "$metric" '
  ($a | length) as $n | (if $n % 2 == 1 then $a[($n - 1) / 2] else ($a[$n / 2 - 1] + $a[$n / 2]) / 2 end) as $med
  | {arm: $arm, metric: $m, n_a: $n, n_b: ($b | length), median_a: $med,
     b_below_median_a: ($b | map(select(. < $med)) | length)}
  | . + {fraction: (.b_below_median_a / .n_b)}'
