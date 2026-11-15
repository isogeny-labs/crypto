#!/usr/bin/env bash
# PROTOCOL.md's statistics from runs/: per row, the median of per-round
# A/B (a/d) and A/A (a/q) ratios of verify and set-up, with a 95% percentile
# bootstrap over rounds (2,000 resamples, exact LCG step, high-bit index,
# seed 20261118), the identity check and the decision. Writes summary.json.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; runs="$here/runs"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { jq -c "$med"' {v: ([.repetitions[].setup_phases_ns.verify] | med),
            setup: ([.repetitions[].setup_ns] | med), id: ({c: .certificates, n: .counts} | tostring)}' "$1"; }
contended() { jq -rs --arg l "dr/$2/$1-r$3" '[.[] | select(.label == $l)] | last | (.run.contended // .contended // false)' "$runs/isolation.jsonl"; }
out='{}'
for row in k0n53 k0n61; do
  ref=$(per_run "$runs/$row/a-r1.json" | jq -r .id)
  rows='[]'; dropped=0; mismatch=0
  for r in $(seq 1 24); do
    ok=1; for l in a q d; do
      f="$runs/$row/$l-r$r.json"
      if [ ! -s "$f" ] || [ "$(contended $l $row $r)" = true ]; then ok=0; fi
    done
    if [ $ok = 0 ]; then dropped=$((dropped + 1)); continue; fi
    a=$(per_run "$runs/$row/a-r$r.json"); q=$(per_run "$runs/$row/q-r$r.json"); t=$(per_run "$runs/$row/d-r$r.json")
    for x in "$a" "$q" "$t"; do [ "$(jq -r .id <<<"$x")" = "$ref" ] || mismatch=$((mismatch + 1)); done
    rows=$(jq -c --argjson a "$a" --argjson q "$q" --argjson t "$t" \
      '. + [{bv: ($a.v / $t.v), qv: ($a.v / $q.v), bs: ($a.setup / $t.setup), qs: ($a.setup / $q.setup),
             v_a_ms: ($a.v / 1e6), v_d_ms: ($t.v / 1e6)}]' <<<"$rows")
  done
  out=$(jq -c --arg row "$row" --argjson rows "$rows" --argjson dropped $dropped --argjson mismatch $mismatch "$med"'
    def boot($v; $seed): ($v | length) as $n |
      [foreach range(2000 * $n) as $i ({s: $seed};
         # exact step: 1103515245 = 16838 * 65536 + 20077, every product below 2^53
         .s = (((((.s * 16838) % 32768) * 65536) + (.s * 20077) + 12345) % 2147483648);
         .s) | ($n * . / 2147483648 | floor)] as $idx |
      [range(2000) as $k | [range($n) as $j | $v[$idx[$k * $n + $j]]] | med] | sort |
      {median: ($v | med), ci95: [.[50], .[1949]]};
    def stat($k): boot($rows | map(.[$k]); 20261118);
    . + {($row): {rounds: ($rows | length), dropped: $dropped, identity_mismatches: $mismatch,
         ab_verify: stat("bv"), aa_verify: stat("qv"), ab_setup: stat("bs"), aa_setup: stat("qs"),
         verify_ms: {dense: ($rows | map(.v_a_ms) | med), direct: ($rows | map(.v_d_ms) | med)}, ratios: $rows}}' <<<"$out")
done
jq 'def est($ab; $aa): ($ab.ci95[0] > 1 and $ab.ci95[0] > $aa.ci95[1]);
    def reg($ab; $aa): ($ab.ci95[1] < 1 and $ab.ci95[1] < $aa.ci95[0]);
    with_entries(.value += {
      verify_established: est(.value.ab_verify; .value.aa_verify),
      setup_established: est(.value.ab_setup; .value.aa_setup),
      setup_regression: reg(.value.ab_setup; .value.aa_setup)})
    | . + {keep_direct_default: ((map(.verify_established) | any) and ((map(.setup_regression) | any) | not))}' <<<"$out" > "$here/summary.json"
jq 'del(.k0n53.ratios, .k0n61.ratios)' "$here/summary.json"
