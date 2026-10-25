# Separate-chart maps: both directions checked, compositions open

This packet defines the two curve-coordinate rings separately. The affine ring
is `K[X,Y]/(Y²-(X⁵+a3 X³+a2 X²+a1 X+a0))`; the infinity chart ring is
`Common22ExplicitChart.ChartRing`. The candidate principal opens invert the
images of `X` and `z`, respectively. The pinned Lean source
`Common22SeparateChartMap.lean` defines the candidate algebra homomorphisms
in both directions with `X ↦ z⁻¹`, `Y ↦ w z⁻³` and `z ↦ X⁻¹`, `w ↦ Y X⁻³`.

Lean checks the affine equation maps to zero, descends the map through the
affine quotient, proves the image of `X` is a unit, and extends the map to the
affine principal-open localization. It symmetrically checks the infinity
equation maps to zero and constructs the reverse localized map. The pinned
replay exits zero and the theorem/definition axiom audits use only `propext`,
`Classical.choice`, and `Quot.sound`. Two maps do not alone prove an isomorphism.

| Gate | Status | Evidence or missing step |
| --- | --- | --- |
| Separate ring and principal-open definitions | Defined | `AffineRing`, `AffineOpen`, `InfinityOpen` |
| Affine equation maps to zero | Checked | `affineEquation_maps_zero` |
| Affine quotient and localization descent | Checked | `affineRingToInfinityOpen`, `affineOpenToInfinityOpen` |
| Reverse quotient and localization map | Checked | `infinityEquation_maps_zero`, `infinityOpenToAffineOpen` |
| Both compositions and separate-ring isomorphism | Open | Prove inverse identities on the quotient generators and use localization extensionality |
| Scheme gluing, projective stalk, Pic⁰, Rust semantics | Open | Require the separate-ring equivalence and later geometric/semantic bridges |
| Rho cost | Not measured | No native experiment in this packet |

The previous `PRINCIPAL_OPEN_EQUATION` packet proves an equality of ideals and
quotients in one ambient localization. This packet provides actual maps of
the separately presented localized coordinate rings. The missing composition
proofs prevent promotion to a ring equivalence, scheme gluing, or any
projective-law result. The graph `separate-chart-map.svg` and gate table
`separate-chart-map.csv` retain those open entries. No cost follows.

Replay in this directory with the pinned `lean-toolchain` and existing Mathlib
dependencies:

```sh
TMPDIR=/dev/shm lake env lean -o /dev/shm/Common22PrincipalOpenEquation.olean Common22PrincipalOpenEquation.lean
TMPDIR=/dev/shm LEAN_PATH=/dev/shm lake env lean Common22SeparateChartMap.lean
```

The captured final theorem/definition audit is `separate-chart-map-build.log`.
