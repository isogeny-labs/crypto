# analysis/summary.json from the three sessions' band reports and saved
# comparisons (jq -n, see scripts/summary.sh). Copies values; computes only
# the band verdict per alternative and the measured/predicted rho ratio.
def band($c): $bands[] | select(.curve_tag == $c);
def mean_s($b; $arm): ($b.rows[] | select(.arm == $arm) | .mean_s);
[ $curves[] as $c
  | (band($c)) as $b
  | { curve_tag: $c,
      set: (if ($c | startswith("r")) then "primary (amendment 1, registered curves)" else "supplementary (original protocol, unregistered curves)" end),
      curve: $b.rows[0].curve, log2_r: $b.rows[0].log2_r,
      predicted_plan_over_rho: ($predicted[$c]),
      weights: $b.weights,
      mean_s: ([ $b.rows[] | { (.arm): .mean_s } ] | add),
      plan_over_rho: ([ $cmps[] | select(.curve_tag == $c and .a == "rho-neg" and .b == "plan") ][0] | { ratio, ci95 }),
      alternatives: [ $cmps[] | select(.curve_tag == $c and .a == "plan" and .b != "plan-aa")
        | . as $x
        | (mean_s($b; "plan")) as $p | (mean_s($b; $x.b)) as $q
        | { arm: $x.b, ops_ratio: $x.ratio, ci95: $x.ci95,
            band_ratio: [ range(0; $p | length) | $q[.] / $p[.] ],
            verdict: (if ($x.ci95[0] > 1) and ([ range(0; $p | length) | $q[.] > $p[.] ] | all) then "confirmed"
                      elif $x.ci95[1] < 1 then "refuted"
                      else "not separated across the band" end) } ],
      aa: ([ $cmps[] | select(.curve_tag == $c and .b == "plan-aa") ][0] | { ratio, ci95 }) } ]
