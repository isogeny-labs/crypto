#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set par(justify: true)

= Primary N83 K2048 five-summand model panel
Run 20261027T2012Z · 27 October 2026

Both matched 2,048-orbit bases installed the same-size chained-S3 model on
`EC1N83Ckb0h252dacc102a4`. *All four natural searches returned Unknown at
the frozen 5,000-conflict cap.* The exact model had 111,478 variables,
1,140,557 initial clauses and 4,980 XOR rows in each arm.

#image("capped_search.svg", width: 100%)

_Figure 1._ Measured SAT search time through 5,000 conflicts. One native run
per arm and public point; no sampling interval. Source:
`evidence/panel-native-audit.json`.

== Recorded cells

#table(
  columns: (auto, auto, auto, auto),
  inset: 5pt,
  [Public point], [U search], [D search], [U/D (derived)],
  [Fixture 0], [152.482 s], [166.213 s], [0.917],
  [Fixture 1], [159.272 s], [153.158 s], [1.040],
)

The ratio divides recorded U and D search milliseconds on the same point.
Its direction reverses between fixtures. These are capped solver-stage
durations on a shared local host, without a CPU wall-time comparison tier.
No group relation or useful rank was verified by this panel.

== Method and audit

The frozen protocol paired two existing public points on the primary N83
curve and fixed U/D object hashes, five-summand chained-S3 encoding, zero
split bits, a 5,000-conflict cap and a 240-second solver deadline. The native
replay ran U0/D0/D1/U1 after the versioned order correction. Model binary
SHA-256: `5123ef012623a0f425f93a91979a918f7e2df369c1309352919a47e9eef52f06`.
The Darwin arm64 shared host ran the pinned Linux arm64 image with one CPU,
4 GiB memory, zero swap, no network, read-only root filesystem and a
300-second host watchdog. All six guards passed without OOM or watchdog
activation. The native audit validated input and binary hashes, exact target
pairing, order, model dimensions, resource limits and every terminal status.
Changed input and target-point controls were rejected.

The raw cells and audit are in `research/n83_k2048_m5_pair_20261027/evidence/`.
The historical Python runner and audit source are retained as provenance;
the Rust guard and auditor are the active path. Final matrix, target-log and
same-point rho times are unmeasured, so candidate selection and online
speedup stay null. The next gate is a verified natural group relation.
