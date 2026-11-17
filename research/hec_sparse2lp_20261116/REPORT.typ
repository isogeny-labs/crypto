#set page(paper:"a4",margin:22mm)
#set text(size:11pt)
= Supplied-target sparse native LP assembly
Genus 2, 3 and 4 · 15 November 2026

#table(columns:(1fr,1fr),[Correctness gate],[Observed],[Reference/candidate equivalence],[12 / 12 pass],[Additional supplied targets],[3 / 3 verify],[Pivot sequence hash],[Identical in all twelve comparisons],[Stopping trials / rows],[Identical],[Raw execution],[Exit zero; empty stderr])

== Public target boundary
The solver receives a public divisor instead of constructing its own target. Validity, nonzero and ell-torsion checks precede execution. Final recovery is checked against the original source generator. Torsion membership alone need not imply cyclic-subgroup membership; capped recovery remains possible.

The additional fixed controls construct their public inputs outside the solver. Their expected scalar is not a solver argument. They are known-scalar correctness controls, not blinded target selection.

== Sparse assembly
Previously: allocate and scan one coefficient vector over the entire rational-place catalogue for every accepted decomposition.

Now: assemble an ordered sparse map from at most g root entries and min(m,64) walk columns, then add the target and constant. Reuse the signed affine graph unchanged. Keep exact decomposition, projected-potential, full recombination and source scalar checks.

Catalogue enumeration, order certification, by-x lookup, projected points and small-base counts remain. Map allocation and symbolic potential fill still require measurement; sparse representation alone establishes no wall-time ratio.

== Custody and next gate
Source and binary frozen at eeefde0f1d8e. S3 checkpoint20261116T043229Z precedes the once-only controls. Initial compile failure retained and corrected before the freeze. Raw output and status accompany the result.

Next: freeze complete-cost comparison against the existing native reference on the same supplied targets. This report contains no performance measurement. Established large-prime algebra is the baseline; no algorithmic novelty is asserted.
