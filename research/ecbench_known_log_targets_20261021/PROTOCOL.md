# Arbitrary-target known-log factor bases

Protocol frozen before measuring; baseline source: `43e01b10e0e5ec6bb74a1cd24f283ef006da586d`.

## Question and scope

Does a target-blind known-log two-summand construction reduce the complete
one-target recovery cost against the existing abscissa index-calculus pipeline?
How does that comparison change against matched negation rho, across generators,
and on synthetic curves with a nontrivial cofactor?

The primary unit is cold group-addition equivalents (GAE), additions plus
doublings, normalized as `S = GAE/sqrt(r)`. The negation generic reference
constant is `sqrt(pi/4)` in S; the measured reference is `rho.negation` in
the same executable and on exactly the same target. Native operations are
listed for every arm and repriced at 0, 0.1 and 1 GAE per counted operation.
Rank arms only if the ordering holds throughout that band. No fastest-tool,
asymptotic, or standard-curve recovery conclusion follows from this study.

## Frozen design

- Primary panel: registered prime-order curves constructed by
  `find_prime_order_curve(bits, 59297)` at bits 12, 16, 20 and 24; 8 independent
  public hash-to-subgroup targets per curve; 5 measured rounds and 1 warmup.
  These are separate one-target workloads, with cold setup charged in each.
- Holdout: the same curves and arm settings with a disjoint target seed.
- Factor-base size: 32 unsigned pairs (64 signed points) on every arm;
  no selection using target answers. Known-log base seed 20261021.
- Baseline: unchanged `ic.pipeline`, `prime-abscissa:size=32`, negation-folded
  two-summand MITM, guarded `walk` target stream and `incremental-gauss`.
  Matrix rank, dependent rows and total elimination cost remain in the records.
- Candidate: new `ic.known_log_mitm2_v1`, seeded distinct ±[s]G pairs, the
  same native folded pair-table primitive, and the injective query stream
  `Q + [a+t]G`. Every seed scalar multiplication is charged. A full-point
  checked relation determines the target immediately; unknown base logs and
  matrix work are zero, recorded explicitly rather than requiring full rank.
- Both baseline and candidate have an identical-method A/A control; rho is a
  separate reference. A/A operation counts must agree exactly at paired seeds.
  Wall time needs L2 and an admitted comparison; otherwise it is descriptive.
- Per-execution timeout: 120 seconds for every arm. Candidate pair-construction
  cap: 1,100 attempts, checked before allocating; target cap: 2,000,000 queries
  and at most one complete subgroup cycle. Baseline trial cap: 2,000,000.
  Record process peak RSS and the exact shared host/resource conditions.
- Generator panel: one registered 16-bit curve, generators [c]G for c in
  {1, 2, 7}, 8 public targets and 5 measured rounds. Include rebuilding the
  known-log base from the current generator and transporting it with
  `base_multiplier = c^-1 mod r`. The latter preserves its points and query
  anchor. Verify converted scalars, point-set hashes and replay counts.
- Cofactor panel: construct and register synthetic prime-field representations
  using `toy_curve_with_cofactor(16, 20261021, h)` with h in {2, 4, 8}.
  Recount each full group independently using Legendre symbols and choose the
  first nonzero cofactor-projected point as its prime-subgroup generator.
  Use 8 public subgroup targets and 5 measured rounds, preserving exact
  field size, group order, subgroup order, cofactor and generator. Add the
  existing `prime-projected-abscissa` pipeline as a subgroup-matched control.
  Cross-curve cofactor differences are observational, not a causal h-fold gain.

The native study utility writes exact specs and a manifest before running;
all their hashes are retained. Target seeds: 2026102101 (primary),
2026102102 (holdout), 2026102103 (generator panel), 2026102104 (cofactor panel).
Algorithm seed: 20261021, shared across paired arms by ecbench.

Pre-run fixture amendment: inspection found no small registered prime-field
representations covering all three cofactors. The explicit bounded constructors
above replace the proposed registry selection before any measurement. Curve
construction and target derivation are fixture preparation, excluded equally
from each solver's interval; factor-base construction remains charged.

## Accounting and validation

The candidate prepares its base and table through an API that accepts no target.
The query API receives only a point, a seed and a budget. It records preparation,
table construction, query generation, decomposition, witness checking, scalar
recovery and internal verification separately. The ecbench runner verifies
`[d]G = Q` independently; public workloads have no planted scalar. The five
exclusive online phase clocks must sum exactly to the complete online interval.

Record pair attempts, distinct signed sum classes, collisions, queries, useful
target relations, matrix rank/dependencies where applicable, native counters,
RSS and every failure. Report cold costs first and online costs separately.
No cross-target search state or rho collision table is reused. Actual amortized
multi-target performance is outside this one-target experiment; do not infer it
from setup-excluded costs.

Run the native release library tests, touched example tests and ecbench
integration tests. Audit each completed session with `verify --replay-all`;
retain SHA-256 replay receipts. Recompute per-target and per-curve totals from
the sealed records with the native summarizer. A same-host replay is identified
as such; independent host validation remains a separate gate.

## Decision and stop conditions

A measured improvement against the existing IC baseline requires every paired
workload to verify and a lower candidate cold cost at all three pricing weights;
report per-curve and per-target exceptions. A rho comparison has the same rule.
Confidence intervals are computed by ecbench on paired workloads, never by
treating all correlated rows as independent. Failed/timeout rows remain and make
the relevant comparison incomplete. A/A mismatch, invalid witness, count replay
failure, or missing phase/counter blocks the corresponding conclusion.

Stop an execution at the frozen timeout/budget; retain it without replacing its
seed. Fix implementation defects in new versioned sessions, preserving failed
ones. A finite-size total-cost result is an engineering measurement. Four small
sizes do not establish an asymptotic exponent or a P-192/P-224 speedup.

## Delivery and unresolved gates

Commit native code, frozen specs, raw sealed sessions, commands/build provenance,
replay receipts, a result report, graph and PDF in the same PR. The leaderboard
and benchmark index require independent validation under `docs/VALIDATION_GATE.md`;
unvalidated study evidence stays here. The fastest published external ECDLP
implementation is not available on this host; this experiment compares the
repository's matched implementations, and makes no fastest-tool claim.
