#!/bin/bash
# Compare a native re-execution of the Round-5 prime or random-binary ladder (`ic boundary`, frozen
# configuration) with the frozen ledger, per instance, for the recipe the board shows (the minimum mean S
# over the 3 repetitions): total_gae, recovered scalar and verified flag of each repetition, the planted
# scalar of each repetition, and the matched-rho reference S.  Instances match by position within a regime
# (the replay names them by ICV1 slug, the ledger by legacy name).
# usage: compare_B.sh <prime|char2> <replay.json>        Needs jq.  Run from the repository root.
set -euo pipefail
reg="$1"; new="$2"
jq -n --arg reg "$reg" --slurpfile a docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json --slurpfile b "$new" '
  def mean: add / length;
  [ $a[0].ledger.instances | map(select(.regime == $reg)) | to_entries[] | . as $e
    | $b[0].ledger.instances[$e.key] as $y | $e.value as $x
    | ($x.r | tonumber | sqrt) as $q
    | ($x.variants | group_by(.name) | map({name: .[0].name, S: (map(.total_gae) | mean / $q)}) | min_by(.S).name) as $best
    | ([range(0; ($x.variants | length)) | select($x.variants[.].name == $best)]) as $idx
    | { instance: $x.curve.name, slug: $y.curve.name, best_recipe: $best,
        seeds_equal: ($x.seeds == $y.seeds), repetitions: ($idx | length),
        gae_equal: ([$idx[] | $x.variants[.].total_gae == $y.variants[.].total_gae] | all),
        recovered_equal: ([$idx[] | $x.variants[.].recovered == $y.variants[.].recovered] | all),
        planted_equal: ([range(0; ($idx | length)) as $k | $y.variants[$idx[$k]].recovered == $x.targets[$k]] | all),
        verified: ([$idx[] | $y.variants[.].verified] | all),
        reference_S_new: $y.rho_s_mean }
  ]' -c
