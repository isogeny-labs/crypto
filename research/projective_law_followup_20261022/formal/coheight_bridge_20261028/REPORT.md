# Weighted-Proj coheight bridge — 2026-10-28

This is an observation record for the projective-law proof boundary, not an experiment verdict. It adds no new attack-cost measurement. The operation-count comparisons and their separate PDF, graphs, and per-target rows remain in the [four-followups review packet](../../../four_followups_review_20261022/REPORT.md).

## Question and scope

Can an irreducible quadratic Mumford support be represented as a point on the actual weighted-Proj genus-2 curve? The search covered the pinned Mathlib projective-scheme, adjoined-root, and DVR dimension lemmas and the repository's rational-point constructors. The curve is the weighted Proj of `degreeSubmodule a3 a2 a1 a0` for the odd-characteristic quintic model `y² = x⁵ + a3 x³ + a2 x² + a1 x + a0`. This round did not execute targets or alter any cloud job.

## Derived proof boundary

For any scheme point `z`, if its stalk is a domain and a discrete valuation ring, then `coheight z = 1`. [Mathlib's stalk dimension lemma](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/AlgebraicGeometry/Properties.lean#L376) identifies the stalk Krull dimension with coheight; [its DVR dimension lemma](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/RingTheory/DiscreteValuationRing/TFAE.lean#L244) makes that dimension one. `WeightedProjCoheight.lean` states the composition. The compiler receipt below governs whether the local declaration is independently checked.

## Quadratic-support chart point

`WeightedProjQuadraticPoint.lean` now defines field-extension evaluation on the affine quotient and transports its prime-spectrum point into the actual weighted-Proj `z` chart. For irreducible `u` satisfying `u ∣ v²−f`, `root_curve_of_dvd` proves that `(root u, v(root u))` lies on the curve. `rootEvalAlgHom_surjective` proves the evaluation reaches all of `AdjoinRoot u` because its image contains `root u`, and `rootPointIdeal_maximal` proves the chart ideal maximal. `rootProjStalkIso` identifies the projective stalk with localization at that ideal. The theorem is valid for any irreducible `u`; a degree-two `u` is the intended Mumford case. This proves a maximal affine-chart point and its projective image, without yet proving a global closed-point theorem.

## Open source-specific boundary

The next proof must show that the localization at this quadratic-support ideal is a DVR, then transport the structure through `rootProjStalkIso`. Only then does the generic coheight lemma apply. The global principal divisor and Pic⁰ class equation also remain open. No runtime-law or cost claim follows from these point-construction lemmas.

## Evidence and reproducibility

The direct Lean 4.34.1 check exited 0, and `#print axioms` for both modules listed only `propext`, `Classical.choice`, and `Quot.sound`. Both declarations are registered as default Lake library targets. The machine-readable command, source hash, and build status are in [status.json](status.json). The editable [diagram source](diagram.json) and [SVG](diagram.svg) show the proved/conditional/open boundaries. The PDF rendering is [report.pdf](report.pdf). The Lean source and pinned `lakefile.toml` are in the parent directory. No cost ratios were recomputed or changed in this round.
