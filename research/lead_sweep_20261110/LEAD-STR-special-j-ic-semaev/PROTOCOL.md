# LEAD-STR-special-j-ic-semaev

## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-30). Family: structure.
- Title: Special j (0, 1728, small height, Semaev degeneration locus) in Semaev and index-calculus systems.
- Next-step excerpt (truncated in leads.json): "On matched-order families (fixed N, as in MATCHED-ORDER-DESIGN.md), measure the first-fall degre..."
- Atlas survey text: j=0/1728 S_3 support 9/10 vs 13 generic (EV-ENDO-e8d3e5); j=0 order-6 base 3x signed points +4% Ir, no cost gain. Next: matched-order families, first-fall, F4 time and charged S for S_3/S_4 vs sham-curve null, rho discounted by sqrt|Aut| too. Expected: support reduction is cost-invisible.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 50-55, 100-102, 263);
  /home/user/crypto/research/prime_field_leads_20261031/L1_lower_bound/atlas_update.md (grep context only).
  MATCHED-ORDER-DESIGN.md and the first-fall/F4 data were not read (no Groebner engine here).

## 2. Hypothesis
Toy proxy for the "Aut-closed factor base" half of the recorded obstruction. At fixed factor-base size B (points, not orbits) and fixed prime group order N, an Aut-orbit-closed factor base on j=0 (|Aut|=6) or j=1728 (|Aut|=4) curves gives a two-point-decomposition coverage |F+F|/N equal to that of a +-closed base on generic curves, within the A/A noise band. I.e. the Aut symmetry yields no relation-yield gain beyond what the sqrt|Aut| accounting already credits (support reduction is cost-invisible).

## 3. Preregistered prediction and falsification criterion
- Statistic: per curve, c = |F+F|/N, normalised by the random-model value 1-exp(-B^2/(2N)); arm value = mean over curves (3 random bases per curve); r_k = arm_k / generic_A.
- Prediction: |r_j0 - 1| and |r_j1728 - 1| <= band, band = max(3*SE(A vs B ratio), 0.02, 3*SE(r_k)).
- Outcomes: both inside -> consistent_with_prediction (no yield gain; lead's yield channel stays closed, only the sqrt|Aut| accounting remains). Either outside (up or down) -> falsified (a real yield effect at fixed B exists; justifies the F4/first-fall measurement on matched-order families). Inconclusive if the positive control fails, or the A/A ratio leaves the band, or A/A SE > 0.03.
- Negative control / null: generic prime-order curves (|Aut|=2), random +-closed bases (the "sham-curve null"), arm generic_A.
- A/A arm: generic_B_AA, an independent generic sample with another seed, same code path.
- Positive control: structured base F={+-1..+-B/2} (arithmetic progression, sumset about 2B); the detector must see normalised coverage ratio < 0.5 (it is about 0.14 in smoke).

## 4. Method
Stdlib only. For each arm draw primes p in [40000, 200000) (j0: p=1 mod 3, j1728: p=1 mod 4), random curve (j0: y^2=x^3+b; j1728: y^2=x^3+ax; generic: random a,b), count points by Legendre sum. Require prime order N (j1728 always has a 2-torsion point, so accept cofactor 2 or 4 and work in the prime-order subgroup via a multiple of a random point). Enumerate kG, store dlog table, read the Aut eigenvalue lambda from the map (x,y)->(wx,y) or (-x,iy). Exponent groups: generic {+-k}, j0 {+-lambda^i k}, j1728 likewise. Base F: random full orbits until |F| = B = 12*round(1.2 sqrt(N)/12) (same B for all arms). Sumset by exact double loop. 12 curves per arm, seeds 101 (A), 202 (A/A), 303 (j0), 404 (j1728), 505 (positive control). Cost estimate: about 2-4 minutes, <100 MB, one core. Not measured: only the smoke run (sizes 600-1500) was tested.

## 5. Classification: runnable_local
A stdlib toy decides the yield-at-fixed-B question. The first-fall degree and F4 time clauses need a Groebner engine and are not covered.

## 6. What a run cannot establish
- It measures only two-point sumset coverage (the S_3-like decomposition yield) on exponent sets, not Semaev polynomial degrees, first-fall degree, F4 time, or charged S versus rho; no matched rho and no 0/0.1/1 GAE pricing (so no cross-method ranking, no leaderboard row).
- The base is chosen at random in exponent space, not by an x-coordinate criterion, so it ignores coordinate-defined factor bases and the real lookup cost.
- Orders are not exactly matched across arms (normalised by a random model instead), and j1728 uses cofactor subgroups.
- Small-height j and the Semaev degeneration locus are not tested. Toy sizes (N about 2^15-2^17) say nothing about asymptotics. Results are [pcc:unverified].
