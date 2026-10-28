# Weighted-Proj coheight bridge — 2026-10-28

This is an observation record for the projective-law proof boundary, not an experiment verdict. It adds no new attack-cost measurement. The operation-count comparisons and their separate PDF, graphs, and per-target rows remain in the [four-followups review packet](../../../four_followups_review_20261022/REPORT.md).

## Question and scope

Can an existing local DVR proof establish coheight one for a point on the actual weighted-Proj genus-2 curve? The search covered the pinned Mathlib projective-scheme and DVR dimension lemmas and the repository's rational-point DVR constructors. The curve is the weighted Proj of `degreeSubmodule a3 a2 a1 a0` for the odd-characteristic quintic model `y² = x⁵ + a3 x³ + a2 x² + a1 x + a0`. This round did not execute targets or alter any cloud job.

## Derived proof boundary

For any scheme point `z`, if its stalk is a domain and a discrete valuation ring, then `coheight z = 1`. [Mathlib's stalk dimension lemma](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/AlgebraicGeometry/Properties.lean#L376) identifies the stalk Krull dimension with coheight; [its DVR dimension lemma](https://github.com/leanprover-community/mathlib4/blob/d13f23b723b8a846827a245b89c10fc7d3f11612/Mathlib/RingTheory/DiscreteValuationRing/TFAE.lean#L244) makes that dimension one. `WeightedProjCoheight.lean` states the composition. The compiler receipt below governs whether the local declaration is independently checked.

## Open source-specific boundary

The existing projective DVR constructors take rational `a, y : K` or the rational point at infinity. An irreducible quadratic Mumford support has a degree-two residue field, so these constructors do not identify its point. The next proof must construct the affine evaluation at `K[X]/(u)` for irreducible monic quadratic `u` satisfying `u ∣ v²−f`, prove its kernel maximal, transport the resulting `Spec` point into the weighted-Proj chart, and prove its stalk is a DVR. Only then does the generic coheight lemma apply. A coheight statement alone does not establish the global principal divisor or Pic⁰ class equation.

## Evidence and reproducibility

The final Lean 4.34.1 check exited 0, and `#print axioms` listed only `propext`, `Classical.choice`, and `Quot.sound`. The machine-readable command, source hash, and status are in [status.json](status.json). The editable [diagram source](diagram.json) and [SVG](diagram.svg) show the proved/conditional/open boundaries. The PDF rendering is [report.pdf](report.pdf), inspected as a single legible page. The Lean source and pinned `lakefile.toml` are in the parent directory. No cost ratios were recomputed or changed in this round.
