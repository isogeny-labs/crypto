# Frozen destination-only second-jet gate

Use Q1 overF127 from geometry13, normalized by transport13. No source point
image will be inferred in this gate. Enumerate its rational points in the pinned
implementation's order. For every unordered pair of affine points with distinct
x coordinates and nonzero F_y, take their chord. Retain exactly chords meeting
the rational point list in four distinct points; all other pairs excluded with
counts, not replaced. Four distinct intersections certify transversality by
Bezout. No extension-point decoder or exceptional chart is claimed.

Compute D(P),D(Q) and the Hessian on the chord from the formula in PROOF.md.
Decode by testing all six unordered pairs among the four intersection points.
Require exactly one zero pair, equal to the original unordered pair. This is a
check of the geometric decoder and its uniqueness calculation, not independent
validation of the derivative formula. A sum-surface control changes only the
second diagonal's sign and must admit exactly the original and complementary
pairs. A zero Hessian control must admit all six pairs and be rejected as
ambiguous. Print the first8 complete cases, all counts, final PASS or failure.

Run once in release after source+lock+protocol commit and S3 source checkpoint;
no adaptive fixture changes or rerun. Build beforehand permitted. Exit failure
on any discrepancy or no admissible pairs. Retain stdout/stderr. This is a
correctness/theory gate; no runtime comparison, performance record, source map
or independent implementation claim. Existing studies and shared resume untouched.
