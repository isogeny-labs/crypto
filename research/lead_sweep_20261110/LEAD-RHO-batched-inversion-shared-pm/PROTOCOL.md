# LEAD-RHO-batched-inversion-shared-pm

## 1. Lead as recorded
- disposition: closed_known (last reviewed 2026-10-24); family: generic; source: atlas_survey_20261107 --pursuit.
- title: "Batched affine steps: Montgomery simultaneous inversion and the free x(P-Q) beside x(P+Q)".
- next-step excerpt (truncated): "Revisit when: An inversion-free step cheaper than 1I/n + 3M per walk."
- Recorded number (atlas_survey.md): P-256 179 ns/step at 1024 walks.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 64-67, 315-317).
  The other hits (atlas_tally.json, candidate_matches.json) are tallies only, not read. Nothing in cryptanalysis was searched.
- The lead's own YAML is encrypted here, so the full next step is unseen. The protocol rests on the excerpt and the known mechanism.

## 2. Hypothesis
For an r-adding rho walk on a prime-field short-Weierstrass curve, W walks advanced in lockstep with one Montgomery
batch inversion per round cost exactly 2M + 1S + 3(W-1)/W M + 1I/W field operations per step (counted by instrumentation).
This is the recorded "1I/n + 3M" bound: about 5M + 1S + I/W. The batching leaves the walk's statistics unchanged
(steps to solve are independent of W). The batched step beats an inversion-free Jacobian mixed addition
(8M + 3S = 11 M-equivalents) once I/W < 5. No variant measured here goes below 6 M-equivalents of inversion-free cost.
This re-derives the recorded closure. It is not a new speed-up.

## 3. Preregistered prediction and falsification criterion
- Statistic: per-step counted M, S, I averaged over all solved runs at each W, against the formula above.
- Prediction (all must hold): (a) |M/step - (2 + 3(W-1)/W)| / formula < 2% at every W;
  (b) S/step is about 1 and M+S is never below 6 M-eq per step;
  (c) I/step is within a few percent of 1/W;
  (d) the solve rate is > 90% (the correct log is found and checked);
  (e) the W=1 arm (unbatched) does not beat Jacobian mixed at I = 50 M, and the W=256 arm does.
- Outcomes: all hold gives `consistent_with_prediction`, so the closure stands and the lead stays closed_known.
  Any of (a)-(c) failing gives `falsified`: the cost model or the counting is wrong, so the atlas bound needs re-checking.
  Solve rate <= 90% gives `inconclusive`.
- Positive control: W=1 has I/step = 1 and W=256 has I/step < 0.1, so the detector sees the batching effect.
- Negative control / null: W=1 (no sharing) must lose to Jacobian mixed at I >= 50; this checks that the cost accounting is not trivially favourable.
- A/A arm: the same configuration at independent seeds (6 per cell). The steps-to-solve ratio W=256 over W=1 is reported
  and should be near 1, because lockstep batching must not change the walk's mixing. Bands are descriptive only, with no formal test.

## 4. Method
- Curves y^2 = x^3 + ax + b over F_p, p in {16411, 32771, 65537, 131101}. Search random (a,b) with a fixed seed
  until the group order N is prime (N about 2^14..2^17). N is counted by a Legendre-style point count.
- Instance: random G, secret x, H = xG. Walk: 16-way r-adding table (precomputed uG + vH), index x mod 16,
  distinguished points at 3 low bits of x, W in {1,4,16,64,256} walks, 6 seeds per (p,W). Seeds: curve seed s in 101..106, walk seed 7s+W.
- Per round: one Montgomery batch inversion of the denominators (3(W-1) M + 1 I), then per walk 2M + 1S.
  Degenerate x-equal cases take an uncounted exact fallback and are tallied. Correctness: every reported log is verified by dG = H.
- Cost: pure Python, one core. Expected well under 10 min, below 200 MB. No network, no files.
- Output: one JSON object with the statistic, controls, seeds, parameters and a verdict. `--smoke` uses p about 1009, W in {1,16}, 2 seeds.
- The free x(P-Q) beside x(P+Q) (x-only Montgomery ladder pairing) is not tested. It is a separate mechanism for
  x-only curves, and the toy covers only the batch-inversion part.

## 5. Classification
runnable_local. The cost counts are exact and a stdlib toy decides whether the recorded "1I/n + 3M" formula holds.

## 6. What a run cannot establish
- It does not give wall-clock speed. The 179 ns/step P-256 figure depends on 256-bit arithmetic, cache and SIMD, none of which Python toys show.
- The 2% tolerance on M/step absorbs the rare degenerate fallbacks, so it is not an exact match.
- Toy sizes (N about 2^17) say nothing about the rho-versus-index-calculus gap at 2^32-2^48.
- The Jacobian mixed-add cost (11 M-eq) and the I = 20/50/100 M ratios are assumptions.
  A real ratio depends on the field and the implementation.
- It does not test the x(P-Q) trick or any inversion-free step cheaper than the formula, so it cannot reopen the lead.
- It does not price native work at 0, 0.1 and 1 GAE per operation as section 5a requires, so it supports no cross-method ranking.
- The result is labelled [pcc:unverified].
