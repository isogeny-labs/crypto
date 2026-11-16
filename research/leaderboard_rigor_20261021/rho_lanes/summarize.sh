#!/bin/bash
# Per curve and rho lane count: mean over the targets of IC S, rho S (set-up + walk) and their ratio, and the rho units per step,
# all from the ic price single-target reports of lane_sweep.sh (one process, one unit, IC and rho together).
for f in icv1-*-L*.json; do
  jq -c --arg f "$f" '(.r|sqrt) as $q | (.repetitions[0]) as $r | ($f|capture("-L(?<L>[0-9]+)\\.json").L|tonumber) as $L | ($f|capture("(?<t>T[0-9]+)-L").t) as $t
    | {curve, n, lanes:$L, target:$t, steps:$r.rho_online.steps, rho_units_per_step:$r.rho_online.units_per_step,
       S_ic:(($r.setup_units+$r.ic_online.units)/$q), S_rho:(($r.rho_setup_units+$r.rho_online.units)/$q), verified:.all_verified}
    | .ratio=(.S_ic/.S_rho)' "$f"
done | jq -s 'group_by([.curve,.lanes]) | map({curve:.[0].curve, n:.[0].n, lanes:.[0].lanes, targets:length,
   steps_mean:(map(.steps)|add/length), units_per_step_mean:(map(.rho_units_per_step)|add/length),
   S_ic:(map(.S_ic)|add/length), S_rho:(map(.S_rho)|add/length), ratio_mean:(map(.ratio)|add/length), all_verified:(map(.verified)|all)}) | sort_by([.n,.lanes])'
