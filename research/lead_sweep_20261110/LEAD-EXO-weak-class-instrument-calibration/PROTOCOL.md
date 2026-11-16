# LEAD-EXO-weak-class-instrument-calibration

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step; family: exotic; title: empty in leads.json; source: atlas_survey_20261107 --pursuit.
- next_step_excerpt (truncated): "Done 2026-11-04 (WEAKCLASS-20261104, WEAKCLASS-I1-20261104, PAIRED-ANOMALY-20261104). No instrum..."
- Recorded finding (atlas_survey.md A6): no instrument detects anomalous or small-embedding-degree curves (36 curves/class, p<=2^14, min detectable shift ~0.13 in u at 50% power); only the zeta-stable j=0 factor base is detected (p<1e-5). README.md calls this lead a gate, not a lead.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json;
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (grep context, lines 69-75, 149-153, 198-202);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/README.md (lines 255-260). The WEAKCLASS-* run records were not located or read.

## 2. Hypothesis
The small-x discrete-log character-sum instrument (u = max_{1<=k<=K} |sum_{P in S} e(2 pi i k DL(P)/N)| / sqrt(|S|), S = points with x < p/8) cannot separate anomalous curves (#E(F_p) = p) from random prime-order curves at the same p, in a stdlib re-implementation at p ~ 3000 and 6000 with 36 curves per class. Formally, the two-sided AUC of anomalous vs random u lies within 0.5 +/- 0.2 with permutation p > 0.01 in every cell, while a planted bias of known size is detected.

## 3. Preregistered prediction and falsification criterion
- Statistic: AUC(u_anomalous > u_random) with a 1000-permutation two-sided p-value, per prime.
- Prediction (consistent): |AUC-0.5| < 0.2 and p > 0.01 in all cells, with controls valid.
- Falsified: any cell with |AUC-0.5| >= 0.3 and p < 0.01 (the instrument sees the anomalous class, contradicting the recorded blindness), controls valid.
- Inconclusive: controls fail, too few anomalous curves found, or intermediate AUC.
- Negative control / null: random prime-order curves, 2 independent groups of 36 per prime; A/A arm = group A vs group B, require AUC in [0.25, 0.75].
- Positive control: random-class curves whose small-x DL labels are replaced, with probability w = 0.3, by multiples of N//7 (planted character at k=7); require AUC vs group B >= 0.9. This fixes the instrument's detectable scale; it is not a real weak class.
- Decides: consistent means the "nulls bound only what the instrument sees" caveat stands for this instrument; falsified means the recorded blindness is an artefact of the earlier setup.

## 4. Method
- Python 3 stdlib. Primes p = next_prime(3000), next_prime(6000). Curves y^2 = x^3+ax+b, (a,b) uniform, nonsingular; #E by Legendre-table sum, O(p).
- Anomalous class: #E = p. Random class: #E prime, != p, > 50. Rejection sampling, max 60000 tries per class.
- For each accepted curve: random point P (prime order N), enumerate kP for k=1..N-1 with affine arithmetic, record k for points with x < p//8 (the DL labels); K = 48 frequencies.
- Seeds: random.Random(20261110 + p) per prime, one stream per prime, classes drawn in the order anomalous, random(72), planted control(36). Permutation seeds come from the same stream.
- Cost estimate: roughly 2 to 6 minutes, under 50 MB, one core; smoke finishes in under 1 s.
- Output: one JSON object with cells, controls and verdict.

## 5. Classification: runnable_local
Pure stdlib toy at group orders ~2^11.5..2^13; no network, no builds.

## 6. What a run cannot establish
- Only one instrument (additive character sum) on one weak class (anomalous). Small-embedding-degree and supersingular classes, and the Groebner/F4 instruments, are not tested.
- The labels use full DL enumeration, which already gives anomalous curves no special help; the result says nothing about attacks (Smart lift) that do detect them.
- K=48 depends on the generator choice; both classes treated identically but not invariant. Anomalous curves at p~6000 are few and may be special (Weil-restriction-free but CM-structured); 36 per class gives low power for small effects.
- It does not reproduce WEAKCLASS-20261104, whose records were not read. Results are labelled [pcc:unverified].
