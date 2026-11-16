#set page(paper: "a4", margin: (left: 17mm, right: 17mm, top: 16mm, bottom: 16mm))
#set text(font: "Arial", size: 10pt)
#set par(justify: true)

#align(center)[
  #text(size: 17pt, weight: "bold")[Direct-index torsion pair probe]
  #linebreak()
  #text(size: 11pt)[Source and correctness report · 2026-11-16]
]

#v(6pt)
#block(fill: rgb("eff6ff"), inset: 9pt, radius: 4pt)[
  *Scope.* Producer correctness checks and exact allocation arithmetic only.
  There is no timing result or performance conclusion. Source commit
  `de23b92b5e4929a4fa44926155c1e6d67f41ee26`; frozen protocol commit
  `4ae60f1f92`.
]

= Mechanism

The existing torsion-harvest builder fixes one first witness per binary sum
abscissa. The opt-in `layout=dense` arm converts its completed hash table to
an array indexed by `x + 1`. Zero means absent; the high word holds `i + 1`
and the low word holds `j`. Collision rows, target scan and group re-addition
are unchanged. The hash build still occurs; the conversion adds zero-fill,
witness writes and memory before relation collection.

#figure(
  image("diagram.svg", width: 100%),
  caption: [Derived data flow from the candidate source. The green nodes are the unchanged correctness path.],
)

= Fixed controls and results

Koblitz E0 curves over GF(2^13), GF(2^19), GF(2^23), standard-subspace
dimensions 5, 7, 8 and signed Frobenius-orbit columns were checked. Full
curve UIDs and test custody are in `REPORT.md`. The release library test
passed (1/1): all stored first witnesses match on all three curves; every
possible K13 key and sampled signed group probes agree. The release ecbench
binary suite passed 21 tests, with one ignored. Its same-target K13 pipeline
checks scalar 641 in both layouts: 23 target trials, 13 target-bearing rows,
rank 18, and 2,262 group-addition equivalents. Unsupported layout use and
n=24 conversion refuse. These are source-local checks, not independent replay.

The pinned release `ecbench` binary (SHA-256 `2d9307249c036f5744551035d88c72c85193fa96f72b0ba4d8863ece42c96fd9`)
accepted the plan-only spec. Plan `ECS1h7bd090d1bd39` expands to four
matched K13 public targets and 144 scheduled executions across F6, hash,
dense and dense A/A arms. No scheduled execution was launched.

#pagebreak()
= Exact storage and remaining cost

#figure(
  image("dense_storage.svg", width: 100%),
  caption: [Derived direct-array allocation: 8 × (2^n + 1) bytes. Three declared degrees; no measured RSS or wall time.],
)

#table(
  columns: (1fr, 1.5fr, 1.5fr),
  inset: 6pt,
  [Degree], [Direct-array bytes], [Approximate MiB],
  [13], [65,544], [0.0625],
  [19], [4,194,312], [4.0000],
  [23], [67,108,872], [64.0000],
)

#v(8pt)
*Dominance check.* The same-source hash layout is the reference. Tested
outputs and counted group work are equal; the dense layout adds conversion
and initialized memory. Whether cheaper lookups repay those costs is unknown.
An earlier N83 packed-root table also has a dense option, but it uses a
different data structure and curve (`research/n83_dense_root_20261013/PROTOCOL.md`).
This integration is not claimed as novel.

*SOTA delta.* Relation-search exponent: zero change. Fixed relation and rank
trajectory: zero change in the source checks. Complete-call wall ratio,
F4/F5/F6 comparison, rho ratio and peak RSS: unknown. The next check is a
frozen alternating hash/dense/dense-A/A same-target panel with cold setup,
native work and memory. Its branch-specific S3 resume checkpoint must be
valid first; Conductor T-6342 still reserves `.resume`.

#v(6pt)
#text(size: 8pt, fill: rgb("475569"))[
  Evidence: `REPORT.md`; raw `test-lib.stdout`, `test-lib.stderr`,
  `test-bin.stdout`, `test-bin.stderr`; editable `diagram.dot` and
  `make_visuals.py`. Provenance: internal source and producer tests. This
  report does not change the frozen EKS L2 comparator panel.
]
