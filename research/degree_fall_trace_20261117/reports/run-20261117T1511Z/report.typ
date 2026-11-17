#set page(width: 22cm, height: auto, margin: 1.4cm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(justify: true)

#align(center)[
  #text(size: 16pt, weight: "bold")[How relations drive the degree of regularity]

  #text(size: 9pt, fill: gray)[
    ecbench `f4_fp` degree-fall diagnostic · run-20261117T1511Z ·
    source commit on branch `claude/nifty-heisenberg-khbw4f`
  ]
]

#v(6pt)

*What this shows.* A coloured trace of the measured degree-fall diagnostic
added to ecbench's F4 engine (`src/cryptanalysis/f4_fp.rs`). For a family of
polynomial systems over $F_101$ in three variables — the same three quadrics,
then one, two and three more relations added (3 → 6 relations), every quadric
forced through a common root so the over-determined systems stay consistent —
the engine records, at each working degree d, the semi-regular (Bardet)
expectation (the coefficients of the product over i of (1 − t^(d#sub[i])) over
(1 − t)^n, truncated at the first non-positive coefficient) and the realized
standard-monomial count of the computed Gröbner basis. A *degree fall* is a degree where
the realized count drops below the expectation: the relations removed more
structure than a generic system of the same shape would, which is exactly what
makes a structured system cheaper than its generic degree bound.

#figure(
  image("degree_fall_trace.svg", width: 100%),
  caption: [
    Left: the recorded Hilbert deviation (semi-regular expected − realized
    standard monomials) at each working degree, one row per relation count;
    blue outlines are the degrees the engine flagged as falls, the ▽ marks the
    observed degree of regularity (first degree the quotient vanishes). Right:
    the observed degree of regularity and solving degree against the number of
    relations, with the first-fall degree annotated. All values are measured by
    the Rust engine; the figure only lays them out.
  ],
)

#v(4pt)

*How to read it.* With only the three quadrics the system is a regular sequence:
the realized Hilbert function matches the semi-regular expectation at every
degree (no fall, deviation 0), and the degree of regularity sits at the generic
bound. As each further relation is added the realized counts fall below the
expectation — the coloured cells and blue outlines appear at lower degrees, and
the observed degree of regularity drops. The first-fall and last-fall degrees
and the observed degree of regularity are the quantities the user asked ecbench
to measure (first-fall degree, last-fall degree, degree of regularity).

*Method and status.* The diagnostic is post-hoc (it reads the finished basis),
gated by `F4Options::record_degree_falls`, and purely diagnostic: it is never
priced into the counted cost `S`, never compared by the audit replay, and never
a stopping rule. Every number here is *measured* by the engine, whose
definitions are unit-tested (`f4_fp::tests`: the semi-regular series against
hand-computed values, a regular sequence with no fall, and an over-determined
system with a fall at degree 1). This is a harness/diagnostic figure on a
constructed family, not an ECDLP result or a rho comparison.

*Data and source.* Recorded dataset: `degree_fall_dataset.json`
(`ecbench.degree-fall-dataset/v1`, written by the `emit_degree_fall_dataset`
test). Figure source: `plot_degree_fall_trace.py` (layout only, no computation).
