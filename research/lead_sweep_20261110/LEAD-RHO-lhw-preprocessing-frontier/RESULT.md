# RESULT: LEAD-RHO-lhw-preprocessing-frontier

## Outcome
- status: ran (single run of run.py, no retries). exit code 0, wall 12 s (run.status: exit=0 wall=12). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): chi2 <= 9 + 4*sqrt(18) (about 26.0) in main and A/A arms; negative control (600 weight w+2 keys) recovers 0; positive control (S=L, T=R) recovers at p_split = 0.3183 within 4 sigma; zero wrong recoveries. Both under threshold = consistent_with_prediction.

Measured:
- chi2 main (seed 20261110) = 3.44; chi2 A/A (seed 777) = 8.81; threshold 25.97; dof 9.
- Largest |z|: main 1.08 (cell S=T=1365), A/A 2.43 (cell S=341, T=682).
- Positive control: 497/1500 = 0.331 in the full cell of the main arm (pred 0.3183, z=1.08); the run reports positive_control_rate 0.3313, ok = true.
- Negative control hits = 0. Wrong recoveries = 0 in all cells, both arms.
- Verdict (run.py): consistent_with_prediction. Controls: null PASS, positive PASS, A/A PASS.

## Reading
- run.py's verdict field matches the numbers: both chi2 values are far under the threshold, and the controls pass.
- The main-arm chi2 of 3.44 on 9 dof is on the low side (lower-tail probability roughly 6 percent), not alarming for one run. A/A at 8.81 is near the mean. Neither suggests a problem.
- The toy tests only the product law eps = p_split*(S/L)*(T/R) for random-subset advice and random-subset scan. That is close to an identity for a uniform key and an independent table, so a pass confirms the implementation and accounting rather than a new frontier point.
- Not tested: the "weighted" incumbent row (text of IDEA-20260930-79abd9 was not identified), optimised advice tables, preprocessing cost P, storage cost U, and any section 5a pricing. The T/rho column is descriptive only (T/rho_ref between 0.008 and 0.033 per cell, at success 0.02 to 0.32); that is not a comparison with matched rho.
- Toy size only (q about 2^31). Nothing surprising observed.

## Bearing on the sub-sqrt(p) question
none: this confirms an accounting identity for random advice at toy scale, with no evidence about any sub-sqrt(p) algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
