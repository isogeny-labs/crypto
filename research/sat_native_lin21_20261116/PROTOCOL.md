# PROTOCOL: native 2+1 linear-leaf encoding for the field-product-gate CDCL (pilot)

Written 2026-11-16 before any measured run; committed alone before the first measured row.
Pilot at toy sizes in a shared 4-core container. No benchmark record, leaderboard row or pcc
claim; every result line will be tagged `[pcc:unverified]`.

## Question

`examples/pdp_lin_cdcl.rs` (`LinCdcl`, arm "lin") refutes a non-decomposable 3-summand
point-decomposition target in about 1.2 to 1.5 * C(2^l, 2) conflicts: one or two per PAIR
(x1, x2). Binary S3 is F2-affine in (s = x2 + x3, p = x2 x3) once t = x(R - P1) is fixed, so
only x1 has to be enumerated. Does an encoding with branch priority on x1 only bring the same
solver to about c * 2^l conflicts, c in [1, 4]?

## Frozen code (sha256)

- `examples/pdp_lin_cdcl.rs` 996770f4b4ecbc84c451203f6edce5cb17ac40b44892def19d06ca63bc0d2f47, included UNCHANGED through `#[path]`
- `lin21.rs` 2631df21b8177ac645ac31568f3ec2bbf37c9762bdcc8692a21f08865925e3f1 (field, curve, pair oracle, plain scan, the encoding, harness, selftest)
- `run.py` dd76658fe89c186e883e7a0cbe3d9afa8a29b713e82a5122aa7074e26d3300fd (driver, cell plan, cap)
- `analyze.py` e6e4656404c0c49cb084f0fbbf494e017f379fa102f391c091761390f69a4c42 (tables, decision rule, figure)

Build: `rustc -O lin21.rs` (rustc 1.97.0, standard library only, no cargo, no crate).

## Setting (exact)

- Field F_{2^n}, polynomial basis, n = 3l + 2: (l, n) = (5, 17), (6, 20), (7, 23), (8, 26),
  (9, 29), (10, 32), (11, 35), (12, 38). Modulus: the first irreducible in the order
  "trinomials x^n + x^k + 1 by increasing k, then pentanomials by increasing (k3, k2, k1)";
  its bit mask is recorded in every row.
- Curve y^2 + xy = x^3 + x^2 + 1 (a = b = 1, the family of the other session's B0 harness).
- V = span(1, z, .., z^(l-1)) = field elements < 2^l. span(V.V) = elements < 2^(2l-1).
- Problem (called ALG0): are there x1, x2, x3 in V and t in F with S3(x1, t, xR) = 0 and
  S3(t, x2, x3) = 0, S3(u, v, w) = (uv + uw + vw)^2 + uvw + b. Zero is allowed as a summand
  abscissa in every arm (it is the 2-torsion point), because `ChainInstance` allows it and is
  used unchanged.
- Target admission: xR outside V; R rational; sqrt(b)/xR outside V. The last condition removes
  R = (base point) + (2-torsion point), where the algebraic system has twist solutions that do
  not lift. The point at infinity cannot occur as t (xR is not in V).
- Targets per cell, from SplitMix64 seeded with 20261116 ^ (n << 32) ^ (l << 16): uniform
  admissible abscissae are drawn until 16 are non-decomposable by the oracle (kind `uniform`);
  decomposable draws are kept as kind `uniform-dec` and run but stay out of the medians; then
  4 planted targets R = P1 + P2 + P3 with distinct abscissae in V \ {0} (kind `planted`).

## Arms (same targets for every arm)

- `oracle`: exact pair enumeration over all x1 <= x2 in V with both quadratics solved in F.
  Ground truth for every verdict. Independent of the scan and of the solver.
- `scan`: plain native 2+1 scan, no SAT: for x1 in V \ {0}, the roots t of S3(x1, t, xR) = 0;
  per root one n x (3l - 1) F2 elimination for (s, p), then one quadratic for z. Reports
  leaves, field multiplications, inversions, Artin-Schreier solves, shifts and row XORs; wall
  time = median of 21 repeats in one process. This is the reference implementation for time.
- `lin`: `ChainInstance::build(n, l, irr, xR, 1, true)`; `lin-nolex`: the same with `false`.
- `lin21` (primary): the new encoding, priority on the l bits of x1 only.
- `lin21-pu` (secondary): the same encoding with u_0 added to the priority set.

Encoding of `lin21` (15 blocks of n bits: x1, u, A2, B, k, E, w, C, t, s, p, ST, H, zz, Q):
static XOR rows A2 = (x1 + xR)^2, B = xR x1, C = B^2 + b, w = u^2 + u, p^2 + ST^2 + H = b,
zz^2 + Q + p = 0; product gates B = k A2, E = B k, C = E w, t = k u, ST = t s, H = t p,
Q = s zz; unit clauses put x1, s, zz in V and p in span(V.V). So k = B/A2, t = k u, and
S3(x1, t, xR) = A2 t^2 + B t + C = E (u^2 + u) + C: the two roots t differ only in bit 0 of u,
which is the single free bit left after Gauss. Once t is assigned the leaf
p^2 + s^2 t^2 + p t = b is linear in (s, p), and once s is assigned zz^2 + s zz + p = 0 is
linear in zz. Model: x2 = zz, x3 = zz + s. Solver settings as in `lin`: no restarts, default
phase, no conflict budget.

Cell plan (frozen in `run.py`): all arms on all targets at l = 5, 6, 7; at l = 8 `lin-nolex`
only on the first 4 uniform targets plus the planted ones; at l = 9 `lin` on the first 8
uniform targets plus the planted ones and no `lin-nolex`; at l = 10, 11 `scan`, `lin21`,
`lin21-pu`; at l = 12 `scan`, `lin21`. `lin` is not run at l >= 10 (expected above the cap).
Each (target, arm) is one process, at most 2 at a time, killed at 600 s and then reported as
censored. One other CPU-heavy job shares the machine, so wall times carry that noise.

## Checks

- Selftest (`lin21 selftest`, part of every run): exhaustive over all rational targets outside
  V at (n, l) = (11, 3), (8, 3), (7, 3) and a sample at (14, 4): pair oracle = brute-force sums
  of all triples of rational points with abscissa in V (on admissible targets) = scan = all
  four SAT arms; every SAT model lies in V^3, satisfies the chain, and on admissible targets
  lifts to points summing to the target. Non-admissible targets are run too and their
  non-liftable models counted (the twist case).
- Every measured SAT model: abscissae in V, all three lift to rational points, some signed sum
  has abscissa xR, chain equations hold. Every UNSAT verdict: equal to the oracle.
- A run with any disagreement, unverified model or error is reported as such and not used.

## Metrics

Per target and arm: conflicts, decisions, propagations, XOR propagations, GJE calls, gate
linearisations, learnt clauses and literals (solver `Stats`), solve wall time. Conflicts are
reported as multiples of 2^l (scan yardstick) and of C(2^l, 2) (pair yardstick R). Medians and
[min, max] over the non-decomposable uniform targets.

Learnt-clause reading for `lin21`: `Stats.learnt_priority_width`, the histogram of the number
of x1 priority literals per learnt clause (width w < l excludes 2^(l-w) values of x1).
Diagnostic defined now: D = conflicts(lin21) - (1 + #{x1 without roots} + #leaves), the scan's
own count on the same target plus 1 for x1 = 0. D < 0 would mean some x1 value was excluded
without being visited; D >= 0 on every target means learnt clauses pruned nothing.

## Decision rule (pre-registered)

- "Lead holds": the median `lin21` conflicts over non-decomposable uniform targets are
  <= 4 * 2^l at every measured l >= 7, AND at L* (the largest l with at least 4 common
  uncensored non-decomposable targets in `lin` and `lin21`) median(lin) / median(lin21) on the
  common targets is >= 2^(L* - 3).
- "Fails": that ratio is < 4 at L*.
- Otherwise inconclusive.

## Predictions

Exploratory smoke runs were made before this file on seed 999 only (l = 5, 6 all arms, 16 + 4
targets; l = 7 and 9: 2 uniform + 1 planted, lin21/lin21-pu/scan; lin and lin-nolex on one
l = 7 target) and the selftest on seed 77. Seed 999 is not used for measured rows. The
predictions below were written after seeing those smoke numbers.

1. `lin21` median conflicts / 2^l in [1.3, 1.7] at every l (mechanism: 1 conflict for an x1
   without roots t, 2 for an x1 with roots, about half each). On the pair yardstick
   R(lin21) = about 3 / 2^l, halving per unit of l.
2. D in [0, 16] on every target: no pruning by learnt clauses. Histogram: about a third of the
   clauses have width 0 (first-leaf clauses), the rest follow the chronological-backtracking
   pattern (about 2^(w-1) per target at width w), i.e. subtree-exhaustion resolvents derived
   after every x1 below them was visited.
3. `lin` median R = conflicts / C(2^l, 2) in [1.2, 1.7] at every l; `lin-nolex` in [2.1, 2.7].
4. Conflict ratio lin / lin21 at L* (expected 9, else 8) between 2^(L*-1) and 2^(L*); rule
   outcome expected: "lead holds".
5. Wall time, lin21 vs lin: median paired ratio wall(lin) / wall(lin21) >= 2^(l-4) at every
   common l >= 7 and growing with l (smoke: about 8 at l = 5, 10 at l = 6, 40 at l = 7).
6. Wall time, lin21 vs the plain scan: lin21 is slower by a factor between 300 and 5000 at
   every l, with no trend toward parity (smoke: about 1900 at l = 5, 6; 1400 at l = 9). The
   encoding reproduces the scan's leaf count inside the solver; it is not expected to be
   faster than the scan at any size.
7. `lin21-pu` within 15 % of `lin21` in conflicts.

## Not covered

Other fields, curves, subspaces or n/l ratios; m > 3; any change to the solver; CryptoMiniSat
or other external solvers; restarts; per-leaf incremental use; any size where the scan
itself matters for an index-calculus cost. No tuning after measured rows.
