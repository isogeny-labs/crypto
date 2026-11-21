# LEAD-EXO-coordinate-statistic-leakage

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-29). Family: exotic. Source: atlas_survey_20261107 --pursuit.
- Title: Coordinate-statistic hunts: bias of x/y, slopes, bits, cross-ratios and trace functions under the group law.
- Next step (truncated in leads.json): "Measure the digit-character family (IDEA-20260921-1ec106), the one natural candidate the census ..."
- Census as summarised in the survey: every structured additive statistic is PGL_2-unstable, every stable multiplicative coset statistic sits at random level; EV-ECDLP-81f4d4 was lowered to preliminary by CORR-20261026-c94668.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry), /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 114, 153, 178), /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (title only). IDEA-20260921-1ec106 itself was not found; the digit-character definition below is my reconstruction. Atlas YAML is encrypted here.

## 2. Hypothesis
For prime-order E(F_p), p ~ 2^12..2^18, the base-16 digit characters chi_j(k) = exp(2 pi i d_j(x(kP))/16) (j = lowest digit, second digit, top digit) show no lag correlation along the scalar index k (lags 1..8, k -> 2k, k -> 3k) beyond what a negation-symmetric random labelling of the same orbit gives. I.e. the digit-character family is at random level, as the census predicts for stable statistics.

## 3. Preregistered prediction and falsification criterion
- Statistic: T = max over digits j in {0,1,top}, lags in {1..8, doubling pairs (k,2k), tripling pairs (k,3k)} of |sum_k chi_j(k) conj(chi_j(k'))| / sqrt(m), m = number of pairs.
- Null / negative control: per size, x-labels redrawn uniformly in [0,p) with x(k) = x(-k) (negation-matched), same N, same statistic. Per-size threshold = empirical 95th percentile of null T (30 / 8 / 3 draws at p~2^12 / 2^16 / 2^18; coarse, so the max is used when fewer than 20 draws exist).
- Positive control: the real orbit with the low digit overwritten by a slow symmetric function of k ((min(k,N-k)//7) mod 16) with probability 0.3; must exceed the threshold at every size, else verdict inconclusive.
- A/A arm: independent second set of null draws (different seeds); at most 1/3 may exceed the threshold, else inconclusive.
- Prediction: real curves exceed the null q95 no more often than 5% of curves + 2 (10 real orbits total). Consistent_with_prediction if so; falsified (digit leakage present) if more exceed; inconclusive if controls fail. A falsified outcome would open the lead for a scaling test; a consistent outcome closes the digit family at toy size only.

## 4. Method
Python stdlib only. For p = next prime above 2^12, 2^16, 2^18: random a,b (seeded random.Random(20261110 + 7p + c)), count points by Legendre/square-set, accept only prime N (so every nonzero point generates), take a random point P, build x(kP) for k=1..N-1 by affine addition/doubling. Curves: 6, 3, 1 per size. Statistic computed with Counter histograms of digit differences mod 16. Seeds fixed (base 20261110). Cost estimate: roughly 1-2 minutes, <150 MB (largest orbit 2^18 ints); limit 10 min via `timeout 660`.

## 5. Classification: runnable_local
A stdlib toy decides the toy-size question; run.py provided and smoke-tested (`python3 -I run.py --smoke`).

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- Not the original IDEA-20260921-1ec106 definition; only my reconstruction of a digit-character statistic along the scalar index.
- Orbit sequence k->x(kP) is not what an attacker sees without the DL, so a null says nothing about index calculus; it only bounds a bias of coordinate bits against group structure.
- Toy sizes (<=2^18), few real curves (10), coarse null quantiles; low power. The atlas notes that similar instruments are blind to known weak classes (weak-class-instrument-calibration), so a null is not evidence of absence for special curves.
- No matched rho gap, no replay certificate, no VALIDATION.json: cannot produce a leaderboard row or improvement claim.
- The full run was not executed in phase 1; its runtime estimate is untested.
