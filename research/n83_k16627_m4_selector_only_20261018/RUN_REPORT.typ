#set page(paper: "a4", margin: 12mm)
#set text(font: "New Computer Modern", size: 9.1pt)
= N83 K = 16,627: selector-only four-summand run

*Run:* 2026-10-18 09:21:26--09:26:49 UTC. *Derived result:*
`PASS_THREE_CHECKED_ARMS`. The worker source is committed as
`48ebe3b370810e6da2d48e5234e42ba92bba259b`; the retained object,
six compiled files, five panel inputs, offline Linux image and binary are
bound in `BUILD_RECEIPT.md`. A1/B/A2 used one binary, the same public
seven-x-block witness, one CPU, hard 4 GiB, zero swap and a 1,800-second
per-arm cap. All exited 0 without OOM.

#image("selector-only-comparison.svg", width: 90%)

== Common model and measured stages

The pre-pin all-affine chained-S3 model had 201,598 variables, 5,915,238
clauses and 3,901 XOR rows. A1/A2 pinned four selectors and 28 phase-zero
wires. B pinned the same four selectors but no phase wires.

#table(
  columns: (1.2fr, 1fr, 1fr, 1fr, 1fr),
  inset: 3pt,
  [*Arm*], [*Import ms*], [*Root ms*], [*SAT ms*], [*Wall ms*],
  [A1 full], [38,117.484], [138.582], [10.046], [38,614.440],
  [B selectors], [35,445.011], [187.508], [6,487.733], [42,786.035],
  [A2 full], [35,656.408], [203.909], [17.314], [36,311.396],
)

A1 and A2 assigned all 201,598 variables at root, with zero decisions and
three completed XOR passes. B assigned 197,586 at root, then returned one
SAT model with 517 completed decisions, 222 conflicts, two restarts,
826 XOR passes and 1,124,379 row scans. Its completed XOR-pass timer includes
work beyond the separately timed SAT call, so it is not an exclusive CPU
fraction. The worker wall includes cold base import; these three L0
observations are not a complete IC timing or robust throughput comparison.

== Verification and open work

All three workers checked the original-field S3 chain, exact seven x blocks,
stored-point group sum, rank-one row and changed-target rejection. Their row
digest is `52b06cef457ecd736ba3f125f4c62bacd31d9bbfc84f2bf6b8d84513f947fe0f`.
Separate native replay read the compressed base and each receipt, repeated
the field, group and row checks, and rejected one changed-x and one changed-row
receipt per arm. The worker did not export SAT assignment bits. The verifier
shares the library's field and curve arithmetic and has not run on a second
host. B returned before its cap, so conditional phase-only C was omitted.

Free phase wires can be resolved for this constructed x-chain after its
representative selectors are supplied. Natural-query relation yield,
useful rank, selector acquisition for unknown decompositions, the best
prebuilt base at m = 2, 3 or 4, complete cold IC cost and matched rho remain
open. `RUN_REPORT.md` links the raw receipts, replay and 70-file hash manifest.
