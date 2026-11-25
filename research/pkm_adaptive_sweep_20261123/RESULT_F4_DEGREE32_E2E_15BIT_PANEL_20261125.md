# 15-bit multi-instance closed-pipeline panel — result — 2026-11-25

Run under `PROTOCOL_F4_DEGREE32_E2E_15BIT_PANEL_20261125.md`, binary
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`. Three
**distinct** 15-bit roster curves, selected by the predeclared first-three rule,
base sizes 19/17/14, 16 independent cold solves per call, order interval,
candidate, candidate, interval. All 48 solves per arm-side recovered and
passed the group-law check; every row `full_rank_reached = 1` with
`rank == columns`; no budget exhaustion.

| Seed | Instance | Base | Combined GAE | Pair 1 GAE / wall / CPU | Pair 2 GAE / wall / CPU | Relations int/cand | Trials int/cand | Median s/rho int/cand |
|---:|:--|---:|:--|:--|:--|---:|---:|---:|
| 20261300 | `icv1-fp15-tm103-a317aba6` | 19 | **0.2746** | 0.2850 / 0.2712 / 0.4372 | 0.2645 / 0.2754 / 0.4331 | 1110 / 1128 | 42,994 / 42,470 | 3,330,600 / 916,704 |
| 20261301 | `icv1-fp15-tm87-adf2b6f1` | 17 | **0.3765** | 0.3756 / 0.3910 / 0.5149 | 0.3774 / 0.3715 / 0.5444 | 838 / 1080 | 40,906 / 53,220 | 2,080,433 / 766,763 |
| 20261302 | `icv1-fp15-t271-e83c6554` | 14 | **0.2734** | 0.2780 / 0.2712 / 0.4306 | 0.2691 / 0.2740 / 0.4234 | 714 / 662 | 49,906 / 49,098 | 2,556,450 / 620,535 |
| **Pooled** | three curves | — | **0.3054** | — | — | 2662 / 2870 | 133,806 / 144,788 | — |

Pooled charged GAE: 67.97B interval / 20.76B candidate over all six calls.

## The per-system factor is still 32×

| Seed | Arm | F4 calls | MA/call |
|---:|:--|---:|---:|
| 20261300 | int / cand | 20,916 / 20,696 | 127.30M / 3.96M |
| 20261301 | int / cand | 19,886 / 25,704 | 127.29M / 3.96M |
| 20261302 | int / cand | 24,160 / 23,744 | 127.29M / 3.96M |

32.1× at 15 bits, against 32.15× at 14 and 32.0–32.1× at 13. Four field sizes,
three instances per rung, one constant. Multiply-adds per call do not depend on
`p` because the matrix shape is set by arity; what grows with `p` is trials per
relation (~4× per two bits), which is why a solve took minutes instead of
seconds.

## What this settles and what it does not

Relation yield is flat across arms: per trial 0.0258/0.0266, 0.0205/0.0203,
0.0143/0.0135. The whole win is per-system cost. Instance 20261301 is the
outlier (0.3765) because the candidate needed 30 % more trials there; its
per-system factor is identical, so the spread is relation density, not F4
behaviour.

Settled: the component lead survives a 4× larger field, three **independent**
curves rather than one selected curve, three base sizes, and a closed pipeline
with rank certificates. Pooled 3.3× less total charged work, instance spread
0.27–0.38.

Not settled: the seeds are the *first matches* in a window, not a random draw
from the 15-bit roster, so this is still a selected panel — weaker evidence of
generality than a uniform sample, stronger than a single instance. One auxiliary
map seed per curve. `s_over_rho` is 6.2e5–3.3e6, so the map is still far above
rho in the counted-cost model; there is no crossover and none is claimed.
Nothing here speaks to 18+ bits, s59, P-192 or the NIST curves.
