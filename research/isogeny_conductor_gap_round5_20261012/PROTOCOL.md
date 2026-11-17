# Protocol: round 5, the yield of a pushed-forward factor base (E6)

Frozen 2026-10-12, before E6 ran.  It follows
[round 3](../isogeny_conductor_gap_round3_20261011/README.md), whose finding 1
this round corrects, and
[the scaling study](../fiber_ic_scaling_20261011/README.md).

Result class: **stage diagnostic** of the decomposition step (yield and
F4 cost per system).  No whole-method or rho comparison.

## Why

Round 3 compared an `S₃` system on `E` over an `H`-stable base of `B`
points with the same base pushed to `E/H` (degree `B/|H|`).  After the
`[h]` projection, every orbit `{P + K}` is one column.  So that comparison
was at `B/|H|` columns on both sides, and its 4–20× saving was the cost of
the redundant orbit base.

The lever round 3 should have measured is different.

**Derivation.**
- Take an arbitrary base `V` of `c` abscissae on `E`, not `H`-stable, and
  `W = X(V)` its image under the Vélu map of `ψ: E → E/H`.
- Both systems have `c` columns and a factor-base polynomial of degree `c`
  in each of two unknowns.  F4's cost is then the same (round 3: cost
  depends on the degree alone).
- A solution on `E` is `±P₁ ± P₂ = ±T`.
- A solution on `E/H` is `±ψ(P₁) ± ψ(P₂) = ±ψ(T)`, that is
  `±P₁ ± P₂ ∓ T ∈ H`.  That is a valid row after the `[h]` projection.
- `ψ(V)` lies in `ψ(E(F_p))`, a subgroup of `E'(F_p)` of index `|H|`, and
  `ψ(T)` lies there too.  So a pair of base images hits the target `|H|`
  times as often as a pair of base points on `E`.

**Prediction P-E6.**
- At matched `c`, the quotient system yields `|H|` times as many
  decompositions per target as the direct one: the 95 % Poisson interval of
  the ratio contains `|H|`.
- Its F4 cost (solving degree, widest matrix) is the same as the direct
  system's, and its field operations agree within 10 %.

This is round 1's fiber-closed table gain at no extra algebraic cost.  For
comparison, round 1's combined descent paid 234–529× the blind solver work
for the same gain (round 3, E5).

**What would refute it:** a yield ratio interval excluding `|H|`.  A ratio
near 1 would mean the pushed-forward base gains nothing.

**Correctness.**  Every quotient hit is lifted back.  The run checks that
some signs and some `K ∈ H` give `±P₁ ± P₂ = ±T + K` on `E`, and every
direct hit the same with `K = O`.  A hit that does not lift stops the lead.

## Inputs

- **Curves.**  Five round-1 pairs at 22 bits, each with crater, floor and
  isomorphic floor model (15 curves):
  - D = −7, 3⁶ (h = 72);
  - D = −7, 5⁴ (h = 200);
  - D = −3, 2⁸ (h = 12);
  - D = −8, 3⁶ (h = 18);
  - D = −4, 5⁴ (h = 50).
- **Subgroups `H`.**
  - `C2`, `C4`, and `E[2]` where rational, as in round 3.
  - The whole rational `h`-torsion `E(F_p)[h]`, generated from `[r]P`.
- **Base.**  `V` = the `c` smallest abscissae on the curve, outside `H`,
  with `c ∈ {32, 64}`.  If two base points share an image, `|W| < c`; this
  is recorded, and yield is compared per column.
- **Targets.**  20,000 points `[h]·(random point)` per curve, shared by
  every arm.
- **Yield.**  Exact enumeration of base pairs per target: the number of
  targets with at least one decomposition, and the total number of
  decompositions.
- **F4.**  One system per (curve, `c`, arm) on one random target, with
  `cryptanalysis::f4_fp` (grevlex): solving degree, widest matrix and
  multiplications.
- **Runner.**  `examples/isogeny_gap_pushforward_yield.rs`, which writes
  `e6/records.jsonl`.

## Visual record

A run report with a graph (yield ratio against `|H|`) and a PDF under
`reports/run-<UTC>/`, and a README with a PDF.

## E7 (frozen 2026-10-18, after reading E6 and before E7 ran)

### What E6 implies

E6 measured yield growing as `|H|^{1.00}` for a base whose images lie in
an index-`|H|` subgroup.  Read generally:
- a base whose points lie in a subgroup `G' ⊆ E(F_q)` of index `k` that
  contains the targets' subgroup decomposes targets `k` times as often as a
  base spread over all of `E(F_q)`;
- it does so at the same number of columns and the same system size.

Every IC pipeline here builds its base from small abscissae or a
subspace, so most base points lie outside `E[r]`.  The cofactor penalty
that rounds 1 and 2 fought with fiber-closed tables (`h` times the table)
is therefore self-inflicted.

Two cheap bases avoid it.

1. **Prime-field pair tables (E7a).**  The table can use any public
   points.  `prime-abscissa:subgroup=1` keeps the smallest abscissae whose
   points satisfy `[r]P = O`, and charges each test to the factor-base
   phase.
   - Derivation, as in the scaling study but with every pair sum in
     `E[r]`: cost `≈ r/B + B²`, so `∝ r^{2/3} = (N/h)^{2/3}`.  The
     fiber-closed table cost `N^{2/3} h^{−1/3}`.
   - **P-E7a.**  At matched size, the cheapest subgroup-base arm costs about
     `h^{−1/3}` times the cheapest closed arm: 0.63 at `h = 4` and 0.24 at
     `h = 72`, each within a factor of 1.5.
   - **P-E7a.**  Its ratio to rho grows with slope ≈ 1/6 in `log₂ #E`, so
     there is still no crossover.  The cofactor stops mattering, and the
     generic `r^{2/3}` against `r^{1/2}` remains.
2. **Binary subspace bases (E7b).**  On `y² + xy = x³ + ax² + b`, a point
   is in `2E(F_q)` iff `Tr(x) = Tr(a)`, which is linear in `x`.
   `binary-subspace:dimension=d,trace_kernel=1` (for `Tr(a) = 0`) spans the
   subspace with `d` independent elements of `ker Tr`.  Every base point is
   then in `2E`, an index-2 subgroup containing `E[r]`.
   - **P-E7b.**  On curves with `Tr(a) = 0` and even cofactor, relations
     per F4 solve (yield) double against the default base of the same
     dimension: the pooled ratio's 95 % interval contains 2.
   - **P-E7b.**  The descent's maximum solving degree is unchanged, and F4
     solver operations per relation fall by about 2 (within 1.6 to 2.5).
   - The same holds for the MITM oracle on the same base.

### Inputs

- **E7a.**  Round-4 curves (`research/fiber_ic_scaling_20261011/specs`),
  `h = 4` and `h = 72`, 18 to 30 bits, two per size.
  - `h = 72` arms: `rho.negation`; the closed coset table at
    `B ∈ {8, 16, 32}`; the blind table over the subgroup base at
    `B ∈ {8, 16, 32, 64, 128}`.
  - `h = 4` arms: `rho.negation`; closed at `B ∈ {32, 64, 128, 256}`;
    subgroup base at `B ∈ {16, 32, 64, 128, 256}`.
  - 3 targets × 2 rounds, `ecbench` sessions audited with 12 replays.
- **E7b.**  The `a = 0` Koblitz curves over `F_{2^19}` and `F_{2^23}`
  (`h = 4`) from round 1's `fiber_koblitz_k0_n19` and
  `fiber_koblitz_k0_n23` specs.
  - Arms: `rho.negation`; F4 descent (`descent-algebraic:m=2`, `f4-f2`)
    and MITM (`mitm:m=2`), each on `binary-subspace:dimension=9` (round 1's
    dimension) with and without `trace_kernel=1`.
  - 4 targets × 2 rounds, no warm-up, audited the same way.
