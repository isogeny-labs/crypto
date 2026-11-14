# Idea 1 pilot protocol: proof-size slope as a go/no-go gate

Written 2026-11-14 before any measured run. Pilot only, toy sizes, one core.
All numbers produced under this protocol are unverified pilot assertions.

## Claim under test

For non-decomposable m=3 point-decomposition instances (UNSAT), the
DRAT refutation that CDCL finds contains a much smaller *necessary* part:
the number of core lemmas after backward trimming grows with l at a
materially lower rate (bits per l) than the conflict count.

## Prediction

Trimmed-proof slope (log2 core lemmas per unit l) is materially below the
conflict slope (log2 conflicts per unit l).

## Kill criterion (fixed in advance, from the task card)

Kill if BOTH hold on the primary series:
- trimmed slope is within 0.3 bits/l of the conflict slope, and
- trimmed slope is above (m-1) - 0.3 = 1.7 bits/l.

"Alive" needs: trimmed slope more than 0.3 bits/l below the conflict slope
OR trimmed slope at or below 1.7 bits/l, with the fit resting on >= 3 sizes
of l. Anything else (fewer sizes, fits whose bootstrap interval straddles the
threshold) is reported as inconclusive.

## Instance generator

- Field F_{2^n}, n odd prime, polynomial basis, modulus = lexicographically
  first irreducible polynomial of degree n found by a deterministic search.
- Curve y^2 + xy = x^3 + a x^2 + b with a = 1, b drawn from the seeded RNG
  (b != 0), one curve per (n, seed-of-cell).
- Factor-base subspace V = span{1, t, ..., t^(l-1)} (coordinates of x_i are
  the l low bits).
- Target: x(R) for a random affine point R on the curve (random x until
  the curve equation is solvable), from the seeded RNG.
- PDP relaxation actually encoded: exists (x1,x2,x3) in V^3 and X in F_{2^n}
  with S3(x1,x2,X) = 0 and S3(x3,x(R),X) = 0 (chained S4, binary Semaev
  S3(u,v,w) = (uv+uw+vw)^2 + uvw + b). x_i are NOT forced to be x-coordinates
  of rational points (twist values allowed), as in the plain summation
  encoding; UNSAT in this relaxation implies non-decomposable.
- CNF: bits of x1,x2,x3 (l each), X (n), AND variables for x1_i*x2_j,
  s_i*X_k (s = x1+x2), p_k*X_j (p = x1*x2 as a (2l-1)-coefficient polynomial),
  x3_i*X_k; the 2n bit equations are XORs over these, cut into chunks of
  at most 4 literals with Tseitin auxiliaries. No symmetry breaking, no
  other preprocessing (untreated arm).
- Seeds: instance seed = 20261114*1000003 + 1000*n + 10*l, target index
  0..; solver default (CaDiCaL has no seed exposed via pysat; a second
  solver, Glucose 4, is run on the same CNF as a second proof source).

## Validation before measuring

1. Smallest cell (n=13, l=3): the full model set projected on (x1,x2,x3)
   (all-models enumeration with blocking clauses) must equal the
   brute-force solution set of the relaxed problem, for >= 6 targets.
2. Planted targets R = P1+P2+P3 with x(Pi) in V, Pi rational: instance must
   be SAT; fixing the planted x-bits by assumption must be SAT; a model
   found among enumerated models must lift to rational points with a sign
   choice summing to R.
3. Ground truth (SAT/UNSAT) for every measured instance by brute force
   (table of X roots over all pairs, then scan x3), independent of the solver.
4. Every proof used is verified by the trimmer, which also produces the core.
   Amendment before any measurement (2026-11-14): drat-trim source was
   fetched but running a build of downloaded code was refused by the
   session's permission layer, so "drat-trim" everywhere in this file means
   the own backward DRUP checker `rup_trim.c` (RUP only, deletions ignored,
   core-first propagation, backward core marking). Its validation: (a) a
   bogus proof (empty clause for a satisfiable formula) must FAIL; (b) the
   core original clauses alone must be UNSAT by an independent solver;
   (c) at small cells every core lemma is re-checked RUP, in order, by an
   independent pure-Python forward checker against core originals + earlier
   core lemmas.

## Arms (same instances, interleaved per instance)

- B0 counts (no solving): enumeration 2^(3l)/6 tuples; MITM 2^(2l) pairs;
  linear 2+1 scan 2^l leaf solves. These are the reference slopes 3, 2, 1.
- B1 untreated SAT: CaDiCaL 1.5.3 (pysat `cd15`), default options, proof
  logging on. Metrics: conflicts, decisions, propagations, proof lemmas
  (added clauses in the DRAT file).
- B1' Glucose 4.1 (pysat `g4`), same CNF, proof logging on, same metrics.
- T1 trimmed: drat-trim backward check of the B1 (and B1') proof, core
  lemma count (`-l` output), core original clauses (`-c`).
- T2 iterated trim: feed the core lemma file back to drat-trim as the proof
  until the lemma count stops shrinking (max 6 rounds).
- T3 (smallest sizes only, if time permits): re-solve the core original
  clauses only, with core lemmas removed and with core lemmas added as
  clauses, take the proof, trim; and take the minimum trimmed size over
  solver variants (cd15, g4, g3, lingeling). T3 is exploratory and is
  reported separately from T1/T2.

One variable per comparison: T1 vs B1 differ only in trimming the same
proof. Conflicts and core lemmas are the primary metrics; wall time is
secondary.

## Sizes

- Primary series A (n fixed): n = 23, l = 3,4,5,6,7 (l = 7 only if a run
  stays under 5 minutes including trimming). Fixed n isolates l.
- Secondary series B (n = 3l+2 or nearest odd prime above): (l,n) =
  (3,11), (4,13)->13, (5,17), (6,19)->19, (7,23), to see whether the slope
  depends on holding n fixed. Run if time permits.
- Targets per cell: random targets until 8 UNSAT (non-decomposable) are
  collected (cap 40 draws), plus up to 4 SAT (decomposable or planted)
  targets reported separately (no proof for SAT; conflicts only).

## Metric and fit

Per cell: geometric mean and median of conflicts, proof lemmas, core
lemmas, iterated-core lemmas over the UNSAT targets. Slope = least-squares
fit of log2(value) against l over all UNSAT rows (per-instance rows, not
means), with a bootstrap (resampling rows within cells, 2000 draws, fixed
seed 1) 95% interval. Also reported: ratio core/conflicts per cell.

## Budget

About 40 minutes wall in total, one core, any single solve+trim <= 5 min.
What is cut for time is listed in README.md.
