# RESULTS: KM3-LQR-CLOSURE (wave 2, lab A), 2026-11-15 [pcc:unverified]

PROTOCOL.md sha256 (frozen before any data): f503f3d5f3c93f1f8f78c1509f496e5cfe59f0fdc4e424805693476f2e61e3a8

Every number below is [pcc:unverified]: it comes from `hky_closure.py` / `run_cells.py` with no replay certificate and no pcc claim id.

## Route
Independent Python HKY closure (`hky_closure.py`) on the monomial/Fourier basis of R = F_p[z1,z2,z3,u]/(z_i^K - 1). The Rust engine could not be built here: `cargo build --offline` fails with "no matching package named aws-credential-types". So the route is Python, validated against committed Rust d_F values (Controls). `src/cryptanalysis/hky_fp_tower.rs` was not edited.
Instances are the N1 draws (`ns_degree.make_instance`, same seeds as lab N1).

## Main table (m = 3, primes 40961 and 12289, 3 draws each; all 12 cells agree)
| t | d_F (closure) | d_F^(f2) | c* = largest c with lambda(V_c) = 0 | c* = d_F - 1 | lambda(1) != 0 | dim V_{d_F-1} | rank pi(V_{d_F-1}) in R/(f2) |
|---|---|---|---|---|---|---|---|
| 2 | 5 (6/6) | 5 (6/6) | 4 (6/6) | yes 6/6 | 6/6 | 25 | 1 |
| 3 | 6 (6/6) | 6 (6/6) | 5 (6/6) | yes 6/6 | 6/6 | 248 | 96 |

Per cell, for every c < d_F: lambda vanishes on a basis of V_{F,c} (0 non-zero rows), and pi(1) is not in pi(V_{F,c}). At c = d_F, 1 is in V_{F,c} and lambda(1) != 0. Per-cell rows: `table.md`, `table.json`, `raw/m3_*.json`.

Predictions P1 and P2 held on all 12 cells: no counterexample. [pcc:unverified]

## m = 2 control (lambda_f(g) = sum x1 x2 g/f, "Conjecture Lambda")
| t | d_F | c* = d_F - 1 | lambda_f(1) != 0 | cells |
|---|---|---|---|---|
| 2, 3, 4, 5 | 4 | 3 | yes | 24/24 (both primes, 3 draws) |
| 6 | 5 | 4 | yes | 3/3 (p = 40961 only) |
| 7 | 5 | 4 | yes | 1/1 (p = 40961, draw 0 only) |

lambda_f kills V_c for every c < d_F on all 28 cells and lambda_f(1) != 0, so c* = d_F - 1 on all 28. P3 held for the lambda part. The d_F part of P3 ("d_F = 4") held for t <= 5 and failed for t = 6, 7 (d_F = 5), which matches the committed Kummer m = 2 values (d_F steps to 5 at t = 6; `research/pkm_falldegree_unblock_20261107/RESULTS.md` line 189). My preregistered number for P3 ignored this; the kill rule concerned lambda_f only. [pcc:unverified]

## Controls
- Engine validation against committed Rust values (`RESULTS_STAGE_C.md` Tables 1, 2; `RESULTS.md` G2/"D - ffd 1 at t = 6-9"): m = 2 d_F = 4 at t = 2, 3 and m = 3 d_F = 5 (t = 2), 6 (t = 3): matched on all draws. m = 2 d_F steps to 5 at t = 6: matched. Passed. Caveat: values are generic across draws, so this checks the engine, not instance identity (my draws are N1's, not the pilot's).
- Planted common root (`planted_instance`, both primes, t = 2 to c = 7, t = 3 to c = 6): closure never contains 1 (dims n-3 or n-1 at the top c). Passed (the system is satisfiable, so pi(1) is not in pi(V_c) either, by evaluation at the root).
- Negative control for the functional (test has power): lambda of the next draw (same p, t) evaluated on V_{F,d_F-1} of this draw is non-zero on all or all but one of the basis rows (m = 3: 25 of 25 at t = 2, 247 of 248 at t = 3; m = 2: equals dim on all cells). Passed.
- Functional sanity (`sanity.py`, `sanity.log`): lambda via DFT columns equals the pointwise formula (explicit polynomial division mod f2) on random column vectors, 4 of 4 (p, t) cells, 3 vectors each. Passed.
- c = d_F: 1 is in V_c and lambda(1) != 0, so lambda is not a witness there (as it must not be).
- A/A determinism: m = 3, p = 40961, t = 3, draw 1 run twice; outputs identical apart from timings. Passed.

## Commands
```
export OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1 LQR_OUT=<scratch>/raw
python3 run_cells.py m3 <p> <t> <idx>      # t = 2, 3; idx = 0, 1, 2; p = 40961, 12289
python3 run_cells.py m2 <p> <t> <idx>      # t = 2..5 both primes; t = 6 p=40961 (3 draws); t = 7 p=40961 draw 0
CMAX=7 python3 run_cells.py plant <p> 2 ; CMAX=6 python3 run_cells.py plant <p> 3
python3 sanity.py ; python3 aggregate.py <scratch>/raw
```
(`run_all.sh` is the batch wrapper; it was stopped during m = 2 t = 7 and the remaining cells were run by hand as above.)
Runtime (1 core, shared host): m = 3 t = 2 about 0.3 s per cell; t = 3 50 to 72 s per cell (12 cells about 12 min); m = 2 t = 6 about 105 to 125 s; t = 7 1175 s; total about 45 CPU-minutes. Memory < 1 GB.

## Deviations from the protocol
- m = 2 coverage: t = 6 for p = 40961 only; t = 7 one draw (p = 40961). t = 6, 7 at p = 12289 and t = 7 draws 1, 2 were not run (time). t = 8, 9 not run.
- m = 3 t = 4 not run (about 5000 columns, estimated about an hour per draw on the shared core). The lead's t = 4 cells are therefore untested.
- Controls not done: quotient by f1 instead of f2; null-support chain.
- Plant t = 3 stopped at c = 6 (= d_F of unplanted draws).
- Rust engine not used (cannot build offline); the lead's "engine API" step is not exercised.
- Session-log / Conductor / research-kb steps of CLAUDE.md not done (lab restricted to this directory; no MCP servers here).

## What is and is not shown
Shown, on these cells only: at m = 3, t = 2 and 3, both primes, 3 draws each, (a) d_F^(f2) = d_F; (b) lambda3 vanishes on V_{F,c} for every c < d_F with lambda3(1) != 0, i.e. lambda3 acts as a closure pseudo-solution at exactly these cells; (c) the same holds for lambda_f at m = 2, t = 2 to 7 (28 cells). Ground truth is the Python closure, validated only against d_F values.
Not shown: anything at t >= 4 for m = 3; any proof; behaviour for other primes or non-Kummer towers; that the equality persists as t grows (d_F is only 5 and 6 here, and the closure's structure at these small c may be degenerate: at t = 2, rank of pi(V_4) is 1). The Python closure was not compared row by row with the Rust echelon, only through d_F.


## Addendum: m = 3, t = 4, one draw (main session, after the lab finished) [pcc:unverified]

`OMP_NUM_THREADS=1 LQR_OUT=<dir> python3 run_cells.py m3 40961 4 0`. This is the same code and the same N1 instance generator as the table above. Raw output is in `raw/m3_p40961_t4_i0.json`, log in `m3_t4_main_session.log`; runtime 3561 s.

| p | t | draw | d_F | d_F^(f2) | c* | c* = d_F − 1 |
|---|---|---|---|---|---|---|
| 40961 | 4 | 0 | 6 | 6 | 5 | yes |

- λ vanishes on V_{F,c} for c = 2…5, and 1 ∈ V_{F,6}. `dim V_5` = 348 and `rank π(V_5)` = 132.
- d_F stays at 6 going from t = 3 to t = 4.
- One draw only. The t = 4 prediction held on it; the other draws and t ≥ 5 were not run.
