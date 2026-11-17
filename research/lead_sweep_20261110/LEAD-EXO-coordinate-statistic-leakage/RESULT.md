# RESULT: LEAD-EXO-coordinate-statistic-leakage

## Outcome
- Status: ran to completion. Exit code 0. Wall 10 s (run.status), run.py reports 10.6 s. run.stderr empty.
- One run only, no edits to PROTOCOL.md or run.py.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): real curves exceed the null q95 no more often than 5% of curves + 2 (10 real orbits total); positive control exceeds at every size; A/A at most 1/3 above threshold. Consistent_with_prediction if so.

Measured (T = max |digit-character lag autocorr| * sqrt(m)):
| p | N | real curves | null draws | null q95 (max if <20 draws) | real > q95 | leaky control | A/A > q95 |
|---|---|---|---|---|---|---|---|
| 4099 | 4139 | 6 (1.53..2.82) | 30 | 3.16 | 0 | 5.5..7.0, all detected | 1 of 10 |
| 65537 | 65213 | 3 (2.25..2.51) | 8 | 2.90 (max) | 0 | ~23, all detected | 0 of 3 |
| 262147 | 261509 | 1 (3.63) | 3 | 2.51 (max) | 1 | 45.4, detected | 0 of 3 |

- Real exceedances: 1 of 10 (allowed: 0.5 + 2). Bound met.
- Controls: positive control detected at every size (true); A/A within bound (true); null control is the threshold source. All controls passed.
- Verdict (run.py): consistent_with_prediction.

## Reading
- The toy tests only my reconstruction of a digit-character statistic along the scalar index (PROTOCOL section 6), not IDEA-20260921-1ec106 itself.
- run.py's verdict field matches the numbers: 1 <= 2.5 bound, positive and A/A controls pass.
- Caveat: the single exceedance is the lone p~2^18 curve (T = 3.63) against a threshold from only 3 null draws (max 2.51). That threshold is very noisy. The largest null value anywhere is 3.42 (p~2^12, 30 draws), so 3.63 is only slightly above the pooled null range. With 10 lags/digits maxed per orbit, a value near 3.4-3.6 is not clearly anomalous. It is not strong evidence of leakage, but at 2^18 the instrument has not shown the real orbit is inside the null. A replication with more null draws at 2^18 would settle it cheaply (run time is seconds), but was not permitted here.
- The positive control (T 5.5 to 45) is far above the real values, so the instrument sees a slow symmetric leak easily. This does not show sensitivity to subtler or special-curve structure.
- The A/A arm had one exceedance at 2^12 (1/10, within the 1/3 allowance).
- Runtime was far below the 1-2 minute estimate. Nothing else surprising.

## Bearing on the sub-sqrt(p) question
none: toy-size digit-character autocorrelation along k is at the random level for 9 of 10 orbits and marginal for 1, which only bounds coordinate-bit bias against group structure and says nothing about index calculus or any sub-sqrt(p) algorithm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
