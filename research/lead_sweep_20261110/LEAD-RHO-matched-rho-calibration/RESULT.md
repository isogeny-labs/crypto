# RESULT: LEAD-RHO-matched-rho-calibration

## Outcome
- status: ran; wall 22 s (run.py reports 21.09 s); exit code 0 (run.status: exit=0 wall=22). Stderr empty/no errors.
- Single run, no retries, run.py/PROTOCOL.md untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): plain excess(2^20) <= 1.10 and negation excess(2^20) <= 1.10; falsified if negation excess(2^20) >= 1.30 and >= excess(2^12) - 0.05.

Measured (R = mean steps / sqrt(pi N/2); excess = R_arm / R_null):
- plain excess at 2^20: 1.007 (R 1.016 vs null 1.009). Plain R at 2^12..2^20: 1.024, 1.052, 1.040, 1.051, 1.016 (nulls ~1.00-1.01).
- negation excess at 2^20: 0.0706 (R 0.0718 vs null 1.017); at 2^12: 0.601. Negation R by size: 0.594, 0.367, 0.376, 0.167, 0.072.
- Controls: A/A |z| = 0.52 (pass); positive control 2-jump excess 9.52 at 2^20 (> 1.3, pass); null arm R ~1.0 (pass). controls_ok = true.
- run.py verdict: consistent_with_prediction (formally both <= 1.10, no falsification).

## Reading
- Plain arm: excess ~1.00-1.05 at every size; the recorded 1.92x plain toy excess is not reproduced and looks like set-up (finite-size/stat definition), consistent with the hypothesis.
- Negation arm: the number does NOT mean "no excess". An excess of 0.07 (collapsing with size, 0.60 -> 0.07) means the walk reports a "first repeated class" about 14x EARLIER than a random function. That is the signature of short fruitless cycles/2-cycles (x, min(y,p-y)) with no cycle fix: a trapped walk re-visits a class within a few steps and full-memory first-repeat detection counts that as a collision. Censoring (only 0-1.25% censored) does not capture it, because traps are detected as early repeats rather than running to the cap. The protocol's s3 assumed traps would hit the cap and bias excess upward; the opposite occurred.
- So the statistic is dominated by trivial (non-collision) repeats; it does not measure the useful collision rate of negation rho. The verdict field is arithmetically faithful to the pre-registered thresholds but the "consistent" label for negation is an artifact of a degenerate statistic, not evidence that the 1.45x toy excess is finite-size. The recorded 1.45x (which used a different, presumably cycle-handled, set-up) was not tested.
- Other caveats: one curve per size; cofactors up to 4; sizes 2^12-2^20 only; 32-jump walk; full-memory counting, not distinguished points; a 2^14 curve has r=29473 with p=29399 (cofactor 1).
- The plain-arm result is the only clean part of the test. A valid negation test needs fruitless-cycle detection/escape and counting only genuine distinct-walk collisions.

## Bearing on the sub-sqrt(p) question
none: the plain arm shows no excess over the matched null and the negation arm's statistic is degenerate (early trap repeats), so nothing about sub-sqrt(p) is learned and the recorded obstruction is neither closed nor confirmed.

## Standing
[pcc:unverified], single local toy run, no independent replay.
