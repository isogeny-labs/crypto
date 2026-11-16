# RESULT: LEAD-ICB-quasi-subfield-polynomials

## Outcome
- status: ran (single run, no retries); exit code 0; wall 11 s (run.status: exit=0 wall=11).
- stderr empty. RESULT.json is one JSON object, verdict "consistent_with_prediction".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): max <= 3 in every cell; mean in [0.7,1.3]; A/A means differ < 0.35.
Falsified if any f has > 3 roots at prime n; inconclusive if A/A gap >= 0.35 or positive control fails.

Measured (N=1500 QSP per cell, x^(2^k+1)+ax+b):
| n | k | QSP mean | QSP max | A/A halves | gap | QSP hist | dense null mean/max |
|---|---|---|---|---|---|---|---|
| 11 | 2 | 1.0333 | 3 | 1.0413/1.0253 | 0.016 | 0:472 1:767 3:261 | 0.938 / 5 |
| 11 | 3 | 1.0073 | 3 | 0.9960/1.0187 | 0.023 | 0:505 1:737 3:258 | 0.990 / 4 |
| 13 | 2 | 0.9860 | 3 | 0.9653/1.0067 | 0.041 | 0:519 1:732 3:249 | 0.942 / 5 |
| 13 | 3 | 1.0100 | 3 | 1.0440/0.9760 | 0.068 | 0:489 1:759 3:252 | 0.940 / 5 |
Positive control (n=12, k=2, 3000 samples): max 5 (46 hits), hist 0:1213 1:734 2:1007 5:46.

Verdict: consistent_with_prediction. Controls: positive control passed (reached 5 roots);
A/A passed (all gaps < 0.07); null produced (dense polynomials, N/3 per cell; note it includes
roots counts 2, 4, 5, so the counter distinguishes them). Null control as a falsifier: no QSP had > 3 roots.

## Reading
- The toy checks the Bluher root-count ceiling for one sparse trinomial family at n=11,13. It does
  not test correspondence-defined or non-sparse QSP families, ECC2K-130 sizes, or any relation cost.
- run.py's verdict field matches the numbers.
- Discrepancy with the protocol wording: PROTOCOL.md says the QSP root-count distribution "matches that of
  random same-degree polynomials". Only the means match (about 1). The histograms differ in shape: QSP
  gives only {0,1,3} roots (the Bluher pattern, ~17% with 3 roots), never 2, while dense random polynomials
  show 2, 4 and 5 roots. This is expected from the theory, not a defect, but the shape claim was
  not what the preregistered statistic tested (it only tested the mean).
- The dense null max of 5 > 3 does not bear on the falsifier, which applies to QSPs only.
- Nothing surprising; a theorem was re-confirmed, so this is an implementation sanity check.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at prime n the sparse QSP root count stays <= 3, far below the 2^k needed, so this family gives no subspace-like factor base (toy sizes only).

## Standing
[pcc:unverified], single local toy run, no independent replay.
