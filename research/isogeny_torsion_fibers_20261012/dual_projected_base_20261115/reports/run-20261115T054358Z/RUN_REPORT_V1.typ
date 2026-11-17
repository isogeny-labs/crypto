#set page(paper: "a4", margin: 1.6cm)
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)

= Dual projected-base RUN_V1 execution

*2026-11-15 05:43:58–05:53:35 UTC · shared Darwin arm64 L0*

The frozen protocol used a degree-three dual map between the
different-$j$ curves $y^2=x^3-21x+37$ and $y^2=x^3-81x+179$ over
$F_(211^2)$. The order-229 subgroup panel contained four held-out
targets, three rounds, base widths 8/16/32/64 and ten cold arms per
cell. Each child had 1,832 relation trials and a 300-second wall cap.

All 480 children and 480 independent checker replays completed.
Four exhaustive pair gates and replays passed. Both dual composition
identities were checked on every 44,655 point of each curve. The
paired audit compared 96 dual/inverse cells and 13,482 relation
witnesses with zero mismatches. The raw records and hash manifest are
in `../../RUN_V1/`; the manifest verified.

#align(center)[#image("DUAL_GRAPH_V1.svg", width: 98%)]

The graph reports the whole-child counted group work divided by
$sqrt(229)$, with mean and observed range for 12 children per row.
At B=64 the direct dual, direct inverse, destination direct, BSGS and
signed rho means were respectively 2492.134, 2559.868, 2431.537,
4.361 and 7.979. The inverse/dual group-count ratio is 1.027179.
This excludes native costs outside group calls, so complete portable
$S$ is unset. Shared-host CPU is descriptive; isolated wall timing
is pending. The full table and phase ledger are in `../../RESULTS_V1.md`.
