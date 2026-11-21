# LEAD-EXO-augmented-oracle-ggm

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: closed_obstructed (last reviewed 2026-10-29; theorem 2026-11-04 per survey). Family: exotic.
- Title: Augmented oracles (x-, y-, full-point, halving, endomorphism-image, incidence) and GGM simulability.
- Next-step excerpt: empty in leads.json. Survey line: "label-valued oracles have explicit GGM simulators (endomorphism image = public scalar lambda)".
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 69-75, 153-160);
  /home/user/crypto/research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (entry only).
- Not read: the atlas YAML (encrypted), the theorem's own write-up (not found by the search). Coordinate-revealing oracles are a separate, open lead and are out of scope.

## 2. Hypothesis
In a generic group with random labels, an endomorphism-image oracle E(label) -> label(lambda*x), lambda of order 3, gives only a constant-factor gain in query count for DLP, at most about sqrt(3), once its queries are charged. Its transcript is reproduced exactly by a simulator that uses only the group-law oracle and the public scalar lambda (double-and-add). So the augmented oracle adds no generic power beyond a constant factor.

## 3. Preregistered prediction and falsification criterion
- Statistic G = mean(group-law queries of plain rho) / mean(add + E queries of lambda-orbit rho), pooled over 4 prime orders near 2^18 (N = 1 mod 3), 40 seeded trials each.
- Also: simulator identity (real-E vs simulated-E runs return the same answer and restart count on every trial) and simulator overhead S = (simulated add queries) / (real add + E queries).
- Prediction: simulator identical in 100% of trials, and 0.7 <= G <= 1.25*sqrt(3) = 2.165, and S >= 1.
- Falsified if any trial's simulated and real transcripts differ, or G > 2.165 (an unsimulated gain beyond the orbit constant). Consistent_with_prediction if all hold. Inconclusive if the controls fail.
- Positive control: a non-simulable comparison oracle on the discrete log (bisection) must give gain >= 10 over plain rho, so the detector can see a real gain.
- A/A arm: plain rho vs plain rho with another seed; the ratio of means must lie in [0.7, 1.4].
- Negative control / null: plain rho with no oracle (G = 1 by definition).

## 4. Method
- Group Z_N (N prime, N = 1 mod 3, ~2^18, primes derived from seed 20261110); labels are lazy random 60-bit injective strings; only add(l1,l2) and (arm "real") endo(l) are exposed.
- Rho: 16-way additive walk on labels with known step scalars, per-seed restarts, dictionary collision store, x recovered from collision coefficients and checked against the hidden x. Orbit arm canonicalises each state to min label of {y, lambda y, lambda^2 y}, tracking coefficients times lambda^k.
- Arms per seed: plain, real-E, sim-E (E built from add queries only), plain A/A on seed+77. Seeds: seed0 + 1000*N%99991 + trial.
- Cost: roughly 10^5 to 10^6 dictionary operations; under 1 min, under 50 MB, stdlib only. The smoke run uses N ~ 2^12 and 3 trials (<1 s).

## 5. Classification: runnable_local
A stdlib toy decides the constant-factor claim and checks simulator identity directly.

## 6. What a run cannot establish
- It is a toy GGM in Z_N with one endomorphism class, not real elliptic curves, and not other oracle types (halving, incidence, x/y-valued).
- It does not prove the general simulability theorem. It only checks one instance and the charged-cost gain.
- Identity of transcripts holds by construction if the code is right; a pass is a sanity check, not independent evidence.
- Gain bounds are statistical (small trial counts); the fixed 2.165 threshold is a choice. Results are [pcc:unverified].
