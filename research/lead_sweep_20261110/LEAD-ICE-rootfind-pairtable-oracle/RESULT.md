# RESULT: LEAD-ICE-rootfind-pairtable-oracle

## Outcome
- status: ran; exit code 0; wall 6 s (run.status: exit=0 wall=6). Sizes 12,14,16,18,20 all completed, 5 replicates. run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL s3): R_w(b) > 1 for all b, all w, and the slope CI lower bound of S_w > 0 for all w. Falsified only if R_w < 1 at both of the two largest sizes for any w with controls passing; otherwise inconclusive.

Measured IC/rho ratio R_w(b):
| w | 12 | 14 | 16 | 18 | 20 |
|---|----|----|----|----|----|
| 0 | 2.165 | 2.015 | 3.853 | 1.741 | 0.819 |
| 0.1 | 2.255 | 2.040 | 3.729 | 1.549 | 0.710 |
| 1 | 2.660 | 2.164 | 3.282 | 1.050 | 0.451 |

Slope of log2 R vs bits (point, CI lo, CI hi): w=0 -0.151 [-0.239, 0.012]; w=0.1 -0.187 [-0.283, -0.015]; w=1 -0.308 [-0.397, -0.130].

Prediction fails: R < 1 at 20 bits for every w, and every slope is negative (CI upper bound below 0 for w=0.1 and 1). Falsification rule not met, because R > 1 at 18 bits for all w (only the largest size is below 1). Verdict: inconclusive (run.py agrees).
Controls: all passed. Planted d recovered in every run; A/A median log2 = -0.104 (band 0.3); null yield obs/model = 0.483 (band [0.3, 1.6]); pricing visible w1/w0 = 2.08 (> 1.05); free-table cost fraction 0.054.

## Reading
- The toy runs the protocol as written, and controls pass, so the instrument is sound. The result is a narrowing ratio, the opposite of the "gap widens" claim, but it is not formally a falsification.
- The preregistered prediction is nonetheless contradicted: the trend is downward, and the 20-bit point is an IC win by 1.2x to 2.2x.
- Caveats: only 5 replicates per size, medians are noisy (16-bit bump at 3.85 is non-monotone). Rho cost at small sizes is inflated by DP overhead and restarts at these parameters (DP bits = bits//2-4), which favours IC at the large end. IC cost grows roughly n^(1/3) in B and table size, while rho grows n^(1/2), so the crossing at toy scale is plausible and expected from the exponents; the table cost is only about 5% of the free-arm cost. Pricing makes IC worse (w=1 ratio is lower at 20 bits because rho has more DP native ops, not IC; the ratio fell as w grew). Non-sealed pricing, affine arithmetic, h>1 bias against IC.
- run.py's verdict field matches the numbers; no discrepancy.

## Bearing on the sub-sqrt(p) question
possible signal that needs replication: the ratio decreases with size and crosses 1 at 20 bits, but with a toy implementation, 5 replicates, and the known n^(1/3)-vs-n^(1/2) table/walk constants, this is likely constant-factor behaviour of a non-asymptotic regime, and says nothing about any larger-size exponent.

## Standing
[pcc:unverified], single local toy run, no independent replay.
