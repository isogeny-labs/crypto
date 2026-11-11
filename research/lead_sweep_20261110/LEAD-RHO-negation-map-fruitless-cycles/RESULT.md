# RESULT: LEAD-RHO-negation-map-fruitless-cycles

## Outcome
- status: ran (single run, no retries). exit code 0, wall 9 s (run.status), run.py elapsed 8.78 s.
- RESULT.json is one JSON object; run.stderr empty/no errors. All collisions verified; wrong_or_capped = 0 at every size.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): pooled ratio of mean steps/sqrt(n), neg_escape over plain, bits>=18, 95% bootstrap CI (400 resamples) within [0.62, 0.85]; expected about 0.71-0.76. Falsified if CI entirely outside the band. Gates: A/A CI contains 1; birthday positive control 0.707 +-0.12; neg_noescape fail rate > 0.

Measured:
- ratio = 0.7313, CI95 [0.6880, 0.7783]: inside [0.62, 0.85]. Verdict (run.py): consistent_with_prediction.
- A/A (plain vs plain2): ratio 1.0167, CI [0.9573, 1.0868], contains 1: ok.
- Positive control (birthday, n/2 vs n states): 0.7509, within 0.707 +-0.12: ok.
- Null/no-escape arm: fail rate 0.905 (2^14), 0.972 (2^16), 0.995 (2^18), 0.996 (2^20): nonzero, as predicted.
- Per-size mean steps (plain / neg_escape): 2^14 145.2/109.2; 2^16 300.9/229.6; 2^18 653.2/464.5; 2^20 1090.4/833.1.
- Mean/sqrt(pi n/4): plain 1.44, 1.40, 1.45, 1.44; neg_escape 1.08, 1.07, 1.03, 1.10.
Controls passed: null yes, positive yes, A/A yes.

## Reading
- The toy does test the narrow claim: that escape-by-doubling negation rho gains about 1/sqrt2 over plain a-adding rho at toy sizes. It reproduces the recorded ~0.757 ratio (here 0.731) and the plain excess (~1.4 vs recorded 1.92 for plain; the constants differ, so the recorded excess is only qualitatively reproduced).
- No discrepancy between run.py's verdict field and the numbers.
- Caveats: (1) the neg_noescape mean steps (45-64) are conditional on the few walks that succeeded (0.4-9.5%), so they are survivor-biased and should not be read as a speed. (2) The no-escape arm dying in >90% of walks at toy size shows that fruitless cycles dominate there, so escape is needed, but this says nothing about cycle frequency at log2 r >= 32. (3) Group orders 258991 and 730867 are about 2^17.98 and 2^19.5; the "bits>=18" pooling includes both. (4) Pooled ratio only (no per-size CI); neg_escape/ideal sits 3-10% above 1, so the gain is slightly short of the ideal sqrt2.
- Nothing surprising beyond the extreme noescape failure rates.

## Bearing on the sub-sqrt(p) question
constant factor only: negation gives the known ~sqrt2 constant-factor speedup of rho on a prime curve; the exponent stays sqrt(n), and nothing here touches log2 r >= 32 or inversion and GAE pricing.

## Standing
[pcc:unverified], single local toy run, no independent replay. Reproduction of a known result, not an improvement; no matched-rho band (s5a), replay certificate or validation gate.
