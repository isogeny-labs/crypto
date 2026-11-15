# RESULT: LEAD-RHO-cheon-deployed-applicability

## Outcome
- Status: ran (single run, no retries). Exit code 0. Wall time 0 s (under 1 s; run.status: exit=0 wall=0). run.stderr empty.
- run.py verdict field: "inconclusive". controls_ok: false.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): Consistent if E(22) in [0.70, 0.90] and |E(22)-E(12)| < 0.10, with all controls passing. Falsified if either condition fails with controls passing. Inconclusive if any control fails.

Measured (400 primes per size):
| bits | E | SE | null E | pos. ctrl flag rate | A/A diff | A/A SE |
|---|---|---|---|---|---|---|
| 12 | 0.8600 | 0.0173 | 0.8025 | 1.00 | -0.050 | 0.0346 |
| 14 | 0.8500 | 0.0179 | 0.7700 | 1.00 | +0.050 | 0.0356 |
| 16 | 0.8225 | 0.0191 | 0.7800 | 1.00 | -0.035 | 0.0382 |
| 18 | 0.7700 | 0.0210 | 0.7625 | 1.00 | -0.020 | 0.0421 |
| 20 | 0.8000 | 0.0200 | 0.7275 | 1.00 | +0.010 | 0.0400 |
| 22 | 0.7925 | 0.0203 | 0.7875 | 0.99 | -0.015 | 0.0405 |

- Both thresholds hold on the numbers: E(22) = 0.7925 is in [0.70, 0.90]; |E(22)-E(12)| = 0.0675 < 0.10.
- Positive control: failed the "100%" requirement at 22 bits (0.99; the other sizes are 1.00). That is the sole control failure.
- A/A: all |diff| <= 1.45 SE, so it passed (limit 3 SE).
- Negative control (random odd integers) is reference only: E 0.73 to 0.80, close to the prime E.
- Verdict: inconclusive, because the positive control did not reach 100%.

## Reading
- The toy tests only the census of "r-1 or r+1 has a divisor in [r^0.4, r^0.5]" at 12..22 bits. It does not test the lead's real claim about Cheon applicability or priced auxiliary supply (PROTOCOL.md s6 says so).
- run.py's verdict field matches the numbers and the preregistered rule: the control failure forces "inconclusive".
- The failing control is a 1% miss (likely 4 of 400 planted primes at 22 bits). I did not investigate the cause (planting at d ~ r^0.4 near the window edge, or a generator artefact). I did not edit or rerun anything, per the rules.
- If the control miss is disregarded, the point estimates would have been "consistent". That is not the preregistered outcome.
- The null (non-prime odd integers) is only a few points below E, so the census barely distinguishes primes from random integers at this size. E is also slowly decreasing, from 0.86 to 0.79.
- The protocol itself notes the thresholds were partly calibrated on a smoke look.

## Bearing on the sub-sqrt(p) question
none: this is a toy-scale divisor census with a failed control. It prices no auxiliary supply and says nothing about deployed parameters or any sub-sqrt(p) attack.

## Standing
[pcc:unverified], single local toy run, no independent replay.
