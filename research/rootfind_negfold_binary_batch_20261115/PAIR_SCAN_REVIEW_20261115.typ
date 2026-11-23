#set page(paper: "us-letter", margin: (x: 0.82in, y: 0.72in))
#set text(font: "Helvetica Neue", size: 10pt)
#set heading(numbering: none)

= K13 rootfinding scan: signed-pair denominator sharing

_Source-level review, 2026-11-15 PST. No scientific run or performance observation._

The fixed Koblitz E0 study over the binary field of degree 13 uses a
33-point signed base: one self-negative point, then 16 adjacent sign pairs.
The fixed eight-point blocks cut across pair boundaries. The candidate
now places one copy of each adjacent equal denominator into the batch chain.

#figure(
  image("pair-scan-products.svg", width: 100%),
  caption: [Derived portable full-block denominator work; no wall values are plotted.],
)

== Arithmetic boundary

Both paths make one field inversion per ordinary eight-point block.
The existing path sends eight entries through Montgomery batch inversion;
the paired path sends five for a nonexceptional full K13 block. The current
four-lane portable kernel makes 3k+9 field products for k nonzero inputs,
so this denominator chain changes from 33 to 24 products. The difference
is nine. An available eight-lane SIMD kernel remains in use. The eight slopes,
point abscissae, ordered lookups and first witnesses remain. This count
excludes all other field and pipeline work; it is not a wall estimate.

== Correctness and measurement gate

Amendment 4 records the source change before data. The focused arithmetic
test compares ordered lazy sums, completed points, exceptional cases, odd
tails and fallback layouts. The ecbench test checks first-witness and
lookup parity against the scalar collector. The registered 336-execution
study has not run. Amendment 3 still requires complete cold L2 wall with
the A/A and target-cluster gates. The original F4/F5/F6 panel is separate.

== Sources and next action

Internal sources: `src/cryptanalysis/ic_boundary.rs`,
`src/cryptanalysis/semaev_decomp.rs`, `src/cryptanalysis/koblitz_fast.rs`,
`src/bin/ecbench/pdp3_koblitz.rs`, and `AMENDMENT-4.md`. The supplied
rootfinding manuscript and prior Koblitz collector notes were checked;
`PAIR_SCAN_REVIEW_20261115.md` has exact provenance and limits. Publish a
new source receipt, then recheck the EKS budget gate before any study job.
