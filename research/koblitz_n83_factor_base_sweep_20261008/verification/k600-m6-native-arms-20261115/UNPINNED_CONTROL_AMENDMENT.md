# K600 m6 unpinned positive-control solver gate

Status: **implemented and locally tested; no real K600 unpinned solve run**.
This amendment follows the [pinned replay gate](PINNED_REPLAY_AMENDMENT.md)
and keeps the previously frozen arm order and resource limits.

The Rust `solve-unpinned` command accepts a positive-unpinned XCNF export
only alongside the exact pinned export, pinned SAT solver receipt, and
terminal passing pinned replay receipt. The replay gate checks their SHA-256
bindings to the same clean source, arm lock, zero-trial preflight, image,
exporter/replayer and solver bytes, original pinned XCNF, and model stdout.
It also checks the pinned replay result's 600-column signed-Frobenius row,
six chosen indices, subgroup order and null total runtime. A terminal pinned
replay failure receipt rejects admission.

The existing solver path then checks the unpinned XCNF and runs one bounded
CryptoMiniSat invocation with the same one-thread seed, 10,000-conflict,
4 GiB memory, zero additional swap, CPU, PID, network, read-only and outer
wall controls as the pinned arm. It rechecks both the unpinned input and
pinned control receipts after the process and records the pinned replay-stage
digest in its solver receipt. SAT remains pending native model replay; a
solver exit alone accepts no relation. Natural export and query are still
gated on the completed unpinned control.

Local macOS arm64 Rust 1.99.1 release example tests passed **11/11** and
the production release example build exited 0. The [test](unpinned-control-test.log.gz)
and [build](unpinned-control-build.log.gz) logs retain the exact compiler
output. The negative test rejects an unpinned solver call without a pinned
replay before creating output. These tests do not exercise the success path
on a source-matched Linux image. The unchanged root library gate remains
red at the prior 6,305 passed, 60 failed, 138 ignored result.

The frozen K600 input is in astronomy-store custody, but this amendment made
no S3 request, image build, preflight, XCNF export, solver invocation, or
model replay. The distinct proposed 3,600-second charged K600 allowance and
exact future N83 S3 writer/destination are pending the owner's answer.
No relation yield, rank, target descent or cold one-target total runtime was
measured. No performance chart changed for this implementation-only step.
