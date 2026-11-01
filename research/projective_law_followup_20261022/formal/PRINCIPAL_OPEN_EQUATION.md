# Genus-2 principal-open equation: common-ambient quotient packet

Let `R` be any commutative ring, with `x*s=1`, and coefficients `a3,a2,a1,a0`
in `R`. Put

`F = y²-(x⁵+a3*x³+a2*x²+a1*x+a0)` and

`G = (y*s³)²-s*(1+a3*s²+a2*s³+a1*s⁴+a0*s⁵)`.

The checked Lean theorem `equation_unit_multiple` proves `G=s⁶*F`. Because
`s` is a unit, `equation_iff_on_principal_open` proves `F=0 ↔ G=0`, and
`principal_equation_ideals_equal` proves `(F)=(G)` in **the same ambient ring**.
`common_ambient_quotient_equiv` therefore constructs `R/(G) ≃+* R/(F)`.
`explicit_common_ambient_quotient_equiv` instantiates this to the actual
localization `R=Localization.Away X₀` of a two-variable polynomial ring over
an arbitrary field, with `x` the image of `X₀`, `y` the image of `X₁`, and
`s` Mathlib's inverse of `x` in that localization. All checked theorems and
definitions have axiom audits using only `propext`, `Classical.choice`, and
`Quot.sound` or a subset.

| Gate | Status | Exact evidence or missing construction |
| --- | --- | --- |
| Universal unit-multiple equation | Checked | `equation_unit_multiple` |
| Vanishing and principal-ideal equality | Checked | `equation_iff_on_principal_open`, `principal_equation_ideals_equal` |
| Same-ambient quotient-ring equivalence | Checked | `common_ambient_quotient_equiv` |
| Concrete `Localization.Away X₀` instance | Checked | `explicit_common_ambient_quotient_equiv` |
| Equivalence between separately presented chart localizations | Open | Define affine quotient ring and prove localization–quotient interchange; then construct the variable-change map to `Localization.Away z` of `Common22ExplicitChart.ChartRing` and its inverse |
| Principal-open affine-scheme isomorphism | Open | Apply `Spec` to the preceding specific ring equivalence and identify principal opens |
| Smooth projective completion and infinity stalk | Open | Specific gluing/open immersion, smoothness, projectivity, stalk transport |
| Principal divisor/Pic⁰, Rust semantics, rho cost | Open / not measured | No new proof or cohort result |

The same-ambient equivalence does **not** by itself identify the two original
coordinate-ring presentations. The concrete missing map has generators
`x ↦ z⁻¹`, `y ↦ w*z⁻³` and inverse `z ↦ x⁻¹`, `w ↦ y*x⁻³`. The previous
field-level overlap packet checks these substitutions on points; this packet
checks their equation ideals after adjoining an inverse. A future proof must
turn those formulas into quotient/localization ring homomorphisms and check
both compositions before scheme gluing.

The report is `PRINCIPAL_OPEN_EQUATION.pdf`, the graph is
`principal-open-equation.svg`, the standardized gate table is
`principal-open-equation.csv`, and the Lean source is
`Common22PrincipalOpenEquation.lean`. Replay with
`TMPDIR=/dev/shm lake env lean Common22PrincipalOpenEquation.lean`; captured
axioms are in `principal-open-equation-build.log`. No empirical cost follows.
