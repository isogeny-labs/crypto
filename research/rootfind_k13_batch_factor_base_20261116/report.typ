#set page(paper: "a4", margin: 18mm)
#set text(font: "Libertinus Serif", size: 10pt)
= Opt-in batched Koblitz factor-base lifts

Source-bound correctness screen, 16 November 2026. The candidate source is
`src/cryptanalysis/koblitz_index_calculus.rs` at amended commit `0c23166d62`.
Protocol commit: `3febee7878`; fixture amendment: `0c23166d62`. Exact test status and commands are in
`REPORT.md` and `CHECKS.json` beside this PDF.

The polynomial-basis factor base has $2^d$ abscissae, including zero. The
legacy packed path invokes one field inverse for each nonzero abscissa.
The opt-in path calls the existing zero-tolerant batch inverter once, then
uses the shared lift formula. Counts below are derived from source structure,
not timed measurements. Batch inversion also performs field multiplications.

#image("inversion-counts.svg", width: 100%)

The implementation keeps the production benchmark builder on its prior path.
The next integration must update the field-operation ledger and preserve
source, target, answer and A/A controls before timing can be compared.

The focused release screen passed 28 tests. Both broader library-suite attempts
reached the watchdog and reported one existing chained-S3 assertion failure;
a pre-candidate executable repeats that failure, with its build commit not
independently pinned. These producer checks establish no speed advantage.

The source manuscript is
`papers/rootfinding-decomposition/rootfinding_decomposition.tex`; the
existing inversion routine is in `src/cryptanalysis/semaev_decomp.rs`.

#align(center, image("flow.svg", width: 75%))
