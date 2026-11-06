# Genus-2 affine/infinity chart overlap: formal result packet

For any field `K` and coefficients `a3,a2,a1,a0`, consider the affine equation

`y² = x⁵ + a3 x³ + a2 x² + a1 x + a0`

and the infinity-chart equation

`w² = z(1 + a3 z² + a2 z³ + a1 z⁴ + a0 z⁵)`.

On `x ≠ 0`, set `z = x⁻¹` and `w = y*x⁻¹³`. The new Lean theorem
`Common22ChartOverlap.affine_chart_equation_iff` proves that these equations
are equivalent. `infinity_to_affine` proves the converse direction from a
chart point with `z ≠ 0`; `x_round_trip`, `y_round_trip`, `z_round_trip`, and
`w_round_trip` prove the inverse coordinate identities. The pinned Lean
4.34.1 compile exited 0. Each new theorem's axiom audit was exactly
`[propext, Classical.choice, Quot.sound]`.

| Gate | Status | Evidence or exact missing step |
| --- | --- | --- |
| Equation equivalence on `x,z ≠ 0` | Checked | `affine_chart_equation_iff` |
| Reciprocal coordinate round trips | Checked | `x_round_trip`, `y_round_trip`, `z_round_trip`, `w_round_trip` |
| Isomorphism of principal-open affine schemes | Open | Ring-localization maps and inverse naturality not constructed |
| Two-chart glued curve | Open | Specific `Scheme.GlueData` and open cover not instantiated |
| Smooth projective completion | Open | Smoothness and properness/projectivity not established |
| Completion's infinity stalk `≅ A_m` | Open | Open-chart stalk transport needs the preceding construction |
| Principal divisor / `Pic⁰` transport | Open | No completed curve or divisor-class map |
| Rust law semantics, costs, rho | Open / not measured | No new code or cohort result |

The result is an identity over a field on the punctured overlap. It does not
assert that an affine chart has already been glued into a smooth projective
curve. The previous `Common22AffineStalk.lean` identifies the stalk of
`Spec A` at its origin with `A_m`; the current theorem supplies the coordinate
transition algebra needed to construct the gluing map. The precise next step
is a ring isomorphism between the principal opens `D(x)` and `D(z)` followed
by `Scheme.GlueData` and a stalk-isomorphism application.

`chart-overlap.csv` is the standardized gate table, `chart-overlap.svg` is the
dependency graph, and `CHART_OVERLAP.pdf` is the printable report. Replay from
this directory with `TMPDIR=/dev/shm lake env lean Common22ChartOverlap.lean`.
The captured theorem and axiom output is `chart-overlap-build.log`.
