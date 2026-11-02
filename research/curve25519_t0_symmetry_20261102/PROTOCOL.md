# T₀ symmetry in Curve25519-family relation collection — protocol

Frozen 2026-11-02 before any count was computed. Follows
[`../curve25519_ic_baseline_20261102`](../curve25519_ic_baseline_20261102/README.md)
(baseline: cofactor-projected small-x base, `mitm:negation_folded=1`).

## Question

On `E : y² = x³ + Ax² + x`, translating by the 2-torsion point `T₀ = (0,0)`
acts as `x ↦ 1/x`, and `[8](P + T₀) = [8]P`. Does exploiting it in an
`m = 2` pair-table decomposition raise relation yield? Does it improve on
the baseline?

## Mechanism and the bound (stated before measuring)

`E(F_p) ≅ Z/8 × Z/r`. A target `R` lies in the order-`r` subgroup `G`.
Three oracles, all with `N` columns (one per base point up to sign), base
points `P_u` at the smallest Montgomery abscissae `u ≥ 2`:

- **blind**: table keyed by `x(P_i ± P_j)`; it decomposes `R` only when a pair
  sum lies in `G` (coset 0 of 8).
- **t0**: table keyed by `κ(S) = x(S) + 1/x(S)`, which is invariant under
  `S ↦ S + T₀`. A probe with `κ(R)` decomposes `R` whenever a pair sum lies in
  `G` or `G + T₀` (cosets 0 and 4), because `R = ±P_i ± P_j + T₀` is a
  valid relation after the `[8]`-projection. Columns are unchanged.
- **proj** (the baseline's base): `P'_u = [8]P_u`; every pair sum lies in `G`.

The yield scales with the number of cosets covered: blind 1, t0 2, proj 8
(of 8). The ceiling for **any** torsion-translation symmetry is the full
cofactor, which projection already attains. So:

- **Q1** (t0 vs blind): exact yield ratio in `[1.8, 2.2]` at every size and N.
- **Q2** (proj vs t0): exact yield ratio in `[3.6, 4.4]`; t0 cannot beat the
  baseline in the generic-decomposition track.
- **Q3** (oracle works): Monte Carlo on random subgroup targets. Every t0 hit
  gives a relation verified by point arithmetic. The hit rate agrees with
  the exact yield within 4 binomial standard errors.

Falsification: Q1 fails outside `[1.8, 2.2]`. If Q2's ratio were below 1,
the bound argument would be wrong and the t0 oracle would go to a full
ecbench comparison against the baseline.

## Measurement

`curve25519_family t0 <k> <N> <samples> <seed>` on the miniatures `k = 20,
24, 28, 32` at `N ∈ {N_a, 2N_a}` (as in the baseline). Exact: every pair
`i ≤ j` with both signs. Each sum's coset comes from `c_i = [r]P_i ∈ E[8]`.
`D` = distinct target abscissae covered, and `yield = 2D/(r − 1)`, as in
`prime_small_x.rs`. Monte Carlo: 200,000 targets `[a]G` per cell, seed
`2026110300 + k`.

The cost side is stated rather than measured. Per probe, t0 adds one field
inversion (batchable to about 3 multiplications) for `1/x(R)`; per table
entry, the same. Group operations are equal to blind. So relations-phase
cost per relation scales as 1/yield up to that field work.

## What this is not

It is a mechanism measurement with exact counts, not an end-to-end ecbench
`S`. If Q2 holds, no ecbench arm is warranted for the generic track.
