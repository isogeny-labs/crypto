# Protocol: coupled index-calculus obligation programme

Status: frozen analysis and admission rules; individual scientific panels remain
subject to their own preregistration and immutable input hashes.

## Scope

The unit of evaluation is a complete route on one parameter family, not an
isolated solver, factor base, or relation generator. Every route is evaluated
against the six obligation identifiers in `obligations.json`.

## Common parameterization

For fixed arity `m`, define `s=log2(p)/m` and factor-base scale `B=Theta(2^s)`.
Record target-independent setup exponent `a` and natural-query exponent `e`
separately. Up to polynomial factors, collection costs

`2^(a s) + 2^((e+1)s)`.

An asymptotic fixed-arity admission requires both strict inequalities
`a<m/2` and `e<m/2-1`. Values at the boundary do not pass. Directly fitted
`p` exponents must still include every phase and are compared with `1/2` only
in an explicitly scoped end-to-end evaluation.

## Population and censoring

- Natural targets are the primary population. Planted, satisfiable controls,
  exact UNSAT controls and curve-free/shape-matched nulls are separate strata.
- Timeouts, OOM, degree caps, unknown solver results, unproved UNSAT and missing
  root extraction are censored failures. No completed-row subset may pass.
- Setup reuse is reported separately from the primary cold single-target route.
- A fixed probe budget is required for yield variance. Adaptive stopping data
  may measure a completed pipeline but may not estimate an unconditional
  per-target count distribution.

## Required fields

1. Geometry: ideal, coefficient field, field equations, saturation/exceptional
   branches, dimension or unit-ideal status, fibre degree, rationality and root
   extraction.
2. Solver: maximum degree, rows, columns, nonzeros, arithmetic operations,
   memory, wall time, failures and a matched topology/shape null.
3. Yield: exact probes, zero-hit targets, verified rational decompositions,
   multiplicity, count vector, mean, variance/Fano factor and solver failures.
4. Rank: rows and rank after every accepted row, support histogram, dead columns,
   connected components/2-core, terminal full-rank status and independent replay.
5. Interface: setup and online classification under G0, G1, GA or C, including
   a same-cost simulation or the precise coordinate predicate that prevents it.
6. Cost: factor-base construction, advice/tables, failed and successful queries,
   relation verification, filtering, sparse linear algebra, target descent,
   recovery verification, orchestration, peak memory and success probability.

## Cross-obligation gates

- First-fall, last-fall or solving degree never substitutes for matrix/work data.
- Mean yield never substitutes for target-wise variance or rational yield.
- Relation count never substitutes for rank.
- `zero_dim=true` must be typed as a nonempty finite ideal, the unit ideal, or an
  engine diagnostic. These meanings are not interchangeable.
- A coordinate-selected factor base does not make a generic online pair search
  non-generic. Setup and online mechanisms are classified separately.
- No route is `ready` unless every required obligation is proved or supported by
  an admitted scaling experiment, and the complete cost gate passes.

## Next panels

### Tensor theorem/code audit

Map every load-bearing theorem object to code; add an arbitrary-precision
symbolic guard evaluator; exhaustively compare tiny networks with direct exact
contraction; and use mutations for mode orientation, precision, taper and packet
incidence. Finite parity is bounded validation, not proof of asymptotic cost.

### Fixed-budget yield and rank panel

Reuse the atlas's `BATCH-79a580` target-count design. Freeze target vectors,
probe count, factor-base points, solver status policy and size-matched random
base. Preserve the entire verified relation population, then compute yield
variance and incremental rank from that same population. Do not stop on rank.

### Arity-three hybrid solver

Implement a typed chained-S3 IR and a source-bound `Fp` trail theory first.
Every propagation/conflict needs a replayable explanation. Only after exact
small-field model-set equivalence, pinned solver hashes, representation counts,
resource preflight and immutable heldout inputs may the `s=6..14` panel freeze.

### PKM theory and confirmation

Primary task: construct a degree-bounded pseudo-solution or another lower bound
that sees the two-dimensional Kummer curve, and derive a width/work consequence.
The `m=4,N=20` compute cell is secondary and requires its existing high-memory
protocol, a matched successful target, complete root extraction and cost custody.

### Wagner filter

Run the already named O2 bucket second-moment statistic `W` across the covered
families and sizes. Advance only if its decay is slower than `1/sqrt(N)` and the
candidate still passes density, enumeration, memory and interface gates.

### Xedni selected-lift bridge

Fix an explicit lift selector and curve family before examples. Formalize the
selected-height/global-minimal-discriminant statement, attempt proof, and retain
the first counterfamily. A one-shot relation bound cannot be promoted to an
adaptive runtime bound without a separately proved conditional-hazard bridge.

## Execution rule

New CPU experiments use the repository's EKS Spot workflow after the applicable
protocol is frozen. Environment readiness is operational evidence only; it does
not satisfy a scientific gate.
