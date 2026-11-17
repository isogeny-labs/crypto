# RESULT: LEAD-ICE-boolean-f4-f5-f6-engines

## Outcome
- status: ran (single run, no retries)
- wall seconds: 0 (run.status: exit=0 wall=0; under 1 s)
- exit code: 0; run.stderr empty

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md section 3): median over cells of same-support replay ratio (fresh cost / replay cost) lies in [1.0, 2.3]; the unrelated-trace null ratio must not exceed it; controls valid. Above 2.3 falsifies.

Measured:
- Same-support ratios: 1.0155 (n9), 1.0178 (n10), 1.0304 (n11). Median 1.0178.
- Unrelated-trace null: 1.0019, 1.0041, 1.0087. Max 1.0087, which is <= 1.0178.
- Positive control (own trace): 1.192, 1.143, 1.172. Min 1.143, which is > 1.
- A/A: 1.0 in every cell, max deviation 0.0.
- Mean pivot-trace misses per cell: 81.5, 100.9, 121.6.

Verdict (run.py): consistent_with_prediction. Controls passed (null, positive, A/A). Rank equality is asserted inside run.py; the run exited 0, so I assume it held (I did not inspect run.py).

## Reading
- The toy tests only a linear-algebra proxy for trace reuse. It is not F4/F5 or the Semaev system (PROTOCOL section 6).
- The verdict field matches the numbers. The statistic is far below 2.3, and the null is above 1 but under the same-support ratio.
- Even the own-trace upper bound is only about 1.14 to 1.19x. The cost model (XOR = test = 1) is dominated by row XORs and reduction bit tests that a trace cannot skip. Pivot search is a minor share, so there is little to save. That makes the same-support gain of about 1.02x close to the noise floor. Same-support is only about 0.01 to 0.02 above the unrelated null, so the effect is barely distinguishable.
- The result is not comparable with the recorded 1.4-2.3x engine gain, which was measured on the real Boolean S3 line. This toy gives a smaller gain, which is consistent with the recorded bound as a ceiling but does not reproduce it.
- Toy sizes are tiny (n 9 to 11, 12 trials per cell). No GAE pricing (AGENTS 5a) is applied.

## Bearing on the sub-sqrt(p) question
none. The toy shows trace reuse gives about 1.02x on proxy matrices, far below the 22x-33x gap to rho recorded at n13. It neither closes nor reopens the lead.

## Standing
[pcc:unverified], single local toy run, no independent replay.
