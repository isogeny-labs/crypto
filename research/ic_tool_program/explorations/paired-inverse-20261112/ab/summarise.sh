#!/usr/bin/env bash
# PROTOCOL.md's statistics from runs/: per row and metric, the median of the
# per-round A/B (a/c) and A/A (a/q) ratios with a 95% percentile bootstrap
# over rounds (2,000 resamples, exact LCG step, high-bit index, seed
# 20261112), the identity check and the decision.  Writes summary.json.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; runs="$here/runs"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { jq -c "$med"' {collect: ([.repetitions[].setup_phases_ns.collect] | med), setup: ([.repetitions[].setup_ns] | med),
            id: ({c: .certificates, n: .counts} | tostring)}' "$1"; }
contended() { # label row round -> true/false from the isolation record
    jq -rs --arg l "l2ab/$2/$1-r$3" '[.[] | select(.label == $l)] | last | (.run.contended // .contended // false)' "$runs/isolation.jsonl"; }
out='{}'
for row in k0n53 k0n61; do
  ref=$(per_run "$runs/$row/a-r1.json" | jq -r .id)
  rows='[]'; dropped=0; mismatch=0
  for r in $(seq 1 42); do
    ok=1; for l in a q c; do
      f="$runs/$row/$l-r$r.json"
      if [ ! -s "$f" ] || [ "$(contended $l $row $r)" = true ]; then ok=0; fi
    done
    if [ $ok = 0 ]; then dropped=$((dropped + 1)); continue; fi
    a=$(per_run "$runs/$row/a-r$r.json"); q=$(per_run "$runs/$row/q-r$r.json"); c=$(per_run "$runs/$row/c-r$r.json")
    for x in "$a" "$q" "$c"; do [ "$(jq -r .id <<<"$x")" = "$ref" ] || mismatch=$((mismatch + 1)); done
    rows=$(jq -c --argjson a "$a" --argjson q "$q" --argjson c "$c" \
      '. + [{abc: ($a.collect / $c.collect), aac: ($a.collect / $q.collect), abs: ($a.setup / $c.setup), aas: ($a.setup / $q.setup)}]' <<<"$rows")
  done
  out=$(jq -c --arg row "$row" --argjson rows "$rows" --argjson dropped $dropped --argjson mismatch $mismatch "$med"'
    def boot($v; $seed): ($v | length) as $n |
      [foreach range(2000 * $n) as $i ({s: $seed};
         # exact step: 1103515245 = 16838 * 65536 + 20077, every product below 2^53
         .s = (((((.s * 16838) % 32768) * 65536) + (.s * 20077) + 12345) % 2147483648);
         .s) | ($n * . / 2147483648 | floor)] as $idx |
      [range(2000) as $k | [range($n) as $j | $v[$idx[$k * $n + $j]]] | med] | sort |
      {median: ($v | med), ci95: [.[50], .[1949]]};
    def stat($k): boot($rows | map(.[$k]); 20261112);
    . + {($row): {rounds: ($rows | length), dropped: $dropped, identity_mismatches: $mismatch,
         ab_collect: stat("abc"), aa_collect: stat("aac"), ab_setup: stat("abs"), aa_setup: stat("aas")}}' <<<"$out")
done
jq '. as $o | with_entries(.value += {
      collect_established: (.value.ab_collect.ci95[0] > 1 and .value.ab_collect.ci95[0] > .value.aa_collect.ci95[1]),
      setup_established: (.value.ab_setup.ci95[0] > 1 and .value.ab_setup.ci95[0] > .value.aa_setup.ci95[1])})' <<<"$out" > "$here/summary.json"
cat "$here/summary.json"
