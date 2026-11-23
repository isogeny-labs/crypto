# Idea 8 pilot protocol: target-parametric incremental solving

Written 2026-11-14 before any measured run. Pilot only (toy sizes, one core of a
shared 4-core container). Every number produced is an unverified pilot assertion.

## Claim under test

For the m=3 point decomposition problem on a binary curve, encode ONE CNF in
which the target abscissa x(R) is a vector of n free Boolean variables fixed by
assumptions. Solving a sequence of never-seen targets in the same CDCL solver
(a) costs fewer conflicts per target than a fresh solver per target, with the
gap growing along the sequence, and/or (b, c) learnt clauses and assumption
cores mention few target bits, so that lemmas transfer and a cheap necessary
condition for decomposability exists.

## Prediction (written before the data)

- (a) No material reuse: the ratio INC / FRESH-A in conflicts on the second half
  of the sequence is between 0.7 and 1.3 at the largest l. If there is reuse it
  is the amortised "table of 2-sums" kind (the set {x(P1+P2)} is target
  independent, 2^(2l) facts), which is meet-in-the-middle and not below the
  2^l * poly linear 2+1 baseline.
- (c) Minimal assumption cores are within a few bits of n. Counting null: a
  core of size k certifies that all 2^(n-k) abscissae in an affine family are
  non-decomposable; with decomposable density p about 2^(3l-n)/6 (1/12 to 1/24
  here) a family of 2^j abscissae is all-non-decomposable with probability
  about (1-p)^(2^j), so j is expected to be about 3 to 6 and k about n-3..n-6.
- (b) Most learnt clauses contain several target-cone literals.

## Kill criterion (fixed)

Killed if BOTH hold at the largest l that ran:
1. sum of INC conflicts / sum of FRESH-A conflicts over the second half of the
   target sequence (all targets fresh, pooled SAT+UNSAT, CaDiCaL) is > 0.7; and
2. the smallest deletion-minimal assumption core over the UNSAT targets has
   size >= 0.75 * n ("nearly full target width").
"Alive" needs ratio <= 0.7 at the largest l or a core < 0.75 n that is not
explained by the counting null. Anything else (e.g. ratio <= 0.7 only at small
l, or only on SAT targets) is reported as inconclusive.

## Instance generator

- Field F_{2^n}, polynomial basis, modulus = lexicographically first irreducible
  trinomial/pentanomial found by the script (recorded in results.json).
- Curve y^2 + xy = x^3 + 1 (Koblitz a=0, b=1).
- V = span of l random field elements (rank checked), seed = 20261114 + 1000*l.
- Factor base F = {P in E(F_{2^n}) : x(P) in V}.
- Cells (l, n): (3,11) validation only; measured cells (4,13), (5,17), (6,19),
  (7,23). n is the prime nearest above 3l+1, the usual n ~ m*l regime; n
  changes with l (stated confound). Fallback: the cell l=7 is dropped if one
  calibration solve on a target outside the measured set exceeds 60 s; that
  would be reported.
- Decomposable targets: R = P1+P2+P3 for random base points (signs random),
  rejected if x(R) in V, x(R)=0, or x(R) already used. Non-decomposable: random
  rational points with x(R) not in V, verified non-decomposable by a 2+1
  meet-in-the-middle over real curve points (set of x(P1+P2), probe x(R-P3)).
- Per cell: 12 decomposable + 12 non-decomposable targets (8+8 at l=7 if time
  demands), shuffled into one fixed order with the cell seed. Table-exhaustion
  control: every target is distinct and never seen before by the solver; the
  number of decomposable targets used (12) against the number of distinct
  decomposable abscissae (about 2^(3l)/12: 341, 2731, 21845, 174763) is
  reported per cell. l=4 is at 3.5% and is flagged as the weakest cell.

## Encoding (same for all SAT arms)

Unknowns: c_i in F_2^l (x_i = sum c_ij v_j, i=1..3), t in F_2^n. Constraints:
S3(x1,x2,t)=0 and S3(x3,xR,t)=0 with binary S3(u,v,w)=(uv+uw+vw)^2+uvw+b,
written with p=x1*x2, s=x1+x2, q=x3*t. Bit products are AND gates (3 clauses);
F2-linear equations are cut into 3-input XORs with auxiliaries. Parametric
form: xR = n Boolean variables r; target-dependent terms are AND(r_i, .).
No symmetry breaking, in any arm.

## Arms (same targets, same order, interleaved per target)

- INC (treatment): one solver for the whole sequence, target set by assumptions.
- FRESH-A (primary baseline, isolates the single variable "solver state kept"):
  new solver per target, same parametric CNF, same assumptions.
- FRESH-U: new solver, parametric CNF, target bits as unit clauses.
- SPEC (untreated SAT arm): new solver, CNF regenerated with xR constant.
- Non-SAT reference counts per cell (not run as solvers): enumeration
  2^(3l)/6 tuples; MITM 2^(2l)/2 table + 2^l probes; linear 2+1 scan 2^l leaf
  solves per target (repository baseline from groebner_free_matched_20261113).
Solvers: CaDiCaL 1.5.3 (primary) and Glucose 4 (secondary), via python-sat,
default options.

## Metrics

Primary: conflicts per target (also decisions, propagations); wall time
secondary. Reported per cell, SAT and UNSAT separately: median and sum per arm;
INC/FRESH-A ratio for first half and second half of the sequence; per-position
ratio (reuse curve). Slope: least-squares fit of log2(median UNSAT conflicts)
on l per arm, bits per l.

(b) Glucose 4 with DRUP logging, fresh solver + assumptions per target:
per learnt clause, number of literals on r variables and on the "target cone"
(r variables and AND gates with an r input); histogram and the share with 0.
(c) UNSAT targets: size of the solver's assumption core, then deletion-based
minimisation (one pass over the core, two bit orders: ascending and a seeded
shuffle); minimal size reported against n. Up to 8 UNSAT targets per cell.

## Validation before measurement

- l=3, n=11: solver verdict equals the real-point MITM ground truth on 40
  random rational targets plus 10 planted ones; every SAT model lifts to
  rational points P1,P2,P3 with x(+-P1+-P2+-P3) = x(R).
- In the measured cells every verdict is compared with ground truth and every
  SAT model is lifted; mismatches are counted and reported, not dropped.

## Not attempted

m=4; native-XOR CryptoMiniSat; cores over a different basis of the target
bits or over target powers; inprocessing options; more than one sequence order
unless time allows (then a second order with seed+1 at the same targets).

## Amendment 1 (2026-11-14, before any measured run)

A smoke solve (not a measurement; one l=3 target) of the chained encoding above
took 74,910 conflicts: with t free, plain CNF does not solve the F2-linear
system in t by unit propagation. Changes, all made before the first measured
row:

- Encoding: t is eliminated by the resultant of the two quadratics in t,
  S4 = (A C' + A' C)^2 + (A B' + A' B)(B C' + B' C) with A=(x1+x2)^2, B=x1 x2,
  C=B^2+b, A'=(x3+xR)^2, B'=x3 xR, C'=B'^2+b. Every auxiliary is then forward
  determined by unit propagation from (c, r). Products are materialised to n
  bits when they have more than n terms.
- Symmetry breaking c1 <= c2 <= c3 (lexicographic) is added in ALL SAT arms.
- Sizes: smoke timing (l=5: about 5 s and 11k conflicts per UNSAT solve;
  l=6: more than 60 s per solve) leaves no room for l=6 or l=7 in the budget.
  Measured cells are (l,n) = (3,11), (4,13), (5,17). l=3 has only about 48
  decomposable abscissae and is reported but flagged as failing the
  table-exhaustion control. Largest l for the kill criterion is therefore 5.
- Targets per cell: l=3: 6+6, l=4: 12+12, l=5: 8+8 (decomposable +
  non-decomposable).
- Glucose 4 runs INC and FRESH-A only (FRESH-A with DRUP logging for (b)).
- Cores: one incremental Glucose 4 solver per cell; up to 6 (l<=4) or 3 (l=5)
  UNSAT targets; two deletion orders. Known conservatism: if a freed family
  contains xR in V, the degenerate case A=A'=0 makes the query SAT, which can
  only make reported minimal cores larger.
- The number of distinct decomposable abscissae per cell is estimated as
  (fraction of random rational points found decomposable while drawing the
  non-decomposable targets) * (number of rational abscissae, about 2^(n-1)).
Kill criterion, arms, metrics and seeds are unchanged.
