#!/usr/bin/env bash
# PROTOCOL.md's statistics from runs/: per row, the median of per-round
# A/B (a/p) and A/A (a/q) ratios of build, collect and set-up, with a 95%
# percentile bootstrap over rounds (2,000 resamples, exact LCG step, high-bit
# index, seed 20261114), the identity check, the decision and the per-arm
# build steps (scan_probes.build).  Writes summary.json.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; runs="$here/runs"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { jq -c "$med"' {build: ([.repetitions[].setup_phases_ns.build] | med), collect: ([.repetitions[].setup_phases_ns.collect] | med),
            setup: ([.repetitions[].setup_ns] | med), id: ({c: .certificates, n: .counts} | tostring),
            steps: (.scan_probes as $s | $s.build.steps | map_values(. / $s.tsc_hz * 1e9 / $s.build.stored_pairs))}' "$1"; }
contended() { jq -rs --arg l "fp/$2/$1-r$3" '[.[] | select(.label == $l)] | last | (.run.contended // .contended // false)' "$runs/isolation.jsonl"; }
out='{}'
for row in k0n53 k0n61; do
  ref=$(per_run "$runs/$row/a-r1.json" | jq -r .id)
  rows='[]'; steps='[]'; dropped=0; mismatch=0
  for r in $(seq 1 42); do
    ok=1; for l in a q p; do
      f="$runs/$row/$l-r$r.json"
      if [ ! -s "$f" ] || [ "$(contended $l $row $r)" = true ]; then ok=0; fi
    done
    if [ $ok = 0 ]; then dropped=$((dropped + 1)); continue; fi
    a=$(per_run "$runs/$row/a-r$r.json"); q=$(per_run "$runs/$row/q-r$r.json"); t=$(per_run "$runs/$row/p-r$r.json")
    for x in "$a" "$q" "$t"; do [ "$(jq -r .id <<<"$x")" = "$ref" ] || mismatch=$((mismatch + 1)); done
    rows=$(jq -c --argjson a "$a" --argjson q "$q" --argjson t "$t" \
      '. + [{bb: ($a.build / $t.build), qb: ($a.build / $q.build), bc: ($a.collect / $t.collect), qc: ($a.collect / $q.collect),
             bs: ($a.setup / $t.setup), qs: ($a.setup / $q.setup)}]' <<<"$rows")
    steps=$(jq -c --argjson a "$a" --argjson t "$t" '. + [{two: $a.steps, packed: $t.steps}]' <<<"$steps")
  done
  out=$(jq -c --arg row "$row" --argjson rows "$rows" --argjson steps "$steps" --argjson dropped $dropped --argjson mismatch $mismatch "$med"'
    def boot($v; $seed): ($v | length) as $n |
      [foreach range(2000 * $n) as $i ({s: $seed};
         # exact step: 1103515245 = 16838 * 65536 + 20077, every product below 2^53
         .s = (((((.s * 16838) % 32768) * 65536) + (.s * 20077) + 12345) % 2147483648);
         .s) | ($n * . / 2147483648 | floor)] as $idx |
      [range(2000) as $k | [range($n) as $j | $v[$idx[$k * $n + $j]]] | med] | sort |
      {median: ($v | med), ci95: [.[50], .[1949]]};
    def stat($k): boot($rows | map(.[$k]); 20261114);
    def stepmed($arm): ($steps[0][$arm] | keys) as $ks | [$ks[] as $k | {($k): ($steps | map(.[$arm][$k]) | med)}] | add;
    . + {($row): {rounds: ($rows | length), dropped: $dropped, identity_mismatches: $mismatch,
         ab_build: stat("bb"), aa_build: stat("qb"), ab_collect: stat("bc"), aa_collect: stat("qc"),
         ab_setup: stat("bs"), aa_setup: stat("qs"),
         steps_ns_per_pair: {two: stepmed("two"), packed: stepmed("packed")}, ratios: $rows}}' <<<"$out")
done
jq 'def est($ab; $aa): ($ab.ci95[0] > 1 and $ab.ci95[0] > $aa.ci95[1]);
    def reg($ab; $aa): ($ab.ci95[1] < 1 and $ab.ci95[1] < $aa.ci95[0]);
    with_entries(.value += {
      build_established: est(.value.ab_build; .value.aa_build),
      setup_established: est(.value.ab_setup; .value.aa_setup),
      collect_established: est(.value.ab_collect; .value.aa_collect),
      collect_regression: reg(.value.ab_collect; .value.aa_collect)})
    | . + {keep_pack_default: ((map(.build_established) | any) and ((map(.collect_regression) | any) | not))}' <<<"$out" > "$here/summary.json"
jq 'del(.k0n53.ratios, .k0n61.ratios)' "$here/summary.json"
