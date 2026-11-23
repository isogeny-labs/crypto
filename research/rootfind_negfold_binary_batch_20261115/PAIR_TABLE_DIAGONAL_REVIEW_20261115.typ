#set page(paper: "us-letter", margin: (x: 0.82in, y: 0.72in))
#set text(font: "Helvetica Neue", size: 10pt)
#set heading(numbering: none)

= K13 pair-table diagonal peel

_Before-data source review, 2026-11-15 PST. No scientific timing or outcome row._

Every pair-table row starts with same-abscissa sums. Those exceptional sums
make the existing SIMD batch routine decline the whole row. The candidate
now completes that diagonal group, then passes adjacent off-diagonal sign
pairs to the existing x-only paired kernel when the runtime accepts them.
All other cases use the paired batch fallback.

#figure(
  image("pair-table-diagonal.svg", width: 100%),
  caption: [Source route, correctness gate, and open measurement boundary.],
)

== Invariants

The scalar pair order, first witness for each abscissa, and abstract
addition charge stay fixed. The paired kernel returns x coordinates and
slopes in interleaved order; the table uses only x coordinates. A declined
kernel appends nothing, so the fallback can process the same inputs.

== Validation boundary

The ARM release scalar-versus-batch parity test passed for K13 and K17,
including first witnesses and m=2/m=3 decompositions. The x86 macOS
cross-target `ecbench` check passed and type-checked the paired path.
An admitted x86 run is needed to exercise AVX-512 and compare complete
cold wall cost. The monthly EKS cost gate and Amendment 7 publication
receipt must be satisfied before the fixed study runs.

Internal sources: `src/cryptanalysis/ic_boundary.rs`,
`src/cryptanalysis/koblitz_fast.rs`, `AMENDMENT-7.md`, and the full
provenance in `PAIR_TABLE_DIAGONAL_REVIEW_20261115.md`.
