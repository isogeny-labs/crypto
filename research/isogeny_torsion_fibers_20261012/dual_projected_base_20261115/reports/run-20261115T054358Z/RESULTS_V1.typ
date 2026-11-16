#set page(paper: "a4", margin: 1.55cm)
#set text(font: "New Computer Modern", size: 9pt)
#set par(justify: true)

= Dual projected-base V1: paired counted cost

*480 cold children · 480 independent replays · 96 dual/inverse cells matched.*

The explicit dual map transports the fixed projected destination base
to the order-229 source subgroup. All relation rows, ranks and
recovered scalars agreed between paired dual and inverse arms. The
independent identity gate checked the two map compositions on all
44,655 rational points of each curve.

#align(center)[#image("DUAL_GRAPH_V1.svg", width: 98%)]

$S_"group"$ counts setup, query and verification additions and
doublings per $sqrt(229)$ for a cold child. Each value is a mean of 12
verified children. The full 40-row table and ranges are in
`../../RESULTS_V1.md` and `../../RUN_V1/SUMMARY_V1.json`.

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  inset: 3pt,
  stroke: 0.35pt + gray,
  [B], [direct dual], [direct inverse], [dest. direct], [BSGS], [rho],
  [8], [108.137], [117.059], [99.349], [4.361], [7.979],
  [16], [163.244], [180.492], [147.054], [4.361], [7.979],
  [32], [578.629], [613.454], [547.637], [4.361], [7.979],
  [64], [2492.134], [2559.868], [2431.537], [4.361], [7.979],
)

At B=64, the derived inverse/dual counted-group ratio was 1.027179;
direct dual/signed rho was 312.322. Mean whole-child thread CPU was
9.006 ms direct dual, 15.557 ms direct inverse, 9.110 ms destination
direct and 0.614 ms signed rho. This was shared-host L0 execution;
the CPU readings do not yield an isolated wall ratio.

The 12 raw B=64 records give a derived group-count stage diagnostic.
Mean setup/query/verification $S_"group"$ is
805.736/1685.374/1.024 for direct dual,
873.470/1685.374/1.024 for direct inverse, and
1.718/5.237/1.024 for signed rho. The dual saves 67.734 in setup
and changes none of the paired query group work. With hypothetical
zero setup, its measured query plus verification count is 1686.398,
which is 211.344 times the measured whole-child rho count. This is
a fixed-target sensitivity calculation, not a 64-target reuse run.

The prespecified generic two-class counting floor is 0.886227.
Complete portable $S$ remains unset because field, square-root,
polynomial/table, allocation, startup and I/O work is not fully
converted. The 64-target reused-setup panel, larger-order trend and
physical-core isolated timing remain pending.
