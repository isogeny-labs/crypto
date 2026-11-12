# LEAD-ICB-kic-golden-base-exact-decomp
## 1. Lead as recorded
- Disposition: open_next_step (last reviewed 2026-10-24). Family: ic-binary. Source: atlas_survey_20261107 --pursuit.
- Title: Co-designed binary factor bases with exact decomposition: affine 2-flats, orbit-canonical abscissa lookup, SAT vs exact MITM (KIC lane).
- Next-step excerpt (truncated in leads.json): "Coordinator evidence review of the five completed KIC runs (EXP-KIC-3df18c, -424885, -7bcef8, -c..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (one-line index row only). Other rg hits (atlas JSON inventories) were not opened; the KIC run records were not found among the searched paths, so the toy works from the mechanism, not from those runs.
## 2. Hypothesis
For m = 2 exact (MITM) decomposition over E/F_{2^n}, a factor base whose x-coordinates form an affine F_2-subspace (2-flat) of dimension l gives the same sumset coverage |{P+Q : P,Q in F}| as a random x-set of equal point count: x-affine structure is not respected by the group law, so mean paired relative difference in distinct-sum count lies within +-2% of zero.
## 3. Preregistered prediction and falsification criterion
- Statistic: mean over curves of (|S_flat|/|S_null| - 1), S = distinct sums of unordered point pairs; z = mean/SE across trials.
- Prediction: |mean| < 0.02. Falsified if |z| > 4 and |mean| > 0.01 (flat base changes coverage: structure carries over, supports the lead's co-design premise). Otherwise inconclusive.
- Negative control / null: random x-sets of equal size on the same curve (the comparator itself).
- A/A arm: second independent random base vs the first; must have |z| <= 4, else verdict inconclusive.
- Positive control: base of +-kG multiples (arithmetic progression, same point count); sumset must shrink by > 30% (mean < -0.3), else detector failed and verdict is inconclusive.
## 4. Method
Pure stdlib. GF(2^13) with x^13+x^4+x^3+x+1 (primitive), log/exp tables; curve y^2+xy=x^3+ax^2+b, a in {0,1}, b random, points via half-trace. Per trial: new random curve, random affine flat of dimension l = 6 (64 x-values, both y per valid x), equal-count random x-set, second random set, progression control. 120 trials, master seed 20261110 (single Random stream). Cost: about 120 x 4 x ~2k group additions, well under 1 minute, < 50 MB. Smoke: n = 9, l = 3, 6 trials.
## 5. Classification: runnable_local
A stdlib toy decides the sharpened question (does flat structure alter exact-decomposition coverage). It does not review the KIC runs themselves.
## 6. What a run cannot establish
- Not the coordinator review of EXP-KIC-* runs, orbit-canonical abscissa lookup, Frobenius orbit reduction, or SAT-vs-MITM timing.
- Only m = 2, toy n = 13, flats in x only (not co-designed with a specific curve's subfield structure); no cost in GAE, no comparison to rho, no asymptotics.
- Results are labelled [pcc:unverified].
