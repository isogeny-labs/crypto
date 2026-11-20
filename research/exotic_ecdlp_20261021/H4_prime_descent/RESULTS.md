# H4 RESULTS: FPPR prime-degree descent to F_2, m=3 (claim C4)

**Native XOR follow-up (2026-11-20 UTC):** On the same fixed `n=17`
ordinary-query system, a native XOR-DIMACS encoding yielded a verified
first witness in 34.85 seconds versus 155.57 seconds for the paired
Tseitin CNF encoding (4.46× solver-time ratio on an ordinary local host).
The native planted arm solved in 33.39 seconds; its CNF pair was censored
at 180 seconds. See [the frozen four-cell packet](xor_followup_20261120/README.md).

**Solver follow-up (2026-11-19):** A fixed `n=17` ordinary-query system
yielded a CNF/SAT witness in 147.97 seconds, verified against the original
equations and exact solution set; its paired planted arm was censored at
180 seconds. See [the frozen SAT follow-up](sat_followup_20261119/README.md).
The earlier run and verdict below remain its own recorded experiment.

**Interpretation update (2026-11-19):**
[COST_INTERPRETATION_AMENDMENT_20261119.md](COST_INTERPRETATION_AMENDMENT_20261119.md)
supersedes the cost-bound language below. The `2^87.4` figure sizes one
dense Macaulay scenario; it is not a lower bound on all solvers. The
original measurements and wording remain below for audit.

Run: isolab job `exotic-h4-prime-descent-r3` (attempts r1, r2 were earlier versions of `run.py`; r2 had a worker NameError that did not affect d_ff; r3 is the reported run). Program: `run.py`. Data: `out/result.json`. Seed 20261022. Total wall 3520 s.

Deviation from pre-registration: pod was **4 CPU / 8Gi**, not 16/32Gi (16 and 8 CPU pods did not schedule on Spot: DeadlineExceeded with no pod). Timing is therefore not a claim.

## Verdict (per the PROTOCOL decision rule): `inconclusive`, leaning to "the claim fails on cost, not on slope"

* d_ff is **flat at 7 for every n in {11,13,17,19,23,29,31,37,41}** (all 6 to 10 instances per n, both arms). Fitted slope b = 0 (CI degenerate [0,0], no variance), so the Kill condition ("b>0 with 95% band excluding 0") is **not met**, and the log fit is also flat.
* The Win condition (bounded d_ff **and** cost < 2^21) is **not met**: even taking d_solv = d_ff = 7 (the most favourable bounded-degree case), the Macaulay size at n=131 is C(132, <=7) columns, giving **2^87.4** bit-ops (omega = 2.37).
  * gap to the 2^21 relation gate: **+66.4 bits (lower bound)**
  * gap to the 2^65 rho floor: **+22.4 bits (lower bound)**
* So rule outcome is "inconclusive", but the gate that matters (cost) fails by at least 66 bits.

## How to read the flat d_ff (important)

All equations have degree exactly 6 (S_4 is degree 4 in each x_i, with x^2, x^4 F_2-linear, x^3 quadratic, so a monomial x1^a x2^b x3^c has degree <= 6). d_ff = 7 = (equation degree) + 1 at every n: the first degree fall appears at the first degree beyond the equations. So "d_ff bounded" is true but **vacuous as a difficulty measure**: it is pinned at d_max + 1 and says nothing about how large a Macaulay degree is needed to solve. The solving degree is the informative quantity, and it is only partly measured.

## Per-n table

d_ff = median over instances of the first degree D with a nontrivial fall F_D = r_D - r_top - r_{D-1} > 0. "cols@d_ff" = number of squarefree monomials of degree <= 7 in N = 3l variables (Macaulay width at d_ff), "bits@d_ff" = 2.37*log2(cols). Time = median per-instance wall of the d_ff pass (several instances ran concurrently on 4 CPUs, so not clean timing).

| n | l | N=3l | eqs (deg) | terms/eq | d_ff | instances (ff) | cols@d_ff | bits@d_ff | time_s (med) | planted solved | d_solv |
|---|---|------|-----------|----------|------|----------------|-----------|-----------|--------------|----------------|--------|
| 11 | 4 | 12 | 11 (6) | 523 | 7 | 10/10 | 3,302 | 27.7 | 0.1 | yes | **12 = N** (measured, Q_D=s only at D=N) |
| 13 | 5 | 15 | 13 (6) | 1,736 | 7 | 10/10 | 16,384 | 33.2 | 0.2 | yes | >= 10 (censored, row cap) |
| 17 | 6 | 18 | 17 (6) | 4,663 | 7 | 6/6 | 63,004 | 37.8 | 0.6 | yes | >= 9 (censored) |
| 19 | 7 | 21 | 19 (6) | 11,054 | 7 | 6/6 | 198,440 | 41.7 | 1.9 | yes | >= 9 (censored) |
| 23 | 8 | 24 | 23 (6) | 23,430 | 7 | 6/6 | 536,155 | 45.1 | 11 | yes | not run |
| 29 | 10 | 30 | 29 (6) | 83,482 | 7 | 6/6 | 2,804,012 | 50.8 | 13 | not checked | not run |
| 31 | 11 | 33 | 31 (6) | 144,088 | 7 | 6/6 | 5,663,890 | 53.2 | 30 | not checked | not run |
| 37 | 13 | 39 | 37 (6) | 376,948 | 7 | 6/6 | 19,311,488 | 57.4 | 148 | not checked | not run |
| 41 | 14 | 42 | 41 (6) | 578,944 | 7 | 6/6 | 33,199,096 | 59.2 | 338 | not checked | not run |

Instances: 5 per arm for n <= 13, 3 per arm for n >= 17 (planted + random arms; 10 or 6 systems per n).

Correctness control ("planted solved"):
* Every planted instance, every n: all n Boolean equations vanish at the planted point (residual 0). S_3 = (x1x2+x1x3+x2x3)^2 + x1x2x3 + a6 verified on 45 random point pairs (a2 does not appear, as expected for the twist).
* n <= 23 (l <= 8): the exact solution set was enumerated over V^3 and contains the planted point in every planted instance. n = 11, 13: PolyBoRi `groebner_basis()` + `variety()` also recovered the planted solution (GB time 1 s at n=11, ~85 s at n=13; the solution counts agree with the enumeration). PolyBoRi did not finish within the budget at n = 17, 19.
* n >= 29: no exact solver was run (enumeration limited to l <= 8, GB infeasible), so `planted_solved` is **False in result.json meaning "not checked" (planted_checked = 0), not "failed"**; the residual check passed.
* Solution counts s per instance are small (random arm: 0 or 6-ish copies; planted: 3 to 24, the 3! orderings times extra), matching the expected 2^(3l-n) size.

## Solving degree (partial, honest)

Deep Macaulay profiles (past d_ff), `result.json -> deep`:

* **n = 11 (N = 12), both arms: d_solv = 12 = N.** Q_D (dim of degree <= D polynomials mod the span) falls 2499, 3159, 2928, 738, 30, 6, 3 for the planted arm (D = 6..12, s = 3) and reaches s only at the last degree D = N, i.e. Macaulay linear algebra needs the entire 2^12 Boolean ring (4096 columns). No saving over enumeration.
* n = 13 (N = 15): at D = 9 (7,488 rows) Q = 20,336 of 22,819 columns, far from s = 6; D = 10 needs 25,233 rows (row cap), so **d_solv >= 10**.
* n = 17 (N = 18): D = 8 completed, Q = 103,838; **d_solv >= 9**. n = 19 (N = 21): D = 8, Q = 397,522; **d_solv >= 9**.
* The lower bounds are consistent with d_solv tracking N (about N - 3 or more at n = 13, 11). Larger n were not profiled (row cap 9000 rows for the d_ff pass; deep pass only n <= 19).
* The PolyBoRi reduced GB has max degree 2 (n = 11, 13), which is just the final lex-like solved form and is **not** a solving-degree proxy; it was not used.

## Fit and extrapolation to n = 131 (N = 132 variables)

* Linear: d_ff(n) = 7.0 + 0.0 n, slope 0 (95% CI [0,0], degenerate: zero variance across 9 points). d_ff(131) = 7.
* Log: d_ff(n) = 7.0 + 0.0 log n. d_ff(131) = 7.
* Cost at d = 7 (**lower bound on any solving cost**, d_solv >= d_ff): C(132, <=7) = 2^36.9 columns, so bits = 2.37 * 36.9 = **87.4**. Gap to 2^21: **+66.4 bits**. Gap to 2^65: **+22.4 bits**.
* The `fit` block in `result.json` also contains `extrapolated_d_solv_131 = 12` (= d_ff + the single measured offset 12 - 7 at n = 11), cost 130.6 bits, gaps +109.6 / +65.6. That number is an artifact of one data point and should not be used; the only defensible statements are the d = 7 lower bound (87.4 bits) and the observation that at n = 11 the solving degree equals N, which at N = 132 would exceed any generic search.
* Even the lower bound 87.4 bits is **above** the 2^65 rho cost by 22 bits, and the width at the measured d_ff grows with n (27.7 bits at n = 11, 59.2 at n = 41, 87.4 at 131) because the number of variables N = 3 ceil(n/3) grows even with d fixed.

## Censoring

* d_ff: nothing censored; all 9 n resolved in all instances.
* d_solv: censored for all n >= 13 (matrix rows exceed the 9000-row cap, or not attempted for n >= 23); only n = 11 resolved. Largest n completed (d_ff): **41**. No n was censored for d_ff, so the "stop at first censored n" rule was never triggered.
* Not done: FGHR 4-torsion symmetrization (random curves; E[4] not rational), a real F4/F5 trace (only a self-built Macaulay profile plus PolyBoRi), larger-n solving degree.

## Methods and limits

* Instance: GF(2^n) in a polynomial basis, E: y^2 + xy = x^3 + a2 x^2 + a6, V = random l-dimensional F_2-subspace, l = ceil(n/3). Planted R = sum of three points with x in V; random R uniform on E.
* System: S_4 = Res_u(S_3(x1,x2,u), S_3(u,x3,xR)) (the chained S_3 with the single intermediate abscissa eliminated; m = 3 needs one intermediate, two would be S_5/m = 4), Weil-restricted to n Boolean equations in 3l variables, Boolean (squarefree) arithmetic, no extra field-equation variables.
* Degrees: own Boolean Macaulay-matrix profile with exact M4RI ranks on a random GF(2) column projection (rows + 48 columns, failure probability about 2^-48); definition in `result.json -> degree_method`. d_ff = first D with F_D > 0; d_solv = first D with Q_D = s (s from exhaustive enumeration).
* The d_ff value 7 is the same in every one of 62 resolved systems; sanity: r_D = row count for D = 6, 7, 8 (rows independent), falls start at 7 on every n.
