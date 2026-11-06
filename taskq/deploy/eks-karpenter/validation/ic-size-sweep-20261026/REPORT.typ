#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9pt)
= EKS Spot index-calculus curve-size validation
2026-10-26 · public synthetic, one target per curve

Four independent `c8a.2xlarge` Spot Jobs ran the frozen `ecbench.spec/v1`
panel at 20, 24, 28 and 32 bits. Each job used five measured rounds and one
warmup, with `ic.pipeline`, matched `rho.negation`, and a rho A/A control.
All 72 records were verified, all four native audits passed, and each job
earned 18/18 L2 records. Each audit included 15 exact replays. The repository
`README.md` and `n*/` receipts provide exact specs, session records, audit
reports, source/image identity, Job/pod IDs, and S3 prefixes.

#figure(image("online-median.svg", width: 100%), caption: [Recorded online
medians on a logarithmic scale. Five measured rounds per size. Source:
`n*/session/records.jsonl` and `n*/control-comparison.json`.])

The recorded IC online medians were 0.849690, 23.555580, 435.896655, and
9520.577446 ms for 20, 24, 28, and 32 bits respectively. Matched rho medians
were 1.183083, 1.646459, 3.453848, and 7.591005 ms. The corresponding IC
total GAE medians were 8529, 165732, 2516038, and 31077180; rho medians
were 1016, 4510, 19263, and 49663. These values are descriptive diagnostics.
Online timing excludes setup; no end-to-end `S`, external comparison, or
precomputation pricing band was established. The 28 and 32-bit A/A confidence
intervals are wide. This report makes no speedup or security claim.

The checked image digest is
`sha256:67eea58f5d4a7a8882b55f089c6063bb49fad58d40260ee9b0ceb349fc0e4581`;
the native executable SHA-256 is
`583e62f0836eae43330c4b36bbe3d145dacb01de3132436c71f6924445c6a5a9`.
The new Actions dispatch still requires a post-merge run. The four-job panel
is separate from the worker-loss recovery diagnostic below.

== CRIU diagnostics

The first resumable n32 request lost Spot capacity before its first committed
checkpoint, and the controller refused to restart from zero. The second
request committed and read back an encrypted CRIU/EBS generation; its native
audit found 17 verified records out of 18 and therefore rejected the
computation before any fresh benchmark. These are failed recovery diagnostics,
not completed measurements. Their durable states and the second audit are in
`criu/v1` and `criu/v2`.

The separate `criu/v3` fixture kept the n32 curve and arms but allowed 600
seconds per round. Two encrypted generations passed S3 readback and EBS
snapshot readiness. AWS reclaimed the original Spot worker; the controller
restored generation 2 on a different Spot worker. Its native audit verified
18/18 records and all 15 exact replays. Interrupted timing was discarded.
The distinct fresh Spot epoch then passed 18/18 L2 executions, 15 exact
replays, an admitted five-pair A/A control and all 19 S3 payload hashes.
The terminal state, CRIU computation archive and fresh Job receipts are in
`criu/v3`. This validates checkpoint and restore for this fixture; it does
not establish a speed result or generalize beyond the tested runtime.
