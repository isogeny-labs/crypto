# PROTOCOL — Idea 9: solution-rich regime, budgets, XOR streamlining

Written 2026-11-14 before any measured run. Pilot only, toy sizes, one core,
not claim data. Every number produced under this protocol is `[pcc:unverified]`.

## Claim under test

For m = 3 point decomposition on a binary Koblitz curve over F_{2^n}, with
factor base F = {P : x(P) in V}, V = span(1, z, ..., z^{l-1}), the cost of a
CDCL SAT solver *per relation found* (conflicts summed over all targets tried,
divided by the number of decomposed targets) depends on solution density
(expected decompositions per target, about 2^{3l-n}/6), and

- **Prediction P1**: the l minimising the modelled total cost
  `T(l) = 2^l * cost_per_relation(l) + m * 2^{2l}` is above the balanced point
  l = n/m.
- **Prediction P2**: SAT conflicts per relation at that l are below the SAT
  conflicts per relation at the balanced l (nearest integers to n/3) by more
  than a constant, i.e. the ratio grows with n across the three n tested.

## Kill criterion (fixed now)

Primary (K-lin): the idea is killed if, at every (n, l) cell, the best SAT arm
(min over arms a, b, c below) has conflicts per relation >= the linear 2+1
scan count per relation (expected leaf solves per relation with early exit,
random scan order) at the same l.

Secondary (K-enum), reported separately: same comparison against plain
enumeration (ordered (x1,x2) pairs tested per relation with early exit).

A SAT arm falling below K-enum but not K-lin counts as killed (the linear scan
is the baseline to beat). If SAT falls below K-lin only in cells where the
linear scan already needs < 4 leaf solves per relation (2l >= n, every x3
works), this is recorded as "inconclusive: unit comparison degenerate", because
a conflict and a leaf solve are not the same unit and the cell is useless for
index calculus (LA term 2^{2l} >= 2^n). Units: 1 conflict vs 1 leaf solve vs 1
tuple test are compared as raw counts; propagations per conflict are reported
so the reader can re-weight.

## Instances

- Curve: y^2 + xy = x^3 + x^2 + 1 (Koblitz, a = 1, b = 1) over F_{2^n},
  n in {13, 17, 19}; moduli x^13+x^4+x^3+x+1, x^17+x^3+1, x^19+x^5+x^2+x+1
  (irreducibility checked in code).
- l sweep: n=13: l = 3..7 (n/m = 4.33); n=17: l = 4..8 (5.67);
  n=19: l = 5..9 (6.33). Five sizes per n.
- Targets: random rational points R, drawn with `random.Random(seed)`,
  seed = 1000003*n + 1009*l + i. 24 targets per cell for n = 13, 17; 16 for
  n = 19 (may be cut to fit the 40-minute budget; any cut is reported).
  Targets are NOT planted: decomposable / non-decomposable is whatever the
  draw gives, reported separately. Planted targets are used only for validation.
- Ground truth per target by meet-in-the-middle on curve points: number of
  ordered x-triple solutions, number k3 of x3 in V that occur in a solution,
  number k2 of ordered (x1,x2) that extend to a solution.

## Encoding (one encoding for all SAT arms)

Chain encoding S3(x1,x2,t) = 0, S3(x3,x(R),t) = 0 with t in F_{2^n} free,
S3(a,b,c) = (ab+ac+bc)^2 + abc + 1. Variables: 3l coordinate bits, n bits of
t, n bits of p = x1*x2 (defined by XOR-of-AND constraints). All equations are
degree <= 2 in these; each monomial gets a Tseitin AND variable, XORs are cut
into chunks of 4 literals. No symmetry breaking. Semantics: x-coordinate
decomposition up to signs (+-P1 +-P2 +-P3 = +-R).

## Arms

- (a) untreated: Glucose 4.1 (pysat), default settings, no budget (safety cap
  2,000,000 conflicts; censored rows flagged). Second untreated solver
  CaDiCaL 1.5.3 on the same CNF, reported separately.
- (b) per-target conflict budget B with abandon-and-move-on, B in powers of 2.
  Computed exactly from arm (a) rows (Glucose is deterministic and a conflict
  budget only truncates the run): cost(B) = sum_i min(c_i, B) /
  #{SAT targets with c_i <= B}. Checked against real `conf_budget` runs on one
  cell.
- (c) XOR streamlining: k random parity constraints (each coordinate bit
  included w.p. 1/2, random right-hand side, seed derived from the target
  seed and k) over the 3l coordinate bits, k in {1, 2, 4, 6, 8}, Glucose.
  A streamlined instance that is UNSAT counts as cost without a relation.
  Cost per relation = total conflicts / number of streamlined-SAT targets.
- Baselines per target, from ground truth (counts, not timed):
  linear 2+1 scan with early exit: (2^l + 1)/(k3 + 1) expected leaf solves,
  2^l if k3 = 0; enumeration with early exit: (2^{2l} + 1)/(k2 + 1), 2^{2l}
  if k2 = 0; no-early-exit counts 2^l and 2^{2l} also listed. MITM has the
  same per-target scan count as the linear scan plus a one-off 2^{2l}/2 table.

Order: for each target, arm (a) Glucose, arm (a) CaDiCaL, then arm (c) for
each k, before moving to the next target (interleaved on the same instance).

## Metrics

Primary: conflicts (also decisions, propagations) per target, split SAT /
UNSAT; conflicts per relation per cell. Secondary: wall seconds.
Slopes: least-squares fit of log2(metric) against l, per n, separately for
(i) UNSAT refutation conflicts, (ii) conflicts per relation, (iii) baseline
counts per relation.
Model: T(l) = 2^l * cost_per_relation(l) + 3 * 2^{2l}, with unit weight 1 for
conflicts, leaf solves and tuple tests alike (stated limitation).

## Validation before measurement

1. Field/curve self-tests (inverse, point on curve, associativity sample).
2. Planted targets R = P1+P2+P3 with Pi in F: must be SAT; the model must lift
   to rational points with +-P1+-P2+-P3 = +-R.
3. SAT/UNSAT of arm (a) must equal ground truth (nsol > 0) on every target of
   every cell; mismatches are counted and reported (twist-type spurious
   solutions are possible in principle and would show up here).
4. Smallest cell (n=13, l=3): ground truth cross-checked against direct
   evaluation of S4-chain over all x-triples (brute force over t).

## Amendment 1 (2026-11-14, after timing probes, before any measured run)

Unmeasured timing probes of the encoding (8 targets at n=13, 2 at n=17, not
part of the data) showed the untreated chain encoding refutes at roughly
2^n * c conflicts (n=17, l=5: about 1.0e6 conflicts, 45 s; n=13, l=5: 3.8e5,
17 s), because the solver has to search the n free bits of t. n=17 and n=19
do not fit the budget. Changes, fixed before measurement:

- n in {7, 11, 13}; moduli x^7+x+1, x^11+x^2+1, x^13+x^4+x^3+x+1 (z primitive,
  checked). l sweep: n=7: 1..4; n=11: 2..6; n=13: 3..7.
- Targets per cell: 24 (n=7), 16 (n=11), 8 (n=13).
- Streamlining k in {2, 4, 8}.
- Safety cap lowered to 400,000 conflicts on every Glucose run; a capped run is
  "CENSORED" and counted as cost without a relation (equivalent to budget
  B = 400,000). CaDiCaL (no conflict cap available through pysat) only for
  l <= floor(n/3)+1.
- Brute-force cross-check cell is n=11, l=3. Planted validation on
  (7,2),(7,3),(11,3),(11,5),(13,3),(13,5),(13,6).
- Everything else (claim, predictions, kill criterion, metrics) unchanged.
  P2 "ratio grows with n" is now judged on n = 7, 11, 13.
