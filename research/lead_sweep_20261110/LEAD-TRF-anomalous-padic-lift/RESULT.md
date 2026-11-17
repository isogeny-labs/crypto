# RESULT: LEAD-TRF-anomalous-padic-lift

## Outcome
- status: ran; exit code 0; wall 64 s (run.status: exit=0 wall=64; run.py wall_s 63.68).
- run.stderr empty. One run only, no retries.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): A and A' success >= 0.95 each and |A-A'| <= 0.10; null pooled successes <= 5; Smart ops / rho_ref decreasing with bit size. Falsified if a positive arm < 0.95 or null > 5; inconclusive if degenerate > 20% or arms differ by > 0.10.

Measured (25 instances per arm per size, 6 sizes, 12..22 bits):
- A: 150/150 = 1.0. A': 150/150 = 1.0. |A-A'| = 0.
- Null (non-anomalous, n = r): 0/150 successes (expected by chance about 0.007 total).
- Degenerate instances: 0.
- Smart ops / rho_ref by size (12..22 bits): 0.379, 0.202, 0.128, 0.066, 0.041, 0.025, monotone decreasing.
- Smart ops per instance: 36, 42, 48, 58, 72, 72 (depends on the bit pattern of p).

Verdict: consistent_with_prediction. Controls: positive (anomalous arm) passed, null passed, A/A passed.

## Reading
- The toy tests the lead's stated claim directly: the Smart lift recovers d whenever #E = p and fails at chance on generic curves. run.py's verdict field matches the numbers; no discrepancy.
- Caveat: the ops/rho ratio counts only the lift (Jacobian operations over Z/p^2). It excludes the anomalous-curve search, which the run reports as 700 to 17000 random tries per instance. Each try costs a scalar multiplication by p, and the tries grow roughly like sqrt(p) (rough scale: 16674 tries at 22 bits against a rho reference of 2868 ops). Generating an anomalous curve at random therefore costs more than rho on that curve. The lift is cheap only if the curve is already anomalous, which is the recorded obstruction.
- Ops are over Z/p^2, a constant factor more expensive than F_p operations.
- Nothing surprising. The 72 ops at both 20 and 22 bits is a popcount effect of the particular primes drawn, not a bug.
- The null uses the same code with n = r, which is a generic map. Other lifts are not tested.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the lift is polynomial in log p on anomalous curves and gives chance-level results on generic curves, so it does not help for non-anomalous groups. This re-derives the known Smart/Satoh-Araki/Semaev closure.

## Standing
[pcc:unverified], single local toy run, no independent replay.
