# K600 m6 natural XCNF export admission

Status: **implemented and locally tested; no natural K600 XCNF exported**.
This amendment follows the [pinned](PINNED_REPLAY_AMENDMENT.md) and
[unpinned](UNPINNED_REPLAY_AMENDMENT.md) native model replay gates. The
frozen fixture-0 natural query is admitted only after both constructed
controls have terminal passing replay receipts.

The Rust `export-natural` command audits the exact pinned export/solver/replay
and unpinned export/solver/replay receipts. Both positive controls must bind
the same source, K600 object, zero-trial preflight, Linux arm64 image and
compiled binaries. The unpinned solver and replay must bind the pinned
replay-stage SHA-256, and the two exports must have identical constructed
target and witness data. Any terminal replay failure or changed hash rejects
the natural export before creating output. The command rechecks all controls
and input hashes after the exporter process.

If admitted, the existing native exporter creates the fixture-0 `natural`
XCNF in an isolated Docker container under the frozen 4 GiB hard memory,
no added swap, pinned one-CPU, 64-PID, no-network, read-only input/root and
outer wall caps. The output receipt binds the unpinned replay-stage digest,
natural XCNF hash, source and preflight. It records export only; it neither
launches a solver nor accepts a relation. A post-creation failure writes a
terminal failure receipt. Total cold IC runtime and factor-base selection
remain null.

Local macOS arm64 Rust 1.99.1 release example tests passed **11/11**, and
the production release example build exited 0. The
[test](natural-export-test.log.gz) and [build](natural-export-build.log.gz)
logs preserve exact output. The negative test rejects natural export without
the constructed controls before creating output. Real S3 staging, image
identity, complete positive control replay and natural export remain
unverified on the frozen Linux host.

This step made no S3 request or charged K600 run. The separate proposed
3,600-second K600 allowance and exact future astronomy-store writer and
destination remain pending. A natural solver/replay, useful rank, target
descent and fully charged one-target runtime comparison remain open. No
performance chart changed because this implementation step produced no
natural relation or timing observation.
