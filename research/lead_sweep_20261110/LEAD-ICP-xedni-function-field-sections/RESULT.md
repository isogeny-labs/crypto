# RESULT: LEAD-ICP-xedni-function-field-sections

## Outcome
- status: ran (single run, no retries). exit code 0, wall 25 s (run.status: exit=0 wall=25).
- stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): alpha_hat >= 0.9 consistent; falsified if alpha_hat < 0.5; 0.5 to 0.9 inconclusive. Null arm alpha expected not smaller than main arm; positive control must flag 1000/1000.
- Measured main alpha_hat = 0.8875 (primes 127..2039, 600k samples each; hits 3060, 1849, 993, 523, 268; all rows had >= 5 hits).
- Null arm alpha = 1.0566 (>= main, as expected; hits 712, 275, 143, 68, 37).
- Positive control detect rate = 1.0 (controls passed).
- A/A: not run (stated limitation in protocol).
- Verdict: inconclusive (0.8875 is just below the 0.9 threshold, within the 0.5 to 0.9 band). Controls passed.

## Reading
- run.py verdict "inconclusive" matches the numbers and the preregistered rule.
- The estimate is near-miss for 0.9, and far above the 0.5 falsification line, so unstructured lifts do not reach the < 1/2 exponent the lead needs. The slight shortfall from 0.9 is plausible small-p and box-size effect (the probability is roughly 1/p-ish scaling times lift-box effects). Hit counts at large p are a few hundred, so alpha_hat has sampling uncertainty of a few hundredths (not computed here).
- The toy does not test the lead's claim: it concerns integer lifts of collinear triples, not elliptic surfaces over F_p(t), Mordell-Weil sections or dimension surplus (PROTOCOL s6).

## Bearing on the sub-sqrt(p) question
none: classical unstructured-lift proxy gives alpha near 0.9 to 1, well above 1/2, but does not touch the function-field family, so the recorded obstruction is neither closed nor confirmed.

## Standing
[pcc:unverified], single local toy run, no independent replay.
