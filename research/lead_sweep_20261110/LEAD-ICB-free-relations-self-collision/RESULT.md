# RESULT: LEAD-ICB-free-relations-self-collision

## Outcome
- status: ran; exit code 0; wall 8 s (run.status: exit=0 wall=8). One run, no retries. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md sec 3): random FB has |R-1| < 3 SE + 0.05 and Y mean + 2 SE < 1 at each size (12 replicates); AP positive control R > 3; A/A |R1-R2| < 3 SE-combined + 0.02. Falsified only if random R excludes 1 or Y - 2 SE > 1 with controls passing.

Measured (mean, SE):
| log2 p | B | R_random | R_AA | R_AP (pos) | Y |
|---|---|---|---|---|---|
| 12 | 73 | 1.0012 (0.0144) | 0.9929 (0.0210) | 23.6 (0.16) | 0.781 (0.0135) |
| 14 | 155 | 0.9975 (0.0079) | 0.9996 (0.0079) | 50.4 (0.11) | 0.786 (0.0076) |
| 16 | 283 | 0.9990 (0.0044) | 0.9969 (0.0040) | 93.0 (0.18) | 0.790 (0.0040) |
| 17 | 393 | 0.9982 (0.0042) | 1.0085 (0.0043) | 129.8 (0.13) | 0.791 (0.0042) |

Verdict: consistent_with_prediction. Null, A/A and positive control all passed at all four sizes; Y < 1 at all sizes.

## Reading
- The toy tests the model-level birthday claim: random-FB self-collisions match the uniform-draw null (R ~ 1), and the AP control shows the detector sees structure. The A/A failure in the smoke test (4 replicates) did not recur at 12 replicates.
- run.py's verdict field matches the numbers; no discrepancy found.
- Y is about 0.78-0.79, above the hypothesis text's "about 0.5" but well below 1. The 0.5 figure was only a rough expectation, not a preregistered threshold; the threshold Y < 1 held. Y rises slightly with size.
- Limits (PROTOCOL sec 6): prime-field toy, shortcut group arithmetic, tiny sizes. It does not cover binary fields, the wide path (n > 63), production duplicate-bucket accounting, rule-v4 cold costs, or structured tables.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: random pair-sum tables give no excess free relations (R ~ 1, Y ~ 0.79 < 1), consistent with the recorded 0.960x harvest.

## Standing
[pcc:unverified], single local toy run, no independent replay.
