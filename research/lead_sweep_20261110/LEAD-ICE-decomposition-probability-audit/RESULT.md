# RESULT: LEAD-ICE-decomposition-probability-audit

## Outcome
- Status: ran to completion (single run, no retries). Exit code 0, wall 9 s (estimate was ~3 min).
- run.stderr empty. 12 curves kept (4 each at p ~ 2^12, 2^14, 2^16).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): |Stouffer z| < 3 and every per-curve |z| < 4 is
consistent_with_prediction; otherwise falsified. Positive control (interval base) needs z > 5 on
every curve; A/A Stouffer |z| must be < 3.
- Stouffer z (x-small vs null): -0.455. Per-curve z range -2.34 .. +0.93; max |z| = 2.34 (lg16, seed 20263113). All < 4.
- Positive control (interval base): all 12 curves z > 5 (min about 1262, Stouffer about 37558). Detector sees clustered sums.
- A/A: Stouffer z = 0.858 (< 3).
- Null D means 2.58 .. 2.91 (sd 0.02 .. 0.12); x-small D within that band on every curve.
- Verdict (run.py): consistent_with_prediction. Controls passed: null, positive, A/A all OK.

## Reading
- The numbers support run.py's verdict; no discrepancy between the verdict field and the data.
- The toy tests a narrow claim: the global index of dispersion of r(T) for a small-x base equals that of a random negation-closed base. By construction x(kG) is pseudorandom against the discrete log k, so a small-x base is nearly a random subset of logs. A null result is expected, and the toy has little power against structure that is not visible in the log-sum distribution.
- Dispersion is not 1 (about 2.6-2.9) because negation closure doubles the sum structure; this is the same for both arms.
- One mild outlier (z = -2.34, a deficit) is consistent with 12 draws and does not move the pooled z.
- Not tested: 28-bit fixtures, P-5 yields, charged p_solve, GNN/SR predictors, per-target predictability, Semaev solving, rho or GAE pricing (PROTOCOL.md section 6).

## Bearing on the sub-sqrt(p) question
none. The toy shows no excess target-conditional variance for small-x bases, as the conservation argument predicts, and says nothing about cost against rho.

## Standing
[pcc:unverified], single local toy run, no independent replay.
