#!/bin/sh
# Table A recomputation from raw per-process price files. Read-only. Run from repo root.
# S_IC  = median_R1(setup_units)/sqrt(r) + mean_R1(ic_online_units)/sqrt(r)
# S_ref = median_R1(rho_setup_units)/sqrt(r) + mean_R1(rho_online_units)/sqrt(r)
# xref = S_IC/S_ref ; xfloor = S_IC / sqrt(pi/(4n))
# also: S_ref at the canonical step (rho_online repriced: steps*canonical_step_units) for the §5a/kernel check
set -eu
base=research/ic_tool_program/rule/v3/runs
for d in $base/k*n*; do
  jq -s '
   def med: sort|if length%2==1 then .[length/2|floor] else (.[length/2-1]+.[length/2])/2 end;
   def mean: add/length;
   (.[0].r|sqrt) as $q | (.[0].n) as $n |
   ((map(.median.setup_units)|med)/$q) as $ss |
   ((map(.median.ic_online_units)|mean)/$q) as $so |
   ((map(.median.rho_setup_units)|med)/$q) as $rs |
   ((map(.median.rho_online_units)|mean)/$q) as $ro |
   (map(.median.rho_units_per_step)|med) as $ups |
   {curve:.[0].curve, n:$n, r:.[0].r, rows_R1:length,
    all_verified:(map(.all_verified)|all), agree:(map(.ic_and_rho_agree)|all),
    S_IC:($ss+$so), S_ref:($rs+$ro), xref:(($ss+$so)/($rs+$ro)),
    floor:((3.141592653589793/(4*$n))|sqrt), xfloor:(($ss+$so)/((3.141592653589793/(4*$n))|sqrt)),
    rho_units_per_step_median:$ups,
    canonical_step_units:(map(.step_prices.canonical_step_units)|med),
    S_ref_canonical_step:($rs + ($ro * ((map(.step_prices.canonical_step_units)|med)/$ups))),
    xref_canonical_step:(($ss+$so)/($rs + ($ro * ((map(.step_prices.canonical_step_units)|med)/$ups))))
   }' $d/T*-R1.price.json
done | jq -s .
