# MP-lattices-2: results (2026-10-31)

Outcome: SUPPORTS the hypothesis being falsified. The rigorous reach of helpful-row sublattices stays <= 1/6 for S_3, and the falsifier did not fire.
Classification: accounting. Toy scale only, no security claim.

## Layer 1: exact Gram-determinant HG surrogate
Setup: p = nextprime(2^128), seeds 1 and 2, delta* found by 12-step bisection.

| l (dim A) | A delta* | B (tau=0) | best B' over tau in {-0.5..1.5} |
|---|---|---|---|
| 1 (9) | 0.0493 | 0 | 0.0493 (tau=0.25, keeps all rows, same as A) |
| 2 (25) | 0.0955 | 0 | 0.0955 (same as A) |
| 3 (49) | 0.1154 | 0.044-0.046 | 0.1154 (same as A) |
| 4 (81) | 0.1265 | 0.070-0.072 | 0.1265 (same as A) |
| 5 (121) | 0.1335 | 0.078-0.079 | 0.1335 (same as A) |

- No threshold tau ever gives B' a larger delta* than A. Every proper sublattice is strictly worse, and max(B' - A) = 0.000.
- In log_p units, the B sublattice's log-volume exceeds the product of its retained diagonal entries by +25.8 at l=3, +48 at l=4 and +88 at l=5. This is the projection inequality vol(L_S) >= prod(retained diag), and it is far from tight here.
- Arm A: the exact determinant matches the closed form prod diag to within 1e-4.
- Arm A closed form for l=1..16: delta* = 0.049 ... 0.1553 at l=16 (dim 1089). Fit delta_inf = 0.1629 (c = 0.139). With p -> infinity, delta* is 0.1641 at l=64, approaching 1/6.
- S_4 arm A closed form: delta* = 0, 0.0113, 0.0192, 0.0239, 0.0270 for l=1..5. Fit delta_inf = 0.0369; the large-p limit at l=12 is 0.0359 (asymptote 1/24).
- S_4 exact sublattice determinants: only l=1 (dim 125) was computed, and delta* = 0 for every arm. l=2 (dim 729) was not computed.

## Layer 2: PARI qflll at p = nextprime(2^256)
Setup: 4 planted instances per cell, seeds 100-103.

Largest delta reached on 4/4 instances:

| arm | l=2 HG | l=2 heuristic | l=3 HG | l=3 heuristic |
|---|---|---|---|---|
| A | 0.11 | 0.11 | 0.12 | 0.13 |
| B | 0 | 0 | 0 | 0 |
| C | 0 | 0 | 0 | 0 |

- B and C never succeed at any delta in 0.06..0.20, under either the HG or the heuristic criterion.
- The second-vector log_p margin of B/C is -0.18 to -0.61, against +0.30 to -0.21 for A. At every delta, B/C sit about 0.3-0.4 log_p below A.
- Arm C behaves identically to B: no x1<->x2 antisymmetric vectors appeared among the leading vectors.
- LLL on arm A beats its own determinant estimate (0.11 vs 0.0955 at l=2, 0.12 vs 0.115 at l=3). The cause is that many vectors (23/25, 47/49) vanish at the root over Z: the lattice has structure that the determinant bound does not capture.

## Layer 3: controls at l=2, seeds 200-203
- Generic polynomial on the same support: A reaches HG 0.08 and heuristic 0.10. B/C get 0 everywhere.
- Unplanted xR: 0 false recoveries across all arms. Arm A's HG margin is a property of the lattice alone, so A still shows HG=4/4 at delta <= 0.10.
- delta = 0.45: nothing succeeds in any arm.
- Arm A reproduces the programme's quoted baselines: HG about 0.10, heuristic 0.12-0.13.

## Decision
- The pre-registered prediction_if_true holds: B/B'/C <= A + 0.02 at every l (in fact strictly below A), and the A fits are <= 1/6 and <= 1/24.
- Named obstruction: dropping rows while keeping their monomial columns makes vol(L_S) >= prod(retained diag), and the excess grows superlinearly in l. Each row lost costs more than the log_p excess it removes.
- Scope of the kill:
  - It covers the "delete rows with lambda.beta > k" construction (and threshold variants) on the JM box for S_3, l <= 5 exact and l <= 3 under LLL, and for S_4 at l = 1.
  - It does not cover Boneh-Durfee-style sublattices with an engineered geometric progression (other shift sets, extra x-shifts, or the unravelled linearization u = x1 x2 - a), and it does not cover S_4 at l >= 2.
- Do not trigger the S_5 computation.

## Caveats
- delta* comes from a vol^{1/r} surrogate. LLL can do slightly better (see Layer 2).
- 4 instances per cell. Layer 1 used one random curve per seed.
- Layer 1 for S_4 is essentially empty beyond arm A's closed form.

## Next step at larger size
- If anything, run the Herrmann-May unravelled linearization (u = x1x2) for S_3, where the leading term x1^2x2^2 = u^2.
- Run S_4 at l=2 with float QR (mpmath or fplll) to cover the 729-dimensional sublattice.

## Files
PROTOCOL.md, common.py, layer1.py, layer1_run.py, layer1_closedA.py, layer2.py, summarize.py, RESULTS.json, raw/
