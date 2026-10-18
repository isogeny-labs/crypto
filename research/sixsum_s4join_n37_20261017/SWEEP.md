# n=37 density sweep: E4/E6 crossover across base sizes

Engine: `examples/koblitz_sixsum_s4join_n37` (same as run.json, trial cap
8e5 for sparse cells). Models from the exact pair-table sizes (|T| distinct
pair sums, cofactor-projected standard-subspace bases, curve a=0 n=37,
r = 230603167, region 4r):

| dim | B points | |T| | E4 (4-sums/target) | E6 (6-sums/target) |
|----:|--------:|------:|--------------------:|--------------------:|
| 4 | 4 | 8 | 3.5e-8 | 9.3e-8 |
| 5 | 10 | 50 | 1.4e-6 | 2.3e-5 |
| 6 | 52 | 1352 | 9.9e-4 | 0.45 |
| 7 | 108 | 5832 | 1.8e-2 | 35.8 |
| 8 | 240 | 28800 | 0.45 | 4316 |
| 9 | 488 | 119072 | 7.7 | 3.05e5 |
| 10 | 998 | 498002 | 134 | 2.23e7 |

E6 crosses 1 at B ~ 60 points (dim ~6.2); E4 crosses 1 at B ~ 280 (dim ~8.1).
E6 scales as |T|^3 per the model (each base step confirms within rounding).
Extrapolated to n=131 (region 4r ~ 2.7e39): B_min(6) ~ 1.1e7 points against
B_min(4) ~ 1.6e10 — three to four orders of base (and linear-algebra)
reduction from the 6-sum arity, the feasibility note's density claim made
measured. Per-relation join cost stays 4r/|T| at every measured size
(the arity buys density, not per-relation cost); the orbit-structured state
scan remains the layer that must convert density into a total-work win.
