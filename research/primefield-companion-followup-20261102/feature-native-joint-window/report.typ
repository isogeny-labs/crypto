#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Joint conversion: shifted points and source lanes
2026-11-03 · Exact geometry arithmetic · Four fixed public fixtures

The native entry point now takes only Q and a prefix length. It generates shifted
points projectively and converts them together with finite grid lanes. Prepared
strides remain reusable. No scalar labels or precomputed shifted points are inputs.

#image("chart.svg", width: 100%)

*Checked tradeoff.* On O,P,2P,-P, target geometry inversions fall from15/16/16/14
to1 each. Field products increase by296/321/330/298. Counts include original
prefix generation versus projective generation, shared conversion and new point
validation; setup is excluded from both target tallies. Products include squares
and constant multiplies. Additions, native integer work and memory costs remain
unpriced, so this chart does not establish elapsed-time or complete-cost savings.

*Derivation.* Co-converting K finite shifted points adds7K products to the
existing batch, with no extra inversion. K is15/16/16/15 here. Separate affine
prefix inversions disappear. Projective prefix products and exceptional cases
were derived before the test and included in its assertions.

*Native checks.* At source20ab3b2a9083, one rustc1.99.1 build and one unit
invocation passed3 methods. All64 generated shifted-point entries and1426 finite
golden slots agree, including exact projective coordinates and inverse-Z values.
Identity/empty/invalid/singleton controls pass. Product/inversion hooks are
cfg(test) only; the production prefix otherwise matches the preserved window.

*Scope and next.* This is fixed-width geometry over the original F509 fixture;
prime/subgroup premises remain caller-supplied. No sign oracle, resultant,
scalar-recovery cohort or cloud job ran. Connect normalized recurrence and the
original product-tree/multipoint resultant before admitting the complete native
4,563-case cohort. Larger-field backend and full costs remain explicit tasks.

*Evidence.* check-receipt.txt, test.log, counts.json, original baseline TSV,
source/input hashes and golden fixtures. DERIVATION.md gives exact exclusions.
The chart contains four exact software/count fixtures, no timing sample or
uncertainty estimate. Original formula/source references are retained there.
