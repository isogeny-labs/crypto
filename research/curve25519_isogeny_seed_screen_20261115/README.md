# Screening isogeny seeds by on-curve count

All claims `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md) (`359bc096f5`); groups chosen and committed before any engine run
(`groups.txt`, `groups.json`). Same engine, binary and conditions as PR #4213; blind arm only. Output: [summary.txt](summary.txt).

Method: 40 fresh candidate seeds (21 to 60); count V x-values on the curve from the tower (107,041 group operations in total);
run the 8 highest (c = 19 to 22 of 32) and the 8 lowest (c = 10 to 14) in the F4 engine, 3.54e9 mul-adds each.

## Result: screening works; one hypothesis failed

| | outcome |
|---|---|
| H13 top > bottom | top 146 hits / 28.3e9 = **5.155** [4.353, 6.062]; bottom 51 / 28.3e9 = **1.800** [1.340, 2.367]; ratio 2.86, one-sided exact p = 4e-12. Supported. |
| H14 top beats unscreened | top lower bound 4.353 > 2.909 (seeds 1-20 unscreened). Supported. Top is 1.77x the unscreened rate; bottom is 0.62x. |
| H15 linear model transfers | bottom: 51 observed, 64.0 expected (interval 49 to 80), inside. **Top: 146 observed, 104.7 expected (interval 85 to 125), outside. Not supported**: the proportional-to-base-points model fitted on seeds 0-20 under-predicts the top group. |
| H16 screening cheap | 107,041 tower operations for all 40 candidates vs threshold 2.83e7 (1e-3 of the top group's F4 mul-adds), at one operation per mul-add only. Supported. |

Spearman of on-curve count vs arm rate over the 16 arms: 0.813.

## Correction to PR #4233's exploratory claim
PR #4233 said blind hits were "consistent with proportional" to base points (chi2 17.8 on 20 df). That check had little power over seeds 0-20
and the screen arms, which span a wider range, contradict it (H15). Exploratory fit (post-hoc, `exploratory.py`): Poisson rate ∝ base_points^k gives
k = 1.95 on all 37 blind arms (95% profile interval [1.45, 2.40]); on the 16 screen arms the log-likelihood at k = 1 is 5.75 below the best
and at k = 2 is 0.01 below it. Hits growing about as the square of base points is a hypothesis, consistent with a relation needing a pair of base
points (m = 2); I did not test that mechanism.

## What this does and does not show
- A seed screen picks hits per mul-add 1.77x above an average seed here, for tower work negligible at unit weight. Selection among 40 candidates also has
  a cost in the 0 to 1 GAE band that was not priced, and nothing here beats rho; the distance to rho is unchanged by this study (no rho arm).
- Selecting the best seed is a choice made per prime; whether the on-curve count is the right screen at other primes, t, or k is untested. Single k = 20, t = 5,
  one target set. Lifts arms not run.
