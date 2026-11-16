#set page(paper: "us-letter", margin: 0.62in)
#set text(font: "Helvetica Neue", size: 9pt)
#set par(justify: true)
#set heading(numbering: none)

= Paged binary pair index: actual-curve parity
_Source-bound correctness record · 2026-11-16_

The standalone paged first-witness index was checked on the ordered torsion
pair-sum streams for Koblitz E0 K13, K19, and K23. Curve IDs and signed-orbit
factor-base digests were pinned. The stream was independently replayed through
the public group law, with each insertion compared to a standard hash map and
the complete collision vector compared to the production torsion builder.
Each duplicate also passed a full four-point group equation check.

#image("diagram.svg", width: 90%)

== Observations

The unchanged test source, committed before execution, ran through a local
Cargo library harness with Cargo/rustc 1.99.1 on macOS arm64. All five tests
passed. A cached same-host replay returned the same three structured parity
records and retained process-level resource counters. This is a same-producer
result.

#table(
  columns: (0.8fr, 1fr, 1fr, 1fr, 1.2fr),
  inset: 5pt,
  stroke: 0.5pt + rgb("c5d1d8"),
  [Curve], [Inserts], [Witnesses], [Collisions], [Used / capacity],
  [K13], [272], [261], [11], [204 / 290],
  [K19], [4,830], [4,800], [30], [4,473 / 4,901],
  [K23], [18,906], [18,842], [64], [18,498 / 19,045],
)

#image("page_use.svg", width: 86%)

== Scope and next gate

The page counts describe key coverage in a preallocated prototype. They are
not process RSS. The test runner's 0.01-second total is not a collector timing.
Root-package test admission failed on macOS because an unrelated binary uses
Linux-only APIs; the local harness runs the same test source against this
checkout's library. Earlier compiler-selection failures and this platform
failure are retained as separate raw logs.

The next step is guarded production integration and same-target answer parity.
An isolated complete-call panel with the fastest comparable F4/F5/F6 arms is
still required for any speed conclusion. The existing complete-cost problem
record tracks native pricing and independent replay. No canonical performance
graph changed here because this run has no timing edge. Exact commands, source
hashes, and raw logs are in RESULTS.json and REPORT.md.
