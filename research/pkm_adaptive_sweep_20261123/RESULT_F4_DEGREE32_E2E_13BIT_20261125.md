# Closed-pipeline comparison at 13 bits — result — 2026-11-25

Run under `PROTOCOL_F4_DEGREE32_E2E_13BIT_20261125.md` with binary
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d`, on the
three equal-18 instances of the published component panel. Each call is 16
independent **cold solves**: collect relations, reach full rank, recover the
planted scalar. Order interval, candidate, candidate, interval.

## Per-instance result

Ratios are candidate / interval; GAE is the full pipeline charge.

| Seed | Instance | Combined GAE | Pair 1 GAE / wall / CPU | Pair 2 GAE / wall / CPU | Relations int/cand | Trials int/cand | Median s/rho int/cand |
|---:|:--|---:|:--|:--|---:|---:|---:|
| 20261185 | `icv1-fp13-tm13-9e2d11f6` | **0.3143** | 0.2908 / 0.3013 / 0.4646 | 0.3417 / 0.3419 / 0.3900 | 1018 / 1052 | 8456 / 8642 | 1,137,429 / 361,808 |
| 20261221 | `icv1-fp13-tm101-1737775c` | **0.2840** | 0.2923 / 0.2883 / 0.3436 | 0.2762 / 0.2750 / 0.3394 | 1050 / 1028 | 11092 / 9874 | 1,633,259 / 343,630 |
| 20261276 | `icv1-fp13-tm137-9a89da76` | **0.2853** | 0.2910 / 0.2836 / 0.3484 | 0.2799 / 0.2745 / 0.3388 | 1092 / 950 | 14108 / 12412 | 1,939,933 / 588,618 |
| **Pooled** | three instances | **0.2929** | — | — | 3160 / 3030 | 33656 / 30928 | — |

Pooled charged GAE: 23.206B interval / 6.798B candidate. All 48 solves per
arm-side recovered the planted scalar and passed the group-law check; every
row reports `full_rank_reached = 1` with `rank == columns` (= 19 on each
instance), and the recovered scalars agree across the four calls at each
repeat index. `budget_exceeded` is 0.

## What the closed pipeline changes

Nothing about the ordering, and less than expected about the ratio: the
linear algebra is priced at ≈0 GAE (25 rows, 259 row ops, 4.6 GAE per solve),
so `total_gae` is 100 % the decomposition solver again. The closed-pipeline
ratio (0.284–0.314) sits inside the component panel's range (0.170–0.511) but
is far tighter — ±5 % across the three instances against ±50 % for the
component ratios.

That tightness is the substantive finding. A single solve averages ~66
relations and ~2,100 trials, so the per-instance noise that dominated the
component rows averages out; what remains is the stable 32× per-system factor
times the per-operation price. The degree-32 map solves these three 13-bit
targets for **3.4× less total charged work** with an identical pipeline.

## Claim boundary

Three selected 13-bit instances, one auxiliary map seed, one binary that does
not self-report a commit. This is a closed-pipeline cost comparison, not a
same-target rho crossover: median `s_over_rho` is 3.4e5–1.9e6, so the map
remains far above rho in the counted-cost model. It is not instance
replication, not a 14- or 15-bit result, and not evidence about s59, P-192 or
the NIST curves. Linear algebra is cheap here because the matrix is 19
columns; that will not hold at larger `|V|`.
