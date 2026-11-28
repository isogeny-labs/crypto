#set page(paper: "us-letter", margin: 0.72in)
#set text(font: "New Computer Modern", size: 10pt)

= N83 Linux ARM64 host admission

#text(size: 9pt)[Run started 2026-11-16T00:03Z · Frozen host protocol · Local Colima VZ aarch64]

*Observed host result: passed.* The Colima VM started with Docker 29.5.2 and cgroup v2. Its terminal receipt records exit 0 after 29 seconds. The first no-work container control stopped before container start with Docker exit 125: its script omitted the last three hex characters of the pinned image digest. That failure and raw log are preserved. A separately committed corrected control exited 0 after 18 seconds; the running Linux ARM64 container reported 4 GiB hard memory, zero swap, one CPU, and a 64 PID limit.

#figure(image("host-flow.svg", width: 100%), caption: [Observed admission and control sequence. Dashed arrow is a pending build gate, not an executed stage.])

== Evidence and limits

The source-linked result is `verification/k600-colima-host-20261115/RESULT.md`. The host-start and attempt-2 receipts and logs, first-failure receipt and log, corrected script, and host-state snapshot are retained there. The pinned no-work image was `rust@sha256:e6f2cff35d0f309acf8433b4f05878ed77092602c5065cc81801177da0bb3a9c`. The three public solver-source archives were verified by SHA-256 before copying to the VM input mount. The attempted container used no network, a read-only root, dropped capabilities and no new privileges; it left no named container behind.

This run establishes one qualified local ARM64 Docker host and an enforced no-work cgroup control. It does not provide a source-locked K600 toolchain image, a Linux release-library pass, K600 solver preflight, relation yield, matrix rank, target descent, matched rho, or a cold one-target IC total. The existing charged-build plan pins an image ID on a different daemon. A versioned amendment must freeze the new image ID before the build. Canonical IC graphs remain unchanged because this host check gives no IC performance row.
