# Integer-factoring mechanisms for index-calculus work

## Outcome

This package imports the reusable ideas from
[`1337isnot1337/integer-factoring`](https://github.com/1337isnot1337/integer-factoring)
at commit `5aebed85b2978ec14868dde354660cb38f55bcb3`. It does not copy a
factoring algorithm or claim an ECDLP improvement.

One mechanism is ready to use as an index-calculus design contract:
**canonical residual identities, unit-aware cancellation and independent
replay of every merged row**. The repository's residual and double-large-prime
collectors already implement most of that seam, so duplicating them would be
counterproductive. The imported contract tightens how future residual types
must be keyed and tested.

The other mechanisms are retained as conditional leads or explicit negative
transfers. In particular, the factoring repository's query reductions are not
wall-clock speedups, and its prime-residual certificate has no direct analogue
for elliptic-curve points.

The exact upstream files and SHA-256 hashes are in
[`SOURCE_MANIFEST.json`](SOURCE_MANIFEST.json). The machine-readable transfer
gate is [`ASSESSMENT.json`](ASSESSMENT.json).

## What transfers

| Upstream mechanism | IC disposition | Reason |
|---|---|---|
| Composite residual-ideal collision keyed by `(R, canonical root)` | **Adopt the soundness contract** | Equality of a coarse residual value is insufficient; the canonical identity, orientation/unit and ambient context must all be bound before cancellation. |
| Retain original integral rows; use exact replay after elimination | **Adopt** | Prevents a rational-span or quotient representative from being mistaken for a genuine relation in the order-`r` row module. |
| Prime-cofactor saturation for a `Y`-rough residual `R <= Y^2` | **No direct port** | Its proof uses prime divisors, splitting roots and a complete prime base. An elliptic residual point has no corresponding primality or smoothness certificate. |
| Selective promotion of only cheap residuals | **Conditional IC lead** | Repeated verified residual vertices may be promoted to columns or known-weight multiplier families, but matrix width, setup, rank and individual-log cost must be charged. |
| Small-word Fourier constructor and streaming query workspace | **Accounting/implementation pattern** | Bounded words and streamed queries are useful engineering constraints, but the constructor is for integer multiplication and supplies no elliptic relation oracle. |
| Hankel/Prony recovery from exact divisor moments | **Conditional interface only** | Sparse-support recovery is exact once moments exist; no uniform IC moment provider that is cheaper than relation collection is supplied. |
| Pole/Cauchy circuit compression | **Conditional table-compression lead** | It may describe compact pair-grid evaluations, but the upstream singular normalization and scalar-extraction obligations remain open. |
| Noncommutative crossing and F4 hyperoval profiles | **Boundary-analysis input only** | The best retained F4 rate is still above the required threshold and has no typed map to the IC relation tensor. |
| Multiplication network to clique evaluator | **No direct solver transfer** | A fast table/count query does not by itself generate verified independent relations or an individual logarithm. |

## The imported residual contract

The upstream factoring identity has the form

```text
target class = fixed-base word * residual_ideal(key)^orientation
```

where the residual key is `(norm, canonical arithmetic root)`. Matching only
the norm is unsound. Two records with the complete key eliminate the same
temporary generator with a unit coefficient, after which the resulting base
row is replayed as an exact class identity.

The index-calculus analogue is a contract, not a map between the two groups:

```text
aG + bQ = factor_base_row + unit * residual_atom(key)
two compatible records with the same key
    -> eliminate residual_atom
    -> independently verify the complete group equation
    -> insert the verified row modulo r
```

Every future residual key must bind at least:

1. full `curve_uid`, subgroup order and group-model/infinity convention;
2. full factor-base/candidate identity and collector version;
3. the canonical residual atom, not a truncated hash or coordinate alone;
4. the automorphism, Frobenius or sign action used to reach that representative;
5. the corresponding unit or scalar coefficient modulo `r`; and
6. the decomposition family and arity, so unlike partial shapes cannot collide.

A hash may index the table, but equality is decided from the canonical payload.
Two rows merge only when their residual coefficients can be eliminated in the
declared row module. The merged equation must then be replayed independently in
the source curve group before rank insertion. Duplicate filtering and rank are
performed modulo the exact subgroup order `r`.

This is consistent with the existing implementation surfaces:

- `ic_framework::residual` already stores group residuals and turns exact
  collisions into rows;
- `ic_framework::large_prime::LargePrimeCollector` carries one- and
  two-large-prime partials;
- `koblitz_large_prime::WideLargePrimeEliminator` tracks signed Frobenius-orbit
  coefficients; and
- the double-large-prime study independently replayed every merged row before
  it reached the matrix.

The imported idea therefore hardens future extensions; it is not evidence that
the current implementations are unsound and it does not add a new relation
source.

## Evidence already present in this repository

The host results delimit what the imported collision idea can claim:

- The residual-walk study found that generic residual collisions still need
  approximately `sqrt(2 r (B + 1))` residuals to recover a logarithm. Cheaper
  residual updates changed the per-step constant, not the collision count.
- The prime residual collector matched a curve-free carrier: all nine null
  intervals contained one. At fixed small base it had rho's exponent and cost
  roughly `3.8–4.9x` the counted rho reference; native table work was not priced
  well enough to rank it across the declared band.
- The Koblitz double-large-prime graph produced a measured `2.1x` constant over
  an equal-table-size control on one 39-bit rung, but the best cell remained
  `17.7x` counted rho and no exponent was fitted.
- Generator-multiplier factor bases already cover the closest safe analogue of
  selective promotion: many table vertices can share a small number of
  unknown-log columns. Their current protocols require complete-cold,
  held-out, A/A-controlled measurements.

These records mean that a new residual representation must beat matched generic
and equal-resource controls. A query-count reduction or a higher merge share is
only a mechanism diagnostic.

## Upstream finite evidence, preserved without promotion

The composite-residual selector returned verified factors on its finite
controls while reducing leading-norm queries:

| Input | Smooth-only queries | Best reported collision run | Reduction |
|---:|---:|---:|---:|
| 92-bit | 227,593 | 88,234 | 61.2% |
| 124-bit | 3,553,722 | 1,400,316 | 60.6% |
| 144-bit | 8,120,765 | 3,440,025 | 57.6% |

The upstream report explicitly says that decoding, storage and exact linear
algebra make these query ratios different from wall-clock speedups. No uniform
short-window estimate or better factoring exponent follows. Those limitations
remain attached here.

## Why prime-cofactor saturation is not copied as code

The upstream certificate proves a residual norm prime from four facts: every
small split prime up to `Y` is already in the base, the residual is coprime to
that base and the discriminant, a root congruence forces every prime divisor to
split, and `R <= Y^2`. A composite residual would then need two factors greater
than `Y`, a contradiction.

For an elliptic residual point `L`, there is no integer `R`, no prime-divisor
argument and no complete split-prime base. Treating a point bound, coordinate
size or orbit length as the missing primality certificate would be invalid.

The admissible analogue is narrower: promote a residual vertex only after its
group identity and coefficient are verified, and only as an explicitly new
matrix column or a known-weight multiplier of an existing column. This changes
the solver dimension. It needs a frozen policy and an equal-resource control;
it is not a free saturation step.

## Falsifier before implementation

The next implementation slice should be a correctness fixture, not a
performance run.

1. Freeze one small registered prime curve and one small registered binary
   curve, including full curve and factor-base identities.
2. Generate partials whose coarse keys deliberately collide: same x-coordinate
   with opposite sign, same orbit representative with different action
   multipliers, and identical residual bytes under different curve/base IDs.
3. Require the typed key to separate incompatible partials and merge compatible
   ones.
4. Replay every admitted merge as a source group equation and require exact
   agreement with the legacy collector on the compatible subset.
5. Include a deliberately under-keyed arm. It must either reject the adversarial
   fixture or produce a replay failure; silent acceptance refutes the design.
6. Record table probes, canonicalization, bytes retained, merge depth, row
   verification, duplicate filtering, rank work and peak memory separately.

Only after that A0/A1 fixture passes should a held-out three-arm measurement be
considered: current collector, typed-key candidate and an equal-memory/equal-
table control. The primary scope remains cold one-target end to end. Source
reference, relation collection, linear algebra, individual log, recovery and
verification all remain charged.

## Other retained leads

### Sparse moments

The upstream Hankel/Prony construction is useful as a clean interface theorem:
if exact moments of a positive sparse measure are available, its support can be
recovered with exact rational arithmetic and integer root isolation. An IC
version would first need to define a sparse relation-support measure and a
moment provider whose cost is genuinely below enumerating that support. Without
that provider, the moment method merely relocates the relation oracle.

### Pole/Cauchy compression

The circuit-moment results suggest asking whether an explicit pair or bilinear
relation grid admits a compact rational evaluator. The upstream work also
preserves the obstruction: singular normalization needed for factor counting
is not supplied, and direct localization can require prohibitive precision.
Any IC proposal must specify the returned scalar, denominator avoidance,
precision, and how that scalar yields verified rows. A small circuit for an
unusable aggregate is not a relation collector.

### Crossing profiles

Order-sensitive/noncommutative profiles remain a legitimate source of tensor
upper bounds. The retained F4 hyperoval profile has rate below `0.710005861`,
still above `1/sqrt(2)`. The abstract cube profile has no realizing tensor.
These can inform the active boundary-census work, but neither is imported as an
index-calculus exponent claim.

## Narrow verdict

The transfer is **unknown with one checkable next obligation**: implement the
adversarial typed-key replay fixture above and prove that compatible residual
records eliminate to the same verified row as the current collector. Solver
applicability is already present once a genuine row reaches `ic.pipeline`;
correctness and advantage of the new adapter are not established.
