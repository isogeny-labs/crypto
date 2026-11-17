# RESULT: LEAD-RHO-correlated-walks-shared-sums

## Outcome
- Status: ran. Exit code 0. Wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- Single run of run.py, no retries, PROTOCOL.md and run.py untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): "B merge <= 0.15 at every n; C/A256 in [0.95,1.10] at every n. Both hold: consistent_with_prediction. Either fails at any n (given controls pass): falsified. Controls fail: inconclusive."

Measured (300 trials/arm/size):
| n | merge_B | merge_A16 | C/A256 | A/A ratio | A16 steps/sqrt(n) | pos. ctrl |
|---|---|---|---|---|---|---|
| 2^14 | 0.0033 | 1.0 | 0.990 | 0.917 | 1.47 | ok |
| 2^18 | 0.0 | 1.0 | 1.054 | 0.951 | 5.39 | FAIL |
| 2^20 | 0.0 | 1.0 | 0.914 | 0.832 | 9.30 | FAIL |

- B merge fraction <= 0.15 at all n (criterion met).
- C/A256 in [0.95,1.10] at 2^14 and 2^18, but 0.914 at 2^20 (outside; mean steps 1641 vs 1796, SEs about 71 and 74, so roughly 1.5 SE of the difference).
- Controls: null control (B merge ~0, A16 merge 1.0) behaves as designed. Positive control FAILS at 2^18 and 2^20: A16 steps/sqrt(n) is 5.4 and 9.3, far above the [1.0,2.0] birthday band. A/A ratio is outside [0.9,1.1] at 2^20 (0.832).
- Verdict (run.py): inconclusive. controls_ok = false.

## Reading
- The toy does test the merge sub-claim (H-B) directly: shared-index walks essentially never merge (1 of 900 at 2^14, 0 at larger n) while point-function walks always do. This matches the recorded obstruction (no DP detection with shared index sequence) in this toy, but that part is supported mainly by the controls that passed (A16 merge 1.0).
- The positive-control failure is a property of the 16-jump walk: its cost to first collision scales far worse than sqrt(n) at larger n (5x to 9x), most plausibly because 16 jumps give a small effective jump set with additive-walk structure in Z_n (walks collide late, short-range structure), so the preregistered birthday-scale band was too narrow for A16. This is a calibration issue of the preregistration, not evidence about the lead. A256 and B sit near 1.3 to 1.8 sqrt(n).
- The A/A ratio outside band at 2^20 reflects high variance (SE about 6%), with heavy-tailed A16 step counts; ratio of two noisy means.
- The H-C result (C/A256 0.99, 1.05, 0.91) is mixed in direction and compatible with 1.0 within noise; the 2^20 miss is not a clear loss of collision rate (C is faster, not slower, than A256). No conclusion should be drawn since controls failed.
- Surprising: B (shared index sequence) found collisions faster than A256 at 2^20 (1368 vs 1796); B is just collisions between same-time positions across walks with equal jumps, not point-function merging, so this is not a usable rho.

## Bearing on the sub-sqrt(p) question
none: toy Z_n, controls failed (inconclusive), and no addition saving is claimed; at most it is consistent with the recorded obstruction on merging, without establishing anything about subexponential cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
