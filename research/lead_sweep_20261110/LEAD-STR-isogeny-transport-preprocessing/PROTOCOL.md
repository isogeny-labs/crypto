# LEAD-STR-isogeny-transport-preprocessing

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-24; atlas_tally_20261101). Family: structure.
- Title: "Multi-target and preprocessing tables shared across an isogeny class".
- next_step_excerpt: empty in leads.json (closed lead, no next step recorded).
- Recorded obstruction (atlas_survey.md line 54): "closed_obstructed proof; table transport loses 1.5x-21x at odd l 3..31, EV-ENDO-10109d N4".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 52-60 only).
  candidate_matches.json and atlas_tally.json only matched by grep (title confirmed), not read further.
  The evidence record EV-ENDO-10109d was not available here.

## 2. Hypothesis
For an odd prime l with rational kernel K (l | #E, prime-order part N), carrying a precomputed x-table of T points
from E to the l-isogenous E' by Velu's x-formula costs more field operations than regenerating the table on E' by
T affine additions. The cost ratio rho_l = cost(transport)/cost(fresh) grows about linearly in l:
rho_3 in (1.0, 2.0], rho_31 in [10, 25], with rho_l > 1 for every tested l, under inversion cost I in {0,10,30} M.
This is a cheap re-derivation of the recorded 1.5x-21x loss, not a re-test of the proof.

## 3. Preregistered prediction and falsification criterion
- Statistic: mean over 2 curve draws of rho_l (counted M + I*inversions), reported at I = 0, 10, 30; decision on I = 10.
- Consistent: all rho_l > 1, min rho in (1.0, 2.0], max rho in [10, 25], rho nondecreasing in l (tolerance 0.05),
  and both controls pass.
- Falsified: any rho_l <= 1 (transport would be a gain), or rho range outside the bands, with controls passing.
  If transport ever beat regeneration, the lead's obstruction (cost, not correctness) would be wrong.
- Inconclusive: a control fails, or a prime l finds no curve.
- Positive control: transported x are all on E' (Velu a', b') and the map is injective on half the N-part (full table of (q-1)/2
  entries); proves the transport is actually correct.
- Negative control / null: same x-values tested against a perturbed b' on-curve rate must be < 0.8 (expected ~0.5).
- A/A arm: two independent curve draws (seeds s and s+1) for each l; the relative spread of rho is reported
  (expected small since cost counts are deterministic); no decision rule beyond reporting.

## 4. Method
Python 3 stdlib. p = first prime >= 40039 with p = 3 mod 4. Random short Weierstrass curves (seeds 20261110*1000+l, +1000
for the second draw) until #E = l*q with q prime > 60. Kernel K = q*R; Velu: a' = a-5t, b' = b-7w, x' = x + sum over (l-1)/2
representatives (t_Q/(x-xQ) + u_Q/(x-xQ)^2). Table T = 256 points x(kP), P of order q.
Transport arm: T Velu evaluations, counting M (mult/square) and inversions; kernel setup is not charged
(favours transport). Fresh arm: T-1 affine additions on E' from image generator, same counters.
l in {3,5,7,11,13,17,19,23,29,31}. Cost: order counting on p ~ 4e4, a few curve searches per l; estimated < 3 min,
< 50 MB, one core, hard time guard 520 s.

## 5. Classification
runnable_local. Stdlib toy decides the cost ratio; a cheap re-derivation of the recorded bound.

## 6. What a run cannot establish
- Not a proof of the closure and does not reproduce EV-ENDO-10109d (256-bit-scale census); toy p ~ 2^15.3.
- Cost model counts field operations in affine coordinates, no batch inversion, x-only transport; projective
  or batched variants could shift constants (not the ~l/2 factor).
- Does not test multi-target (many instances sharing a table) amortisation or the rho-walk table (distinguished points)
  case, only the table-transport primitive. Results are labelled [pcc:unverified].
