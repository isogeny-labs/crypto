# LEAD-STR-secp256k1-cm-class-audit

## 1. Lead as recorded
- Disposition: closed_obstructed (last reviewed 2026-10-24). Family: structure.
- Title: secp256k1 CM structure (D=-3, j=0, h=1) and its isogeny class.
- Next-step excerpt: empty in leads.json (source atlas_tally_20261101).
- Recorded obstruction (survey line 54/147): "only lever sqrt 6 constant" (ceiling sqrt|Aut|, j=0 gives sqrt 6; class-wide rho invariance floor/crater 1.004; End(G)=Z/N on prime-order G).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 51-55, 144-150);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only).
  Nothing in cryptanalysis was searched beyond these hits.

## 2. Hypothesis
On prime-order j=0 curves y^2=x^3+b over F_p (p=1 mod 3), Pollard rho with an r-adding walk on the 6-element orbits {+-lambda^k X} needs mean total steps (including fruitless-cycle restarts) that are lower than plain rho by a factor R6 in [0.75*sqrt6, 1.10*sqrt6] = [1.84, 2.69]. I.e. the recorded sqrt 6 constant is both an upper ceiling (R6 not above 2.69) and approximately reachable (not below 1.84). The obstruction is "falsified" if the factor exceeds the ceiling.

## 3. Preregistered prediction and falsification criterion
- Statistic: R6 = mean(steps_plain) / mean(steps_orb6), pooled over all toy curves (mean of per-curve means).
- consistent_with_prediction: controls pass and 1.84 <= R6 <= 2.69.
- falsified: controls pass and R6 > 2.69 (a speedup above sqrt|Aut|; would reopen the lead and trigger audit of the implementation).
- inconclusive: any control fails, or R6 < 1.84 (walk degradation hides the ceiling; neither confirms nor refutes).
- Positive control: negation-only canonicalization on the same j=0 curves, R2 = plain/neg must lie in [0.99, 1.63] (sqrt2 +-).
- Negative control / null: generic curves y^2=x^3+x+b (Aut={+-1}, no sextic orbit), same sizes, negation arm; ratio must also lie in [0.99, 1.63]; it must NOT approach sqrt6 (this is the null for "any prime-order curve gets sqrt6").
- A/A arm: plain rho run twice with different seeds on the same curves; ratio must lie in [0.8, 1.25].
- Controls-ok requires all three.

## 4. Method
- Primes p ~ 2^14, 2^16, 2^18 with p=1 mod 3, p=3 mod 4. For each size, 2 j=0 curves (a=0, b drawn from Random(20261110)) with prime group order N found by exhaustive point count (quadratic-residue table), plus 1 generic curve (a=1) of prime order.
- omega = element of order 3 in F_p; lambda = root of l^2+l+1 mod N matched to the action (x,y)->(omega x, y) on a group point.
- Rho: 16-slot additive walk, slot = x(canonical rep) mod 16; each tracked point stores (a,b) with X=aP+bQ, Q=dP with d known. All visited canonical points go to a shared dictionary; a collision with different coefficients and nonzero b-difference yields d, verified against the true d. Trivial collisions (fruitless cycle) abort the trail and restart from a fresh random point; steps accumulate across restarts; cap 60*sqrt(N) steps.
- Arms per curve: plain, plain A/A (seed+500), neg, orb6; 60 trials each with random d. Seeds: master 20261110; arm seeds 1000+i, 2000+i (+1, +2, +500 offsets).
- Cost: pure Python affine arithmetic, well under 10 minutes and a few MB RAM on one core (estimate 1-5 min). Smoke: p~2^10, 6 trials, <1 s.

## 5. Classification
runnable_local. A stdlib toy re-derives the recorded sqrt(|Aut|) bound empirically on the same family (j=0, prime order).

## 6. What a run cannot establish
- Nothing about secp256k1 itself (256 bit), only toy sizes where constant factors and walk degradation (fruitless cycles, small r) differ.
- Does not test index-calculus or any non-rho route; no 0/0.1/1 GAE pricing (step count only, orbit canonicalization cost of ~6x field ops per step is not charged, so wall-clock gain is smaller than R6).
- Does not cover the isogeny class beyond twists, nor completeness of CM relations.
- Statistic is a pooled mean over few curves; 60 trials per arm gives roughly 10% noise per curve. All results are [pcc:unverified].
