#!/usr/bin/env bash
# Correction of a defect in both sessions' bootstrap (CORRECTION.md).
#
# ../summarise.sh and summarise.sh step the LCG s -> (s*1103515245 + 12345)
# mod 2^31 in jq, whose numbers are doubles: the product passes 2^53 on the
# first step and is rounded, the rounded states are almost always even, and
# with an even number of rounds the resample index s mod n is almost always
# even.  The "bootstrap" then draws only the odd-numbered rounds.  This
# script computes the same statistic with the step done exactly (the
# multiplier split as 16838 * 65536 + 20077 keeps every product below 2^53),
# for both sessions, from the raw records, and writes summary-corrected.json
# with the as-run intervals beside the corrected ones.  It changes no record
# and no earlier summary.
#
# Second correction (review of #2858): with the exact step, s mod 2^31
# alternates parity, so `s mod n` for even n still made every resample draw
# exactly n/2 odd and n/2 even rounds (a parity-stratified bootstrap, not
# independent draws).  The index is now taken from the high bits,
# floor(s * n / 2^31), which carry no such pattern; the stratified intervals
# of the first correction are kept as ci95_stratified.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
first="$here/.."

med='def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;'
per_run() { # file -> {collect_ns, total_ns}
    jq -c "$med"' {collect_ns: ([.repetitions[].phases_ns.collect] | med), total_ns: ([.repetitions[].total_ns] | med)}' "$1"
}

# 2026-10-23: A/B = baseline-rN / candidate-rN over 30 rounds; A/A = the
# baseline pairs (2k-1, 2k), k = 1..5, as ../summarise.sh pairs them.
s1=$(
  ab=$(for r in $(seq 1 30); do
        jq -nc --argjson b "$(per_run "$first/runs/time/ab/baseline-r$r.json")" \
               --argjson c "$(per_run "$first/runs/time/ab/candidate-r$r.json")" \
               '{collect: ($b.collect_ns / $c.collect_ns), total: ($b.total_ns / $c.total_ns)}'
      done | jq -sc .)
  aa=$(for k in $(seq 1 5); do
        jq -nc --argjson x "$(per_run "$first/runs/time/aa/baseline-r$((2 * k - 1)).json")" \
               --argjson y "$(per_run "$first/runs/time/aa/baseline-r$((2 * k)).json")" \
               '{collect: ($x.collect_ns / $y.collect_ns), total: ($x.total_ns / $y.total_ns)}'
      done | jq -sc .)
  jq -nc --argjson ab "$ab" --argjson aa "$aa" --argjson seed 20261023 \
     '{seed: $seed, ab_collect: ($ab | map(.collect)), ab_total: ($ab | map(.total)),
       aa_collect: ($aa | map(.collect)), aa_total: ($aa | map(.total))}')

# 2026-10-29: A/B = a/c and A/A = a/q per round, all 42 rounds kept.
s2=$(
  rows=$(for r in $(seq 1 42); do
        jq -nc --argjson a "$(per_run "$here/runs/a-r$r.json")" --argjson q "$(per_run "$here/runs/q-r$r.json")" \
               --argjson c "$(per_run "$here/runs/c-r$r.json")" \
               '{abc: ($a.collect_ns / $c.collect_ns), aac: ($a.collect_ns / $q.collect_ns),
                 abt: ($a.total_ns / $c.total_ns), aat: ($a.total_ns / $q.total_ns)}'
      done | jq -sc .)
  jq -nc --argjson rows "$rows" --argjson seed 20261029 \
     '{seed: $seed, ab_collect: ($rows | map(.abc)), ab_total: ($rows | map(.abt)),
       aa_collect: ($rows | map(.aac)), aa_total: ($rows | map(.aat))}')

jq -n --argjson s1 "$s1" --argjson s2 "$s2" "$med"'
  # $exact: the integer step; otherwise the as-run double step.
  # $mode: "exact" (exact step, high-bit index), "stratified" (exact step,
  # s mod n: the first correction) or "as_run" (double step, s mod n).
  def step($exact): if $exact
      then ((((. * 16838) % 2147483648) * 65536 % 2147483648 + . * 20077 + 12345) % 2147483648)
      else ((. * 1103515245 + 12345) % 2147483648) end;
  def boot($xs; $seed; $mode): ($xs | length) as $n
    | [ foreach range(0; 2000 * $n) as $i ({s: $seed}; .s |= step($mode != "as_run");
          if $mode == "exact" then (.s * $n / 2147483648 | floor) else .s % $n end) ] as $idx
    | [ range(0; 2000) as $b | [ $idx[$b * $n : ($b + 1) * $n][] | $xs[.] ] | med ] | sort
    | {lo: .[50], hi: .[1949], odd_round_share: (($idx | map(select(. % 2 == 0)) | length) / ($idx | length))};
  def summ($xs; $seed): {median: ($xs | med), n: ($xs | length), above_one: ($xs | map(select(. > 1)) | length),
                          ci95: (boot($xs; $seed; "exact") | {lo, hi}),
                          ci95_stratified: (boot($xs; $seed; "stratified") | {lo, hi}),
                          ci95_as_run: (boot($xs; $seed; "as_run") | {lo, hi}),
                          as_run_odd_round_share: boot($xs; $seed; "as_run").odd_round_share};
  def session($s): {ab_collect: summ($s.ab_collect; $s.seed), aa_collect: summ($s.aa_collect; $s.seed),
                    ab_total: summ($s.ab_total; $s.seed), aa_total: summ($s.aa_total; $s.seed)}
    | .established_collect = (.ab_collect.ci95.lo > 1 and .ab_collect.ci95.lo > .aa_collect.ci95.hi)
    | .established_total = (.ab_total.ci95.lo > 1 and .ab_total.ci95.lo > .aa_total.ci95.hi)
    | .established_collect_as_run = (.ab_collect.ci95_as_run.lo > 1 and .ab_collect.ci95_as_run.lo > .aa_collect.ci95_as_run.hi)
    | .established_total_as_run = (.ab_total.ci95_as_run.lo > 1 and .ab_total.ci95_as_run.lo > .aa_total.ci95_as_run.hi)
    | .established_collect_stratified = (.ab_collect.ci95_stratified.lo > 1 and .ab_collect.ci95_stratified.lo > .aa_collect.ci95_stratified.hi)
    | .established_total_stratified = (.ab_total.ci95_stratified.lo > 1 and .ab_total.ci95_stratified.lo > .aa_total.ci95_stratified.hi)
    | .margin_collect = (.ab_collect.ci95.lo - .aa_collect.ci95.hi)
    | .margin_total = (.ab_total.ci95.lo - .aa_total.ci95.hi);
  {schema: "ic_aimed_gather.bootstrap_correction/v2",
   generator: "LCG s -> (s*1103515245 + 12345) mod 2^31, stepped exactly; index floor(s*n / 2^31); 2000 resamples; percentile indices 50 and 1949",
   stratified_generator: "the same exact LCG with index s mod n (the first correction): every resample draws n/2 odd and n/2 even rounds",
   as_run_generator: "the same LCG stepped in jq doubles (rounded past 2^53), as ../summarise.sh and summarise.sh ran it",
   sessions: {"2026-10-23": session($s1), "2026-10-29": session($s2)}}' > "$here/summary-corrected.json"
echo "wrote $here/summary-corrected.json"
