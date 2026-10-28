#set page(paper: "a4", margin: 24mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Weighted-Proj coheight bridge
2026-10-28 · proof-boundary observation record · no new target measurements

The generic declaration `WeightedProjCoheight.coheight_one_of_dvr` proves coheight one conditional on a domain and DVR stalk. The new `WeightedProjQuadraticPoint` module constructs the actual weighted-Proj image of an irreducible support satisfying `u | v^2 - f`. Its affine chart ideal is maximal, and its projective stalk is identified with that chart localization. Both modules compile under the pinned Lean project.

#figure(image("diagram.svg", width: 100%), caption: [Quadratic-support path. Green is compiled; yellow remains open; blue is conditional on a DVR stalk.])

The next source-specific gate is a DVR proof at the quadratic-support localization, followed by its transport through the checked stalk isomorphism. A global closed-point theorem, principal divisor, and Pic0 class equation remain separate. No target data or cost ratio changed.

Sources: Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`, `AlgebraicGeometry/Properties.lean:376`, `RingTheory/DiscreteValuationRing/TFAE.lean:244`, and `RingTheory/AdjoinRoot.lean`; Stacks Tags 01HD and 00PD; repository `WeightedProjNonbranchZeroOrder.lean`.

The full cohort cost graphs, rows, and ratios are in `research/four_followups_review_20261022/`. This report changes none of those results.
