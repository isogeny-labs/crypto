# RESULT: LEAD-STR-torsion-module-leak

## Outcome
- status: ran; exit code 0; wall 1 s (run.status: exit=0 wall=1); run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): count of k with nontrivial delta(kG) = 0 exactly; |z| < 4 for the Legendre control and the A/A arm; positive control accuracy 1.0; consistent if all hold, falsified by any nontrivial delta(kG).

Measured (seed 20261110, 40 curves, k up to 1500):
- nontrivial 2-descent count: 0 of 54214 samples
- positive control (point of order 2N) accuracy: 1.0 over 54214 samples
- negative control chi(x(kG)) vs k parity: z = 0.198
- A/A random labels: z = 0.490
- run.py verdict: consistent_with_prediction. Controls (null, positive, A/A) all passed.

## Reading
The toy tests the recorded l=2 obstruction directly: the homomorphism delta kills odd-order points, and the positive control shows the detector would see a leak if one existed. The numbers match run.py's verdict field; no discrepancy. Nothing surprising. The result is expected from the theory (a re-derivation), not new information. Scope limits per PROTOCOL.md s6: only l=2, full rational 2-torsion, N prime, tiny sizes.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the l=2 descent statistic is identically trivial on the order-N subgroup, so it carries no information on k.

## Standing
[pcc:unverified], single local toy run, no independent replay.
