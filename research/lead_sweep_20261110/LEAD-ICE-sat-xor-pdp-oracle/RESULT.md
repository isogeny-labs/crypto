# RESULT: LEAD-ICE-sat-xor-pdp-oracle

## Outcome
status: ran. exit code 0, wall 0 s (run.status; whole-second resolution, run well under 1 s to a few s). run.stderr empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL 3): S_free(w=1) > 1 for every F, and S_naive(w) > 1 for all w and F. Falsified if S_free(w=1) <= 1 at any F.

Measured (median over 24 targets per B):
| B | F | S_free w=0.1 | S_free w=1 | S_naive w=0 | S_naive w=0.1 | S_naive w=1 |
|---|---|---|---|---|---|---|
| 8 | 16 | 2.52 | 25.2 | 1.10 | 3.62 | 26.3 |
| 16 | 32 | 5.08 | 50.8 | 1.23 | 6.31 | 52.0 |
| 24 | 48 | 8.12 | 81.2 | 1.57 | 9.70 | 82.8 |
| 32 | 64 | 11.37 | 113.7 | 2.17 | 13.54 | 115.9 |

Median propagations 351 to 3326, conflicts 14 to 29. S_free(w=0) = 0 by construction. All S_naive > 1 at every w including w=0 (encoding cost alone exceeds table cost). Verdict: consistent_with_prediction.
Controls: SAT vs table disagreements 0; positive control ok; A/A relative difference 0.7% to 3% (< 25%), all passed. Null control (random points) used as designed.

## Reading
- The toy tests only its stated claim: one-hot DPLL decomposition priced in GAE vs a hash table. Verdict field matches the numbers.
- Observation: median_table_adds (14.5 to 30.5) is below F (16 to 64), so the table arm is charged with early exit rather than the protocol's "F additions". S ratios are therefore somewhat inflated vs a flat-F table; even with flat F the S_free(w=1) would still be about 21 to 52, so the conclusion is unchanged. At w=0.1 B=8 free, S would be about 2.2 with flat F, still above 1.
- S_free grows roughly linearly in F (propagations grow ~F^2 pairwise clauses), so the weak encoding gets worse with size. Python ns per prop is about 1.4 to 2.1 GAE upper bound (includes encoding), consistent with w=1 pricing being generous to nothing.
- Surprising: free-encoding win at w=0.1 never occurs (flag false); no crossing found.
- Not CryptoMiniSat/WDSat, no native XOR, no learning; confirms only that this naive encoding loses.

## Bearing on the sub-sqrt(p) question
none: toy confirms only that a weak one-hot SAT oracle is priced out; it says nothing about industrial XOR-native solvers or asymptotics.

## Standing
[pcc:unverified], single local toy run, no independent replay.
