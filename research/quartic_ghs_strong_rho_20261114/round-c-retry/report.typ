#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set heading(numbering: "1.")
#set par(justify: true, leading: 0.65em)

#align(center)[
  #text(size: 19pt, weight: "bold")[GHS strong-rho retry batching]
  #v(3pt)
  #text(size: 12pt)[Exact two-candidate slab: deterministic count screen]
  #v(5pt)
  #text(size: 8.5pt, fill: rgb("#4d596f"))[Round `ghs-strong-rho-r3-retry-slab2` · 2026-11-18]
]

#box(inset: 8pt, radius: 4pt, fill: rgb("#eef4fb"), stroke: rgb("#b9cce6"))[
  *Result.* The candidate preserved the answer and complete walk trajectory and
  reduced retry-only batch inversions from 45 to 2. It speculated 1,437 extra
  group additions, raising walk `F_q` multiplications from 53,825 to 104,206.
  It is a count regression and does not replace the default arm.
]

= Scope and control

The fixed public synthetic known-answer cell is `q=223`, instance seed 1,
target index 0, target `k=1,765,657`, 32 partitions, and batch width 8. Both
arms ran in one source-bound standalone binary. This is a development
correctness/count control with one deterministic sample. Wall time is excluded:
there was no isolation, A/A control, interleaving, or checkpoint admission.

Both arms recovered the planted logarithm after 1,487 steps, 76 distinguished
points, 53 look-ahead retries, 2 cycles, and 10 cycle additions. Setup,
verification, and every native count match. The helper self-test also covers
an eight-lane input with a lane that reaches the third candidate.

#figure(
  image("counts.svg", width: 100%),
  caption: [Counted tradeoff on the fixed cell. Exact values come from
  `q223.jsonl`; n=1 deterministic control, no uncertainty interval.],
)

= Complete counted delta

#table(
  columns: (2.2fr, 1fr, 1fr, 1.25fr),
  inset: 5pt,
  stroke: rgb("#d7dde8"),
  table.header([*Walk count*], [*strong-b8*], [*retry2*], [*Delta*]),
  [batch inversions], [231], [188], [-43 (-18.6%)],
  [retry-only batches], [45], [2], [-43 (-95.6%)],
  [`F_q` inversions], [241], [198], [-43],
  [`F_q` multiplications], [53,825], [104,206], [+50,381 (+93.6%)],
  [`F_q` additions], [195,560], [383,592], [+188,032],
  [group additions], [1,551], [2,988], [+1,437 (+92.6%)],
)

The first slab executes once for each of 186 outer rounds. The baseline then
executes 45 nonempty retry batches. Retry2 folds the first retry level into the
186 slabs and needs two third-candidate fallback batches. Its second candidate
is nevertheless computed for every lane in every slab.

Including unchanged setup and verification, the field-count difference is

#align(center)[`delta M-eq = 50,381 - 43 r_inv`.]

Break-even is `r_inv = 1,171.65`. The inherited N4-v2 robust observations are
15.00 to 25.88; even discarded migration-noisy observations were at most 325.
At the inherited Xeon sensitivity constants (`r_inv=15`, `r_lmul=1.1`), cold
cost rises from 188,844.1 to 238,580.1 M-eq, a 1.26337 ratio. Therefore this
candidate is count-higher throughout the recorded inversion-price range.

= Exact-order obstruction

Retry membership is known only after `P + M_j` is affine-normalised, hashed,
and compared with partition `j`. The current caller processes completed lanes
in lane-index order, performing canonicalisation, cycle handling,
distinguished-point lookup, table insertion, and collision stopping. Advancing
accepted lanes to gather additions for a future retry batch changes that order
and can change the first collision. Waiting at the barrier preserves order but
provides no later additions to compact.

The bounded alternative computes `P+M_j` and `P+M_(j+1)` from the same original
point in one inversion, inspects them in the original attempt-major order, and
falls back sequentially from `j+2`. That preserves semantics but exposes the
speculation cost measured above.

= Evidence and disposition

#table(
  columns: (1.8fr, 1.2fr, 2.4fr),
  inset: 5pt,
  stroke: rgb("#d7dde8"),
  table.header([*Requirement*], [*Status*], [*Evidence*]),
  [lane transitions], [verified], [`selftest.txt`, including two-retry fallback],
  [answer/RNG/table/native counts], [verified on fixed cell], [`q223.jsonl`, `validation.txt`],
  [retry inversion objective], [verified], [45 to 2 retry-only calls],
  [total counted cost], [failed], [+50,381 multiplications versus -43 inversions],
  [throughput/frontier], [not attempted], [checkpoint and matched-grid gates remain open],
)

Protocol commit: `d04ca66499`. Baseline revision:
`ad2a014c0022cf000845014499a9649b6c2cef2d`. Candidate source SHA-256:
`b788534182d3286f5f88d48bd51eda5309d30ceb468b42cbee9e61246b87444d`.
Binary SHA-256:
`0cd185940bd826c1d7431a395dc3ed1433cf3abcc6dad9ea204e0969ad025b62`.

Planned/attempted/completed arms: 2/2/2. Missing: 0. Both wall observations
are excluded. Producer result; no independent admission, leaderboard move, or
frontier claim. `[pcc:unverified]`
