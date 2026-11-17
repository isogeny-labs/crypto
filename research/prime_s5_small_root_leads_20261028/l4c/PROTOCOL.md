# L4-C: small 3-decompositions in the gap, exact census (pre-registration, 2026-10-28)

**Status.** Registered before any timed run. Toy curves only; no security
claim. **Class: measurement.** Nothing here is an improvement under the
success criterion. It tests the null assumption behind §2.4 of
`RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md` and the README's L4-C record
(Rank 4), with three changes to that record, made before any data:

1. **Census, not sampling.** Every target `R ≠ O` of the group is counted
   exactly, by enumerating all sums of three signed factor-base points once
   and sorting. The record's 10⁵ random targets per cell are replaced by all
   `r − 1` of them.
2. **Counting unit fixed.** `N_R` is the number of multisets of three signed
   points of `±F` that contain no pair `{P, −P}` and sum to `R`. Generically,
   one such multiset is one vanishing of `S₄(x₁, x₂, x₃, x(R))` with `x_i` in
   the factor base. `λ = Σ_R N_R / (r − 1)`; its predicted value is
   `≈ (2|F|)³ / (6r)`.
3. **Null 2 dropped.** A random polynomial with `S₄`'s support has an
   expected root count per target of about `B³/(6p)`, below `10⁻²` at every
   registered cell up to δ = 1/4. A panel large enough to see one root per
   cell costs more than the whole study. It is recorded as not run, for that
   reason.

## Code and curves

- [`examples/semaev_gap_census.rs`](../../../examples/semaev_gap_census.rs),
  native Rust.
- For each `bits ∈ {26, 28, 30}`, two curves:
  - `a = −3`: the largest prime `p < 2^bits` with `p ≡ 3 (mod 4)`, and the
    first seeded random `b` that gives prime order;
  - `a = 0`: the largest such prime with also `p ≡ 1 (mod 3)`, and the
    smallest `b ≥ 1` that gives prime order.
- Prime order is found by baby-step giant-step on a random point in the
  Hasse interval, requiring a unique prime answer. The program prints
  `p, a, b, r` in every line.
- The factor base `F` is one point per abscissa `x ∈ [0, B]` with
  `B = ⌊p^δ⌋`, excluding 2-torsion (there is none at prime order).

## Cells

- δ ∈ {1/24, 1/12, 1/8, 1/6, 1/5, 1/4, 0.3}. The 0.3 row lies past the gap;
  it is included because λ there is near the region where a dispersion index
  has power. Cells with `|F| > 256` are skipped (memory) and listed as
  skipped.
- Per cell: the small-x factor base, and five null-1 factor bases. Each null
  1 is a uniformly random set of `|F|` points with distinct abscissas, from
  seeds derived from `20261028`.

## Measurements (per cell and factor base)

A finite factor base has distinct triple sums unless its points satisfy a
short additive relation. So `N_R ≥ 2` comes from relations, and relations
appear in clumps: one 4-term relation `±P_a ± P_b = ±P_c ± P_d` gives 6
pair-sum collisions, and each one propagates to many triple-sum collisions.
A per-target Poisson baseline is therefore the wrong null. A smoke run at 20
bits (seed 1, not a result) showed this, before registration: zero
collisions at λ = 0.0035 where Poisson predicts 6. The registered
comparison is with null 1. The birthday value `C(M, 2)/(r − 1)` for `M`
sums is printed only for scale.

- `multisets`, `λ`, the dispersion index `Var(N_R)/λ` over all `R ≠ O`,
  `ge2` (the number of `R` with `N_R ≥ 2`), `ge3` and `max_n`.
- `pair_collisions`: colliding pairs of signed pair-sums, i.e. 4-term
  relations, about 6 per relation. Printed with `pair_expected`.
- `triple_unshared` (**primary**): colliding pairs of triple sums whose two
  decompositions share no base abscissa, i.e. genuine 6-term relations,
  which a better-than-generic S₄ oracle would exploit. Also
  `triple_collisions` (all) and `ge2_shared`.

## Registered readings and falsifiers

- **Null claim (prediction).** In every cell, the small-x values of
  `triple_unshared`, `pair_collisions` and `ge2` lie inside the range of the
  five null-1 values, widened by 3σ of those five (σ from the five; where
  all five are equal, the band is that value ± 3√value, a Poisson width).
- **Falsified (a positive lead)** if, for the same δ and curve shape, the
  small-x value lies outside that band at two or more sizes, on the same
  side. Report that with the δ and the shape.
- **The `a = 0` confounder.** The order-3 automorphism `x ↦ ωx` does not
  preserve `[0, B]`, so it is not expected to matter. If `a = 0` deviates
  and `a = −3` does not, that is reported as shape-specific, not as gap
  structure.
- **Limits.** These are toy sizes. Transfer to 40–256 bits assumes the
  readings are stationary in δ, which the three sizes check, weakly. At
  δ ≤ 1/6, λ is so small that `ge2` is 0 or a handful. Those cells test only
  the gross absence of structure.
