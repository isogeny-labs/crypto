# RESULT: LEAD-RHO-native-step-cost

## Outcome
- status: ran (single run, no retries). exit code 0, wall 3 s. run.stderr empty.
- run.py verdict field: consistent_with_prediction.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): "max R_net < 1 over all cells with m >= 17 and w_C in {0, 0.1}" gives consistent_with_prediction; "falsified" if every such cell has R_net >= 1; A/A median step ratio in (0.7, 1.4); positive control canon/plain step ratio < 0.5 at m >= 17.

Measured (median first-repeat steps over 24 paired seeds):
| m | plain | canon | step ratio | R_net wC=0 (wI 3/10) | wC=0.1 (wI 3/10) | wC=1 (wI 3/10) | A/A |
|---|---|---|---|---|---|---|---|
| 13 | 94.5 | 17 | 0.180 | 0.190/0.192 | 0.281/0.231 | 1.102/0.577 | 0.910 |
| 15 | 201.5 | 32.5 | 0.161 | 0.167/0.169 | 0.260/0.208 | 1.097/0.561 | 0.876 |
| 17 | 427.5 | 55.5 | 0.130 | 0.133/0.133 | 0.217/0.169 | 0.973/0.487 | 0.848 |
| 19 | 888.5 | 69 | 0.078 | 0.079/0.080 | 0.135/0.103 | 0.641/0.316 | 0.889 |
| 21 | 490 | 139 | 0.284 | 0.285/0.286 | 0.509/0.380 | 2.523/1.229 | 1.081 |

- Max R_net over m>=17, wC in {0, 0.1}: 0.509 (m=21, wI=3, wC=0.1) < 1. Prediction holds.
- Controls: A/A in (0.7, 1.4) at all m (0.85-1.08), passed. Positive control (step ratio < 0.5 at m>=17): 0.130, 0.078, 0.284, passed. Null: plain arm R_net = 1 by construction.
- The wC=1 column (reported, not part of the claim): R_net >= 1 at m=21 for both wI and at m<=15 for wI=3; ordering does not hold across the band, so no ordering is claimed there.
- Verdict: consistent_with_prediction; controls passed.

## Reading
- run.py verdict agrees with the numbers; no discrepancy found.
- The toy tests only a narrow claim: that the counted canonicaliser price (about 26-40 extra squarings per step, about 20x the plain per-step cost at 0.1 weight) does not erase the step gain from first-repeat step counts. It does not test timing, instruction counts, or EXP-CERTBIN.
- Caveats from the protocol apply and matter: first-repeat steps include fruitless cycles of the x-only index walk, which inflates the canon gain (as PROTOCOL s6 warns). The gain is not the genuine factor ~sqrt(2m) of a real Frobenius+negation quotient.
- Surprising: plain median steps are non-monotone in m (888.5 at m=19, 490 at m=21), and step ratio worsens at m=21 (0.284, CI 0.18-0.48). With 24 seeds and a 5-curve, small-pool structure effect (pool of 16, cycle-prone walk) this shows the step proxy is noisy. The m=21 cell is the closest to 1 and sits near the weak-weight edge.
- run.py's reported base_seed (20261110) is a label only; the code uses seeds 1000*m+i and 5000000+1000*m+i as the protocol says (checked in run.py lines 143-150).
- Polynomial basis only; per-step cost in a normal basis (free Frobenius) is not modelled.

## Bearing on the sub-sqrt(p) question
none (constant factor only at best): the toy can only affect constants of a rho walk, cannot change the exponent (Shoup), and the step proxy is biased toward the canon arm.

## Standing
[pcc:unverified], single local toy run, no independent replay.
