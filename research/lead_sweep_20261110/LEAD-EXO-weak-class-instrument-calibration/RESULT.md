# RESULT: LEAD-EXO-weak-class-instrument-calibration

## Outcome
- status: ran; exit code 0; wall 13 s (run.status: exit=0 wall=13; run.py elapsed_s 12.87)
- run.stderr empty. One run, no retries, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent if |AUC-0.5| < 0.2 and p > 0.01 in all cells, controls valid.
Falsified if any cell has |AUC-0.5| >= 0.3 and p < 0.01 with valid controls.

| p | n_anom | n_rand | AUC anom vs rand | perm p | AUC A/A | p A/A | AUC positive | p pos |
|---|---|---|---|---|---|---|---|---|
| 3001 | 36 | 72 | 0.518 | 0.824 | 0.498 | 0.968 | 1.0 | 0.001 |
| 6007 | 36 | 72 | 0.368 | 0.060 | 0.462 | 0.607 | 1.0 | 0.001 |

Mean u: p=3001 anomalous 2.385 vs random 2.352; p=6007 anomalous 2.238 vs random 2.427.
Controls: positive ok (AUC 1.0 >= 0.9 in both cells), A/A ok (both in [0.25,0.75]), null groups valid.
Verdict (run.py): consistent_with_prediction. Check against the numbers: max |AUC-0.5| = 0.132 < 0.2 and min p = 0.060 > 0.01, so the verdict matches.

## Reading
- The toy tests what the protocol says: one additive character-sum statistic on small-x DL labels, anomalous vs random prime-order curves. It does not test the lead's wider claim (small-embedding-degree, supersingular, Groebner/F4 instruments).
- Verdict field and numbers agree. No discrepancy.
- Mild point: p=6007 has AUC 0.368 (anomalous u lower) with p=0.06. That is not significant, and it is in the opposite direction to "anomalous curves look biased", but at 36 per class it is not excluded either. The prereg margin (0.2) leaves it as consistent, not a clean zero.
- The positive control (w=0.3 planted character) saturates at AUC 1.0, so the instrument can see a large planted bias. Its sensitivity to small biases is not mapped here.
- Anomalous curves were rarer (7668 and 12956 tries) than random (994 and 1443), as expected. Both primes are tiny (about 2^11.5 and 2^12.6), so this says little about cryptographic sizes.

## Bearing on the sub-sqrt(p) question
none: this reproduces the recorded blindness of one instrument at toy size, with a large planted bias detected, and offers no route to sub-sqrt(p) cost.

## Standing
[pcc:unverified], single local toy run, no independent replay.
