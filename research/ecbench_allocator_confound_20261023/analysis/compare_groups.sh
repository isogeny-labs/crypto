#!/bin/sh
# Ratios of summarise.sh groups. Usage: compare_groups.sh SUMMARY.json OUT.json A:B ...
# For each pair and arm: median_a / median_b of solve_wall_ns, online_wall_ns, ns_per_gae
# (a = the "before" group, b = the "after" group).  Raw per-record values are re-read
# from the records by fraction_below.sh for the "share of B below A's median" figure.
set -eu
sum=$1; out=$2; shift 2
pairs=$(printf '%s\n' "$@" | jq -R 'split(":")' | jq -s .)
jq --argjson pairs "$pairs" '
  . as $s | $pairs | map(. as [$a, $b] | {key: "\($a)__\($b)", value:
    ($s[$a].arms | keys | map(. as $arm | {key: $arm, value:
      (["solve_wall_ns", "online_wall_ns", "ns_per_gae"] | map(. as $m | {key: $m, value:
        (if $s[$a].arms[$arm][$m] == null or $s[$b].arms[$arm][$m] == null then null else
          {n_a: $s[$a].arms[$arm][$m].n, n_b: $s[$b].arms[$arm][$m].n,
           median_a: $s[$a].arms[$arm][$m].median, median_b: $s[$b].arms[$arm][$m].median,
           ratio_a_over_b: ($s[$a].arms[$arm][$m].median / $s[$b].arms[$arm][$m].median)} end)}) | from_entries)}) | from_entries)}) | from_entries
' "$sum" > "$out"
