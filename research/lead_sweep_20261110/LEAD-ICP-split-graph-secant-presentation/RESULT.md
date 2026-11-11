# RESULT: LEAD-ICP-split-graph-secant-presentation

## Outcome
Status: ran. Wall 52 s (run.status: exit=0 wall=52). Single run, no retries. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): max over curves of |z_graph| < 3; falsified if >= 4; 3 to 4 inconclusive. Positive control must reach |z| >= 5 on every curve; A/A must have |z| <= 3.5.

Measured (z of m=2 sum coverage vs 300 random size-B bases):
- graph_boolean_offsets z: -1.76 (p=4001), -0.40 (16001), +0.75 (60013), +0.70 (250007). max |z| = 1.76.
- A/A max |z| = 1.33 (<= 3.5, pass).
- Positive control (progression) min |z| = 105.8 (>= 5, pass).
- Null control: random null valid (A/A in band).
- Other controls: hash_x z in [0.19, 1.29]; low_x z = -1.48, -3.13, -2.40, +1.25 (low_x exceeds 3 in magnitude on p=16001, slightly below random coverage, not preregistered as a decision statistic).
Verdict: consistent_with_prediction. All controls passed.

## Reading
The toy tests only the extractor-independent proxy: m=2 sum coverage of an affine-cube x-graph base equals that of a random base of equal size. Graph coverage differs from null mean by at most ~0.0006 in absolute terms (p=4001: 0.4037 vs 0.4100). run.py's verdict field matches the numbers. Mild surprise: low_x base had |z|=3.13 on one curve (coverage deficit ~0.6 percentage points), which is a control arm and does not affect the verdict; with 4 curves x 3 arms some tail is expected but 3.13 is worth noting. No test of the d+1 vs 2d+1 degree claim, Groebner/F4 cost or extraction.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: graph base gives no coverage gain over random, supporting closure of the cost-gain branch (only for this proxy; degree/extraction claims untested).

## Standing
[pcc:unverified], single local toy run, no independent replay.
