#!/usr/bin/env bash
# Reduce runs/ to summary.json, as PROTOCOL.md declares.  jq only.
#
# Per process: the median over its repetitions of the collect phase and of
# the whole set-up (total_ns).  Per round: A/B = a / c and A/A = a / q.  A
# round with a contended or failed process is dropped from both.  Median of
# the per-round ratios, with a 95% percentile bootstrap over rounds (2,000
# resamples, ../summarise.sh's LCG, seed 20261029), and the decision rule.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
runs="$here/runs"

rows=$(for f in "$runs"/[aqc]-r*.json; do
    jq -c --arg file "$(basename "$f")" '
      def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
      {label: ($file | capture("^(?<l>[aqc])-r").l), round: ($file | capture("-r(?<r>[0-9]+)").r | tonumber),
       collect_ns: ([.repetitions[].phases_ns.collect] | med), total_ns: ([.repetitions[].total_ns] | med),
       status, all_verified, recovered}' "$f"
done | jq -sc .)

iso=$(jq -sc 'map({label: (.label | ltrimstr("aa2/")), contended: .run.contended, exit_status: .run.exit_status})' "$runs/isolation.jsonl")

jq -n --argjson rows "$rows" --argjson iso "$iso" --argjson refusals "$(cat "$runs/refusals.count")" '
  def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
  def boot($xs): ($xs | length) as $n
    | [ foreach range(0; 2000 * $n) as $i ({s: 20261029};
          .s = ((.s * 1103515245 + 12345) % 2147483648);
          .s % $n) ] as $idx
    | [ range(0; 2000) as $b | [ $idx[$b * $n : ($b + 1) * $n][] | $xs[.] ] | med ] | sort
    | {lo: .[50], hi: .[1949]};
  def summ($xs): {median: ($xs | med), ci95: boot($xs), n: ($xs | length), above_one: ($xs | map(select(. > 1)) | length)};
  # Isolation records by "<label>-r<round>"; the last record of a label wins
  # (a refused launch writes none).
  ($iso | map({key: .label, value: .}) | from_entries) as $by
  | ($rows | group_by(.round)) as $g
  | [ $g[] | select(length == 3)
      | (map({key: .label, value: .}) | from_entries) as $r
      | ($r.a.round) as $n
      | {round: $n, ok: ([("a", "q", "c") as $l | $by["\($l)-r\($n)"] | (.contended == false and .exit_status == 0)] | all),
         ab_collect: ($r.a.collect_ns / $r.c.collect_ns), aa_collect: ($r.a.collect_ns / $r.q.collect_ns),
         ab_total: ($r.a.total_ns / $r.c.total_ns), aa_total: ($r.a.total_ns / $r.q.total_ns)} ] as $rounds
  | ($rounds | map(select(.ok))) as $kept
  | { rounds_complete: ($rounds | length), rounds_kept: ($kept | length),
      rounds_dropped: ($rounds | map(select(.ok | not) | .round)),
      processes: ($rows | length), refused_then_retried: $refusals,
      ab_collect: summ($kept | map(.ab_collect)), aa_collect: summ($kept | map(.aa_collect)),
      ab_total: summ($kept | map(.ab_total)), aa_total: summ($kept | map(.aa_total)),
      identity: ($rows | map({status, all_verified, recovered}) | unique),
      median_ms: ($rows | group_by(.label) | map({key: .[0].label,
                  value: {collect: (map(.collect_ns) | med / 1e6), total: (map(.total_ns) | med / 1e6)}}) | from_entries),
      per_round: $rounds }
  | .identity_ok = (.identity == [{status: "complete", all_verified: true, recovered: ["518509155375"]}])
  | .established_collect = (.ab_collect.ci95.lo > 1 and .ab_collect.ci95.lo > .aa_collect.ci95.hi)
  | .established_total = (.ab_total.ci95.lo > 1 and .ab_total.ci95.lo > .aa_total.ci95.hi)' > "$here/summary.json"
echo "wrote $here/summary.json"
