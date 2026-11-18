#set page(paper: "us-letter", margin: (x: 19mm, y: 17mm))
#set text(font: "New Computer Modern", size: 10pt)
#set par(justify: true)

= Run report: 64-target reused-setup dual transport

*Run ID:* `run-20261118T170106Z` #h(1em)
*Source commit:* `0077f9d154724ca7ecbf9914040dc95a9eb3ab28`

The run began 2026-11-18 17:01:06 UTC and ended 17:01:11 UTC on an
Apple M4 Pro, macOS 27.2, Rust 1.99.1. The host earned L0: its core was
shared and no address-space limit was enforced. The V1 protocol and V2
pre-run amendment fixed B=64, 64 distinct public order-229 targets, three
rounds, six arms, input hashes, seeds, caps, and child schedule.

All 18 requested children completed and all 18 independent native replays
passed. Each child emitted 64 target records and one whole-batch summary.
The pair audit compared 192 dual/inverse cells and 76,711 relation witnesses
without a mismatch. No child timed out or was censored. Maximum observed
peak RSS was 12,206,080 bytes. The run manifest passed SHA-256 verification.

The table uses only the counted-group unit
$S_"group" = ("whole-batch additions and doublings") / (64 sqrt(229))$,
averaged across three rounds. Native field, square-root, table, allocation,
startup, and I/O work remains unconverted.

#table(
  columns: (2.2fr, 1fr, 1.2fr),
  inset: 4pt,
  [*Arm*], [*Batches*], [*Mean $S_"group"$*],
  [Direct dual], [3/3], [1,714.111],
  [Direct inverse Vélu], [3/3], [1,714.442],
  [Destination direct], [3/3], [1,712.437],
  [Source BSGS], [3/3], [1.659],
  [Signed rho], [3/3], [6.654],
  [Signed rho A/A], [3/3], [6.654],
)

#figure(
  image("S_GROUP_GRAPH_V1.svg", width: 100%),
  caption: [Mean whole-batch counted group work per target, logarithmic axis.
  The plotted data and full-precision table are `RUN_V1/SUMMARY_V1.json`.],
)

The raw whole-batch ranges are 1,576,643–1,720,411 counted operations for
dual, 1,576,964–1,720,732 for inverse, 1,575,022–1,718,790 for destination
direct, 1,607–1,607 for BSGS, and 6,297–6,623 for either rho arm. The
source-linked Markdown run report lists every input and receipt path. L0
CPU and wall observations are descriptive. Complete portable cost and an
isolated wall ratio were not measured.
