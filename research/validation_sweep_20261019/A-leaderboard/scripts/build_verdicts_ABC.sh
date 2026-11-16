#!/bin/bash
# Emits the verdict records of Tables A, B and C (one JSON line each) from the raw audit outputs in raw/.
# Judgement text lives here, numbers come from the files. usage: from the repo root.
D=research/validation_sweep_20261019/A-leaderboard; RAW=$D/raw
# ---------------- Table A
jq -c --slurpfile out <(grep -v 'VERIFIED\|FAILED' $RAW/replay_A.out | jq -c .) --slurpfile cert <(grep -c 'VERIFIED' $RAW/replay_A.out) '
 .[] | . as $r | ("k"+(if ($r.n==47 or $r.n==59) then "1" else "0" end)+"n"+($r.n|tostring)) as $d |
 ($out|map(select(.run|startswith($d+"/")))) as $o |
 ($r.curve) as $slug |
 {row_id:("A:"+$slug), table:"A",
  source_paths:["research/ic_tool_program/rule/v3/analysis.json","research/ic_tool_program/rule/v3/runs/"+$d+"/","research/ic_tool_program/rule/v3/PROTOCOL.md"],
  recomputed_ok:true,
  recomputed:{S_IC:$r.S_IC,S_ref:$r.S_ref,x_reference:$r.xref,x_floor:$r.xfloor,rows_R1:$r.rows_R1},
  replay_n:($o|length),
  replay_ok:(($o|length)>0 and ($o|map(.verified_new and .recovered_equal and .rho_counts_equal and .certificates_equal and .archived_counters_equal)|all)),
  rho_reference_ok:"partial", pricing_ok:false, protocol_frozen_ok:true, verdict:"validated_with_caveats",
  notes:("S, x reference and x floor recompute exactly from the 64 R1 price files (median setup + mean online; matches the published digits). 3 seeded runs: 6/6 scalars verified by an independent GF(2^n) verifier and replayed natively with identical counts, rho counts and certificates (the current binary adds a row_ops counter; nothing archived differs). Reference is the repository measured rho as built, 32 lanes, "+($r.rho_units_per_step_median|tostring|.[0:5])+" units/step against a canonical step of "+($r.canonical_step_units|tostring|.[0:5])+"; the IC unit is the 1024-lane batched add, so IC got the batching kernel that rho did not. Against rho at the canonical step the ratio is "+($r.xref_canonical_step|tostring|.[0:5])+"x, not "+($r.xref|tostring|.[0:5])+"x (understates the gap by "+(($r.xref_canonical_step/$r.xref)|tostring|.[0:4])+"x). Rho also improved 20% in instructions after this row was measured (2026-10-14). Unit is wall-clock for both arms; no section 5a 0/0.1/1 band was run. Protocol rule/v3 precedes the raw runs in git with no later edits. No row is below its reference.")}' $RAW/recompute_A_haiku.json
# ---------------- Table B
jq -c --slurpfile leg docs/ic/leaderboard.json '
 ($leg[0].board|map(select(.family=="B")|{key:.legacy,value:.slug})|from_entries) as $slug |
 .[] | . as $r |
 {row_id:("B:"+$slug[$r.instance]), table:"B",
  source_paths:["docs/ic/runs/ic-boundary-ledger-round5-2026-09-22.json","research/ic_rho_reference_20260923/reprice/ic-boundary-ledger-round5-2026-09-22.json","docs/ic/BOUNDARY_TARGETS.md"],
  recomputed_ok:(($r.S_ref-$r.S_ref_reported)|fabs<1e-9),
  recomputed:{best_recipe:$r.best_recipe,S_IC:$r.S_IC,S_ref:$r.S_ref,x_reference:$r.xref,x_floor:$r.xfloor,band_ratio_w0:$r.band[0].ratio,band_ratio_w0_1:$r.band[1].ratio,band_ratio_w1:$r.band[2].ratio},
  replay_n:3, replay_ok:true,
  rho_reference_ok:true, pricing_ok:false, protocol_frozen_ok:"partial", verdict:"validated_with_caveats",
  notes:("S_IC (mean of 3 repetitions of the best of 9 recipes) and the matched negation-map rho reference recompute exactly from total_gae and per-run gae. The 3 planted scalars equal the recovered ones, and the whole prime and random-binary ladders replay natively on the current binary with identical best-recipe counts and scalars; the replay also reproduces the matched rho S. The reference is the strongest available for this regime (negation, A = 2) and replaced a weaker frozen walk, which is conservative. Pricing: IC natives are priced by pinned calibration while the reference leaves canonicalisations unpriced ("+($r.rho_uncharged_canon_mean|tostring|.[0:6])+" per run), so section 5a requires a band; recomputed here: x reference is "+($r.band[0].ratio|tostring|.[0:5])+" / "+($r.band[1].ratio|tostring|.[0:5])+" / "+($r.band[2].ratio|tostring|.[0:5])+" at 0 / 0.1 / 1 GAE per native op, so the order (IC above rho) holds at every weight but the magnitude moves. The recipe is chosen as the minimum over 9 recipes on the same 3 planted targets (selection effect), and BOUNDARY_TARGETS.md is a living document edited many times after the data; the preregistered content of Round 5 was not separately frozen.")}' $RAW/recompute_B_haiku.json
# ---------------- Table C
jq -c --slurpfile leg docs/ic/leaderboard.json --slurpfile out $RAW/replay_C.out '
 ($leg[0].board|map(select(.family=="C")|{key:(.log2_r|tostring|.[0:5]),value:.slug})) as $x |
 .[] | . as $r |
 ($leg[0].board|map(select(.family=="C" and (.log2_r-$r.log2r|fabs)<0.01))|.[0].slug) as $slug |
 ("k"+(if ($r.a==1) then "1" else "0" end)+"n"+($r.n|tostring)) as $d |
 ($out|map(select(.run|startswith($d+"/")))) as $o |
 {row_id:("C:"+$slug), table:"C",
  source_paths:["research/ic_descent_20260930/analysis-isolated.json","research/ic_descent_20260930/runs-isolated/main/"+$d+"/","research/ic_exponent_20260926/analysis.json","research/ic_descent_20260930/PROTOCOL.md"],
  recomputed_ok:true,
  recomputed:{S_IC:$r.s_after,S_ref_s20:$r.s_rho_s20,x_reference:$r.ratio_after,x_floor:$r.x_floor,pairs_per_set:$r.pairs_per_set},
  replay_n:($o|length), replay_ok:(($o|length)>0 and ($o|map(.all_verified and .recovered_equal and .archived_counters_equal)|all)),
  rho_reference_ok:"partial", pricing_ok:false, protocol_frozen_ok:true, verdict:"validated_with_caveats",
  notes:"S_IC is s_before divided by the geometric-mean wall-time speedup (20 pairs, isolated), averaged over sets M1-M4; both it and x reference recompute exactly from the 40 baseline/candidate price files per size and S20 analysis. 3 seeded runs replay natively with identical counts and all 32 recovered scalars (current binary adds a row_ops counter); no target points are archived for these runs, so no independent [d]G = Q check is possible. The reference S is not measured in the same process as the IC figure: it is S20 (2026-09-26) batch rho priced at the canonical step; the isolated re-run only re-priced rho at n = 41 / set M1 (counts equal S20, priced S 0.0600 vs 0.0610) so the header claim of same-process rho holds for S20 but not for this isolated re-run. S_IC is derived (baseline S over a time ratio), not a direct measurement of the candidate. Historical diagnostic under the one-target rule (32 targets). No 5a band; unit is wall-clock. Protocol precedes the raw runs; v2 amendment (isolation) was declared before the isolated runs."}' $RAW/recompute_C_haiku.json
