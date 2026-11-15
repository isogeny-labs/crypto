# LEAD-ICP-coincidence-harvest-structured-base

## 1. Lead as recorded
- Disposition: open_next_step (reviewed 2026-10-30). Family: ic-prime.
- Title: Harvesting x-key / pair-sum coincidences of structured bases (label-GAP, window, log-GAP) and escaping conservation by target preselection.
- Next step (excerpt): "Run the break-even growth ladder (IDEA-20261018-a0830c): measure kappa_rel for the best FAM-1 ge..." Cost: medium, ladder of 6 log-GAP lines at 16-32 bits; "not yet approvable".
- Why it might beat sqrt(p): kappa_rel growing with N (>= 2^(8 delta*) per rung) would be the first structural relation source on F_p; 44/44 powered cells null, max |kappa-1| = 0.041.
- Files read: research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 21, 84, 140, 231); research/prime_field_lead_assessment_20261107/README.md (grep hits); docs/session-logs/2026-10-31.md (line 4693 region). Recorded results beyond these (EXP-PFDR-bb0d26, EXP-PFDR-2d36f9) were not read directly: unverified.

## 2. Hypothesis
For a window base (the n points of E(F_p) with smallest x, n = 2*floor(sqrt p)), the relative pair-sum x-key coincidence excess kappa_rel = C_window / C_random-base (same n, same curve) satisfies kappa_rel = 1 +- 0.1 at every rung p ~ 2^12..2^17 and has log-log slope d ln kappa / d ln N within +-0.03 of 0 (conservation: no growth with N). Falsifier: a slope above 0.05 at > 3 standard errors.

## 3. Preregistered prediction and falsification criterion
- Statistic: C = number of coinciding unordered pairs among x-coordinates of sums P_i+P_j (i<j; sums equal up to negation); kappa_rel = sum C_window / sum C_random over reps per rung; slope = OLS of ln(C_window/C_random) on ln N per replicate curve-series, mean and SE over 6 replicates.
- Prediction: all |kappa_rel-1| <= 0.1 and |slope| < 0.03 -> consistent_with_prediction (supports closure as sub-break-even, toy sizes only).
- Falsified: mean slope > 0.05 and mean/SE > 3 (kappa growing with N, window is a structural source).
- Otherwise (or if any control fails): inconclusive.
- Null / negative control: random-x base of the same size on the same curve (the denominator).
- A/A arm: a second independent random base vs the first; ratio must be in [0.85, 1.15] on every rung.
- Positive control: log-GAP base {g, 2g, ..., ng}; kappa must be >= 3 on every rung (detector sees planted structure). Failure of either control makes the run inconclusive.

## 4. Method
Python 3 stdlib. For each bits k in 12..17, p = first prime > 2^k; for rep r in 0..5, seed 1000003*k + r: random short-Weierstrass curve, enumerate all points by Euler scan (a table of squares), build four bases (window, random1, random2, log-GAP) of size n = 2*floor(sqrt p), compute all n(n-1)/2 pair sums with affine addition, count x-key collisions. Work: up to ~260k additions per base at 2^17, ~4 bases x 6 reps x 6 rungs; est. 1-3 minutes, <100 MB. --smoke uses p ~ 2^8..2^10, 2 reps (not a verdict). Command: `python3 -I run.py`.

## 5. Classification
runnable_local. A toy census of an explicit coincidence statistic with controls fits in the limits. The real ladder (16-32 bit log-GAP lines, native harness) is not what runs here.

## 6. What a run cannot establish
- Not the registered ladder EXP-PFDR-2d36f9: sizes are 12-17 bits, base is a window (x-interval), not the FAM-1 best log-GAP line, and no target preselection is tested.
- Pair-sum x-key coincidences are a proxy for relation yield; no decomposition, linear algebra or cost vs rho (no 5a pricing).
- Six rungs and six replicates give a coarse slope; a null cannot exclude growth at 2^(8 delta*) beyond 2^17, nor other structures.
- Results are labelled [pcc:unverified].
