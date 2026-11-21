#set page(paper: "a4", margin: 1.6cm)
#set text(font: "New Computer Modern", size: 9pt)
#set par(justify: true)

= RUN_V1: different-j factor-base grid

*2026-10-16 23:25:10–23:42:40 UTC · source-linked run report*

The frozen V1 protocol fixes the degree-three map from
$y^2 = x^3 - 21x + 37$ to $y^2 = x^3 - 81x + 179$ over $F_(211^2)$,
the order-229 subgroup, four held-out targets, three rotated rounds,
and 8, 16, 32, 64 factor-base columns. The eight arms are χ with
inverse Vélu or full-image setup, direct pair lookup with either setup,
destination-direct, source BSGS, signed rho, and rho A/A. Every one
of the 384 cold solves and independent Rust checker replays exited zero.
The four exhaustive χ pair-set checks and their replays also passed.
The raw run, source and binary snapshots, SHA-256 manifest, full
machine table, and public inputs are in `../../RUN_V1/`; the frozen
protocol is `../../PROTOCOL_V1.md`.

Pair support for B=8, 16, 32, 64 was 99/229, 219/229, 229/229,
229/229. The five relation arms used the same projected base in
every paired cell. Their mean trials to useful rank were 33.583,
27.667, 126.167, 398.000; final ranks were 9, 17, 33, 65.
Their relation counts and recovered scalars matched within each cell.

#align(center)[#image("GRID_GRAPH_V1.svg", width: 95%)]

The graph and table use $S_"group" = ("additions" + "doublings") / sqrt(229)$
for the *whole cold child*, including setup, query and final scalar
verification. This counted-group view leaves other native work
unpriced. Means use 12 children per B and arm. The generic two-class
count floor was frozen as $sqrt(pi / 4) = 0.886227$ in this unit.

#pagebreak()

== Count table and measurement limits

#table(
  columns: (auto, 1fr, auto),
  inset: 3pt,
  stroke: 0.35pt + gray,
  [B], [Arm], [Mean $S_"group"$],
  [8], [χ inverse Vélu], [168.823],
  [8], [χ full image], [6,054.866],
  [8], [Direct inverse Vélu], [117.059],
  [8], [Direct full image], [6,003.102],
  [8], [Destination direct], [99.349],
  [16], [χ inverse Vélu], [247.333],
  [16], [χ full image], [6,116.063],
  [16], [Direct inverse Vélu], [180.492],
  [16], [Direct full image], [6,049.222],
  [16], [Destination direct], [147.054],
  [32], [χ inverse Vélu], [1,672.455],
  [32], [χ full image], [7,513.563],
  [32], [Direct inverse Vélu], [613.454],
  [32], [Direct full image], [6,454.562],
  [32], [Destination direct], [547.637],
  [64], [χ inverse Vélu], [14,876.431],
  [64], [χ full image], [20,657.933],
  [64], [Direct inverse Vélu], [2,559.868],
  [64], [Direct full image], [8,341.370],
  [64], [Destination direct], [2,431.537],
  [all], [Source BSGS], [4.361],
  [all], [Signed rho], [7.979],
  [all], [Rho A/A], [7.979],
)

At B=64 the descriptive mean thread CPU was 4,224.214 ms for χ
inverse, 15.719 ms for direct inverse, 9.561 ms for destination
direct, and 0.475 ms for rho. This Darwin run had shared L0
placement. A release-library test overlapped its early phase and was
interrupted at 23:29 UTC; no isolated wall ratio is admitted.

V1 grouped relation search, row insertion and scalar recovery into
one query CPU phase, and its solve JSON omitted the χ fallback-scan
counter. The predeclared V2 rerun records those separately and will
audit all deterministic V1/V2 records. Square-root internals,
polynomial and table work, allocation, startup and I/O still lack a
complete portable price. This run is a local count result at one
subgroup order, not a scaling or complete-price result. The full
Markdown report beside this PDF links every raw artifact.
