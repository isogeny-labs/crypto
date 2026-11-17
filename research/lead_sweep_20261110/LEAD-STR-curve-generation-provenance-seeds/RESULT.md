# RESULT: LEAD-STR-curve-generation-provenance-seeds

## Outcome
- status: ran; wall 221 s; exit code 0 (run.status: exit=0 wall=221). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): |z| < 3.3 on all 5 screens for G (n=2000) vs A (n=3000), and |mean_trials/64 - 1| < 0.15; A/A within |z|<3.3; positive control W fails not_anomalous on 100% while A passes >90%.

Measured:
- z G vs A: order_prime 1.62, not_anomalous -1.94, embedding_deg_gt_30 0.09, twist_large_factor -1.39, order_large_factor 2.45 (max |z| 2.45).
- z A vs A2: -0.82, 1.94, -0.81, -0.21, -0.61 (max 1.94).
- mean grind trials 63.33 (ratio 0.9895, deviation 1.1%).
- Positive control: W not_anomalous pass 0/150 (100% fail); A passes 2997/3000 = 99.9%.
- Mean trials to anomalous curve: 756.3.

Verdict: consistent_with_prediction. Controls passed: null ok, A/A ok, positive ok, trial cost ok.

## Reading
The toy tests only what it was built to: a predicate independent of the curve (keyed hash of the seed) cannot be seen by five public screens, and grinding cost is geometric with mean 2^6. The result is near-guaranteed by construction. run.py's verdict field agrees with the numbers. Caveats: order_large_factor G-vs-A z=2.45 is the largest, below 3.3 but not trivial; with 5 screens that is unremarkable, and the sibling A/A reached 1.94 on not_anomalous, so the noise floor is comparable. Nothing surprising. It says nothing about real NIST/Brainpool seeds, the 2^-30 density, or p at 256 bits.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: it reproduces, at toy scale, that public screens are blind to an independent hidden class, so a clean screen bounds only visible weaknesses.

## Standing
[pcc:unverified], single local toy run, no independent replay.
