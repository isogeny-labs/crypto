# RESULT: LEAD-RHO-pool-class-quotient

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- Single run, no retries. PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): bootstrap 95% CI of mean-steps ratio neg/plain inside [0.66, 0.80] for all three sizes; A/A CI contains 1. Falsified if any CI lies wholly outside the band; inconclusive if the A/A check fails.

| n | neg/plain | CI95 | A/A | A/A CI95 | half-group ref | traps/trial |
|---|---|---|---|---|---|---|
| 65521 | 1.253 | [1.167,1.345] | 1.045 | [0.980,1.117] | 1.036 | 10.7 |
| 262139 | 0.691 | [0.635,0.744] | 1.766 | [1.614,1.915] | 0.608 | 28.1 |
| 1048573 | 0.571 | [0.527,0.617] | 1.069 | [0.999,1.136] | 0.740 | 106.2 |

- run.py verdict: "falsified" (n=65521 CI wholly above band; n=1048573 CI wholly below band).
- Controls: null/A/A FAILED at n=262139 (ratio 1.77, CI excludes 1) and is borderline at 1048573 (lower bound 0.9987). Positive control (half-order plain walk, ideal ~0.707) FAILED at all sizes (1.036, 0.608, 0.740 vs 0.707, and not consistent with each other). Controls passed: no.

## Reading
- The measured numbers do not support the lead's 0.70 claim, but they do not cleanly refute it either. The A/A arm gives a ratio of 1.77 at n=262139 with identical algorithms, so the instrument noise (between jump-table/seed draws) is far larger than the effect being measured. Under the protocol's own rule (A/A fails -> inconclusive) the honest reading is inconclusive; run.py's "falsified" verdict ignores the control failure.
- Likely cause (inference, not tested): with only 32 jumps drawn per arm and one fixed jump table per arm, mean steps depend heavily on the particular table (plain_over_sqrt_n varies 1.38, 1.89, 2.36 across sizes, far from the usual ~1.25 sqrt(n) scale), so the arm-to-arm ratio is dominated by table/seed variance, and trap counts (10 to 106 per trial) show the self-repeat restart rule is firing often and distorting step counts. The ratio is not a clean test of the sqrt2 quotient.
- No monotone trend that would indicate a real effect: ratio 1.25 -> 0.69 -> 0.57 mostly reflects noise plus trap growth with n.
- The toy is Z_n, not EC arithmetic, and does not test the pool v2 walk, PR #56 or Frobenius; step counts only.

## Bearing on the sub-sqrt(p) question
none: constant-factor question on a toy whose controls failed; no exponent information and no usable constant-factor measurement.

## Standing
[pcc:unverified], single local toy run, no independent replay.
