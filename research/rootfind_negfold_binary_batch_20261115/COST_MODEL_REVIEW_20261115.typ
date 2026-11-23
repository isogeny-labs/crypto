#set page(paper: "us-letter", margin: (x: 0.82in, y: 0.72in))
#set text(font: "Helvetica Neue", size: 10pt)
#set heading(numbering: none)

= K13 binary batch collector: cost-model boundary

_Source-level review, 2026-11-15 PST. No scientific run or performance observation._

The fixed six-arm, eight-target Koblitz E0 study over the binary field of
degree 13 is unrun. Its
source receipt binds revision `99ecffd3172302f05d3df47bfc2eb810c6a05aa2`.
The candidate shares inversions across table rows and eight-point m=3 scan
blocks, and leaves ordinates unfinished when only an abscissa is needed.

#figure(
  image("cost-model-boundary.svg", width: 100%),
  caption: [Accounting paths derived from current source and the bitop/v1 rule; no run values are plotted.],
)

== Finding

The current `bitop/v1` rule prices each recorded binary group addition at
one fixed affine weight, I + 2M + S + 9A. Scalar and batched table builds
record the same additions and first witnesses. The batched m=3 scan can
record up to seven extra additions in its last completed block. Actual
inversion sharing and omitted ordinate products do not enter these counters.
Thus a strict batched bitop win is not a discriminating primary criterion
for this implementation change. This is a source-level deduction, not an
observed wall improvement.

== Before-data correction

Amendment 3 selects complete cold process wall for the m=3 paired decision:
all 48 pairs correct and L2, all eight target geometric means below one,
candidate 95% target-cluster upper endpoint below one and below the A/A
lower endpoint, with the A/A interval containing one. The fixed spec and
failure retention remain. M2 setup wall and bitop totals stay diagnostic.
The original F4/F5/F6 study is separate.

== Evidence and next gate

Internal sources: `docs/units/BITOPS.md`; `src/cryptanalysis/ecbench/work.rs`;
`src/cryptanalysis/ic_boundary.rs`; `src/bin/ecbench/pdp3_koblitz.rs`;
`src/cryptanalysis/koblitz_fast.rs`; `AMENDMENT-3.md`. The full derivation,
source provenance and limitations are in `COST_MODEL_REVIEW_20261115.md`.
Before a run: publish the new amendment receipt and recheck the EKS monthly
budget gate. A result still needs exact-answer audit and independent replay.
