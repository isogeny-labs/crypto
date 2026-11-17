# A §5a band applied alike to every arm over the native counters the band
# tool (src/cryptanalysis/ecbench/band.rs, PRICED_NATIVE) sweeps for IC:
# square roots, AS solves, S4 pairs, lookups and guard probes, row ops,
# Legendre symbols, inversions, Frobenius maps, canonicalisations; for rho
# the same operations as its `natives` phase names them
# (canonicalisations_uncharged; distinguished_points as a lookup).
# Group work = every arm's adds + doubles at 1 GAE (IC: phase adds+doubles;
# rho: total_gae minus its natives phase). S_core(w) = (group + w*ops)/sqrt(r).
# Usage: jq -s --arg session NAME -f analysis/core_band.jq sessions/NAME/records.jsonl
def core: ["sqrt_solves","as_solves","s4_pairs","lookups","target_guard_probes","row_ops",
           "legendre_symbols","inversions","frobenius_maps","canonicalisations",
           "canonicalisations_uncharged","distinguished_points"];
def mean: add / length;
[ .[] | select(.warmup == false and .outcome.status == "verified")
  | . as $r | ($r.arm | test("^ic-")) as $ic
  | (if $ic then [ $r.phases[] | .native | to_entries[] ]
     else [ $r.phases[] | select(.name == "natives") | .native | to_entries[] ] end) as $nat
  | ([ $nat[] | select(.key as $k | core | index($k)) | .value ] | add // 0) as $ops
  | (if $ic then ([ $r.phases[] | .adds + .doubles ] | add)
     else $r.cost.total_gae - ([ $r.phases[] | select(.name == "natives") | .gae ] | add // 0) end) as $group
  | ($r.workload.curve.r | sqrt) as $sq
  | { arm: $r.arm, ops: $ops, group: $group, s: [ (0, 0.1, 1) as $w | ($group + $w * $ops) / $sq ] } ]
| { session: $session,
    arms: (group_by(.arm) | map({ arm: .[0].arm, n: length, mean_core_ops: ([.[].ops] | mean),
                                  mean_group_gae: ([.[].group] | mean),
                                  mean_s_core: [ range(0; 3) as $i | ([ .[].s[$i] ] | mean) ] })) }
