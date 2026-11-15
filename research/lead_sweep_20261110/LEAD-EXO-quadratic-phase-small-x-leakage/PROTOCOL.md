# LEAD-EXO-quadratic-phase-small-x-leakage

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step. Family: exotic. Title: empty in leads.json. Source: atlas_survey_20261107 --pursuit.
- Next-step excerpt (truncated): "Done at N ~ 2^12-2^14 (QUADPHASE-20261104) and at one 2^16 curve (QUADPHASE-2P16-20261104): NULL".
- Atlas survey: NULL at N~2^12-2^14 (9/9 decisive curves) and one 2^16 curve; the instruments are blind to anomalous / small-embedding-degree curves (LEAD-EXO-weak-class-instrument-calibration), so the null bounds only what they can see.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 70-74, 153, 198, via rg context);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (line 215, via rg context).
  The QUADPHASE-* run records themselves were not found; the mechanism below is reconstructed from the lead name (an assumption).

## 2. Hypothesis
For random prime-order prime-field curves with group order N in ~2^10..2^16, the small-x point set S = {P : x(P) < p/16} (negation-closed) shows no quadratic-phase bias in its discrete logs d = log_G(P): the statistic T = max over a fixed random grid of (a,b) of |sum_{d in S} e((a d^2 + b d)/N)| / sqrt(|S|) is distributed as for a uniformly random negation-closed subset of Z_N^* of the same size. Falsifiable: at least 3 of 8 curves reject at alpha = 1/16.

## 3. Preregistered prediction and falsification criterion
- Prediction: NULL persists (n_reject_obs is consistent with Binomial(n, alpha)).
- Statistic per curve: empirical rank p-value of T_obs against 16 null draws (p_min = 1/17).
- Threshold: curve rejects if p <= 0.0625. Aggregate: exact binomial upper tail P(X >= rejects | n=8, 0.0625).
  - tail < 0.01 (i.e. >= 3 of 8 reject): verdict "falsified" (a quadratic-phase leak on the small-x set exists at this scale).
  - tail >= 0.01 and positive control passes: "consistent_with_prediction" (decides only that this instrument sees nothing at these sizes).
  - positive control detects in < 90% of curves: "inconclusive" (instrument not shown able to see effects).
- Negative control / null: 16 uniformly random negation-closed subsets (d and -d) of the same size from the same group, same grid.
- Positive control: planted set with a0*d^2 mod N in [0, N/4) (same size, negation-closed), grid includes (a0, 0); the observed and null statistics use the grid WITHOUT (a0, 0).
- A/A arm: one extra independent null draw per curve ranked against the same 16 nulls; its reject rate must be about alpha (reported as n_reject_AA; large values flag a broken instrument).
- Limits: with 8 curves and 16 nulls power is low for weak effects; a null here is not evidence of absence.

## 4. Method
- Seed 20261110 (curves) and seed+1000*(i+1) (null draws per curve). Python 3 stdlib only, one core.
- Per curve: random prime p in [8000, 65000), random (a,b), random point G; walk multiples of G (affine, <= p+2sqrt(p)+3 steps) to get order N; keep curve if N is prime and >= 1000. Record x(dG) for d=1..N-1.
- S = {+-d : x(dG) < p/16}, |S| ~ p/16. Grid: 99 random (a,b) with a in 1..N-1 (plus planted pair for the positive control only). Phases by lookup in a table of N-th roots of unity.
- 8 curves, 16 nulls, 1 A/A draw, 1 positive set: about 18 sets x 99 pairs x ~2^12 points per curve, roughly 7M operations per curve. Estimated 1-6 min total; the run loop stops after 480 s of elapsed time and reports the curves used. Memory < 100 MB (dict of N points, root table).
- Smoke (`--smoke`): 2 curves, p in [1500,2500), 6 nulls, 20 pairs, well under 1 s (positive control weak at that size; verdict is not meaningful).

## 5. Classification: runnable_local
A stdlib toy can sharpen whether the recorded NULL extends to a larger number of curves at N up to 2^16 with explicit positive and A/A controls; no external resource is needed.

## 6. What a run cannot establish
- It is a reconstruction of the instrument, not a re-run of QUADPHASE-20261104; the exact original statistic is not known.
- Toy sizes (N <= 2^16) say nothing about cryptographic sizes; the lead itself notes such instruments are blind to anomalous/small-embedding-degree curves, and no weak-class positive control is included.
- Only quadratic phases on a random grid of (a,b) are probed, not higher-degree or other leakage; max over a grid of 99 reduces power.
- Not an ECDLP speedup, not a validation-gate row; results are [pcc:unverified].
