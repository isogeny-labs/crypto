# Lower t, G and the indicative distance to matched rho

All claims `[pcc:unverified]`. Protocol: [PROTOCOL.md](PROTOCOL.md) (`ba90688d61`; amendments A1 and A2 before any arm). Groups committed before the arms.
Same engine, binary and conditions as PR #4213, blind arm, 24 arms (t = 4: 16, t = 3: 8), all exit 0. Output: [summary.txt](summary.txt); `analyse.py`; raw JSON in `runs/`.
Matched rho: not rerun (this session's AWS identity is invalid); the lab's replay-audited T12 rho-neg is the reference, S = 1.544 / 1.654 / 2.651 at w = 0 / 0.1 / 1.

## Result: five of six hypotheses supported

| | outcome |
|---|---|
| H29 pairs law | t = 3 G = 2: 74 observed vs 58.7 expected (44..74): supported, at the edge. **t = 4 G = 3: 129 vs 99.5 (80..119): not supported.** The law under-predicts by 26% to 30% here, more than at t = 5 (5% to 20%). Seed 214 had 38 hits against 22 expected, so seeds are overdispersed against the law. |
| H30 lower t pays | t = 4 G = 3: **9.167** hits per 1e9 mul-adds [7.653, 10.892] vs the t = 5 unscreened 2.909. Supported. Cost per target 1.759e4 (10x below t = 5). |
| H31 t = 3 beats t = 4 | t = 3 G = 2: **12.340** [9.690, 15.492] vs 9.167, one-sided p = 0.026. Supported. Cost per target 3,748. |
| H32 G = 3 beats G = 2 at t = 4 | the two cells have identical hits (129, same targets and seeds); G = 3 costs 1.759e4 per target vs 5.196e4 (2.95x less). Supported as a cost ratio. The conditional binomial p (1.5e-17) is not meaningful: the two cells are not independent samples of hits. |
| H33 screening survives | t = 4 G = 3: top 4 15.777 vs bottom 4 2.558 per 1e9 (6.2x), p = 7.7e-18. Supported (hits differ by seed here, so the p is valid). |
| H34 below the atlas's best gap | best cell t = 3 G = 2: indicative gap at w = 0.1 = 6.76e4 vs the atlas's best 1.83e4. **Not supported.** |

## Indicative distance to matched rho (not an ecbench S; see PROTOCOL.md for what it omits)
| cell | w = 0 | w = 0.1 | w = 1 |
|---|---|---|---|
| t = 5 G = 3 seed 1550 (previous study, same method; recomputed) | gap 3.4 | 5.3e5 | 3.3e6 |
| t = 4 G = 3 (8 screened seeds) | gap 3.6 | **1.55e5** | 9.7e5 |
| t = 3 G = 2 | gap 2.7 | **6.8e4** | 4.2e5 |
| atlas best F4 arm (T13, t = 4 G = 3 seed 1193, trace replay; ecbench measured) | 27.9 | **1.83e4** | 1.13e5 |

Lowering t from 5 to 3 moves the indicative w = 0.1 gap by about 8x (5e5 to 6.8e4), a measured F4-work change on this engine. It does not beat the
atlas's measured best arm, which is 3.7x (t = 3) to 8.5x (t = 4) ahead of these cells at w = 0.1; the difference is plausibly the atlas's trace replay and
its end-to-end accounting, not tested here. At w = 0 my indicative gap (2.7 to 3.6) looks better than the atlas's 27.9, but it charges only one seed's tower
(about 1,500 to 2,000 group operations); charging the 40-candidate screen (82,384 and 47,075 operations) would add about 227 and 130 S, so that comparison is not like for like.

## Findings
- Lowering t is the lever the t = 6 result implied: hits per target fall about 4x per step while cost per target falls about 10x (t = 5 to 4) and 4.7x (t = 4 to 3), so hits per mul-add rise. Screening by on-curve count still separates groups at t = 4 (6.2x).
- The pairs law drifts upward at lower t (obs/exp 1.26 to 1.30); `b(b-1)/2` is a first-order law, not exact.
- Not a leaderboard row: no matched rho paired run, no replay certificate, no validator report, no pcc claim ids. The distance to rho remains four to five orders of magnitude at w = 0.1 on this engine.

## Decisions
G = 4 is infeasible at t < 5 (engine rule G < t); t = 2 not run (b too small); a fresh matched rho arm needs working AWS credentials.
