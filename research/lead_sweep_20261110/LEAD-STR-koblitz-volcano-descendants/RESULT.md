# RESULT: LEAD-STR-koblitz-volcano-descendants

## Outcome
- status: ran (single run, no retries); wall 170 s; exit code 0; run.stderr empty/benign.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): Frobenius/plain collision ratio R_m in [0.75m, 1.3m] for m=11,13,15,17 over 5 seeds; A/A ratio in [0.7,1.4]; order formula equals brute count at m=11,13. All in band => consistent_with_prediction.

Measured (mean over 5 seeds):
| m | R_m | predicted | A/A | order formula / brute |
|---|-----|-----------|-----|-----------------------|
| 11 | 10.994 | 11 | 0.988 | 1982 / 1982 |
| 13 | 12.912 | 13 | 0.990 | 8374 / 8374 |
| 15 | 14.956 | 15 | 0.996 | 32494 / not brute-forced |
| 17 | 16.948 | 17 | 1.005 | 131174 / not brute-forced |

Verdict (run.py): consistent_with_prediction. Controls: positive control passed (R_m within about 1% of m), A/A passed (0.99 to 1.005), order check passed at m=11,13.
Volcano table (reported only): f=1 only at m=13 among primes up to 83; f for m=37..83 is large (1.9e5 .. 3.5e11) with no small prime factors found (m=7: f=7; m=11: f=23).

## Reading
The toy confirms that Frobenius-orbit labelling gives a collision-rate factor of essentially exactly m on the Koblitz curve, and that the plain-label arm has no gain (ratio 1). The run.py verdict matches the numbers; no discrepancy. Caveats: the descendant arm is a theoretical stand-in (plain labels), not a constructed descendant curve; no Velu isogenies were built; collision rate only, no rho step costs. R_m is slightly below m at small m (0.99m), a minor finite-size effect. The m=15,17 order formula was not brute-checked.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: the only lever is the factor m (sqrt(m) in steps), which is a constant factor and is absent below the surface.

## Standing
[pcc:unverified], single local toy run, no independent replay.
