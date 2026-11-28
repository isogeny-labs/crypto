#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)
#set par(justify: true, leading: 0.62em)

#align(center)[
  #text(size: 18pt, weight: "bold")[Quadratic quotient auxiliaries for a symmetric Curve25519 relation system]
  #v(3pt)
  #text(size: 10pt)[EXP-C25519-SYMREM-001 · 2026-11-25 · [pcc:unverified]]
]

#v(8pt)
#block(fill: rgb("f8e8e8"), inset: 9pt, radius: 4pt)[
  *Decision: stop before scaling and m=4.* At the frozen `d=19` decision
  point, the quotient/direct paired geometric-mean F4-work ratio is
  *1.2864624565*, with descriptive 95% interval
  *[1.2864612925, 1.2864635223]*. The required upper endpoint was below
  *0.50*.
]

== Question

For three distinct factor-base roots, write
$q(X) = X^3 - e_1 X^2 + e_2 X - e_3$. Both arms require $q$ to divide
$X^d - 1$, with the same binary multiplication schedule, intermediate
residues, symmetrized $S_4$ relation, and targets. The direct arm eliminates
each modular quotient. The candidate retains its two coefficients so every
membership-gadget equation is quadratic. Does the lower equation degree repay
the added state?

== Counted result

#table(
  columns: (auto, 1fr, 1fr, auto, auto),
  inset: 5pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*d*], [*Direct mean ops*], [*Quotient mean ops*], [*Q/D*], [*Degree D/Q*],
  [7], [43,405], [87,503], [2.015966], [5 / 4],
  [9], [1,434,127], [1,806,786], [1.259851], [5 / 4],
  [19], [1,643,659,599.25], [2,114,506,365.625], [1.286462], [5 / 4],
)

#figure(
  image("figure.svg", width: 100%),
  caption: [Paired geometric-mean F4 work ratio over eight random targets per order. Lower is better.],
)

#pagebreak()

== Frozen gates

#table(
  columns: (2.3fr, 1fr),
  inset: 6pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Gate*], [*Outcome*],
  [Correctness; zero invalid relations], [Pass],
  [Candidate censoring no worse], [Pass: 0 / 0],
  [At least six `d=19` random pairs], [Pass: 8],
  [Upper 95% Q/D endpoint below 0.50], [*Fail: 1.286464*],
  [Productive degree lower on at least 6/8 pairs], [Pass: 8 / 8],
  [Build-work and peak-RSS ratios at most 2], [*Fail: RSS 2.4275*],
)

The work-ratio failure closes the arm by itself. The peak-RSS failure
independently shows that the one-degree reduction was purchased with too much
state. The exact membership-build coefficient count did not grow
(`Q/D <= 0.9838`); F4's expanded variable space is the problem.

== Correctness and flow

All 72 planned rows completed: 8 random and 4 planted targets for each of two
arms at `d in {7, 9, 19}`. No row timed out, hit the degree ceiling, was killed,
or was excluded. All 24 planted rows evaluated exactly and group-verified.
Every direct/quotient pair used the same subgroup scalar and affine target.

The native preflight evaluated 1,088 unordered distinct-root triples per arm
over the main field and an exhaustive tiny-curve panel with 24 positive and 12
negative target abscissae. The initial `g=1` fixture had no planted subgroup
target at `d=7`; before solver data, the first admissible coset was frozen as
`g=3` (4/10/34 planted target abscissae at `d=7/9/19`).

== Scope

This chart covers distinct roots only. Repeated-abscissa strata `[2,1]` and
`[3]` are outside it. Relation probability, setup, relation collection,
filtering, sparse linear algebra, individual logarithms, and target recovery
are unpriced. Therefore this is a negative solver-stage qualification, not a
total-work or subexponential ECDLP claim.

== Provenance

Native Rust instrument and analyzer; Isogeny Labs `crypto` repository. Raw
rows, paired CSV, JSON analysis, protocol, amendments, runner, SVG/PNG, and
`experiment.export/v1` are retained beside this report. Measurements used the
disclosed compile-repair PR #4650 overlay because the task branch has unrelated
duplicate-definition/module failures. No JavaScript was used.
