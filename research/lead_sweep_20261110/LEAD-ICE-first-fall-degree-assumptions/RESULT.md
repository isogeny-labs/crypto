# RESULT: LEAD-ICE-first-fall-degree-assumptions

## Outcome
- Status: ran to completion. exit=0, wall=1 s by shell (run.py reports wall_s=1.48). run.stderr empty.
- Single run, no retries, run.py and PROTOCOL.md untouched.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): consistent_with_prediction if delta <= -1.0 at every n with controls valid; falsified if delta > -0.5 at every n; otherwise inconclusive.

Measured (mean D_ff, censored at 6):
| n | count | structured | null | A/A null2 | planted | delta_Dff | AA gap | delta_Dsolve |
|---|---|---|---|---|---|---|---|---|
| 6 | 30 | 2.667 | 3.600 | 3.100 | 2.0 | -0.933 | 0.500 | -0.067 |
| 8 | 30 | 2.567 | 4.867 | 4.800 | 2.0 | -2.300 | 0.067 | +0.133 |
| 10 | 8 | 3.000 | 5.000 | 4.750 | 2.0 | -2.000 | 0.250 | +1.000 |

Controls: positive control passed (planted D_ff = 2.0 <= 3 at all n). A/A passed (max gap 0.5, exactly at the <=0.5 limit at n=6). Null control is the random arm itself.
Verdict: inconclusive (run.py). n=6 gives delta = -0.933, which misses the -1.0 threshold, and is not > -0.5, so neither branch is met.

## Reading
- The numbers support run.py's verdict field; no discrepancy.
- n=6 is marginal: A/A gap is at the limit (0.5), and the null D_ff itself shifted by 0.5 between the two null seed sets. With 30 instances the n=6 delta is within noise of -1.0. At n=8 and 10 the drop is large (-2.3, -2.0).
- The D_ff drop does not carry over to D_solve (delta_Dsolve about 0 or positive). Structured systems are not easier to solve at these degrees, only to hit a rank defect earlier. This points to structure or sparsity in the structured equations causing early rank deficiency, not a genuine lowered solving degree.
- Null D_ff saturates near the censor (4.9 to 5.0 at n=8, 10), so the delta there is partly a ceiling effect of the proxy.
- Caveats from the protocol apply: S_3 written from memory and unverified; the null is not sparsity-matched; n'=n/2 is not the Semaev/Petit-Quisquater regime; the D_ff proxy is not the Dinur-Kosters-Hodges first fall degree; n=10 has only 8 instances.
- The toy tests the weak structured claim at tiny scale, not an Assumption-1 statement.

## Bearing on the sub-sqrt(p) question
none. Toy at n<=10, D<=5, with an unverified S_3 and an unmatched null; D_solve shows no improvement, and nothing here speaks to asymptotic behavior or to prime-field ECDLP.

## Standing
[pcc:unverified], single local toy run, no independent replay.
