#!/bin/sh
# Table B recomputation + the §5a pricing band, from the frozen ledger and the matched-rho re-pricing.
# Read-only. Run from the repo root.
#  S_IC(recipe)  = mean over the 3 repetitions of total_gae / sqrt(r)
#  best recipe   = argmin over the 9 recipes of that mean (as the board does)
#  S_ref         = mean over every matched-rho run of gae / sqrt(r)
#  xref = S_IC/S_ref ; xfloor = S_IC / sqrt(pi/4)
#  band: G(w) = G - C + w*(U + n_C), C and n_C the GAE and operation count the instance's pinned calibration
#        put on PRICED_NATIVE counters, U = rho's uncharged canonicalisations (IC has no named unpriced counter
#        where every priced unit is pinned); ratio(w) = S_IC(w)/S_ref(w), w in {0, 0.1, 1}.
F=docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json
R=research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json
jq -n --slurpfile L $F --slurpfile P $R '
  def pn: ["sqrt_solves","as_solves","s4_pairs","lookups","row_ops","legendre_symbols","inversions","frobenius_maps","canonicalisations","target_guard_probes"];
  def unit($c): {"sqrt_solves":"ns_per_sqrt","as_solves":"ns_per_as_solve","s4_pairs":"ns_per_s4_pair","lookups":"ns_per_lookup","row_ops":"ns_per_row_op","legendre_symbols":"ns_per_legendre","inversions":"ns_per_inversion","frobenius_maps":"ns_per_frobenius","canonicalisations":"ns_per_canon","target_guard_probes":"ns_per_lookup"}[$c];
  def mean: add/length;
  [ $L[0].ledger.instances[] | select(.regime=="prime" or .regime=="char2") ] | map(
    . as $i | ($i.r|tonumber|sqrt) as $q |
    ($P[0].instances|map(select(.instance==$i.curve.name))|.[0]) as $p |
    ($i.calibration) as $cal | ($i.calibration_pinned.pinned) as $pinned |
    ( $i.variants | group_by(.name) | map({name:.[0].name, reps:., S:(map(.total_gae)|mean/$q)}) | sort_by(.S) | .[0] ) as $best |
    ( $best.reps | map(
        . as $v |
        ([$v.factor_base,$v.relations,$v.linear_algebra,$v.verify]|map(.native // {})) as $nat |
        ( [ pn[] as $c | select(true) | ($nat|map(.[$c] // 0)|add) as $n |
            ($pinned|index(unit($c))) as $pin |
            if ($n>0 and $pin!=null) then {n:$n, gae:($n*($cal[unit($c)]/$cal.ns_per_add))} else empty end ] ) as $cn |
        {G:$v.total_gae, C:($cn|map(.gae)|add // 0), nC:($cn|map(.n)|add // 0)} ) ) as $icr |
    ( $p.matched_reference.per_run | map({G:.gae, U:(.counters.canonicalisations_uncharged // 0)}) ) as $rr |
    { instance:$i.curve.name, regime:$i.regime, r:$i.r, log2r:($i.r|tonumber|log/(2|log)),
      best_recipe:$best.name, S_IC:$best.S,
      S_ref:(($rr|map(.G)|mean)/$q), S_ref_reported:$p.matched_reference.mean_s,
      xref:($best.S/(($rr|map(.G)|mean)/$q)), xfloor:($best.S/((3.141592653589793/4)|sqrt)),
      all_verified:($best.reps|map(.verified)|all),
      band: ([0,0.1,1] | map( . as $w | {w:$w,
          S_IC:(($icr|map(.G - .C + $w*.nC)|mean)/$q),
          S_ref:(($rr|map(.G + $w*.U)|mean)/$q)} | .ratio=(.S_IC/.S_ref)) ),
      rho_uncharged_canon_mean:($rr|map(.U)|mean), ic_priced_native_ops_mean:($icr|map(.nC)|mean) } )'
