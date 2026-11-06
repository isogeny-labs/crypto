#!/usr/bin/env bash
# Reduce runs/ to summary.json: Callgrind totals per arm, the perf sampling
# shares, and the native timing statistics PROTOCOL.md declares.  jq only.
#
# Native: per process, the median over its repetitions of the collect phase
# and of the whole set-up (total_ns).  Per round, the paired ratio
# baseline / candidate; the median of the 30, with a 95% percentile
# bootstrap over rounds (2,000 resamples, a fixed-seed LCG, so it re-derives
# exactly).  The A/A's 5 pairs give the same statistic for the noise floor.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
runs="$here/runs"
collected() { grep -o 'Collected : [0-9]*' "$1" | grep -o '[0-9]*$'; }

cg=$(for n in k0n53 k0n61; do
    jq -nc --arg n "$n" --argjson b "$(collected "$runs/callgrind/$n-baseline.valgrind.log")" \
        --argjson c "$(collected "$runs/callgrind/$n-candidate.valgrind.log")" \
        '{params: $n, baseline_ir: $b, candidate_ir: $c, ratio: ($b / $c), saved_ir: ($b - $c)}'
done | jq -sc .)

perf=$(cat "$runs/perf/shares.jsonl" | jq -sc .)

per_run() { # dir -> {phase, arm, round, collect_ns, total_ns}
    for f in "$runs/time/$1"/*-r*.json; do
        jq -c --arg phase "$1" --arg file "$(basename "$f")" '
          def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
          {phase: $phase, arm: ($file | capture("^(?<a>[a-z]+)-r").a), round: ($file | capture("-r(?<r>[0-9]+)").r | tonumber),
           collect_ns: ([.repetitions[].phases_ns.collect] | med), total_ns: ([.repetitions[].total_ns] | med),
           status, all_verified, recovered}' "$f"
    done
}
rows=$( (per_run aa; per_run ab) | jq -sc .)

stats=$(jq -nc --argjson rows "$rows" '
  def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
  # Percentile bootstrap of the median, deterministic LCG (seed 20261023).
  def boot($xs): ($xs | length) as $n
    | [ foreach range(0; 2000 * $n) as $i ({s: 20261023};
          .s = ((.s * 1103515245 + 12345) % 2147483648);
          .s % $n) ] as $idx
    | [ range(0; 2000) as $b | [ $idx[$b * $n : ($b + 1) * $n][] | $xs[.] ] | med ] | sort
    | {lo: .[50], hi: .[1949]};
  def summ($xs): {median: ($xs | med), ci95: boot($xs), n: ($xs | length), above_one: ($xs | map(select(. > 1)) | length)};
  def paired($p; $metric): [ $rows[] | select(.phase == $p) ] | group_by(.round) as $g
    | if $p == "ab" then [ $g[] | (map(select(.arm == "baseline"))[0][$metric]) / (map(select(.arm == "candidate"))[0][$metric]) ]
      else [] end;
  def aa_pairs($metric): [ $rows[] | select(.phase == "aa") ] | sort_by(.round)
    | [ range(0; length / 2) as $k | .[2 * $k][$metric] / .[2 * $k + 1][$metric] ];
  { ab_collect: summ(paired("ab"; "collect_ns")), ab_total: summ(paired("ab"; "total_ns")),
    aa_collect: summ(aa_pairs("collect_ns")), aa_total: summ(aa_pairs("total_ns")),
    identity: ($rows | map({status, all_verified, recovered}) | unique),
    median_ms: ([ $rows[] | select(.phase == "ab") ] | group_by(.arm)
      | map({key: .[0].arm, value: {collect: (map(.collect_ns) | med / 1e6), total: (map(.total_ns) | med / 1e6)}}) | from_entries) }
  | .established_collect = (.ab_collect.ci95.lo > 1 and .ab_collect.ci95.lo > .aa_collect.ci95.hi)
  | .established_total = (.ab_total.ci95.lo > 1 and .ab_total.ci95.lo > .aa_total.ci95.hi)')

iso=$(cat "$runs"/time/*/isolation.jsonl | jq -sc '{runs: length,
    contended: map(select(.run.contended == true)) | length, nonzero_exit: map(select(.run.exit_status != 0)) | length}')
jq -n --argjson cg "$cg" --argjson perf "$perf" --argjson stats "$stats" --argjson iso "$iso" \
    --argjson refusals "$(cat "$runs/time/refusals.count")" \
    '{callgrind: $cg, perf: $perf, native: ($stats + {isolation: ($iso + {refused_then_retried: $refusals})})}' > "$here/summary.json"
echo "wrote $here/summary.json"
