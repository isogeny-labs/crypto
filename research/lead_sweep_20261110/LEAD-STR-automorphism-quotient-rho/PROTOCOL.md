# LEAD-STR-automorphism-quotient-rho

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_known (last reviewed 2026-10-24; same on 2026-11-01). Family: structure.
- Title: "Rho on E/Aut (negation, j=0 order-6, j=1728 order-4) and anything beyond sqrt(|Aut|)".
- Next-step excerpt (truncated in leads.json): "Revisit when: A canonicalisation with per-step cost kappa below 1 group operation is found for a ..."
- Recorded evidence (atlas_survey.md): ceiling sqrt|Aut|; j=0 rho.cubic/rho.negation 0.573 [0.562,0.585] vs 0.577 theory; ratio-to-ceiling 1.0043 (j=0), 1.0442 (j=1728), 20 seeds, p=100057 (EV-ENDO-10109d). Related: LEAD-RHO-negation-map-fruitless-cycles (rho.negation 1.453 x sqrt(pi/4), fruitless cycles).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 52-66, 145-159, 320-322 only); /home/user/crypto/docs/session-logs/2026-11-06.md (grep context only). Not read: atlas_tally.json. KB not queried (sweep task limits).

## 2. Hypothesis
For a cyclic group of prime order N with an automorphism subgroup of order k (k in {2,4,6}, acting as multiplication by lambda, ord lambda = k), r-adding rho run on canonical orbit representatives (fruitless short cycles restarted) needs a mean number of steps to first genuine collision equal to the k=1 mean divided by sqrt(k), within [0.9, 1.4] x (1/sqrt(k)). I.e. the quotient gains at most sqrt(k) and no more (re-derivation of the recorded ceiling at toy size, counting steps only, kappa = 0).

## 3. Preregistered prediction and falsification criterion
- Statistic: R_k = mean_steps(k, real walk) / mean_steps(k=1, real walk), with SE from per-trial SD (400 trials/cell), at N ~ 2^16, 2^18, 2^20 (primes = 1 mod 12), walk r=32, seed 20261110.
- Falsified (needs controls OK): R_k + 1.96 SE < 0.9/sqrt(k) for any k, size (a gain beyond sqrt(k) would contradict the ceiling).
- Consistent: every R_k has R_k - 1.96 SE >= 0.9/sqrt(k) and R_k <= 1.4/sqrt(k), controls OK. Otherwise inconclusive (includes excess fruitless-cycle loss beyond 1.4x).
- Negative control / null: k=1 (no quotient) must give R=1 by construction. Positive control: "ideal" arm, a fresh random mapping on classes, must give R_k within 12% of 1/sqrt(k) (shows the detector sees a sqrt(k) gain). A/A: second independent k=1 real arm with different seed; means must agree within 3 combined SE.
- Control failure makes verdict inconclusive.

## 4. Method
Group Z_N (element = discrete log, so group operation = addition; automorphism = multiplication by lambda = g^((N-1)/k)). Canonical rep c = min over {x*lambda^i}. Step: x' = c + M[h(c) mod r], M random fixed offsets. Walk from random start; first revisit of a class with cycle length > 8 is a collision (steps recorded); revisits with cycle length <= 8 are fruitless cycles: restart from a random point, keep seen set and step count. Python stdlib only, single core. Cost: 3 sizes x (4 k x 2 modes + A/A) x 400 trials x ~1-2e3 steps, est. 1-3 minutes, <100 MB.
Smoke: `python3 -I run.py --smoke` (N~4093, 20 trials, <1 s). Full: run_cmd below.

## 5. Classification: runnable_local
A stdlib toy decides the question: re-derivation of the sqrt|Aut| ceiling at toy size, with a fruitless-cycle-faithful walk.

## 6. What a run cannot establish (honest limits; results will be labelled [pcc:unverified])
- It uses Z_N logs, not elliptic curve points: no curve arithmetic, no actual canonicalisation cost kappa, no j=0/1728 curve structure. Counts steps only, not GAE cost or section 5a band pricing, no replay certificate or matched-rho validation (validation gate not met; not a leaderboard row).
- Toy N (2^16-2^20); fruitless-cycle cutoff 8 and r=32 are arbitrary; a restart rule differs from production rho (distinguished points, cycle escape).
- Cannot test the revisit condition (kappa < 1); it can only confirm or contradict the step-count ceiling. Does not touch the open special-j IC-Semaev lead.
