# PROTOCOL: subcube refutation below the linear 2+1 scan (written 2026-11-14, before any measured run)

Pilot only. Every number that follows from this protocol is `[pcc:unverified]`.

## Claim under test

For m = 3 point decomposition on a binary curve with x(P_i) in an l-dimensional F2-subspace V,
a clause-learning solver with native XOR reasoning can refute a subcube of x1 values (k < l of
x1's coordinates fixed) for less work than the 2^(l-k) leaf solves the linear 2+1 scan spends on it.

## Prediction (written before the runs)

No. Fixing k < l coordinates leaves the products of the free x1 coordinates with the n bits of u
as free variables of the XOR system, so propagation + Gauss refutes nothing above depth l, and
learnt clauses will cost at least one conflict per leaf they cover. Expected: primary ratio >= 1
in every budget arm at every l, alpha close to 1 or above.

## Instances

- F_{2^n}, n prime chosen per l: (l, n) = (3, 11) validation only, (4, 13), (5, 17), (6, 19),
  (7, 23), (8, 29). n is not fixed: it is the smallest prime with n >= 3l + 1, so that n exceeds
  the 3l - 1 unknowns of the (s, p) system. The excess n - (3l - 1) is 2, 3, 2, 3, 6: a confound
  for trends in l, stated in the README.
- Field polynomial: first irreducible trinomial, else pentanomial, in the order of `sub.Field`.
- Curve y^2 + xy = x^3 + x^2 + B, B drawn from the cell seed. V = span(1, t, .., t^(l-1)).
  Factor base: rational points with x in V \ {0} (x = 0, the 2-torsion point, is excluded in the
  ground truth and by the clauses "x1 != 0" and "p != 0" in every encoding). Targets have
  x(R) not in V and x(R) != 0, so no summand pair can sum to infinity.
- Cell seed 2026111400 + l. Per cell: the first 8 random targets that the pair-table ground
  truth says are non-decomposable, then 4 planted targets (decomposable, reported separately).
- Development (encoding and solver-option choice for Part A) used seed 999 at l = 4, 5, 6 only;
  no measured seed was run before this file was written.

## Encodings and engines

System (`sub.System`): (1) S3(x1, xR, u) = 0, (2) S3(x2, x3, u) = p^2 + u p + (u s)^2 + B = 0,
(3) z^2 + s z + p = 0 with x2 = z, x3 = s + z. XOR rows over AND gates c_i&u_j, u_j&p_k, u_j&s_i,
s_i&z_j.
- `plain`: AND gate as 3 clauses (what the earlier pilot's arm amounted to).
- `dual`: additionally r = ~sel & other and XOR t + r + other = 0 (parity encoding).
- CryptoMiniSat 5.18.0 via pycryptosat, one thread, options frozen for all arms:
  `autodisablegauss=0, sls=0` (chosen on the development seed).
- Own engine `sub.Eng`: Gauss-Jordan on the XOR rows + AND-gate substitution to fixpoint, no
  learning; below a full x1 it branches on free base variables (u first).

## Part A arms (per non-decomposable target, all 2^l values of x1 unless stated)

A-plain-inc, A-dual-inc: one incremental solver, x1 by assumptions, conflicts per leaf.
A-plain-cold, A-dual-cold: fresh solver per leaf. Leaves sampled: all for l <= 6, first 64 in a
seeded shuffle for l >= 7 (plain arms: 32 for l >= 6). Cap 20 000 conflicts per leaf (censored
leaves counted at the cap and flagged). A-eng: own engine, nodes and row XORs per leaf.
Metric: mean conflicts per leaf vs l. Target O(1).

## Part B arms (same targets)

DFS over x1's coordinates (value 0 first). At a node with k fixed, j = l - k: solve under the
k assumptions with conflict limit B; UNSAT -> prune, SAT -> stop, unknown -> descend. Leaves
(k = l) are solved without a limit. Encoding: `dual`.
- Budgets: abs 0, 10, 100, 1000; prop c * 2^j, c = 0.25, 0.5, 1. (CMS returns "unknown" without
  searching at limit 0 and may overshoot a limit by a few conflicts; conflicts actually spent
  are what is counted.)
- keep: one incremental solver per DFS. discard (control): fresh solver per node.
  discard runs all 7 budgets for l <= 7; at l = 8 only abs 0, abs 100, prop 1.
- Controls, keep arm, budgets abs 0, 10, 100, prop 1: `rand` = seeded random order of the
  coordinates (seed cell*13 + target index); `alt` = a second basis of the same V
  (`Cell.alt_basis(cell*7 + 1)`), coordinates in index order.
- E (own engine): DFS with fixpoint refutation at each node, orders std, rand, alt.
- Decomposable targets: keep arm, std order, all 7 budgets + E; reported separately.
- Recorded per (target, arm): solver calls, leaves visited, conflicts, prune-depth histogram,
  wall time; engine: nodes, leaves, sub-nodes, fixpoint calls, row XORs. CMS propagation counts
  are not exposed per call by pycryptosat; wall time is the only CMS work proxy besides
  conflicts.

Primary metric: rho = (leaves visited + conflicts) / 2^l per non-decomposable target, mean
over the 8 targets; trend = least-squares slope of log2(mean rho) in l.
Secondary: rho' = (solver calls + conflicts) / 2^l, and rho relative to the same solver's
abs-0 keep arm (the scan done by the SAT solver itself).

Subcube cost (alpha): per non-decomposable target and j = 0..l, 2 seeded random subcubes (first
l - j coordinates in index order set at random; 1 subcube for j = l), fresh solver, cap 20 000
conflicts; once both samples at some j are censored, larger j are not run for that target.
alpha(l) = least-squares slope of log2(mean conflicts) against j over j >= 2 for which no sample
is censored (needs >= 3 such j). Reported with conflicts / 2^j at each j. The alpha used in the
rule is the one at the largest l that has it.

## Decision rule (fixed)

- KILL: for every CMS budget arm (keep and discard, std order) rho >= 1 at the largest l run,
  and alpha >= 0.9.
- ALIVE: some arm has rho < 0.7 at the largest l with a negative trend over at least 3 sizes.
- Otherwise INCONCLUSIVE.
The engine arm E cannot go below 1 by construction when it prunes only at depth l; it is
reported, not used in the rule.

## Validation before measurement (`validate.py`)

Cell l = 3 (and l = 4 for the answer checks): 24 random + 8 planted targets. Pair-table truth
= enumeration of all signed triples; CMS free solve (plain, dual), CMS DFS, engine DFS agree
with it; every SAT model lifts to curve points with signs summing to R; per-x1 leaf answers
(dual, engine) equal the enumeration of decompositions that use that x1.

## Limits fixed in advance

One core per process, each process at most 10 minutes (deadline 540 s; rows not reached are
listed as NOT_RUN), about 75 minutes of wall time in total for the whole pilot.
