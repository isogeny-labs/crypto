# Separate-chart map: checked generators and an open descent gate

This packet defines the two curve-coordinate rings separately. The affine ring
is `K[X,Y]/(Y²-(X⁵+a3 X³+a2 X²+a1 X+a0))`; the infinity chart ring is
`Common22ExplicitChart.ChartRing`. The candidate principal opens invert the
images of `X` and `z`, respectively. The pinned Lean source
`Common22SeparateChartMap.lean` defines an algebra homomorphism from the
**affine polynomial ring** to the localized infinity chart with
`X ↦ z⁻¹` and `Y ↦ w z⁻³`.

Lean checks the two generator images, `z⁻¹ z=1`, the coefficient expansion
of the infinity-chart equation, and that equation after localization. The
replay exits zero and the five theorem axiom audits use only `propext`,
`Classical.choice`, and `Quot.sound`. This is a map of polynomial rings; no
theorem yet says it kills the affine defining equation.

| Gate | Status | Evidence or missing step |
| --- | --- | --- |
| Separate ring and principal-open definitions | Defined | `AffineRing`, `AffineOpen`, `InfinityOpen` |
| Candidate polynomial algebra map and generator images | Checked | `affinePolynomialToInfinityOpen_X0`, `_X1` |
| Localized inverse and infinity equation | Checked | `infinityInvZ_mul_z`, `infinity_ubar_expansion`, `infinity_open_relation` |
| Affine equation maps to zero | Open | Multiply the localized chart equation by `z⁻⁶` and cancel `z z⁻¹=1` in Lean |
| Quotient and localization descent | Open | Prove kernel inclusion, then use `Ideal.Quotient.lift` and `IsLocalization.Away.lift` |
| Reverse map and separate-ring isomorphism | Open | Define `z ↦ x⁻¹`, `w ↦ y x⁻³`, then check both compositions |
| Scheme gluing, projective stalk, Pic⁰, Rust semantics | Open | Require the separate-ring equivalence and later geometric/semantic bridges |
| Rho cost | Not measured | No native experiment in this packet |

The previous `PRINCIPAL_OPEN_EQUATION` packet proves an equality of ideals and
quotients in one ambient localization. It does not discharge these separate
presentation obligations. The graph `separate-chart-map.svg` and gate table
`separate-chart-map.csv` intentionally retain null/open entries. The PDF
`SEPARATE_CHART_MAP.pdf` is a review rendering of this result, not a claim of
a projective group law or a cost improvement.
