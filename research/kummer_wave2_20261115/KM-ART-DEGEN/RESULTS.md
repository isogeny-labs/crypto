# RESULTS: KM-ART-DEGEN (lab C, wave 2, 2026-11-15)

Every number below is [pcc:unverified]. Own Python HKY closure (`hky_eps.py`), no Rust touched, local, 1 core.
PROTOCOL.md sha256 (written before any data): `d0310d316d8c03ecc46584beb8211f53be57f3d57762e8f58e37cf85293b2078`

## Question and prediction

Replace the top relation y_{t-1}^2 = 1 by y_{t-1}^2 = eps (eps = 1 real tower, eps = 0 Artinian, eps = 5 generic control)
and measure d_F^eps of the m = 2 generator for t = 2..8. Prediction (lead C1): d_F^0 = d_F^1 on every cell.

## Result table (all finished cells; per-cell table in table.md, raw rows in raw_cells.jsonl)

| family | t | cells (p x draw) | d_F eps=1 | d_F eps=0 | d_F eps=5 | ffd eps=1/0/5 | reference d_F (eps=1, committed) |
|---|--:|--:|--|--|--|--|--|
| tower | 2 | 6 | 4 | 4 | 4 | 4/4/4 | 4 |
| tower | 3 | 6 | 4 | 4 | 4 | 4/4/4 | 4 |
| tower | 4 | 6 | 4 | 4 | 4 | 4/4/4 | 4 |
| tower | 5 | 6 | 4 | 4 | 4 | 4/4/4 | 4 |
| tower | 6 | 6 | 5 | 5 | 5 | 4/4/4 | 5 |
| tower | 7 | 2 | 5 | 5 | 5 | 4/4/4 | 5 |
| tower | 8 | 1 | 5 | 5 | 5 | 4/4/4 | 5 |
| w1 | 2 | 6 | 3 | 3 | 3 | 3/3/3 | 3 |
| w1 | 3 | 6 | 3 | 3 | 3 | 3/3/3 | 3 |
| w1 | 4 | 6 | 4 | 4 | 4 | 3/3/3 | 4 |
| w1 | 5 | 6 | 4 | 4 | 4 | 3/3/3 | 4 |
| w1 | 6 | 6 | 4 | 4 | 4 | 3/3/3 | 4 |
| w1 | 7 | 2 | 4 | 4 | 4 | 3/3/3 | 4 |

- Cells: t = 2..6: 2 primes x 3 draws per family (36 per t, 3 eps each = 108 closures per t); t = 7: 2 primes x 1 draw per family;
  t = 8: tower, p = 40961, draw 0 only (about 11 CPU-minutes per closure; the remaining t = 8 cells were not run).
- Comparable cells (d_F finite at both eps = 1 and eps = 0): 65. Equal: 65. Maximum |d_F^0 - d_F^1| = 0.
- Every eps = 0 draw was a unit in R_0 (constant coefficient of f is non-zero); none were skipped.
- eps = 5: d_F^5 = d_F^1 and ffd^5 = ffd^1 on every cell (65/65). f was a unit in R_5 on every cell (closure reached 1).
- First fall degree: ffd = 4 (tower), 3 (W = 1 box) at every eps and t, as D3 / Theorem 4 state.
- W = 1 box parameter: coefficients (c00, c10, c01, c11) uniform non-zero, same draw at every t and eps. The W = 1 d_F is 3 at t = 2,3 and
  4 at t = 4..7 and 5 at t = 8 (the t = 8 value comes from the committed reference only; the W = 1 t = 8 cell was not run here).

## Controls

- Positive control (instrument sees the known value): at eps = 1 the closure reproduces the committed m = 2 values on every run cell:
  tower d_F = 4,4,4,4,5,5,5 (t = 2..8) with ffd 4, W = 1 d_F = 3,3,4,4,4,4 (t = 2..7) with ffd 3
  (growth_law README sec. 5). Closure dimensions also match the committed Rust rows (t = 6: 605 at c = 4, 1586 at c = 5;
  t = 8: 1061 and 6885 at c = 4, 5).
- Negative control (planted consistent target; f vanishes at the grid point (z1, z2) = (1, -1), x_R a root of S3): 24/24 closures at
  eps = 1 (t = 2..5, 2 primes x 3 draws, cmax = 7) never reached 1 (censored at c = 8). The same planted f at eps = 0 gave d_F = 4
  on 24/24 and at eps = 5 gave d_F = 4 on 24/24: R_0 and R_5 do not see the grid zero, as the lead notes.
- Exact twist identity (engine check of Prop. 1): for eps = s^K, d_F^{eps}(f) = d_F^{1}(f(s z)), t = 2..5, 24 pairs: all 24 pairs equal 4
  (table.md, kinds twist_eps_sK and twist_ref_eps1_fs).
- A/A: not run as a separate arm; repeat draws agree (every cell of a given family and t gives the same value across 6 draws at t <= 6).

## Exact commands

```
cd research/kummer_wave2_20261115/KM-ART-DEGEN
OMP_NUM_THREADS=1 python3 run_cells.py main 2 6 3      # t=2..6, 3 draws per prime
OMP_NUM_THREADS=1 python3 run_cells.py planted
OMP_NUM_THREADS=1 python3 run_cells.py twist
OMP_NUM_THREADS=1 python3 run_cells.py main 7 7 1
OMP_NUM_THREADS=1 python3 run_cells.py main 8 8 1      # stopped after 3 closures (tower, p=40961, draw 0)
python3 aggregate.py raw_cells.jsonl > table.md ; python3 gen_svg.py raw_cells.jsonl dF_by_eps.svg
```

Runtime: sum of per-cell wall times 2959 s (about 49 minutes, on a host shared with three other labs, load average about 5). The
runner was restarted once after an interrupted first loop (completed cells are skipped on resume; two loops briefly ran in parallel,
no cell was duplicated: the resume key is (kind, family, p, draw, t, eps)).

## Deviations

- t = 7 has 1 draw per prime and t = 8 has a single draw (CPU budget); the protocol asked for 3 draws per prime at every t.
- cmax = 7 for t <= 7; t = 8 closures stop at c = 5 because 1 entered there (cmax was 6).
- A/A arm not run separately (see above). No eps other than 0, 1, 5 and the twist values s^K.
- Draw acceptance (no zero on the t = 8 grid) is applied once per (p, family, draw) and the same f is used at every t.
- C2 (socle functional as pseudo-solution) was not tested: it needs the closure rows of KM3-LQR-CLOSURE.

## What is and is not shown

- Shown on these 65 cells [pcc:unverified]: d_F^0 = d_F^1 = d_F^5 and ffd identical, for the tower and the W = 1 box, t = 2..7 (both
  families, 2 primes) and t = 8 (tower, one draw). The prediction C1 (equality on every cell) was not contradicted; the kill rule
  (|difference| >= 2 anywhere, or more than 10% of cells differing) did not fire. d_F^0 grows with t exactly as d_F^1 does, so the declared
  alternative (flat d_F^0) did not occur.
- Not shown: anything for t >= 9, anything about a proof of C1 or C2, behaviour at W >= 2 boxes other than the tower, or other primes
  than 40961 and 12289. No claim about instances where f is not a unit. Nothing here changes an index-calculus cost estimate.

## Files

PROTOCOL.md, RESULTS.md, hky_eps.py, run_cells.py, aggregate.py, gen_svg.py, table.md, summary_table.md, raw_cells.jsonl, run.log, dF_by_eps.svg.
