#set page(paper: "a4", margin: 16mm)
#set text(font: "Helvetica", size: 10pt, fill: rgb("233746"))
#set par(leading: .55em)
#text(size: 23pt, weight: "bold")[Zero-core genus 2: cold comparison]

Six new targets · 102 timed children · Six full gates · 15 November 2026

*Exactness passes; the registered 10× cost criterion fails.* Full weighted
elimination preserves the target while removing the place-log core. All six
scalars verify and all candidate stopping signatures agree. An existing
compact implementation is faster on every target.

#image("ZERO-COLD.svg", width: 100%)

#table(columns: (1fr,1fr,1fr,1fr,1fr), align: right,
 [Prime],[Best / zero],[Positive / zero],[A/A max],[Setup share],
 [1,009],[0.245×],[1.018×],[1.091×],[57.4%],
 [16,381],[0.362×],[1.136×],[1.036×],[32.4%])

*Measured workload.* Two reused family-0 genus-2 fixtures, three new targets each
(seed indices 17–19). Source a500c14f9a. Each arm runs in a fresh child.
Primary ratio is the per-target minimum complete cost among fourteen existing
implementations divided by zero-core cost. Fixture figures are three-target
medians; positive-core and A/A comparisons are secondary.

*Accounting.* Clocks include catalogue, allocations, sixteen public jumps,
walk, oracle, row/forest handling, algebra and final group verification.
Common fixture checks and known-answer construction are excluded equally;
extra harness verification is outside method clocks. Fixed 32,768-trial
candidate cap, 30-second timed/60-second gate limits, 600-second aggregate
child budget. All 108 children exit successfully, no missing or capped cell;
aggregate child wall time 2.378 seconds. Local ARM L0, no isolation guarantee.

*Exactness.* Gate, zero, positive and zero-AA agree on scalar, target
coordinates, trials, smooth relations, raw-row hash and terminal status.
Z1 passes. Z2 requires six recoveries and both fixture medians at least 10×;
it fails on both fixtures. A/A spread is observed, not a confidence interval.

*Scope.* Known weighted-forest specialization applies to degree-at-most-two
Gaudry rows. It does not automatically extend to genus 3/4 or degree-three
principal rows. Earlier forest-storage ratios are estimates, not runtime gains.
Neither mathematical novelty nor independent controlled replay is asserted.

*Evidence.* COLD-PROTOCOL.md, SUMMARY.json, native jq reduction, all child
outputs/PID/terminal receipts, hashes and raw archive. Record
20261115T1208Z-hec-zero-core-cold. Source checkpoint
20261115T120848Z-a500c14f9a20 preceded execution. Rust worker/collector/plot,
Typst report; registry tooling only records native observations.
