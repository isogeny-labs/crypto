# Exact execution, version 1

This is an execution amendment to E00–E17. The original program's full proof,
solver, transport and complete-cost criteria remain in force. Structural and
bounded diagnostics below are explicitly separate from those criteria.

## Inputs and source

The fixture-preparation source/protocol was committed at
`d86eb011034066532d3645369a783f44be158b1c` before preparation. The manifest
`execution-v1/fixtures.json` fixes all 13 curves, field moduli, tower/flat
conversion generators, orders, points and rejected candidates. Its eight ladder
fixtures have A=-3 and the planned four field sizes. No live or secret target is
used. Register the emitted identities and commit this manifest/protocol before
the following curve executions. Each output directory must be fresh.

## Exact phases

- **E01:** exhaustive Fp2 trace, Frobenius and anti-invariant census on the two
  structural fixtures plus all three even/odd-composite controls. Check every
  trace fiber, rational doubling image and rational two-torsion intersection.
- **E02–E04:** H=2,4,8 on all ten prime-order fixtures. Enumerate coordinate
  boxes, slope/real-coordinate boxes and norm-x restrictions; compare the full
  slope witness set, including v=0, with the rational quartic control. Norm-y
  and norm-power arms use complete extension populations where p<=827 and
  explicitly named coordinate-box intersections on larger fixtures. Inapplicable
  H-power arms remain rows. The unrestricted larger norm-y/power arms are open,
  not silently claimed by the intersection diagnostics.
- **E05:** on both structural fixtures, a0+a1*x+a2*x²+y with a_i in {0,1,-1},
  a not zero. Factor by exact Fp2 root division, including multiplicity; retain
  unsplit degree-three/four factors as incomplete. Check complete divisors and
  distinguish rational zeros, generic quadratic places and principal pure-twist
  pairs. The pure-pair control includes every anti-invariant conjugate pair.
- **E06:** m=2,3,4 and B=8,12,16 for every emitted base. Preserve undersized
  cells. Enumerate every unordered signed multiset, deduplicate coefficient
  rows, compare coverage to an independent iterated-sum oracle, compute rank
  modulo r and test actual target-row span and scalar recovery for every
  decomposable target. This is a complete toy census, not a uniform-target
  performance estimate. Cache only byte-identical projected bases.
- **E09:** all rational points translated by all three nonidentity two-torsion
  points in the cubic extension. Verify doubling and Frobenius descent and
  test whether each translate remains rational. This checks the torsion
  mechanism; a solver cost reduction still requires E07–E08.
- **E10:** all c in Fp, on all ten prime-order fixtures, for the explicit
  sextic family. Check f(c), nonsingularity, complement counts, j, and an actual
  Fp coefficient isomorphism to the requested twist. Keep same-j-only and
  trace-only controls. Freeze the first successful c before E12.
- **E11:** enumerate the equivariant two-torsion bijections and the degree-three
  companion-Frobenius anti-intertwiners. An admitted three-gluing case still
  needs an explicit quotient, curve and polarization check; the filter alone
  does not complete that original experiment.
- **E13:** exact rational cover images at H=2,4,8, finite/full populations and
  both infinity points. Decomposition/pipeline cost remains a later phase.
- **E16:** exact small-fixture embedding degrees and gcd(r,twist order); for
  all five NIST P-curves check the public-parameter gcd and embedding degrees
  through 1024. No hit means a strict lower bound, not the exact large degree.
- **E17:** bounded integral ingress with d=-64..64 excluding 0 and multiples of
  p, x=-64..64, |y|<=4096 on both structural integer models. Enumerate all
  d*y²=x³+A*x+B solutions in the box, separate square/nonsquare reduction,
  count exact projected coverage and eight independent public toy targets.
  Opposite pairs are trivial controls. General rational height search,
  nontrivial relation independence and rank certification remain open.

## Transfer phase E12

Freeze `E12-selected-covers.json` from E10 before this phase. Carry the original
two-infinity sextic into an explicit monic odd-degree chart over the declared
F_(p^6), using a certified branch root r and lambda=g'(r):
u=r+lambda/X, v=lambda*Y/X³. Use checked Cantor composition/reduction on that
chart. Verify every source and complement rational-point pullback, the origin,
the complement's exceptional infinity images, both norms, [r] on the generator,
and eight scalars. Verify generator addition at every source point.

Enumerate all E×D-derived rational classes when p=59; otherwise check the fixed
256 index pairs (31i mod #E, 67i+1 mod #D). Check norm additivity and associativity
on the first 128 canonical classes. Independently count C(Fp) and C(Fp²) for
p<=827 and compare the genus-two order with #E*#D. A same-host replay is not an
independent CAS/host certificate or a Lean group/chart proof.

## Commands, resources and stops

```sh
BIN=research/twist_routes_1_3_20261021/runner/target/release/twist-route-experiments
$BIN trace research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_TRACE_DIRECTORY
$BIN geometry research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_GEOMETRY_DIRECTORY
$BIN conditional research/twist_routes_1_3_20261021/execution-v1/fixtures.json FRESH_CONDITIONAL_DIRECTORY
$BIN transfer research/twist_routes_1_3_20261021/execution-v1/fixtures.json FROZEN_SELECTED_COVERS FRESH_TRANSFER_DIRECTORY
```

Correctness-only L0 local processes; no wall-time or native-speed claim. Use
at most three concurrent diagnostic processes, 4 GiB per process and a 30-minute
per-command stop. Preserve nonzero exits, partial directories and stderr. Stop
the affected mathematical interpretation on any assertion failure, repair the
implementation under a new source revision, and use a fresh directory.

E07–E08, E14 and E15 require their separate solver/map/pipeline execution
amendments and runnable adapters. They are not replaced by this phase. Preserve
the requested work as open until those experiments actually run. Keep original
larger norm, three-gluing, independent replay and arithmetic-lift gaps visible.
