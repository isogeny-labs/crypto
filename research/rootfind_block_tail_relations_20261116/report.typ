#set page(paper: "us-letter", margin: 0.62in)
#set text(font: "Helvetica Neue", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= First-hit block-tail relations
_Fixed-stream negative result · 2026-11-16_

The m=3 batch oracle computes a block of up to eight residuals, then
returns the first valid triple. This source-pinned screen probed the
remaining positions of that already-computed hit block and reduced
every valid row. First-hit, block-tail and full-scan arms used identical
Koblitz curves, factor bases, first-witness tables, seed rows and target
streams. Every triple was independently re-added; all full-rank arms
recovered the planted scalar 641.

#image("diagram.svg", width: 100%)

== Fixed-stream result

#table(
  columns: (0.5fr, 0.8fr, 0.8fr, 0.8fr, 1.65fr),
  inset: 4pt,
  stroke: 0.5pt + rgb("c5d1d8"),
  [Cell], [First queries], [Tail queries], [All queries], [First / tail / all group proxy],
  [K13], [47], [47], [35], [1,193 / 1,199 / 1,202],
  [K19], [154], [154], [154], [11,862 / 11,874 / 21,767],
  [K23], [659], [659], [508], [136,257 / 136,267 / 140,276],
)

Block-tail saved zero distinct queries in every cell and added 67,
306 and 813 pair lookups. It used exactly the same computed subtraction
blocks as first-hit through full rank, with 6, 12 and 10 more pair
re-additions. Later-hit rows advanced rank 0, 2 and 2 times across the
complete K13 and sampled K19/K23 streams, but never moved the final
query threshold.

#pagebreak()

== Counts and boundary

#image("tradeoff.svg", width: 100%)

K13 traversed its whole order-2,003 subgroup in a different order from
the earlier screen. K19 and K23 each used 1,024 stride-53 residuals
disjoint from the earlier sampled coefficients. This is deterministic
holdout evidence, not a population estimate or an independent replay.

The group proxy is the number of computed batch rests plus pair
re-additions. It excludes native field operations, lookup latency,
row-reduction time, setup, target generation, memory and verification.
The test runner's process wall does not measure the collector.

== Next discriminator

Protocol commit `fe79c218f3`; source commit `d9ba9ca5d6`;
machine result `RESULTS.json`; raw run `run.*`. The simple block-tail
policy is not advanced for integration on these cells. A rank-aware
cross-block search with a frozen cap would need complete-cost controls,
then matched F4/F5/F6 timing and independent replay.
