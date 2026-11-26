#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)
#set par(justify: true, leading: 0.62em)

#align(center)[
  #text(size: 18pt, weight: "bold")[Selective quotient cuts in a symmetric Curve25519 relation system]
  #v(3pt)
  #text(size: 10pt)[EXP-C25519-SELCUT-001 · 2026-11-25 · [pcc:unverified]]
]

#v(8pt)
#block(fill: rgb("f8e8e8"), inset: 9pt, radius: 4pt)[
  *Decision: stop before scaling and m=4.* The primary B arm's paired
  geometric-mean F4-work ratio is *1.3292420766*, with descriptive 95%
  interval *[1.3292409735, 1.3292431924]*. The frozen upper endpoint had to
  be below *0.75*. Peak RSS independently reaches *2.2876x* D.
]

== Question

The predecessor made every multiplication in the binary computation of
$X^19 mod q$ explicit, where
$q(X)=X^3-e_1 X^2+e_2 X-e_3$. That lowered maximum productive degree from
5 to 4 but raised counted work and memory. This study retains quotient
variables only at selected schedule cuts. The primary B policy cuts the last
two nonlinear squares, adding four variables instead of twelve.

== Counted result

#table(
  columns: (auto, 1.3fr, auto, auto, auto),
  inset: 5pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Arm*], [*Mean field ops*], [*ratio / D*], [*degree*], [*max RSS KiB*],
  [D], [1,643,659,599.250], [1.000000], [5], [130,272],
  [L], [3,293,143,403.375], [2.003543], [5], [212,652],
  [B], [2,184,821,498.875], [1.329242], [4], [297,796],
  [N], [2,177,023,623.000], [1.324498], [4], [309,648],
  [U], [1,604,442,717.625], [0.976141], [5], [139,896],
  [A], [2,114,506,365.625], [1.286462], [4], [315,728],
)

#figure(
  image("figure.svg", width: 100%),
  caption: [Paired geometric-mean F4 work over eight random d=19 targets. Only B was eligible to qualify.],
)

#pagebreak()

== Frozen gates

#table(
  columns: (2.55fr, 1fr),
  inset: 6pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Gate*], [*Outcome*],
  [Self-test, planted exactness, and paired targets], [Pass],
  [D/A exact predecessor reproduction], [Pass: 0 / 0 mismatches],
  [U degree drops below D on at most 2/8 pairs], [Pass: 0 / 8],
  [B censoring no worse and at least six pairs], [Pass: 0 / 0; 8 pairs],
  [Upper 95% B/D work endpoint below 0.75], [*Fail: 1.329243*],
  [B productive degree at most 4 on at least 6/8], [Pass: 8 / 8],
  [Every B/D peak-RSS ratio at most 1.25], [*Fail: 2.287642*],
  [Every B/D build-operation ratio at most 1.25], [Pass: 0.986357 max],
)

The variable reduction was real but insufficient: four local quotient
variables still increased F4's matrix-state cost enough to erase the
one-degree benefit. L retained degree 5 and was much worse. N and A retained
degree 4 but were also slower than D. U remained degree 5 and is a diagnostic
control, not a post-hoc replacement for B.

== Correctness and flow

All 72 planned rows completed: 8 random and 4 planted targets for each of six
arms at d=19. No row timed out, hit the degree ceiling, was killed, or was
excluded. All 24 planted rows evaluated exactly and group-verified. Every
arm used the same scalar and affine target within a target block.

The native preflight checked 1,088 unordered distinct-root triples per arm,
four planted subgroup targets per order and arm, and a tiny exhaustive panel
with 24 positive and 12 negative target abscissae. D and A reproduced the
predecessor direct and all-quotient rows exactly on targets, counted work,
degrees, matrix extrema, steps, and verdicts.

== Scope and next lead

This chart covers distinct roots and one small prime-field family instance.
Repeated-abscissa strata, relation probability, setup, filtering, sparse
linear algebra, individual logarithms, and target recovery are unpriced.
Thus it is a negative solver-stage qualification, not a total-work or
subexponential ECDLP claim.

The next defensible auxiliary-variable question is ordering rather than
quantity: can local quotient variables be placed in elimination blocks so
their sparse quadratic constraints are consumed without carrying their full
matrix state? That requires a new frozen study and fresh targets; the present
data do not authorize it.

== Provenance

Native Rust instrument and analyzer in Isogeny Labs `crypto`; hash-pinned
release measurement binary; disclosed PR #4650 compile-repair overlay because
the task branch has unrelated build failures. Raw rows, protocol, analyzer,
paired CSV, JSON, SVG/PNG, Typst/PDF, and export are retained beside this
report. No JavaScript was used.
