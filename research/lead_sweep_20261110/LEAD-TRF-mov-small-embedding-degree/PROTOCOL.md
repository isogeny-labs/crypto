# LEAD-TRF-mov-small-embedding-degree

## 1. Lead as recorded
- Disposition: closed_known (last reviewed 2026-10-24). Family: transfer. Source: atlas_survey_20261107 --pursuit.
- Title: MOV / Frey-Rueck pairing transfer of the ECDLP to F_{p^k}^* for small embedding degree k.
- Next-step excerpt (truncated): "Revisit when: a deployed curve is found with small k (an audit hit), or a construction yields a ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58, 326-328);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/README.md (lines 58, 149). Recorded obstruction: "embedding degree ~N for random curves".

## 2. Hypothesis
For ordinary prime-field curves with a cryptographic-like group (largest prime factor r >= p/8, r != p) at toy size p in [2^12, 2^13+600], the embedding degree k = ord_r(p) is not small: the fraction of random curves with k <= 10 is <= 0.01, and median k/(r-1) matches a random-(p mod r) null (within 0.15) and is >= 0.1. So the MOV transfer gives no route for random curves (the recorded obstruction).

## 3. Preregistered prediction and falsification criterion
- Statistic: pooled fraction of curves with k <= 10 over arms A1, A2; and pooled median of k/(r-1).
- Prediction: fraction <= 0.01 and median >= 0.1 and |median - null median| <= 0.15 -> consistent_with_prediction (obstruction re-derived at toy size).
- Falsified: controls pass but fraction > 0.01 or median deviates from null (random toy curves show small k more than the heuristic says).
- Negative control / null: random prime r in [p_min/8, p_max] and random p, k = ord_r(p)/(r-1) (same instrument).
- Positive control: supersingular y^2 = x^3 + 1, p = 2 mod 3, #E = p+1; instrument must return k = 2 for every such curve (detector sees small k).
- A/A arm: two independent seeds (1001, 1002) of the identical random-curve arm; |frac difference| <= 0.01 required.
- If positive control or A/A fails: inconclusive.

## 4. Method
Pure stdlib. Primes p in [4096, 8792]. Random short Weierstrass y^2 = x^3+ax+b (non-singular), #E by Legendre-symbol sum with a precomputed chi table (O(p) per curve). r = largest prime factor of #E; keep r >= p/8, r >= 5, r != p (skip anomalous). k = ord_r(p) via factoring r-1. 700 curves drawn per arm, 2 arms, seeds 1001/1002; null seed 7. Cost estimate: well under 1 minute, < 50 MB, one core.

## 5. Classification
runnable_local. A stdlib toy decides the sharpened question (cheap re-derivation of the recorded bound).

## 6. What a run cannot establish
- Nothing about deployed curves, large sizes, or constructed pairing-friendly families (Cocks-Pinch, BN, etc.); a toy cannot revisit the "audit hit" condition.
- No actual MOV DLP solve or cost comparison to rho (no GAE pricing, no 5a band); only the embedding-degree distribution.
- Small p makes k <= 10 thresholds relatively generous; result is [pcc:unverified], not a leaderboard row.
