# Weighted-Proj integrality: checked prerequisite and open gluing gate

This packet studies the actual weighted-Proj curve of the common 2+2 formula. It does **not** establish `IsIntegral` for that curve, a global function field, a principal divisor, Picard-class transport, or compiled Rust semantics.

## Checked result

`WeightedProjIntegral.infinity_chart_z_ne_zero` proves `zbar a3 a2 a1 a0 ≠ 0` for every field `K` and coefficients `a3 a2 a1 a0 : K`. The proof uses the checked chart relation `wbar² = zbar * ubar` and the checked `chart_wbar_ne_zero`. Pinned Lean 4.34.1 compilation exited 0. The direct axiom print is `[propext, Classical.choice, Quot.sound]`; there is no `sorry` or new axiom.

This is an algebraic prerequisite for a nonempty overlap: in the domain infinity chart, its principal open `D(zbar)` contains the zero prime. That spectrum-level and Proj-level transport is **not** proved by the new theorem.

## Exact remaining construction

The existing `WeightedProjCover.two_basic_opens_cover` covers the curve by X- and Z-basic opens. `WeightedProjInfinityChart.xBasicOpenIsoInfinity` and `WeightedProjSchemeChart.zBasicOpenIsoAffine` identify these opens with spectra. `Common22Domain.chart_ring_domain` proves the X/infinity chart is a domain without separability; `Common22SeparableAffineDomain.affine_ring_domain_of_separable` proves the Z/affine chart is a domain when `q a3 a2 a1 a0` is separable.

Two domain opens by themselves do not imply an integral union: two disjoint affine lines are a counterexample to that inference. The missing checked bridge is to show the X- and Z-open intersection is nonempty in the actual Proj and apply `PreirreducibleSpace.of_isOpenCover` to those opens. The relation `zbar ≠ 0` supplies the algebraic starting point, but needs transport through the chart isomorphism and identification with the Proj Z-basic open. Then `IsReduced.of_openCover` and `isIntegral_of_irreducibleSpace_of_isReduced` can yield `IsIntegral` under separability.

The cited pinned APIs are in `Mathlib/Topology/Sets/OpenCover.lean` and `Mathlib/AlgebraicGeometry/Properties.lean`. This is a dependency path, not a checked proof of its conclusion.

## Reproduction and scope

Compile `WeightedProjIntegral.lean` in the pinned formal directory with `TMPDIR=/dev/shm lake env lean /absolute/path/to/WeightedProjIntegral.lean`. The direct output is in `projective-integral-lean.log`. The CSV gives each claim's status and gate. The SVG is a dependency graph, not data. No DLP experiments or cost comparisons were run.
