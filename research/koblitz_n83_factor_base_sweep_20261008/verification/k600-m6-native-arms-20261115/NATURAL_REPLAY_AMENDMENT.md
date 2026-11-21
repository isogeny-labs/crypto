# K600 m6 natural SAT-model replay gate

Status: **implemented and locally tested; no real natural K600 model
replayed**. This follows the [natural solver](NATURAL_SOLVER_AMENDMENT.md)
and both constructed-control replay amendments. The frozen fixture-0 query,
full K600 base, m6 circuit, solver limits and arm order are unchanged.

The Rust `replay-natural` command requires a natural solver receipt with
`SAT_PENDING_XCNF_REPLAY`, exit 10 and complete model stdout, bound to the
exact natural XCNF export and passing pinned/unpinned replay controls. It
checks the source, K600 panel, preflight, Linux arm64 image, executable
hashes, original XCNF, solver stdout/stderr and unpinned replay digest before
creating output and again afterward. A natural UNSAT, cap or failure receipt
does not enter replay and remains a terminal or censored result of the
natural solver stage.

The same native replayer evaluates the original natural CNF/XOR, six public
point lifts, four intermediate group sums, final group equation and
600-column signed-Frobenius relation row. It also checks fixture-0 target
identity against the retained public corpus. Docker enforces 4 GiB hard
memory, no added swap, one pinned CPU, 64 PIDs, no network, read-only
root/inputs and an outer wall cap with verified cleanup. A passing model
writes `PASS_NATURAL_MODEL_REPLAY` with the checked row and unpinned control
digest. A failure after output creation writes a terminal failure receipt.
The receipt leaves useful rank, logarithms, cold IC runtime and best-factor-
base selection null; a single verified relation would not establish those.

Local macOS arm64 Rust 1.99.1 release example tests passed **13/13**, and
the production release example build exited 0. The
[test](natural-replay-test.log.gz) and [build](natural-replay-build.log.gz)
logs retain exact output. The new tests reject natural replay without its
controls before output creation, select the natural XCNF in Docker arguments,
and reject a natural solver receipt whose unpinned replay digest differs.
These tests do not establish a passing model on the frozen Linux host.

This step made no S3 request, image build, preflight, XCNF export, solver
call or model replay against the retained K600 input. The separate proposed
3,600-second charged K600 allowance and exact future astronomy-store writer
and destination remain pending. A source-matched image/build receipt,
cumulative budget controller, real arm run, useful rank, target descent and
full cold one-target runtime comparison remain open. No performance chart
changed because this implementation step produced no K600 measurement.
