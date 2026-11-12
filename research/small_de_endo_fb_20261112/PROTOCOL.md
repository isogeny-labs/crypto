# Small D_E with an endomorphism-compatible factor base: does solving cost drop?

Status: **draft protocol, written before any data** (2026-11-12). Freeze it by
hash before the first measured run; after that, add dated amendments only.

## Question (owner, 2026-11-12)

Over `F_p`, choose a curve with a small-norm endomorphism `ψ` (small `|D_E|`).
Pick coordinates and a factor base adapted to `ψ`. Does that reduce the
*actual* cost of solving the summation-polynomial decompositions and of the
whole relation phase? Coefficient sparsity or a low first-fall degree is not
evidence. The standard `S_m` keeps individual degree `2^{m-2}`, so the test is
the measured solving cost.

## Prior work this does not repeat

- `research/gaudry_quartic_d4_20261013`, `research/d4_symmetrised_s5_20261015`:
  the D₄-symmetrised S₅, one phase constant, no end-to-end `S`.
- `research/curve25519_t0_symmetry_20261102`: a symmetry stage effect.
- `research/automorphism_ideas_20261021` DGM-01: automorphism orbit folding of
  factor-base logs. It is a design and was never run.
- `examples/aut_orbit_coverage.rs`: the j = 0 column coverage count.
- Not covered by any of these: an endomorphism of degree > 1 (`D = −7, −8,
  −11`, after Tsakou–Ionica's extension-field mechanism), and an end-to-end
  relation cost on a prime field against a rho that uses the same symmetry.

## Arms (same `p`, prime `r ≍ p`, the same target stream per cell)

- **A0, generic:** a curve with large `|D|`, standard `S_m`, and the standard
  `x`-interval factor base.
- **A1, small D_E with the standard base:** an ablation that separates the
  curve from the adaptation.
- **A2, small D_E with adapted coordinates and base:** an invariant function
  of `ψ` as the coordinate (for example `x³` for `j = 0`, `x²` for `j = 1728`,
  or the `x`-map of the degree-2 or degree-3 endomorphism for `D = −7, −8,
  −11`). The factor base is closed under `ψ`, and logs fold along the orbits
  using the scalar action `ψ(G) = [λ]G`, checked in code before use.
- **Rho references:** plain rho, plus rho with the same automorphism group
  (the √|Aut| reduction) wherever A2 uses an automorphism. A2 is never compared only
  against a rho without that symmetry.

`D_E ∈ {−3, −4, −7, −8, −11}`, `m ∈ {2, 3}` (`S_3`, `S_4`), and `p` from about
`2^10` to as large as the solver allows inside the cap.

## Measured, per arm and cell

1. Solving degree reached by the Gröbner/Macaulay solve, and the largest
   matrix (rows × columns).
2. Decomposition probability per trial, and trials per relation.
3. Factor-base columns after folding, and relations needed.
4. Total relation-phase cost in counted field operations, plus the LA cost,
   priced at 0, 0.1 and 1 GAE per unpriced operation (AGENTS §5a).
5. End-to-end log recovery, with each log certified as `[k]G = T`.

The transfer or curve construction is a one-time cost and is reported
separately; it is never in a per-instance total.

## Decision rule

A2 counts as a gain only if its total relation-phase cost, or its
end-to-end cost, is below A1's in every cell of a `D_E`, across the §5a band,
with every log certified. A2 against A0 and against the matched rho is
reported as the distance remaining, not as a gate.

## Prediction (frozen before data)

- For `D = −3, −4`, A2 lowers columns by `|Aut|/2` and LA by about its square.
  Decomposition probability and solving degree are unchanged within noise.
  Against rho with the same automorphism, the gap stays at least the plain-IC
  gap. P = 0.7.
- For degree-2 or degree-3 endomorphisms, no factor base of useful size is
  closed under `ψ` without growing it by about `deg ψ` per orbit step, so A2
  costs no less than A1. P = 0.65.
- No cell ranks A2 below the matched rho. P = 0.85.
