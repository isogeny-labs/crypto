#set page(paper: "a4", margin: 1.7cm)
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)

= Factor-base grid RUN_V2 execution

*2026-11-15 04:38:25–04:40:50 UTC · shared Darwin arm64 L0*

The frozen V2 protocol repeated the same 384 cold order-229 children
as V1: four held-out targets, three rounds, four base prefixes
(8/16/32/64), and eight arms per cell. The explicit degree-three map
has source $y^2=x^3-21x+37$ and different-$j$ destination
$y^2=x^3-81x+179$ over $F_(211^2)$. Each child retained the
1,832-trial relation cap and 300-second wall cap.

All 384 children and independent checker replays exited zero. Four
exhaustive χ pair-set gates and independent replays passed. The raw
record and SHA-256 manifest are in `../../RUN_V2/`, the exact protocol
in `../../PROTOCOL_V2.md`, and the native V1/V2 audit in
`../../V2_AUDIT.json`. The manifest verified. The audit compared all
384 deterministic records and 33,705 relation witnesses with zero
mismatches; both plan hashes matched. The serialized χ fallback count
summed to zero.

#align(center)[#image("GRID_GRAPH_V2.svg", width: 95%)]

The graph is the whole-child counted group work divided by
$sqrt(229)$. This omits native work outside group additions and
doublings. The result is an accounting replay, not a complete portable
operation price. Shared-host CPU is descriptive; no isolated wall
ratio is admitted. The address-space hard cap was unsupported on this
host. The full same-unit table and disjoint phase readings are in
`../../RESULTS_V2.md`.
