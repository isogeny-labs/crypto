# Idea 10: bit-sliced multi-target propagation. Pilot protocol

Written 2026-11-14 before any measured run. Pilot only: toy sizes, shared
4-core container, one core per run. Every number produced is an unverified
pilot assertion.

## Problem instance (same for every arm)

- Field F_{2^31} = F_2[x]/(x^31 + x^3 + 1). Curve y^2 + xy = x^3 + x^2 + b,
  b fixed (0x2f1a3c5b reduced to 31 bits).
- Factor-base x-set: V = F_2-span of l random elements (seeded, rank l).
- Decision problem ("inner m = 2 problem", the leaf of the linear 2+1 scan
  for m = 3): given a target x-coordinate X, count ordered pairs
  (x1, x2) in V x V, x1 != x2, with
  S3(x1, x2, X) = (x1 x2 + x1 X + x2 X)^2 + x1 x2 X + b = 0.
  All arms count to completion (no early exit), so unit counts do not depend
  on where a solution sits. All arms must return the same count per target;
  a mismatch voids the run.
- x-level only in the compiled arms (no rational-point filter in any arm).
  A separate Python check builds real curve points, plants R = P1 + P2 with
  x(Pi) in V, checks S3 = 0 and that the pair found lifts to points summing
  to +-R, and compares against brute force on the smallest cell.
- The m = 3 scan for ONE outer target produces 2^l inner targets
  X_j = x(R - P3_j), so a multi-target batch of the inner problem is the
  realistic workload even for a single outer target.

## Sizes, targets, seeds

- n = 31 fixed (so the slope isolates l). l in {4,5,6,7,8,9,10}.
- Per cell: 512 targets = 256 planted (X solved from a random pair in V) +
  256 uniformly random X (non-decomposable with probability about
  1 - 2^(2l-32)). Classes kept in separate 64-lane batches and reported
  separately.
- Seed: splitmix64 from seed 20261114 + 1000*l (V), +1 (targets). Three
  repetitions of every timed arm, interleaved arm order A,B,C,D,E per
  repetition; minimum wall time reported, all three kept in results.json.

## Part (a) arms (standalone Rust, rustc -O, target-cpu=native, one thread)

1. `enum_scalar`: for every ordered pair evaluate S3 per target (clmul field
   arithmetic). Unit: tuple evaluations = 2^(2l) per target.
2. `enum_bitsliced` (the honest enumeration baseline): 64 targets per u64
   lane word. Per batch the target-dependent linear maps are precomputed as
   bit planes; Gray code over x1 and x2; a 16-plane filter (false-positive
   rate 2^-16 per lane-tuple) followed by exact scalar verification of
   survivors. Unit: counted u64 word operations per target.
3. `scan_scalar` (the linear 2+1 leaf, strongest scalar baseline): for each
   x1 in V solve the quadratic in x2 over F_{2^31} (half-trace table, batch
   inversion), test membership in V. Unit: leaf solves = 2^l per target.
4. `scan_bitsliced` (TREATMENT, "bit-sliced SAT-style decide"): for each x1,
   the system in the bits of x2 is F_2-affine with target-dependent
   coefficients. 64 targets per lane word; the (l+16) x (l+1) system is kept
   in bit planes, updated by Gray code in x1, and decided by constant-time
   bit-sliced Gauss elimination (the XOR-propagation of a Gauss-in-the-loop
   solver, executed on all lanes at once); survivors verified in scalar.
   Unit: counted u64 word operations per target; lane leaf solves = 2^l.
5. `join` (context baseline, not bit-sliced): enumerate unordered pairs once,
   solve for the <= 2 values X with S3 = 0, look them up in a hash table of
   all T targets. Unit: quadratic solves / T per target. Reported because it
   is the standard way to share work across targets.

Metrics. Primary: counted units per target (word ops for bit-sliced arms,
tuple evaluations / leaf solves for scalar arms) and fitted bits per l
(least squares of log2 unit vs l). Because bit-slicing is a constant-factor
technique and the scalar and lane units are not commensurable, the ratios
between arms are taken on wall nanoseconds per target (min of 3), and the
unit counts are reported next to them.

## Part (b) arms (python-sat, CaDiCaL 1.9.5 with an observing IPASIR-UP
propagator that only records decisions; Glucose/MiniSat for counts)

- Encoding E1 (target-specialised): variables a_i, b_j (bits of x1, x2 in the
  basis of V), t_ij = a_i AND b_j, and 31 XOR rows whose coefficients depend
  on X, cut into width-4 chunks with auxiliary variables in a fixed order.
- Encoding E2 (universal): X bits are variables fixed by assumptions; the
  clause set is identical for all targets.
- Symmetry x1 <-> x2 is not broken in either (same for all targets).
- Sizes l in {4,5,6,7,8}; K = 8 planted + 8 random targets per cell (the first
  8 of each class of the part (a) generator, regenerated in Python with the
  same splitmix stream), identical solver configuration and seed.
- Statistics:
  (b1) decision-trace divergence: for every pair of targets in a cell, length
       of the common prefix of the decision-literal sequences, divided by the
       mean trace length (all variables; and projected on the a, b variables).
  (b2) target-dependence of propagation: at up to 200 sampled decision stacks
       of each run, literals implied by unit propagation on the
       target-independent clauses only (AND gates) divided by literals
       implied on the full formula.
  (b3) E2 only: conflicts per target when the K targets are solved
       incrementally in one solver (learnt clauses are target-independent and
       are kept) versus a fresh solver per target. One variable changed:
       reuse of the solver state.
- Baseline for (b): fresh, untreated solver per target on the same encoding
  (conflicts, decisions, propagations).

## Claim under test

Sharing the search over the summand bits across 64 targets in a machine word
lowers the per-target cost of deciding the PDP leaf below what bit-sliced
enumeration already gives, and a material part (>= 10%) of a CDCL run is
target-independent.

## Predictions (written before running)

- P1: `scan_bitsliced` has slope about 1 bit per l and `enum_bitsliced` about
  2 bits per l, so their ratio falls with l and crosses 1 somewhere in
  l = 6..9. This would come from the scan (2^l vs 2^2l), not from bit-slicing.
- P2: `scan_bitsliced` / `scan_scalar` (wall, per target) is a constant
  between 0.3 and 3, flat in l. Bit-slicing adds no exponent.
- P3: (b1) common decision prefix is below 10% of the trace under E1 and E2;
  (b2) target-independent share of implied literals is below 30%.
- P4: (b3) incremental reuse changes conflicts per target by less than 2x.

## Kill criterion (fixed)

The idea is killed if EITHER
- K1: at the largest l measured, wall time per target of `scan_bitsliced` is
  not below `enum_bitsliced`; OR
- K2: the median common decision-prefix fraction (b1, all variables, E1) is
  below 10% in every cell.

Additional reading rule, fixed now so it cannot be tuned afterwards: even if
K1 is not met, bit-slicing is credited only with the ratio
`scan_bitsliced` / `scan_scalar`; the slope difference between scan and
enumeration is yesterday's linear 2+1 result and is not attributed to this
idea. A lead worth a full study would need that ratio below 1/8 at n = 31
with no upward trend in l, and the `join` arm not below it.

## Not covered by design

m = 4 / S4; n > 31 (lane cost of field maps grows with n, scalar clmul does
not); real WDSat or CryptoMiniSat lane-sharing; early exit on first solution;
AVX2 256-lane words; point-level (y) filtering inside the compiled arms.
