# RESULT: LEAD-ICB-frobenius-orbit-factor-base

## Outcome
- Status: ran. run.py exit code 0, wall 18 s (run.status: exit=0 wall=18). stderr empty. One run only.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): the 2-SE interval of R = coverage(orbit)/coverage(random) lies inside [0.95, 1.05] for every m in {11,13,17}; A/A within 2 SE of 1; progression positive control ratio < 0.5.

Measured (200 draws each):
| m | #E | B | R | SE | 2-SE interval (run.py ci95) | A/A | A/A SE | pos. ctrl ratio |
|---|----|---|---|----|-----------------------------|-----|--------|-----------------|
| 11 | 1982 | 33 | 0.9825 | 0.0049 | [0.973, 0.992] | 1.0019 | 0.0013 | 0.131 |
| 13 | 8374 | 39 | 0.9877 | 0.0031 | [0.981, 0.994] | 1.0002 | 0.0007 | 0.103 |
| 17 | 131174 | 187 | 0.9991 | 0.0006 | [0.998, 1.000] | 0.9998 | 0.0002 | 0.023 |

Verdict (run.py): consistent_with_prediction. All three intervals lie inside the band. Controls: A/A passed at all m (m=11: |0.0019| <= 2*0.0013); positive control passed at all m (ratios 0.02-0.13 < 0.5); the null arm is the random arm. Controls ok.

## Reading
- The toy tests the narrow claim: 2-sum coverage of a Frobenius-orbit-closed base equals that of a random same-size base within 5%. It does so at toy sizes, with a proxy (2-sums only).
- run.py's verdict matches the numbers. No discrepancy.
- Mild observation: at m=11 and m=13 the intervals exclude 1 (R about 0.98-0.99), so orbit closure costs a small, statistically detectable 1-2% of coverage; at m=17 the deficit is about 0.1% and the interval just reaches 1. A plausible reason is orbit-closed bases produce coincident sums (P+Q and its Frobenius images are all in the same orbit), but this was not tested. The effect is inside the preregistered 5% band and is a loss, not a gain, so it does not support a beyond-trivial gain.
- The pre-band reasoning is also limited by the fact that base fractions B/#E are large at m=11 (33 points, ~1.7%) so coverage is a saturating-pair-count regime; it says nothing about relation yield at 3/4-sums or summation-polynomial cost.
- The columns saving factor m is trivial and was not measured, only quoted.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at toy size the orbit base gives no yield above random (R <= 1), so only the trivial 1/m column saving remains, matching "orbit credit only".

## Standing
[pcc:unverified], single local toy run, no independent replay.
