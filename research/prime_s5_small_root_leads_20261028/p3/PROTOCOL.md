# P3 fallback: exact search over triangular shift lattices (pre-registration, 2026-10-28)

**Status.** Registered before the measured pass. Pure counting; no curve and
no reduction. **Class: derived-by-computation** (bounds, not runs).

**What this is not.** Premise P3 of L4-B names the automated-Coppersmith
linear program of Ding et al. (ePrint 2026/1027, "Computing Asymptotic
Bounds for the Automated Coppersmith Method via Linear Programming"). Its
PDF sits behind a Cloudflare browser challenge that this session does not
pass. The abstract page was readable; the paper was not. The paper's LP is
therefore **not** implemented, and nothing here is attributed to it.

## Question

Within a broad, explicit family of triangular shift lattices for a
polynomial whose Newton polytope is the box `[0, d]^m` (S₄, S₅ in the
`x_i`), or the simplex `|β| ≤ D` with weights `w_i = i` (their
symmetrisations in the `e_i`), does any lattice beat the Jochemsz–May
determinant curve of README §3.2 at equal dimension? Does any exceed the
asymptote `1/(m(m+1)2^{m−2})`?

## Code and family

[`examples/semaev_shift_search.rs`](../../../examples/semaev_shift_search.rs)
(see its header for the exact rule).

- **Leading vertex `L`:** every nonzero vertex of the Newton polytope.
- **Monomial sets `M`:** rectangles with caps `dℓ + t·(d/2)`, `t <
  extras`, per coordinate. Each rectangle is also cut by `|β| ≤ s` (7
  values of `s`), and by one half-space with coefficients in `{0, 1, 2}`
  (2 offsets each).
- **`k(β)`:** the largest `k ≤ ℓ` keeping `f^k·x^{β−kL}` inside `M`.
- **Score:** `δ = Σ k / Σ ⟨w, β⟩`, determinant only, dimension factors
  ignored (as in §3.2). The code reproduces the §3.2 formula exactly on the
  JM sets; this is printed as `jm_set_delta` against `jm_formula`.

**Premise.** Every vertex of the Newton polytope carries a nonzero
coefficient.
- For the box this is L4-B step 1 (measured, all 2^m vertices).
- For the simplex (the `D·ε_i`) it is **not** checked here. A frontier
  point whose `L` is `D·ε_i` with `i < m` is conditional on that coefficient
  being nonzero, and is labelled so.

## Runs

- m = 3: levels 1–5, `extras = 3`.
- m = 4: levels 1–3, `extras = 1`. Wider families are too slow in this
  enumeration.

## Readings

- The **frontier** (dimension, δ) per (shape, m, ℓ) is compared with the
  JM (dim_ℓ, δ_ℓ) points by linear interpolation in log dimension.
- **"Beats JM at equal dimension"** is reported per frontier point. It is
  expected to happen: extra shifts are the JM "extended strategy", and the
  family includes them.
- **The registered question is the asymptote.**
  - **Prediction:** no frontier point exceeds `1/(m(m+1)2^{m−2})`
    (1/24 at m = 3, 1/80 at m = 4).
  - **Falsified** if any point does, at any level. That would name the set
    and the lead as a P3 lead, to be checked by an actual reduction (L4-A
    code).
- **Limits.**
  - A finite family cannot show that no lattice beats the asymptote; only
    an LP or a proof can.
  - The determinant score ignores the dimension factors, which grow with
    dimension and favour the smaller sets.
