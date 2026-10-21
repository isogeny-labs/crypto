#!/bin/bash
# A counted lower-bound band (AGENTS.md section 5a, w = 0, 0.1, 1 GAE per native operation) for Table A, from
# the counts the rule-v3 price files record.  DERIVED, NOT MEASURED: the price files record how many pair-table
# entries the IC arm stored, how many summands it scanned and how many Frobenius squarings selection made, but not
# how many group additions each phase performed, so the IC side is a LOWER BOUND:
#   IC  G = stored_pairs                         (one addition per stored pair, at least)
#       N = summands_scanned + frobenius_squarings   (table lookups and field squarings, charged at w)
#   rho G = adds + doubles (online) + setup_additions + setup_doublings   (counted exactly, 1 GAE each)
#       N = canonicalisations_uncharged          (charged at w)
# S(w) = (G + w N) / sqrt(r).  A ratio IC/rho above 1 at every w shows the order IC above rho survives the band
# even with the IC side at a lower bound and rho's other uncharged natives (partition hashes) left out; it is not a
# gate artifact, because the IC counts are not exact.  Needs jq.  Run from the repository root.
set -euo pipefail
base=research/ic_tool_program/rule/v3/runs
for d in $base/k*n*; do
  jq -s '
    def med: sort | if length % 2 == 1 then .[length / 2 | floor] else (.[length / 2 - 1] + .[length / 2]) / 2 end;
    (.[0].r | sqrt) as $q
    | (map(.counts.pass.build.stored_pairs) | med) as $pairs
    | (map(.counts.pass.collect.summands_scanned) | med) as $scan
    | (map(.counts.pass.select.selection_cost.frobenius_squarings) | med) as $frob
    | (map(.rho_counts.target_ops.adds + .rho_counts.target_ops.doubles
           + .rho_policy.counters.setup_additions + .rho_policy.counters.setup_doublings) | med) as $rg
    | (map(.rho_policy.counters.canonicalisations_uncharged) | med) as $rn
    | { curve: .[0].curve, n: .[0].n, rows: length,
        ic: {G_lower_bound: $pairs, N: ($scan + $frob)}, rho: {G: $rg, N: $rn},
        band: ([0, 0.1, 1] | map(. as $w | {w: $w,
              S_ic_lower_bound: (($pairs + $w * ($scan + $frob)) / $q),
              S_rho: (($rg + $w * $rn) / $q)} | .ratio_ic_over_rho = (.S_ic_lower_bound / .S_rho))),
        order_holds_across_band: ([0, 0.1, 1] | all(. as $w | (($pairs + $w * ($scan + $frob)) / ($rg + $w * $rn)) > 1)) }
  ' $d/T*-R1.price.json
done | jq -s 'sort_by(.n)'
