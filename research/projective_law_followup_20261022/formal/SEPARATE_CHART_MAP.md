# Separate-chart principal-open coordinate rings: checked equivalence

This packet defines the two curve-coordinate rings separately. The affine ring
is `K[X,Y]/(Y²-(X⁵+a3 X³+a2 X²+a1 X+a0))`; the infinity chart ring is
`Common22ExplicitChart.ChartRing`. The candidate principal opens invert the
images of `X` and `z`, respectively. The pinned Lean source
`Common22SeparateChartMap.lean` defines the candidate algebra homomorphisms
in both directions with `X ↦ z⁻¹`, `Y ↦ w z⁻³` and `z ↦ X⁻¹`, `w ↦ Y X⁻³`.

Lean checks the affine equation maps to zero, descends the map through the
affine quotient, proves the image of `X` is a unit, and extends the map to the
affine principal-open localization. It symmetrically checks the infinity
equation maps to zero and constructs the reverse localized map. Generator
round trips followed by quotient and localization extensionality prove both
full composition identities. `separateChartOpenEquiv` is consequently an
`AlgEquiv` of the two separately presented principal-open coordinate rings.
The pinned replay exits zero and its theorem/definition axiom audits use only
`propext`, `Classical.choice`, and `Quot.sound`.

| Gate | Status | Evidence or missing step |
| --- | --- | --- |
| Separate ring and principal-open definitions | Defined | `AffineRing`, `AffineOpen`, `InfinityOpen` |
| Affine equation maps to zero | Checked | `affineEquation_maps_zero` |
| Affine quotient and localization descent | Checked | `affineRingToInfinityOpen`, `affineOpenToInfinityOpen` |
| Reverse quotient and localization map | Checked | `infinityEquation_maps_zero`, `infinityOpenToAffineOpen` |
| Both compositions | Checked | `reverse_forward_eq_id`, `forward_reverse_eq_id` |
| Separate-ring algebra equivalence | Checked | `separateChartOpenEquiv` |
| Principal-open affine-scheme identification | Open | Apply the algebra equivalence to `Spec` and identify distinguished opens |
| Smooth projective gluing, infinity stalk, Pic⁰ | Open | Need scheme gluing/open immersion and stalk/divisor transport |
| Rust semantics | Open | No correspondence from the published formulas to runtime operations |
| Rho cost | Not measured | No native experiment in this packet |

The previous `PRINCIPAL_OPEN_EQUATION` packet proves an equality of ideals and
quotients in one ambient localization. This packet establishes the stronger
equivalence of the two separately presented localized coordinate rings. It
does not identify these rings with the smooth projective curve's open charts
or its infinity stalk; those require geometric constructions beyond the ring
equivalence. The graph `separate-chart-map.svg` and gate table
`separate-chart-map.csv` retain those open entries. No cost follows.

Replay in this directory with the pinned `lean-toolchain` and existing Mathlib
dependencies:

```sh
TMPDIR=/dev/shm lake env lean -o /dev/shm/Common22PrincipalOpenEquation.olean Common22PrincipalOpenEquation.lean
TMPDIR=/dev/shm LEAN_PATH=/dev/shm lake env lean Common22SeparateChartMap.lean
```

The captured final theorem/definition audit is `separate-chart-map-build.log`.
