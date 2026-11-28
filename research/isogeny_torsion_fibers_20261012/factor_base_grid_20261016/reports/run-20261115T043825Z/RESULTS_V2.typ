#set page(paper: "a4", margin: 1.55cm)
#set text(font: "New Computer Modern", size: 9pt)
#set par(justify: true)

= Factor-base grid V2: counted cost and phase split

*Frozen 384-child replay; 384 independent checks; native V1/V2 audit passed.*

The order-229 subgroup over $F_(211^2)$ has a prespecified generic
two-class floor $sqrt(pi/4)=0.886227$. Signed rho and BSGS were
run on the same four targets and rounds. Every relation arm used
identical projected destination columns within its paired cell.
The full 32-row machine table is `../../RUN_V2/SUMMARY_V2.json`;
the Markdown results report reproduces every row.

#align(center)[#image("GRID_GRAPH_V2.svg", width: 95%)]

$S_"group"$ counts the whole cold child's additions and doublings,
including setup, query and final verification, divided by $sqrt(229)$.
The table gives means of 12 independently checked children per row.

#table(
  columns: (auto, 1fr, auto, auto, auto, auto, auto),
  inset: 3pt,
  stroke: 0.35pt + gray,
  [B], [χ inverse], [direct inverse], [destination direct], [BSGS], [rho], [χ full],
  [8], [168.823], [117.059], [99.349], [4.361], [7.979], [6,054.866],
  [16], [247.333], [180.492], [147.054], [4.361], [7.979], [6,116.063],
  [32], [1,672.455], [613.454], [547.637], [4.361], [7.979], [7,513.563],
  [64], [14,876.431], [2,559.868], [2,431.537], [4.361], [7.979], [20,657.933],
)

At B=64, mean thread CPU for direct inverse was 16.479 ms, of which
10.489 ms was setup and 5.388 ms relation search. Destination direct
used 9.043 ms total, 3.355 ms setup, 5.134 ms relation search.
χ inverse used 3,689.238 ms total, 3,679.435 ms relation search.
Row insertion and scalar recovery were measured separately and
phase sums checked on every child. These are shared-host L0
diagnostics, not a wall-speed comparison.

The counted-group inverse/destination-direct ratio at B=64 is
1.053; χ/direct inverse is 5.811. This table omits field arithmetic
outside group calls, square-root internals, polynomial/table work,
allocations, startup and I/O. Complete portable $S$, L2 isolated
wall timing, and a larger-order trend remain open. The proposed
dual-isogeny transport requires a separately frozen, fully charged
same-target experiment.
