#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: none)
#set par(justify: true, leading: 0.62em)

#align(center)[
  #text(size: 18pt, weight: "bold")[Pair-trace 2+1 split for a Curve25519 relation system]
  #v(3pt)
  #text(size: 10pt)[EXP-C25519-PAIRTRACE-001 · 2026-11-25 · [pcc:unverified]]
]

#v(8pt)
#block(fill: rgb("f8e8e8"), inset: 9pt, radius: 4pt)[
  *Decision: stop before d=9, d=19, scaling, and m=4.* Primary P uses
  *2371.7223x* D's counted F4 work at d=7, with descriptive 95% interval
  *[2371.7203, 2371.7238]*. Peak RSS reaches *14.4512x* D.
]

== Question

Can three factor-base roots be split as an unordered pair plus one root,
quotient the pair swap exactly, and replace one S4 relation by two quadratic
S3 links? For odd d, the unordered pair is represented by its unique scale
z and ratio trace c=q+q^-1. P uses exact trace-polynomial membership for c,
scalar root-of-unity circuits for z and the lone root, and retained local
products so every input equation is quadratic. R is the raw-root version of
the same quadratized split; D is the direct symmetric baseline.

== Counted d=7 result

#table(
  columns: (auto, 1.35fr, auto, auto, auto, auto),
  inset: 4.5pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Arm*], [*Mean field ops*], [*ratio / D*], [*vars*], [*degree*], [*max RSS KiB*],
  [D], [43,405.00], [1.0000], [15], [5], [2,488],
  [R], [49,652,778.50], [1143.9414], [18], [4], [16,152],
  [P], [102,944,605.75], [2371.7223], [18], [4], [35,044],
)

#figure(
  image("figure.svg", width: 100%),
  caption: [Paired geometric-mean counted F4 work over four fresh random d=7 targets. Lower is better; the dashed line is the frozen P/D ceiling.],
)

#pagebreak()

== Frozen gates

#table(
  columns: (2.65fr, 1fr),
  inset: 6pt,
  stroke: 0.4pt + rgb("bbbbbb"),
  [*Gate*], [*Outcome*],
  [Self-test, planted exactness, and paired targets], [Pass],
  [D exact predecessor reproduction], [Pass: 0 / 8 mismatches],
  [P censoring no worse; at least three pairs], [Pass: 0 / 0; 4 pairs],
  [Upper 95% P/D work endpoint below 1.00], [*Fail: 2371.7238*],
  [P productive degree at most 4 on at least 3/4], [Pass: 4 / 4],
  [Every P/D peak-RSS ratio at most 1.50], [*Fail: 14.4512*],
)

The algebraic mechanism works exactly, and the intended degree reduction is
real. It is overwhelmed by auxiliary-state cost. Even R, before quotienting
the pair swap, is 1,143.94 times D. P costs another 2.07329 times R at the
same d=7 variable count. This rules out solving the full quadratized 2+1
circuit as one ordinary-grevlex F4 ideal on this test family; it does not rule
out using the split operationally with separate image construction and a
join.

== Correctness and flow

All 24 planned rows completed: four random and four planted targets for each
of D, R, and P. No row timed out, hit the degree ceiling, was killed, or was
excluded. All 12 planted rows evaluated exactly and re-added to their target.
The native self-test exhaustively checked unordered-pair trace images and
trace roots at d=7,9,19, every retained circuit assignment, signed d=7
relations on the main curve, and the predecessor's tiny positive/negative
truth panel.

The first unit test found that a written trace-doubling identity expands to
degree three. Before any solver row, Amendment 1 retained the local product
#raw("t_k=c U_(k-1)"), preserving the frozen quadratic-input requirement without
changing the zero set, arms, targets, thresholds, or order. D then reproduced
the pinned predecessor on all eight fresh-seed rows, including counted work,
degrees, matrix extrema, steps, and verdicts.

== Scope and next direction

This is a d=7 relation-system stage diagnostic under ordinary grevlex. The
gated d=9 and d=19 stages were deliberately not opened. Relation probability,
complete collection, filtering, sparse linear algebra, individual logarithms,
and target recovery are unpriced, so no exponent or total-work conclusion
follows.

The next defensible lead must avoid monolithic auxiliary elimination. The
pair and lone-root sides should be solved or represented separately and joined
on the intermediate abscissa. Before implementation, a degree and prior-art
gate must determine whether a target-conditioned pair-image polynomial can be
constructed in subquadratic work in d; otherwise it merely repackages the
known O(d^2) pair enumeration and cannot move the exponent.

== Provenance

Native Rust instrument and analyzer in Isogeny Labs `crypto`; hash-pinned
measurement, predecessor, and analyzer binaries; disclosed PR #4650
compile-repair overlay because the task branch has unrelated build failures.
Raw rows, protocol, amendment, self-test, reproduction, analyzer, CSV, JSON,
SVG/PNG, Typst/PDF, and export are retained. No JavaScript was used.
