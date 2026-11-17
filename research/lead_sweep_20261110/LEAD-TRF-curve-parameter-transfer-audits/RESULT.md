## Outcome
Status: ran. Exit code 0, wall 1 s. Single run, no retries. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): pooled z=(hits-expected)/sqrt(expected) <= 3; positive control flagged 100%; A/A |z| <= 3. z > 3 falsifies.
Measured: pooled z = -0.0054 (hits 238+236 = 474, expected 247.07+227.05 = 474.12). Anomalous (#E=p): 5 and 4 (counted separately).
A/A z = 0.092. Positive control 64/64 flagged.
Verdict: consistent_with_prediction. Controls passed: null (exact expectation), positive, A/A.

## Reading
The toy tests the narrow heuristic: p mod r behaves like a uniform element for small-k hits among random curves at p ~ 2^11. Observed hits match expectation almost exactly (z near 0). run.py's verdict field matches the numbers. Caveats: tiny p and orders, limited power, heuristic null; the 9 anomalous curves are not compared with an expectation. Nothing about deployed curves, twists, descent or seed provenance is tested.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: random curves show no excess of small embedding degree beyond chance at this scale, so no revisit trigger arises.

## Standing
[pcc:unverified], single local toy run, no independent replay.
