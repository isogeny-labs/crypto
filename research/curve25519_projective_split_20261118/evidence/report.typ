#set page(paper: "a4", margin: (x: 20mm, y: 18mm))
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true, leading: 0.65em)

#align(center)[
  #text(size: 20pt, weight: "bold")[Curve25519 projective split qualification]
  #v(3pt)
  #text(size: 10pt, fill: rgb("#56606a"))[Frozen m=3 relation-finding experiment · 25 November 2026]
]

#v(10pt)
#block(fill: rgb("#f5e6e8"), stroke: rgb("#d1495b"), radius: 5pt, inset: 10pt)[
  #text(weight: "bold", fill: rgb("#8d2432"))[Decision: not qualified for m=4.]
  The candidate preserves the exercised correctness boundary, but its observed
  counted cost is indistinguishable from—and slightly above—the affine-chain
  baseline. The t=5 cells timed out and the t=6 cells were memory-killed.
]

_Evidence status: [pcc:unverified]. This component experiment does not establish
a subexponential ECDLP algorithm._

= Question and frozen rule

The experiment asks whether representing the intermediate point in a split
Semaev chain by normalized projective coordinates `(X:Z)`, including the
identity chart, reduces the native F4 work needed to find relations on a
Curve25519-like prime-field miniature.

The frozen qualification uses `p=1,048,573`, subgroup order `r=131,203`,
`m=3`, factor-base bits `t=3..6`, 32 matched random targets and 8 planted
targets per cell. The primary unit is F4 field multiply-adds. Promotion
requires exact group-verified relations, no worse censoring, candidate native
work at most twice baseline, and an upper paired 95% cost-ratio bound below
`0.80`. The later m=4 exponent test is gated on this decision.

= Observations

#figure(
  image("figure.svg", width: 100%),
  caption: [Completed-cell cost and retained censoring outcomes.],
)

#block(breakable: false)[
#table(
  columns: (0.7fr, 1.25fr, 1.25fr, 0.9fr, 1.6fr),
  align: (center, right, right, right, left),
  inset: 4pt,
  stroke: rgb("#d7dbdf"),
  table.header(
    [*t*], [*chain mul-adds*], [*projective mul-adds*], [*ratio*], [*status*]
  ),
  [3], [111,785,088.281], [111,909,801.281], [1.001116], [complete],
  [4], [22,255,293,450.656], [22,265,619,555.656], [1.000464], [complete],
  [5], [—], [—], [—], [both timeout at 900 s],
  [6], [—], [—], [—], [both memory-killed],
)
]

The descriptive geometric mean of the 32 available `t=4` paired ratios is
`1.000463984`; a deterministic 10,000-resample bootstrap gives
`[1.000463984, 1.000463985]`. This interval is descriptive only because the
larger cells are censored. It plainly misses the frozen `<0.80` threshold.
No four-point solver-cost slope can be estimated.

= Correctness and accounting

The projective equations use `Z^2=Z`, the affine chart `Z=1`, the normalized
identity `(1:0)`, and `(1-Z)(X-1)=0` to exclude `(0:0)`. Adjacent `S3`
equations are bihomogenized without division by `Z`. Every returned row is
sign-lifted and re-added in the elliptic-curve group before acceptance.

Three release tests passed: an explicit signed inverse-pair/identity witness;
planted m=3 presentations; and 40-target agreement among affine-chain,
projective-chain, full-S4, and exhaustive-grid verdicts. In each completed
measurement cell, 32/32 random targets completed, all 8/8 planted relations
were exact, hits equaled verified hits, and invalid rows were zero. The
projective planted measurements exercised 13 affine-chart and 3 identity-chart
hits in total.

The task revision was overlaid on compile-repair PR #4650 because the task
branch's root build has unrelated failures. Disposable shims did not touch the
projective equations, curve family, tower solver, prime arithmetic, or test
bodies. Exact source, binary, diff, raw-file, protocol, and toolchain hashes
are retained in the machine export and qualification metadata.

= Interpretation

Projectivization closes an algebraic correctness gap—the split chain can now
represent the identity intermediate—but it does not reduce this F4 system.
At t=3 and t=4 it adds a small amount of counted work. Equal censoring does
not rescue the candidate: there is no completed evidence at t=5 or t=6 and no
measured exponent lever.

Therefore the frozen protocol stops this arm before m=4. The next research
lead must be separately frozen and must attack the permutation and
factor-base-membership structure, not merely reparameterize the same chain.

= Reproduction pointers

The source-of-truth directory contains `PROTOCOL.md`, `protocol.json`,
`export.json`, all eight raw aggregate paths (including the four empty
censored files), `raw/attempts.json`, `raw/qualification-tests.json`, the
128-row per-target JSONL, editable CSV/SVG, native Rust analyzer, host record,
and this Typst source. The documentary benchmark copy is outside the admitted
table until independent validation exists.
