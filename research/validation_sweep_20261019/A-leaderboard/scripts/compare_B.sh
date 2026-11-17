#!/bin/sh
# Compare the native re-execution of Round 5 (replay_B.sh output) with the frozen ledger, instance by instance
# (positional match within a regime: the replay names instances by ICV1 slug, the ledger by legacy name).
# usage: compare_B.sh <prime|char2> <replay.json>
reg="$1"; new="$2"
jq -n --arg reg "$reg" --slurpfile a docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json --slurpfile b "$new" '
 [ $a[0].ledger.instances | map(select(.regime==$reg)) | to_entries[] | . as $e | ($b[0].ledger.instances[$e.key]) as $y | $e.value as $x |
   {inst:$x.curve.name, slug:$y.curve.name, seeds_equal:($x.seeds==$y.seeds), nvar:($y.variants|length), nvar_archived:($x.variants|length),
    variants_gae_equal:([range(0;($x.variants|length)) as $k | ($x.variants[$k].total_gae==$y.variants[$k].total_gae)]|all),
    variants_recovered_equal:([range(0;($x.variants|length)) as $k | ($x.variants[$k].recovered==$y.variants[$k].recovered)]|all),
    variants_verified:([$y.variants[].verified]|all),
    rho_gae_equal:([range(0;($x.rho|length)) as $k | ($x.rho[$k].gae==$y.rho[$k].gae)]|all)} ]' -c
