# Claimed (README table, transcribed in claimed-readme-table.json) against
# recomputed (recompute.json, from the raw reports).  Tolerance per entry:
# half a unit in the last printed place, relative to the claimed value
# (the claims are rounded, so 1e-9 cannot apply), plus 1e-12.
#
# Usage: jq --slurpfile claimed claimed-readme-table.json -f headline.jq recompute.json
def decimals: if test("\\.") then (split(".")[1] | length) else 0 end;
def tol($p; $c): (0.5 * pow(10; -($p | decimals))) / ($c | fabs) + 1e-12;
def agrees($c; $r; $t): (($c - $r) | fabs) <= $t * ([($c | fabs), ($r | fabs), 1e-300] | max);
def rounds_to($r; $p): (($r * pow(10; ($p | decimals))) | round) == ((($p | tonumber) * pow(10; ($p | decimals))) | round);

. as $rec
| $claimed[0] as $cl
| def value($inst; $col):
    if $col == "rho" then $inst.rho.mean_s_recomputed
    elif $col == "ftb_over_if4" then $inst.ratios.ftb_over_if4
    else $inst.arms[$col].mean_s_recomputed end;
  [ $cl.table[] as $row
    | ($rec[] | select(.name == $row.name)) as $inst
    | range(0; $cl.columns | length) as $j
    | $cl.columns[$j] as $col
    | $row.printed[$j] as $p
    | ($p | tonumber) as $c
    | value($inst; $col) as $r
    | tol($p; $c) as $t
    | {
        name: "\($row.name)/\(if $col == "ftb_over_if4" then "S(fes-then-batched)/S(inherited-f4)" elif $col == "rho" then "S(matched rho)" else "S(\($col))" end)",
        claimed: $c,
        recomputed: $r,
        rel_tol: $t,
        printed: $p,
        agrees: agrees($c; $r; $t),
        recomputed_rounds_to_printed: rounds_to($r; $p),
        rel_diff: ((($c - $r) | fabs) / ([($c | fabs), ($r | fabs)] | max))
      } ] as $head
| [ $cl.table[] as $row
    | ($rec[] | select(.name == $row.name)) as $inst
    | {name: "\($row.name)/unknowns", claimed: $row.unknowns,
       recomputed: ([ $inst.runs[] | .n_vars | select(. != null) ] | unique),
       agrees: (([ $inst.runs[] | .n_vars | select(. != null) ] | unique) == [$row.unknowns])} ] as $unk
| ([ $rec[] | .ratios.ftb_over_if4 ]) as $rr
| [ ($cl.secondary.ftb_over_if4_range_printed) as $pr
    | {name: "ftb_over_if4 range (min)", printed: $pr[0], claimed: ($pr[0] | tonumber), recomputed: ($rr | min),
       agrees: rounds_to($rr | min; $pr[0])},
      {name: "ftb_over_if4 range (max)", printed: $pr[1], claimed: ($pr[1] | tonumber), recomputed: ($rr | max),
       agrees: rounds_to($rr | max; $pr[1])} ] as $range
| [ $cl.secondary.distance_to_rho_ftb_printed | to_entries[] as $e
    | ($rec[] | select(.name == $e.key)) as $inst
    | ($inst.arms["fes-then-batched"].mean_s_recomputed / $inst.rho.mean_s_recomputed) as $r
    | {name: "\($e.key)/distance S(ftb)/S(rho)", printed: $e.value, claimed: ($e.value | tonumber),
       recomputed: $r, agrees_at_printed_precision: rounds_to($r; $e.value),
       from_rounded_table_values: ((($inst.arms["fes-then-batched"].mean_s_recomputed * 10 | round) / 10)
                                   / (($inst.rho.mean_s_recomputed * 100 | round) / 100))} ] as $dist
| [ $cl.secondary.lowest_ic_row_claimed | to_entries[] as $e
    | ($rec[] | select(.name == $e.key)) as $inst
    | ($inst.arms) as $A
    | {name: "\($e.key)/fes-then-batched lowest IC row at the run's calibration point",
       claimed: $e.value,
       recomputed: ($A["fes-then-batched"].mean_s_recomputed <= ([ $A[] | .mean_s_recomputed ] | min)),
       strictly_lowest: ([ $A | to_entries[] | select(.key != "fes-then-batched" and .key != "fes-then-inherited") | .value.mean_s_recomputed ] | all(. > $A["fes-then-batched"].mean_s_recomputed)),
       tied_with_fes_then_inherited: ($A["fes-then-batched"].mean_s_recomputed == $A["fes-then-inherited"].mean_s_recomputed),
       band_order_vs_pair_table: $inst.band_orders.ftb_vs_pair.verdict,
       band_order_vs_inherited_f4: $inst.band_orders.ftb_vs_if4.verdict} ] as $lowest
| {
    headline: $head,
    summary: {total: ($head | length), agree: ([ $head[] | select(.agrees) ] | length),
              rounds_to_printed: ([ $head[] | select(.recomputed_rounds_to_printed) ] | length),
              max_rel_diff: ([ $head[].rel_diff ] | max)},
    unknowns: $unk,
    secondary: {ftb_over_if4_range: $range, distance_to_rho: $dist, lowest_ic_row: $lowest}
  }
