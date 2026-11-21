# RESULT: LEAD-TRF-miller-cocycle-nonhom-transfer

## Outcome
- Status: ran to completion. Exit code 0. Wall 3 s (run.py elapsed_s 3.76). run.stderr empty.
- RESULT.json verdict: consistent_with_prediction. Positive control detected: true. A/A within null: true.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "T_real <= 99th percentile of the null for every curve (consistent_with_prediction; lead closes as 'hash')". Falsified only if a curve has T_real > null max AND T_real > 5.

| p | q used | T_real | null q99 | null max | T_planted | A/A |
|---|--------|--------|----------|----------|-----------|-----|
| 8443 | 3,7,9 | 0.316 | 4.07 | 6.60 | 4140 | 2.42 |
| 7919 | 2 | -0.040 | 4.63 | 8.03 | 1327 | 0.16 |
| 6991 | 3,5 | 2.370 | 4.34 | 5.29 | 2252 | 3.20 |
| 6803 | 2 | 0.198 | 3.94 | 5.00 | 1140 | 0.77 |
| 8123 | 2 | 0.609 | 3.96 | 9.74 | 2813 | 2.68 |
| 9479 | 7 | 1.544 | 3.52 | 3.84 | 3930 | 1.68 |

All six T_real are below the null q99. Controls: null (150 uniform sequences per curve) ran; positive control exceeded null max and 5 on every curve (T 1140 to 4140); A/A inside null range on all curves. Verdict: consistent_with_prediction, controls passed.

## Reading
- The toy tests an assumed instantiation (norm of the normalised Miller function at a twist point S), not necessarily the lead's U (IDEA-20261017-e3a6f1 was not found). The lead's claim is only tested through marginal-residue, increment and doubling-defect chi-squares at tiny p (6803 to 9479).
- Discrepancy/caveat: marg_q2 is exactly -0.707 (chi2 = 0, perfectly balanced parity) on all three curves with q=2 (p = 7919, 6803, 8123), and dbl_q2 is also exactly -0.707 at p = 7919. A uniform i.i.d. sequence is exactly balanced only with probability about 1/sqrt(n), so three of three is a structural signature (plausibly a k <-> N-k symmetry). T is a max of one-sided z values, so this low-tail anomaly is invisible to the preregistered statistic; the null's chi2 = 0 frequency was not examined.
- Many curves have only one small q (p-1 mostly 2 times a large prime), so few tests ran; the test power for structure at other moduli is low.
- No other discrepancy between the verdict field and the numbers.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy size the map behaves like a hash on the preregistered tests, with only the parity exact-balance oddity noted, which gives no usable discrete-log structure.

## Standing
[pcc:unverified], single local toy run, no independent replay.
