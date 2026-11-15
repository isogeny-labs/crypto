# LEAD-EXO-four-list-ktree-join

## 1. Lead as recorded
- disposition: open_next_step (last reviewed 2026-10-24); family: exotic; source: atlas_survey_20261107 --pursuit.
- title: Wagner k-tree / four-list joins with cheap sum-compatible filters.
- next-step excerpt (truncated in leads.json): "Measure the relative bucket variance W of the O2 agreement kernel (H-ECDLP-2ade73; IDEA-20260904..."
- Context from the desk notes: the j=2 four-tree exponent 0.4167 is closed for character filters under group-law combining (toy N<=65539); the desk's proposed step is to fit W's decay across N against 1/sqrt(N): slower decay reopens a bucket-concentrated filter, 1/sqrt(N) extends the bucket-averaged barrier to a bucket-resolved one.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 182 only); /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (grep context only). The "O2 agreement kernel" definition itself was not found; the kernel below is my own stand-in (see section 6).

## 2. Hypothesis
For a uniformly random curve E/F_p (p ~ 2^12..2^20, N=#E), bucketing all affine points by the cheap filter h(P)=x(P) mod B with B=floor(sqrt N), the relative bucket variance W = Var(occupancy)/mean^2 equals the random-assignment value (~ B/N ~ N^-1/2) within 25%, and its log-log slope against N is -0.5 +- 0.15. I.e. this filter shows no bucket concentration beyond Poisson.

## 3. Preregistered prediction and falsification criterion
- Statistic: (i) ratio r_p = W_xmod / W_null per size (mean over curves); (ii) OLS slope of log W_xmod on log N across the 5 sizes.
- Prediction: all r_p in [0.8,1.25] and slope in [-0.65,-0.35] -> "consistent_with_prediction" (bucket-averaged barrier extends to this filter; the lead gains no support from it).
- Falsified: controls pass but some r_p outside the band or the slope outside the interval (slower decay: a bucket-concentrated filter exists).
- Negative control / null: same point multiset, bucket i.i.d. uniform; must have slope in [-0.6,-0.4].
- Positive control: planted filter forcing 20% of abscissae into bucket 0; must give W_planted > 3 W_null at every size.
- A/A arm: same kernel on the even-x vs odd-x halves of the same curve; per-size ratio of the normalised W in (0.7,1.43).
- If any control fails: verdict "inconclusive".

## 4. Method
Pure stdlib. Per size, pick prime p in [2^b, 2^b+2^(b-2)) with b in {12,14,16,18,20}; 6 random short-Weierstrass curves (nonsingular). Count points per abscissa from a squares table (O(p)). B=isqrt(N). Compute W for the xmod, null, planted and A/A arms. Seed 20261110 (random.Random, single stream, fixed order). Cost estimate: a few seconds per curve at p~2^20, about 1-3 minutes total, memory < 150 MB (list of ~1.3M ints).

## 5. Classification: runnable_local
A stdlib toy decides the stand-in measurement directly; run.py implements it and passed `python3 -I run.py --smoke`.

## 6. What a run cannot establish
- The filter x mod B is not a sum-compatible filter on the group; it only probes bucket variance of an abscissa-based filter, not the O2 agreement kernel or the actual Wagner four-list join cost exponent.
- Whole-group enumeration at N<=2^20 says nothing about p beyond 2^30 or about sampled (not exhaustive) lists.
- A null result does not exclude other filters (character filters, DL-aware ones). Blind-instrument caveat from the atlas applies. All results are [pcc:unverified].
