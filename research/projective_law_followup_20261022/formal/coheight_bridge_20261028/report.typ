#set page(paper: "a4", margin: 24mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Weighted-Proj coheight bridge
2026-10-28 · proof-boundary observation record · no new target measurements

The generic conditional declaration `WeightedProjCoheight.coheight_one_of_dvr` composes the pinned Mathlib stalk-dimension and DVR-dimension lemmas. At a scheme point with a domain and DVR stalk, coheight is one. The local compiler status is recorded in `status.json` beside this PDF.

#figure(image("diagram.svg", width: 100%), caption: [Actual quadratic-support path. Yellow steps remain open; blue is conditional on the stalk hypothesis.])

The existing source-specific DVR constructors cover rational affine points and rational infinity. An irreducible quadratic Mumford support needs a closed point with residue field `K[X]/(u)`, a maximal evaluation kernel, and a DVR proof at the transported weighted-Proj point. The global divisor and Pic0 equation remains separate.

Sources: Mathlib commit `d13f23b723b8a846827a245b89c10fc7d3f11612`, `AlgebraicGeometry/Properties.lean:376` and `RingTheory/DiscreteValuationRing/TFAE.lean:244`; repository `WeightedProjSmoothPointDVR.lean`, `WeightedProjInfinityDVR.lean`, and `WeightedProjNonbranchZeroOrder.lean`.

The full cohort cost graphs, rows, and ratios are in `research/four_followups_review_20261022/`. This report changes none of those results.
