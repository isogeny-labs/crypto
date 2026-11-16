# RESULT: LEAD-ICB-nonlinear-frob-stable-bases

## Outcome
- status: ran (single run, no retries). exit code 0, wall 4 s (run.status: exit=0 wall=4). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): all R in [0.90, 1.10]; falsified if some n has R > 1.10 with z > 3; R < 0.90 means B_3 worse than orbit-random. Positive control ratio < 0.2; A/A ratio in [0.95, 1.05]; otherwise inconclusive.

| n | k | E_ball | E_ctrl mean (sd) | R | z | A/A | positive |
|---|---|--------|------------------|---|---|-----|----------|
| 11 | 66 | 0.2282 | 0.2289 (0.0021) | 0.9970 | -0.33 | 0.9945 | 0.133 |
| 13 | 156 | 0.1731 | 0.1726 (0.0006) | 1.0028 | +0.85 | 0.9957 | 0.075 |
| 15 | 255 | 0.2442 | 0.2455 (0.0014) | 0.9947 | -0.92 | 1.0046 | 0.032 |
| 17 | 357 | 0.4219 | 0.4406 (0.0044) | 0.9574 | -4.24 | 0.9965 | 0.013 |

Verdict (run.py): consistent_with_prediction. Controls: null population, positive control (all < 0.2) and A/A (all within [0.95,1.05]) passed; positive_control_ok and aa_ok are both true.

## Reading
- All four R lie in [0.90, 1.10], so the preregistered prediction holds; no n has R > 1.10, so no B_3 advantage.
- Caveat: at n=17 B_3 is slightly worse than the orbit controls (R = 0.957, z = -4.2, only 8 controls). That is a statistically notable deficit but inside the preregistered band and in the unfavourable direction for the lead; it is not an advantage. The z at n=17 rests on an sd from 8 controls, so it is weakly estimated. The run.py verdict matches the numbers; no discrepancy found.
- The trend with n (0.997, 1.003, 0.995, 0.957) hints that the deficit may grow with n as the ball becomes a smaller fraction of the group; this is not tested here.
- The toy tests only m=2 sumset efficiency for the Hamming-ball variant, with k about 66..357 at group order 2^11..2^17. It is a proxy for relation yield, not a relation-collection or ECDLP cost, and does not test trace-spectrum, tau-adic, multiplicative-subgroup or Couveignes-Lercier variants, prime fields, linear algebra or the GAE-priced rho comparison.

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: at these toy sizes the weight-3 Hamming ball gives no sumset advantage over random Frobenius orbits (and is slightly worse at n=17), so it shows no route past the existing baseline.

## Standing
[pcc:unverified], single local toy run, no independent replay.
