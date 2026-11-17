# RESULT: LEAD-ICP-pkm-tower-algebraic-oracle

## Outcome
- status: ran (single run, no retries). exit code 0, wall 7 s (run.status: exit=0 wall=7; run.py elapsed_s 6.96).
- run.stderr empty. RESULT.json is one JSON object, verdict "inconclusive".

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): median(t=1) < median(t=2) < median(t=3), all resolved within the degree cap.
Falsified if all resolved and median(t=3) <= median(t=2). Inconclusive if any median is unresolved at its cap/time limit, or the positive control fails.

Measured elimination degrees (8 reps per arm, p=101, caps 8/9/8):
- t=1: s3 [2]*8, null [2]*8, median 2
- t=2: s3 [4]*8, null [4]*8, median 4
- t=3: s3 [8]*8, null [8]*8, median 8 (equal to the cap D<=8 for t=3)
Medians strictly increase 2 < 4 < 8 in both arms. Zero spread in every arm.

Controls:
- Null: matches s3 exactly at every t (as expected).
- A/A (t=1 s3, different seed base): median gap 0.0, passed (trivially, since there is no spread).
- Positive control (planted linear 2-variable system, degree must be exactly 1): positive_control_degree = null, positive_control_ok = false. FAILED.

Verdict: inconclusive (run.py), by the preregistered rule that a failed positive control makes the run inconclusive. Controls did not all pass.

## Reading
- The toy does not test the lead's claim cleanly. The detector failed to return degree 1 on a known linear system, so the elimination-degree readout cannot be trusted as a measurement of degree. I did not debug it (rules forbid editing run.py).
- Likely explanation, not verified: the detector may need a pure last-variable row, and the planted linear system may not produce one in the way the detector tests. This is a guess.
- The data are suspiciously regular: every instance in every arm gives exactly 2^t (2, 4, 8) with no variation, and s3 and null are identical. That points to a degree set by the tower's structure, where each level squares the degree and coefficients do not matter. The surrogate equation then carries no summation-polynomial structure, so the "shape-matched null gives the same D" agreement is by construction.
- t=3 sits exactly at its cap (8). Whether it is "resolved within the cap" or merely at the boundary cannot be told from the output, and there is no t=4 point to show the trend continues.
- Discrepancies between run.py's verdict and the numbers: none. "inconclusive" follows the protocol rule. Read alone, the medians (2<4<8) would have been "consistent_with_prediction", but the failed positive control blocks that.

## Bearing on the sub-sqrt(p) question
none: with the positive control failed, the run neither confirms nor closes the recorded obstruction. The degrees doubling per depth are compatible with it, but the toy is structurally uninformative (surrogate equation, p=101, t<=3).

## Standing
[pcc:unverified], single local toy run, no independent replay.
