# Independent recompute of the AGENTS.md §5a band means from raw records.jsonl.
# Written by the validator; shares no code with src/cryptanalysis/ecbench/band.rs.
#
# Usage: jq -s --slurpfile cal docs/ic/calibration.json --arg session NAME \
#           -f band_recompute.jq sessions/NAME/records.jsonl
#
# Rule (as specified in the validation task, checked against band.rs):
#   S(w) = (G - C + w*(U + n_C)) / sqrt(r)
#   G   = cost.total_gae
#   U   = sum of counts of the counters named in cost.unpriced: top-level
#         counters[name], else the sum over phases of native[name] or
#         native[name without "_uncharged"]
#   C, n_C = over phases, over PRICED_NATIVE (counter, unit) with the unit in
#         detail.calibration_pinned_units and count > 0: count * ratio[unit]
#         and count; ratio from docs/ic/calibration.json row
#         "koblitz/K_<a> / GF(2^<n>)" (the curve_spec of the record).
#         No pinned units: C = n_C = 0.
# Warm-ups skipped; only outcome.status == "verified" records are averaged
# (band.rs Record::counts); every record of this study is verified.
#
# Diagnostics (not the headline rule):
#   S_alt(w)  = (G - C_full + w*n_C)/sqrt(r), C_full = the GAE that
#               detail.full_pricing.gae_by_counter actually put on those same
#               PRICED_NATIVE counters inside G (full_prices.json ratios).
#   S_point   = G/sqrt(r) (the full_prices.json single pricing point).
#   S_group   = group operations only (every arm's native work at 0):
#               IC: sum over phases of adds+doubles; rho: G minus the
#               "natives" phase.

# PRICED_NATIVE of src/cryptanalysis/ecbench/methods.rs:1898 at 45a11de2eb.
def priced_native: [
  ["sqrt_solves", "ns_per_sqrt"],
  ["as_solves", "ns_per_as_solve"],
  ["s4_pairs", "ns_per_s4_pair"],
  ["lookups", "ns_per_lookup"],
  ["row_ops", "ns_per_row_op"],
  ["legendre_symbols", "ns_per_legendre"],
  ["inversions", "ns_per_inversion"],
  ["frobenius_maps", "ns_per_frobenius"],
  ["canonicalisations", "ns_per_canon"],
  ["target_guard_probes", "ns_per_lookup"]
];

def weights: [0, 0.1, 1];

def cal_key: "koblitz/K_\(.workload.curve_spec.a) / GF(2^\(.workload.curve_spec.n))";

def unpriced_total:
  . as $r
  | [ $r.cost.unpriced[]
      | . as $name
      | ($name | sub("_uncharged$"; "")) as $base
      | if ($r.counters | has($name)) then $r.counters[$name]
        else ([ $r.phases[] | .native | (.[$name] // .[$base]) | select(. != null) ]
              | if length == 0 then error("uncounted unpriced counter \($name) in \($r.record_id)") else add end)
        end ]
  | add // 0;

def calibrated($cal):
  . as $r
  | ($r.detail.calibration_pinned_units // []) as $pinned
  | if ($pinned | length) == 0 then {c: 0, n: 0, c_full: 0, row: null}
    else
      ($cal.instances[$r | cal_key]
        // error("no calibration row \($r | cal_key) for \($r.record_id)")) as $row
      | [ $r.phases[] as $p
          | priced_native[] as [$counter, $unit]
          | ($p.native[$counter] // 0) as $k
          | select($k > 0 and ($pinned | any(.[]; . == $unit)))
          | {k: $k, g: ($k * ($row[$unit] // error("no ratio \($unit) in \($r | cal_key)"))), counter: $counter} ] as $items
      | ([ $items[].counter ] | unique) as $counters
      | { c: ([ $items[].g ] | add // 0),
          n: ([ $items[].k ] | add // 0),
          c_full: ([ $counters[] as $x | $r.detail.full_pricing.gae_by_counter[$x] // 0 ] | add // 0),
          row: ($r | cal_key) }
    end;

def is_ic: (.arm | test("^ic-"));

def per_record($cal):
  . as $r
  | (if $r.solver != null then error("solver block present in \($r.record_id); not handled") else . end)
  | ($r.workload.curve.r | sqrt) as $sq
  | $r.cost.total_gae as $g
  | ($r | unpriced_total) as $u
  | ($r | calibrated($cal)) as $cn
  | { arm: $r.arm,
      s: [ weights[] as $w | ($g - $cn.c + $w * ($u + $cn.n)) / $sq ],
      s_alt: [ weights[] as $w | ($g - $cn.c_full + $w * $cn.n) / $sq ],
      s_point: ($g / $sq),
      s_group: ((if ($r | is_ic) then ([ $r.phases[] | .adds + .doubles ] | add)
                 else ($g - ([ $r.phases[] | select(.name == "natives") | .gae ] | add // 0)) end) / $sq),
      u: $u, c: $cn.c, n_c: $cn.n, c_full: $cn.c_full, cal_row: $cn.row,
      s_recorded: $r.cost.s };

def mean: add / length;

($cal[0]) as $cal
| [ .[] | select(.warmup == false and .outcome.status == "verified") | per_record($cal) ] as $rows
| { session: $session,
    slug: (.[0].workload.curve.slug),
    r: (.[0].workload.curve.r),
    records_total: length,
    records_used: ($rows | length),
    max_abs_s0_minus_recorded_s_where_C_is_0:
      ([ $rows[] | select(.c == 0 and .n_c == 0 and .u == 0) | ((.s[0] - .s_recorded) | fabs) ] | max),
    arms: ( $rows | group_by(.arm)
            | map({ arm: .[0].arm,
                    n: length,
                    cal_rows: ([ .[].cal_row ] | unique),
                    mean_u: ([ .[].u ] | mean),
                    mean_c: ([ .[].c ] | mean),
                    mean_n_c: ([ .[].n_c ] | mean),
                    mean_c_full: ([ .[].c_full ] | mean),
                    mean_s: [ range(0; 3) as $i | ([ .[].s[$i] ] | mean) ],
                    mean_s_alt: [ range(0; 3) as $i | ([ .[].s_alt[$i] ] | mean) ],
                    mean_s_point: ([ .[].s_point ] | mean),
                    mean_s_group: ([ .[].s_group ] | mean) }) ) }
