# RESULT: LEAD-ICP-gfp5-rho-reference-audit

## Outcome
- status: ran (single run, no retries). exit code 0. wall 7 s (run.status: exit=0 wall=7).
- run.stderr empty. run.py verdict field: inconclusive.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): |mean(ratio r=20, arms A and B) - 1| <= 0.10 at every size (log2N 12, 16, 20); null within 0.15 of 1; A/A |A-B| <= 0.15; r=2 > 1.15x the r=20 ratio. Predicted verdict: consistent_with_prediction.

Measured (300 trials per size):
| log2N | r20 A | r20 B | mean r20 | null | r=2 |
|---|---|---|---|---|---|
| 12 | 1.241 | 1.264 | 1.253 | 0.982 | 7.19 |
| 16 | 1.442 | 1.470 | 1.456 | 0.968 | 30.42 |
| 20 | 1.871 | 1.614 | 1.743 | 0.935 | 70.75 |

- r=20 criterion: violated at all three sizes (excess 0.25, 0.46, 0.74).
- Null control: passed at every size (|null-1| <= 0.065).
- Positive control: passed (r=2 far above 1.15x r=20).
- A/A: failed at log2N=20 (|A-B| = 0.257 > 0.15); passed at 12 (0.023) and 16 (0.028).
- Controls not all valid, so the preregistered rule gives inconclusive, not falsified.

## Reading
- The JSON flag "null_ok": false is misnamed. In run.py, `ok` combines the r=20 prediction test with the null test, so it is false because the r=20 ratios exceed 1.10. The null itself passed at every size.
- The A/A failure at 2^20 is real: two seeds differ by 0.26. With 300 trials and a heavy-tailed first-revisit time, the spread is plausible, but the toy does not distinguish noise from a defect at that size.
- The r=20 excess grows monotonically with N in both arms (about 1.25, 1.46, 1.74 mean). It exceeds the 10% band even at 2^12, where the A/A agreement is tight. Taken at face value this is a mixing/design defect of this particular toy walk (the hash h(x)=(x*2654435761)>>8 mod 20 on a small additive group, with first-repeat counted via a visited set, so a walk that falls into a short cycle early inflates the count). It is not evidence about the matched-rho ruler used on real curves.
- The toy does not test the lead's claim (EcGFp5/EcMasFp5 rho cost, inversion paths, multi-instance, interval kangaroo). It is a generic Z_N mixing check only.
- The verdict by the preregistered rule is inconclusive. The numbers suggest the prediction would have been falsified had the A/A control held, but I do not upgrade the verdict beyond what run.py and the protocol rule give. I did not edit run.py or the protocol.

## Bearing on the sub-sqrt(p) question
none: a generic toy-group walk whose own A/A control failed at the largest size says nothing about F_{p^5} rho cost or any sub-sqrt(p) route; the closed_known disposition is unchanged.

## Standing
[pcc:unverified], single local toy run, no independent replay.
