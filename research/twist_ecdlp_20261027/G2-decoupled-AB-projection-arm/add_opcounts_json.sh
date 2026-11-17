#!/usr/bin/env bash
# add_opcounts_json.sh -- (v3) writes the "operation_counts" key into results.json from out/opcounts.csv.
# Previously done by hand with jq (DEVIATIONS.md item 7); scripted so run.sh reproduces results.json. Deterministic, no timings.
# Usage: add_opcounts_json.sh [DIR]   (DIR defaults to this script's directory)
set -eu
D=${1:-$(cd "$(dirname "$0")" && pwd)}
cd "$D"
jq -R -s '
 (split("\n")|map(select(length>0))|.[1:]|map(split(","))
  |map(select(.[2]=="1" or .[2]=="2"))
  |map({fam:.[0],T:(.[5]|tonumber),arm:.[8],cand:(.[11]|tonumber),rs:(.[13]|tonumber),pg:(.[15]|tonumber)})) as $r
 | {primary_metric:"operation counts (not wall time)",
    definitions:{
     rootsolves:"Semaev work unit W_D(j)=C(N_E,j-1)*2^(j-1); loop-counted equals closed form in all 110 curve-cells",
     candidate_tuples:"C(n,j-1) enumerated (j-1)-subsets per target",
     projection_group_ops_model:"model: 2 elladd + 1 ellneg + 2 double-and-add scalar mults by 1/2 mod #E*#Etwist (textbook count), per target; not PARI internal",
     polynomial_evaluations_field_mults:"not instrumented (gp counts none); not reported"},
    scope:"primary curves c=1,2; all 22 (family,p,N) cells; 200 targets per cell; totals over those cells",
    source:"out/opcounts.csv (g2_opcounts.gp)",
    totals:($r|group_by([.fam,.arm])|map({family:.[0].fam,arm:.[0].arm,curve_cells:length,
       rootsolves_total:(map(.rs)|add),candidate_tuples_total:(map(.cand*.T)|add),projection_group_ops_model_total:(map(.pg*.T)|add)}))}
' out/opcounts.csv > out/operation_counts.json
jq --slurpfile oc out/operation_counts.json '.operation_counts=$oc[0]' results.json > results.json.tmp && mv results.json.tmp results.json
