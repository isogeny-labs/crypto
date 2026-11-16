# K600 m6 unpinned positive-control replay gate

Status: **implemented and locally tested; no real K600 unpinned replay run**.
This follows the [pinned replay](PINNED_REPLAY_AMENDMENT.md) and
[unpinned solver](UNPINNED_CONTROL_AMENDMENT.md) amendments. The frozen arm
order remains zero-trial preflight, pinned export/solve/replay, then unpinned
export/solve/replay, before any natural query.

The Rust `replay-unpinned` command requires the exact unpinned export and
`SAT_PENDING_XCNF_REPLAY` solver receipt, plus the completed pinned export,
solver and native replay receipts. The unpinned solver receipt must bind the
pinned replay-stage SHA-256. All receipts bind the same clean source, arm
lock, K600 input, zero-trial preflight, image and binary hashes. A pinned
replay failure receipt rejects admission. The command rechecks these inputs
after replay.

The native replayer evaluates the original unpinned CNF/XOR, six point lifts,
four intermediate group sums, final group equation and 600-column
signed-Frobenius relation row. The Docker invocation has the frozen 4 GiB
hard memory, no added swap, pinned one-CPU, 64-PID, no-network, read-only
input/root and outer wall caps. A passing result writes a new
`PASS_UNPINNED_MODEL_REPLAY` receipt with both the unpinned row and pinned
control digest. Failure after output creation writes a terminal failure
receipt. Total cold IC runtime and selected factor base remain null.

Local macOS arm64 Rust 1.99.1 release example tests passed **11/11**; the
production release example build exited 0. The
[test](unpinned-replay-test.log.gz) and [build](unpinned-replay-build.log.gz)
logs retain the exact output. Tests cover mode-specific Docker input paths,
reject an unpinned replay without pinned controls, and reject an unpinned
solver receipt with the wrong pinned replay digest. The real Linux Docker
path and complete positive result remain unverified.

This amendment made no S3 request, image build, preflight, XCNF export,
solver invocation or replay on the retained K600 panel. The proposed
separate 3,600-second charged K600 allowance and exact future astronomy-store
writer/destination remain pending. Natural search, useful rank, target
descent and full cold-runtime selection remain open. No performance chart
changed because this implementation step produced no K600 measurement.
