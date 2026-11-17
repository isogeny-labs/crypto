# LEAD-EXO-padic-jet-canonical-lift

## 1. Lead as recorded
- id LEAD-EXO-padic-jet-canonical-lift; family exotic; disposition open_next_step (disposition_20261101: closed_obstructed); last_reviewed 2026-10-29.
- Title: "Prime-to-p jets, canonical/Teichmuller lifts and p-adic formal-group digits".
- Next-step excerpt (truncated): "A computable non-homomorphic local section shows above-chance scalar information against a match..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry has no next_step_cost / why fields); research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 74, 148, 170, 197); research/ecdlp_subexp_desk_20261101/README.md (line 57); research/prime_field_leads_20261031/C1_lift_torsion/atlas_update.md (head). Recorded obstruction: the only homomorphic section is prime-to-p torsion, killed by the formal log; non-homomorphic sections recover d at rate 1/p; prior toy contrast "at chance, power>=0.90 for 3x" (second-hand, not re-verified here).

## 2. Hypothesis
For prime-order E(F_p) (p = 3 mod 4, N != p), take the section s: E(F_p) -> E(Z/p^2) that lifts (x,y) by Hensel in y (variant 0) or after the fixed x-shift x+p(x^2+3) (variant 1). Define t(P) in Z/p by [N]s(P) = formal point with parameter z = -x/y = p*t. Then t(kG) carries no linear scalar information: the mode frequency of the first differences t((k+1)G)-t(kG) is indistinguishable from that of a random permutation of the same multiset (no additive-in-k structure).

## 3. Preregistered prediction and falsification criterion
- Statistic: M = max_c #{k : t_{k+1}-t_k = c mod p}/(N-2); z = (M - mean_null)/sd_null over shuffled copies of t.
- Prediction: z < 4.0 in every cell (6 prime-size cells x 2 variants as in section 4).
- Falsified: any cell z >= 4.0 (a section whose digit is partly linear in k; would justify a larger-scale lead). Consistent: all z < 4.0 with controls valid. Inconclusive: a control fails.
- Negative control / null: label permutation of the same t values (matched multiset). A/A: two independent shuffle batches (different seeds); batch-2 mean must lie within |z|<1 of batch 1.
- Positive control: synthetic leaky section t_k = c*k mod p must give M = 1.0 (detector sees linear structure).

## 4. Method
Stdlib Python. For each cell: random curve y^2=x^3+ax+b over F_p, p = 3 mod 4, with prime order N != p (seeded rng, seed 20261110+1000*curve+p); generator G; enumerate kG for k=1..N-1; lift to Z/p^2, compute [N]s(kG) in Jacobian coordinates mod p^2, extract t = (-XZ/Y mod p^2)/p. Sizes: p ~ 2^11 (4 curves, 200 shuffles), 2^13 (3 curves, 100), 2^15 (1 curve, 40); both lift variants. Cost estimate: well under 10 minutes, <50 MB. Output one JSON object; `--smoke` uses p ~ 101.

## 5. Classification
runnable_local: a stdlib toy decides the toy-scale question. run.py written and smoke tested.

## 6. What a run cannot establish
- Nothing about p ~ 2^256, or any section other than the two tested; only first-difference (linear-in-k) structure is probed, not arbitrary nonlinear dependence of t on k.
- The recorded 1/p-rate recovery bound is not re-derived. Does not bound sections defined over other rings or lifts (canonical lift of the curve itself is not used; same lifted curve a,b).
- The shuffle null assumes exchangeability of t; weak-class blindness caveat (LEAD-EXO-weak-class-instrument-calibration) applies. All results are [pcc:unverified].
