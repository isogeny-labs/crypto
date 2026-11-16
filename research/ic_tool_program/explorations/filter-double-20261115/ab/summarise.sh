#!/usr/bin/env bash
# PROTOCOL.md's statistics from runs/: per row, the median of per-round
# A/B (a/d) and A/A (a/q) ratios of the filter and admitted stages (scan-probe
# cycles a summand, over the process), collect, build and set-up (medians over
# repetitions), with a 95% percentile bootstrap over rounds (2,000 resamples,
# exact LCG step, high-bit index, seed 20261121), the identity check and the
# decision. Writes summary.json.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; runs="$here/runs"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { jq -c "$med"' {f: ((.scan_probes.stages.filter + .scan_probes.stages.admitted) / .scan_probes.counts.summands),
            ad: (.scan_probes.stages.admitted / .scan_probes.counts.summands),
            c: ([.repetitions[].setup_phases_ns.collect] | med), b: ([.repetitions[].setup_phases_ns.build] | med),
            setup: ([.repetitions[].setup_ns] | med), id: ({c: .certificates, n: .counts} | tostring)}' "$1"; }
contended() { jq -rs --arg l "fd/$2/$1-r$3" '[.[] | select(.label == $l)] | last | (.run.contended // .contended // false)' "$runs/isolation.jsonl"; }
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
      '. + [{bf: ($a.f / $t.f), qf: ($a.f / $q.f), bad: ($a.ad / $t.ad), qad: ($a.ad / $q.ad),
             bc: ($a.c / $t.c), qc: ($a.c / $q.c), bb: ($a.b / $t.b), qb: ($a.b / $q.b),
             bs: ($a.setup / $t.setup), qs: ($a.setup / $q.setup), f_a: $a.f, f_d: $t.f}]' <<<"$rows")
  done
  out=$(jq -c --arg row "$row" --argjson rows "$rows" --argjson dropped $dropped --argjson mismatch $mismatch "$med"'
    def boot($v; $seed): ($v | length) as $n |
      [foreach range(2000 * $n) as $i ({s: $seed};
         # exact step: 1103515245 = 16838 * 65536 + 20077, every product below 2^53
         .s = (((((.s * 16838) % 32768) * 65536) + (.s * 20077) + 12345) % 2147483648);
         .s) | ($n * . / 2147483648 | floor)] as $idx |
      [range(2000) as $k | [range($n) as $j | $v[$idx[$k * $n + $j]]] | med] | sort |
      {median: ($v | med), ci95: [.[50], .[1949]]};
    def stat($k): boot($rows | map(.[$k]); 20261121);
    . + {($row): {rounds: ($rows | length), dropped: $dropped, identity_mismatches: $mismatch,
         ab_filter: stat("bf"), aa_filter: stat("qf"), ab_admitted: stat("bad"), aa_admitted: stat("qad"),
         ab_collect: stat("bc"), aa_collect: stat("qc"), ab_build: stat("bb"), aa_build: stat("qb"),
         ab_setup: stat("bs"), aa_setup: stat("qs"),
         filter_cycles: {off: ($rows | map(.f_a) | med), on: ($rows | map(.f_d) | med)}, ratios: $rows}}' <<<"$out")
done
jq 'def est($ab; $aa): ($ab.ci95[0] > 1 and $ab.ci95[0] > $aa.ci95[1]);
    def reg($ab; $aa): ($ab.ci95[1] < 1 and $ab.ci95[1] < $aa.ci95[0]);
    with_entries(.value += {
      filter_established: est(.value.ab_filter; .value.aa_filter),
      admitted_established: est(.value.ab_admitted; .value.aa_admitted),
      collect_established: est(.value.ab_collect; .value.aa_collect),
      build_regression: reg(.value.ab_build; .value.aa_build),
      setup_established: est(.value.ab_setup; .value.aa_setup),
      setup_regression: reg(.value.ab_setup; .value.aa_setup)})
    | . + {keep_double_default: (((map(.collect_established) | any))
           and ((map(.setup_regression or .build_regression) | any) | not))}' <<<"$out" > "$here/summary.json"
jq 'del(.k0n53.ratios, .k0n61.ratios)' "$here/summary.json"
