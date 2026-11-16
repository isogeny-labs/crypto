#!/usr/bin/env bash
# PROTOCOL.md's statistics from runs/: per row, the median of the per-round
# ratios a/b (build, set-up), a/l (collect, set-up) and a/q (A/A: build,
# collect, set-up), with a 95% percentile bootstrap over rounds (2,000
# resamples, exact LCG step, high-bit index, seed 20261112), the identity
# check and the decision.  Writes summary-pre-restart.json.  (Lever 2's summarise.sh,
# relabelled.)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; runs="$here/runs"
med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { jq -c "$med"' {build: ([.repetitions[].setup_phases_ns.build] | med), collect: ([.repetitions[].setup_phases_ns.collect] | med),
            setup: ([.repetitions[].setup_ns] | med), id: ({c: .certificates, n: .counts} | tostring)}' "$1"; }
contended() { # label row round -> true/false from the isolation record
    jq -rs --arg l "bl/$2/$1-r$3" '[.[] | select(.label == $l)] | last | (.run.contended // .contended // false)' "$runs/isolation.jsonl"; }
out='{}'
for row in k0n53 k0n61; do
  ref=$(per_run "$runs/$row/a-r1.json" | jq -r .id)
  rows='[]'; dropped=0; mismatch=0
  last=37; [ $row = k0n53 ] && last=38; for r in $(seq 1 $last); do
    ok=1; for l in a q b l; do
      f="$runs/$row/$l-r$r.json"
      if [ ! -s "$f" ] || [ "$(contended $l $row $r)" = true ]; then ok=0; fi
    done
    if [ $ok = 0 ]; then dropped=$((dropped + 1)); continue; fi
    a=$(per_run "$runs/$row/a-r$r.json"); q=$(per_run "$runs/$row/q-r$r.json")
    b=$(per_run "$runs/$row/b-r$r.json"); l=$(per_run "$runs/$row/l-r$r.json")
    for x in "$a" "$q" "$b" "$l"; do [ "$(jq -r .id <<<"$x")" = "$ref" ] || mismatch=$((mismatch + 1)); done
    rows=$(jq -c --argjson a "$a" --argjson q "$q" --argjson b "$b" --argjson l "$l" \
      '. + [{bb: ($a.build / $b.build), bs: ($a.setup / $b.setup), lc: ($a.collect / $l.collect), ls: ($a.setup / $l.setup),
             qb: ($a.build / $q.build), qc: ($a.collect / $q.collect), qs: ($a.setup / $q.setup)}]' <<<"$rows")
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
         ab_build_paired: stat("bb"), ab_setup_paired: stat("bs"),
         ab_collect_lookahead: stat("lc"), ab_setup_lookahead: stat("ls"),
         aa_build: stat("qb"), aa_collect: stat("qc"), aa_setup: stat("qs"),
         ratios: $rows}}' <<<"$out")
done
jq 'def est($ab; $aa): ($ab.ci95[0] > 1 and $ab.ci95[0] > $aa.ci95[1]);
    with_entries(.value += {
      paired_build_established: est(.value.ab_build_paired; .value.aa_build),
      paired_setup_established: est(.value.ab_setup_paired; .value.aa_setup),
      lookahead_collect_established: est(.value.ab_collect_lookahead; .value.aa_collect),
      lookahead_setup_established: est(.value.ab_setup_lookahead; .value.aa_setup)})' <<<"$out" > "$here/summary-pre-restart.json"
jq 'map_values(del(.ratios))' "$here/summary-pre-restart.json"
