# K2 flatness test — frozen protocol (2026-11-15)

Lead: `LEAD-KOB-flat-degree-crossover-bar` (K2), `research/leads_20261112/README.md` §2c and
`research/leads_20261112/lanes/lane_K_koblitz_binary.md` §2; lever 3 of
`research/noneric_lane_survey_20261115/README.md`. Frozen before the measurement grid ran; only the
calibration and control smoke runs listed under "Seen before freezing" were executed before this commit.

## Question

Does the degree-bounded GF(2) closure of the Weil-descended binary point-decomposition system at
m = 3 and m = 4 summands solve the system at a degree that stays flat (bounded, here <= 4) as the
field degree n grows with the factor-base dimension k ~ n/m, or does the solving degree grow?

## A premise correction found during calibration (seen before freezing)

The direct descent of S_4 (m = 3) is **not** a degree-<= 4 system: S_4 has degree 4 in each
abscissa, x^3 = x * x^2 is a Boolean quadratic in the k bits of x, and the descended equations reach
Boolean degree 6 (`gen_deg` > 5 on n = 12, k = 4, all 4 targets; `pq_descent_symbolic.rs` says "S_4
comes out of degree at most six"). A degree-<= 4 closure of the direct m = 3 (or S_5, m = 4) system is
undefined. K2's "N = 3k variables at D <= 4" is therefore measured on the **chain presentation**, the
standard degree-<= 3 Boolean form with auxiliary abscissae (the lane note's risk (1) names it):

- m = 3: S_3(x1, x2, U) = 0, S_3(U, x3, xR) = 0; N = 3k + n variables, 2n equations (degrees 3, 2).
- m = 4: S_3(x1, x2, U1) = 0, S_3(U1, x3, U2) = 0, S_3(U2, x4, xR) = 0; N = 4k + 2n, 3n equations.

x_i = sum_j v_{i,j} e_j over a random k-dimensional F_2-subspace V of F_{2^n}; U_j free in F_{2^n}
(n Boolean variables each, polynomial basis). Curve y^2 + xy = x^3 + a x^2 + b with random a, b;
target xR = abscissa of a curve point (Tr(x + a + b/x^2) = 0).

## Instrument

`k2close.rs` (this directory, standalone Rust, no crate dependency). Closure W_D = smallest subspace
of multilinear polynomials of degree <= D containing the equations and closed under f -> x_i f for
deg f <= D - 1; semi-echelon form over monomials ordered by degree descending (a row's pivot is its
leading, highest-degree monomial). All F_2 solutions are found by enumeration (walking the chain
with an exact quadratic solver). **Solved at D** iff rank W_D = M(N, <= D) - rank(evaluation vectors
of the solutions) (for an inconsistent system: 1 in W_D). The symbolic expansion is checked against
the word formula on 64 random points per system; every enumerated solution is checked against every
equation. D is scanned upward from `dmin`; the first solved D is the system's **closure solving
degree D_sol**; a D whose column count exceeds `maxcols` is recorded as skipped (censored).

## Grid (seeds fixed; per cell: curve seeds 1 and 2, `--targets` per curve)

| series | m | (n, k) | N | targets/curve | D scanned |
|:--|--:|:--|:--|--:|:--|
| A (m = 3, k = round(n/3)) | 3 | (11,4) (12,4) (13,4) (14,5) (15,5) (16,5) (17,6) (18,6) | 23 24 25 29 30 31 35 36 | 2 | 3..5 (maxcols 80000) |
| A-ext | 3 | (19,6) | 37 | 1 | 4..5 (maxcols 80000) |
| B (m = 3, n = 15, vary k) | 3 | (15,3) (15,4) (15,6) | 24 27 33 | 2 | 3..5 (maxcols 80000) |
| C (m = 4, k ~ n/4) | 4 | (8,2) (9,2) (10,2) (11,3) (12,3) | 24 26 28 34 36 | 2 | 3..5 (maxcols 80000) |
| P (positive control, m = 2 direct S_3) | 2 | (16,8) (20,10) | 16 20 | 2 | 2..5 |
| N (negative control, random system of the chain's shape, planted solution) | 3, 4 | (12,4), (8,2) | 24, 24 | 1 | 3..4 |

Order: P, N, A by increasing n, B, C, A-ext. Compute cap: about 2 CPU-hours in total under the shared
`flock` CPU lock; a cell is dropped (and reported as not run) only if the cumulative budget is spent.
(15,5) is shared by A and B.

## Predictions and decision rule (registered)

Registered prior (lead K2): flatness at the square cells k = 10, 11 (m = 3) and k = 8 (m = 4) has
probability 0.15. The cells here are smaller (k <= 6 at m = 3, k <= 3 at m = 4), so this test can
only reject flatness early or report "flat on the measured range"; it cannot confirm the lead's cells.

Generic reference: the Boolean semi-regular degree d_reg of each shape (`dreg.py`, same series as
`ic_descent_degrees::semi_regular_degree`): 7 at N = 23-24 rising to 9 at N = 36 (m = 3), 7 / 9 at
N = 24 / 36 (m = 4).

Controls must pass first: P solves at D <= 4 on every target (reproduces lane A's m = 2 closure
degree 4); N fails at D = 4 with deficit > 100 (the instrument does not "solve" structure-free systems
of the same shape).

Per series A and C, with D_sol >= 5 for a target unsolved at the largest D scanned:

- **flat (on the measured range)**: every target of every cell solves at D_sol <= 4;
- **growing**: the share of targets with D_sol <= 4 is 1 on the smallest two n and at most 1/2 on
  the largest two n of the series;
- **inconclusive**: anything else (including isolated D_sol >= 5).

The overall verdict is "flat" only if A and C are both flat; "growing" if either grows.
Series B is descriptive (dependence on k at fixed n). All numbers are `[pcc:unverified]`.

## What a "flat" verdict does and does not mean

Flat D_sol = 4 makes one solve cost polynomial (M(N, <= 4) columns, N = O(n)), so the exponent of a
m = 3/4 binary IC would be set by relation yield and linear algebra, not by the solve. It says nothing
about n beyond the measured range (lane A saw the m = 2 stall start at n >= 35), about the lead's
square cells, about D_sol of the direct presentation, or about cost against rho (the per-relation
yield 2^{mk - n}/m! is unchanged). Closure degree is a proxy (WEAKNESSES E-GB-064).

## Seen before freezing

- n = 12, k = 4, direct S_4 presentation: generator degree 6 (4 targets).
- Chain m = 3 (11,4): D_sol = 4 on 3 targets (seed 1, all inconsistent), D = 3 fails.
- Chain m = 3 (15,5): D_sol = 4 on 2 targets (seed 1, D = 4 only, 12 and 6 solutions), 29 s each.
- Chain m = 4 (8,2): D_sol = 4 on 2 targets (seed 1), D = 3 fails.
- Negative control m = 3 (11,4) seed 1: D = 4 deficit 7657 (stall).
- Positive control m = 2 (16,8) seed 1: D_sol = 3 on 2 targets (both inconsistent), D = 2 fails.
These smoke rows are not part of the grid; the grid reruns those cells from its own seeds 1 and 2
(identical seeds, hence identical systems, for the seed-1 rows).
