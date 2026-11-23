#set page(paper: "us-letter", margin: 0.62in)
#set text(font: "Helvetica Neue", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= Multiple decompositions per target
_Bounded relation screen · 2026-11-16_

The current batch rootfinding oracle returns the first verified triple
`P_i + P_j + P_k = R`. This source-pinned screen continued the ordered
factor-base scan, admitting each valid triple as another row for the
same target. Both arms used the identical curve, base, pair table,
collision seed rows and target stream. Every triple was independently
re-added, and every modular row was checked for consistency.

#image("diagram.svg", width: 100%)

== Fixed-stream result

#table(
  columns: (0.55fr, 0.9fr, 0.9fr, 0.95fr, 0.95fr, 0.9fr),
  inset: 4pt,
  stroke: 0.5pt + rgb("c5d1d8"),
  [Cell], [First queries], [All queries], [First lookups], [All lookups], [Extra independent],
  [K13], [26], [16], [418], [528], [3],
  [K19], [115], [92], [8,741], [12,788], [18],
  [K23], [740], [599], [152,582], [164,725], [19],
)

Both arms reached full rank and recovered the planted scalar 641 in every
cell. K13 traversed its whole order-2,003 subgroup; K19 and K23 used
1,024 distinct fixed-stride residuals each. The additional rows reduced
target queries but increased pair probes on each cell.

#pagebreak()

== Query/probe tradeoff

#image("tradeoff.svg", width: 100%)

The first-hit control stops its scalar scan at the first valid triple;
the all-hit arm scans every base point for that target. Production's
eight-point batch charges a complete hit block, so these counters are
not an exact production timing. The screen excludes target generation,
native field work, matrix time, setup, allocation and process memory.

Every target with a hit in this sample had multiple hits, while only
3/18/19 later-hit rows were independent in K13/K19/K23 arrival order.
The plausible permutation redundancy was not classified per hit.
The run is a same-producer source check, not independent replay.

== Evidence boundary

Protocol: `PROTOCOL.md`; before-data correction: `AMENDMENT-1.md`;
post-run prose annotation: `AMENDMENT-2.md`; source commit:
`aa129e0ab0`; machine record: `RESULTS.json`. The first root-level Cargo
launch built a Linux-only binary and failed before the test; its raw output
is retained. The corrected locked library harness passed one release test.

Next: test a rank-aware continuation with a frozen probe cap, then compare
complete same-target calls with first-hit and F4/F5/F6. No complete-cost
or runtime ordering follows from this scan model.
