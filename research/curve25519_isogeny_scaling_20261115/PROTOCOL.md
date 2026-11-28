# Protocol: pairs law, scaling in t, deeper seed screen, distance to rho (frozen before the new data)

2026-11-15. Follows PRs #4213 and #4233 on the k=20 Curve25519 miniature (p = 1048573, A = 926, #E = 1049624 = 8 x 131203).
All results [pcc:unverified]. Same engine and binary as before (sha256 `ed99cb1653aaa35c9a87c0aa0817a63de0687dc498600e5694b3f7bc443f99bd`),
blind arm (`lifts=0`) unless stated: `curve25519_family f4cost 20 isogeny@SEED T 3 20000 2026111403 8000 2 full 0 30000000 0 0`.
Items refer to the owner-facing list of next steps (1 distance to rho, 2 raise t, 3 test the mechanism, 4 search more seeds; 5 drop lifts and
6 skip descent/torsion tricks are decisions, not experiments).

## Part A: the pairs law (item 3), existing data, no new runs
Conjecture from the data (not from a fit): with m = 2 a target R is hit when R = P1 + P2 with P1, P2 distinct base points, so
expected hits per target = b(b-1) / (2 #E) with b = `base_points`. No fitted constant.
- H22: for each of three groups of existing blind arms (seeds 0 to 20; screen top 8; screen bottom 8; 20,000 targets each), the observed hit total lies
  inside the exact Poisson 95% count interval of the sum over arms of 20000 x b(b-1) / (2 x 1049624).
- Descriptive: per-arm observed/expected, and the same for the `lifts=1` arms with expected scaled by the arm's measured lift targets tried per target
  (`pkm_lift_targets_tried` / `targets_done`).

## Part B: raise t (item 2)
- Candidates and arms: t = 6, seeds 101 to 108, unscreened (the first eight, fixed now), blind, 20,000 targets. A pilot of 500 targets on seed 101 is allowed
  only to size the wall time (its hits are not used); if one arm at 20,000 targets cannot finish within 4 hours the target count is lowered for all eight
  arms and recorded as an amendment before they run.
- H23: pooled t = 6 blind hits per 1e9 counted mul-adds over the eight seeds have a Poisson 95% lower bound above 3.334 (upper end of the unscreened t = 5 interval
  of PR #4213, 2.909 [2.525, 3.334]).
- H24: the pairs law of Part A holds at t = 6 (observed pooled hits inside the Poisson 95% interval of the sum of 20000 x b(b-1) / (2 x 1049624)).
- H25: blind mul-adds per target at t = 6 are within a factor 2 of t = 5's 1.7705e5 (i.e. between 0.885e5 and 3.541e5). Not a hypothesis about a gain: a record of how cost scales.
- Descriptive: tower search ops at t = 6, per-seed rates, where the rate ratio of t = 6 to t = 5 falls against the 4x the pairs law gives for hits per target.

## Part C: a deeper seed screen (item 4)
- Candidates: t = 5 seeds 61 to 3060 (3,000 new seeds), tower dumps only. Chosen seed = the largest on-curve count c (ties: smaller seed).
- H26: the chosen seed has c >= 25 of 32.
- H27: total tower group operations over all 3,000 candidates are below 1e-2 of 18 x (the screened-seed blind cost per relation, from its own arm).
  Reference and arithmetic are fixed in the analysis script; unit weight only, not the 0 to 1 GAE band.
- One blind arm on the chosen seed (20,000 targets) tests H22's law out of sample: observed hits inside the Poisson 95% interval of 20000 x b(b-1) / (2 x 1049624). Reported as H28.

## Part D: distance to rho (item 1), descriptive only
Back-of-envelope, not a measured gap and not priced in the 0 to 1 GAE band: required relations = `columns` + 1 for the arm; F4 mul-adds to obtain them =
(relations required) x (mul-adds per hit) for the best arm; compared with the expected rho group operations for the prime-order subgroup, sqrt(pi x 131203 / 4).
The unit mismatch (a mul-add is not a group addition) is stated, not resolved. A measured matched-rho arm needs the isolab-eks route (absent here). No hypothesis.

## Decisions (not experiments)
Item 5: lifts are neutral at best (rate ratio 0.88 to 0.93, p = 0.09 to 0.19) at 8x the cost per target: not carried forward. Item 6: T2 is closed in
`research/curve25519_torsion_leads_20261103` and descent class did not predict the seed rate (Spearman +0.08): no new work.

## Out of scope
lifts arms, other k or G, rho runs, a Conductor task, isolab runs and S3 checkpoints (deterministic counted mul-adds; the AWS route is absent here).
Decision rule for every H: supported only if its stated bound holds, otherwise not supported and reported as such. Seven hypotheses, no correction.
