# RESULT: LEAD-EXO-elliptic-code-list-decoding

## Outcome
- Status: ran. Exit code 0. Wall 1 s (run.status: exit=0 wall=1). run.stderr empty.
- run.py verdict field: "inconclusive" (ok_ratio=false, positive_control_ok=true, gs_bound_ok=true).

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): ratio and A/A ratio in [0.7,1.3] in every cell; M_gs > k in all cells.
Falsified if any ratio > 1.5; inconclusive if positive control fails or a ratio falls in (1.3,1.5] or < 0.7.

| cell (pbits,B,k,T) | n | expected | mean | ratio | A/A ratio | var/mean | planted_min |
|---|---|---|---|---|---|---|---|
| 12,32,2,600 | 3253 | 0.152 | 0.147 | 0.962 | 1.170 | 0.95 | 1 |
| 12,64,2,600 | 2269 | 0.888 | 0.922 | 1.037 | 1.004 | 1.01 | 1 |
| 12,32,3,400 | 3343 | 1.484 | 1.498 | 1.009 | 1.060 | 0.89 | 1 |
| 14,48,2,400 | 15349 | 0.0735 | 0.045 | 0.612 | 1.225 | 1.18 | 1 |
| 14,64,3,300 | 16417 | 2.538 | 2.650 | 1.044 | 1.049 | 1.10 | 1 |

- M_gs = ceil/real sqrt(Bk) is 8.0 to 13.9, above k in all cells. ln counting bound is -40 to -85 (reported only).
- Controls: null is the ratio test itself (random targets). Positive control passed (planted min count >= 1 in all cells, EC scalar model agreed with real EC addition in all cells). A/A ratios all in [1.00,1.23], within band.
- Verdict: inconclusive, by the preregistered rule, because one cell (14,48,2) has ratio 0.612 < 0.7. No cell exceeds 1.5 (max 1.044), so the falsification criterion was not met.

## Reading
- The 0.612 is a small-count artefact. In that cell the expected total hits are 400 x 0.0735 = 29.4; observed about 18. Poisson sd is about 5.4, so the deficit is about 2.1 sd (one of 10 ratios tested; a mild look-elsewhere effect). Its own A/A ratio is 1.22, and dispersion 1.18 is Poisson-like. The band [0.7,1.3] is too tight for low-expectation cells at T=400, so the "inconclusive" is a power/design artefact rather than structure. I did not re-run or alter anything.
- The other four cells sit within 4% of the generic count with Poisson-like dispersion (0.89-1.10), as hypothesised. The count matches C(B,k)/n, so no excess weight-k words appear.
- The toy tests only the counting statistic and the M > k inequality. It implements no AG code and no Guruswami-Sudan decoder (PROTOCOL s6). It uses scalar-index arithmetic and tiny n. It does not test the lead's claim about the decoder directly.
- No discrepancy between run.py's verdict field and the numbers: the verdict follows mechanically from the 0.612.

## Bearing on the sub-sqrt(p) question
Closes or confirms the recorded obstruction (weakly): generic counts match C(B,k)/n and M_gs > k everywhere, consistent with "enumeration in disguise", though the formal verdict is inconclusive on one under-powered cell.

## Standing
[pcc:unverified], single local toy run, no independent replay.
