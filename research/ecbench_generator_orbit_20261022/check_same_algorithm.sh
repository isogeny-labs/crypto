#!/usr/bin/env bash
# H1 (PROTOCOL.md §5): for every (workload, round, warm-up) pair, ic-lazy and
# ic-layout must agree on trials, relations, rank, the recovered logarithm and
# the pair table's entries. Prints one line per curve; exits 1 on any
# disagreement or a missing partner.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
bad=0
for d in "$HERE"/sessions/*/; do
  line=$(jq -s -r --arg name "$(basename "$d")" '
    def key: [.workload.workload_id, .round, .warmup];
    def sig: {trials: .phases[2].native.trials, relations: .phases[2].native.relations,
              rank: .phases[3].native.rank, recovered: .outcome.recovered,
              entries: .phases[1].native.pair_table_entries};
    (map(select(.arm|test("^ic-lazy-c"))) | map({k: key, s: sig})) as $a
    | (map(select(.arm|test("^ic-layout-c"))) | map({k: key, s: sig})) as $b
    | [ $a[] as $x | ([$b[] | select(.k == $x.k) | .s] | first) as $y | ($y != null and $x.s == $y) ] as $eq
    | "\($name) lazy=\($a|length) layout=\($b|length) equal=\($eq|map(select(.))|length)"
      + (if ($a|length) == ($b|length) and ($eq|all) and ($a|length) > 0 then " OK" else " FAIL" end)' \
    "$d/records.jsonl")
  echo "$line"
  case "$line" in *" OK") ;; *) bad=1 ;; esac
done
exit $bad
