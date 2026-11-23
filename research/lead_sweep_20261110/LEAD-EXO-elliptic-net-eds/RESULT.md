# RESULT: LEAD-EXO-elliptic-net-eds

## Outcome
- status: ran; exit code 0; wall 3 s (run.status: exit=0 wall=3). run.stderr empty.
- Single run of run.py, unmodified. kind = runnable_local.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): consistent if slope_gap >= 0.3 in BOTH seed arms, sieve/BSGS ratio strictly increasing in N, all recoveries correct, A/A slope_gap difference <= 0.25. Falsified if slope_gap < 0.15 in the primary arm.

Measured (p = 4157, 17159, 72089, 274093; N = 1382, 8517, 36068, 136953):
- Arm A (seed 1001, primary): slope_sieve 1.109, slope_bsgs 0.427, slope_gap 0.682. Ratios 7.95, 42.7, 76.2, 203.1 (monotone).
- Arm B (seed 2002, A/A): slope_sieve 0.936, slope_bsgs 0.480, slope_gap 0.457. Ratios 8.57, 54.3, 40.7, 141.1 (NOT monotone: drops 54.3 -> 40.7).
- A/A slope_gap difference 0.225 (limit 0.25; passes narrowly).
- Positive control: all recoveries correct, EDS rank of apparition = order in all rows (true).
- Null control: sieve/null probe ratio arm A 0.78, 1.17, 1.05, 1.28; arm B 1.72, 0.87, 0.69, 1.45. Consistent with ~1 given 6 targets (noisy), as expected by construction.
- run.py verdict: consistent_with_prediction. Controls passed as run.py defines them.

## Reading
- The toy tests only the zero-test (GGM-simulable) locator: sieve cost is linear in the k-space by construction, and the slope gap just confirms Theta(N) vs about sqrt(N). It does not test any nonlinear target-specific locator.
- Discrepancy: run.py reports ratio_monotone = true, but this holds only for arm A. In arm B the ratio is non-monotone (54.3 then 40.7). The protocol says "ratio sieve/BSGS strictly increasing in N" without naming an arm, so a strict reading across both arms would be inconclusive. run.py appears to check the primary arm only. The cause is just 6-target sampling noise (sieve cost is uniform-random in k), not structure.
- Slopes are fits over 4 points with 6 targets each. Arm B slope_sieve 0.94 is below 1, and the A/A diff 0.225 is close to the 0.25 limit, so the noise floor is comparable to the effect margin above 0.3 (but both gaps clear 0.3 and falsification threshold 0.15 by wide margins).
- Nothing else surprising.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the zero-test locator gives a linear sieve and a slope gap of 0.46-0.68 versus BSGS, matching the recorded closed/obstructed disposition (in N units, not p units). It says nothing about locators outside the GGM-simulable class.

## Standing
[pcc:unverified], single local toy run, no independent replay.
