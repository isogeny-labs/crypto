# K600 m6 pinned positive-control replay gate

Status: **implemented and locally tested; no real K600 replay executed**.
This versioned amendment follows the [solver-stage amendment](SOLVER_STAGE_AMENDMENT.md).
The frozen sequence is zero-trial preflight, constructed pinned XCNF export,
single-thread bounded CryptoMiniSat solve, then native pinned-model replay.
Unpinned and natural arms remain gated on successful earlier controls.

The Rust `replay-pinned` command in
`examples/n83_k600_m6_native_arms.rs` requires the exact source commit,
read-only staged K600 panel, arm lock, preflight lock and receipts, pinned
export receipt/XCNF, and a `SAT_PENDING_XCNF_REPLAY` solver receipt. It
checks the solver executable and stdout/stderr hashes, exit code 10, frozen
seed/conflict/thread parameters, and Linux arm64 image ID before creating a
new output directory. The model stdout is capped at 20 MB. The replayer runs
in Docker with 4 GiB hard memory, no additional swap, one pinned CPU, 64
PIDs, no network, a read-only root and inputs, and only the new output
directory writable. A separate outer wall guard kills and removes a capped
container. The command rechecks source, image, panel, and input hashes after
the process.

The underlying `primary-k600-m6-replay` implementation checks the original
CNF and XOR rows, six public point lifts, the group equation, and the signed
Frobenius relation row. The outer command accepts only its exact
`PASS_ORIGINAL_XCNF_GROUP_AND_ROW_REPLAY` schema and frozen XCNF/model hashes,
then writes `PASS_PINNED_MODEL_REPLAY` with the checked row. A failure after
output creation writes a terminal failure receipt. Neither the model nor a
relation is accepted on an exit, wall cap, mismatch, or cleanup failure.
The replay receipt leaves total IC runtime and best-arm selection null.

Local macOS arm64 Rust 1.99.1 release validation: the example's **11/11**
tests passed and the production example build exited 0. Added tests check
Docker isolation arguments, reject replay before a pinned solver receipt,
and reject changed solver stdout despite an otherwise well-formed receipt.
The exact [test](pinned-replay-test.log.gz) and
[build](pinned-replay-build.log.gz) logs preserve compiler warnings and
results. These synthetic tests do not prove the Docker path works on the
frozen Linux host. The prior root library release run remains red on
unchanged root library sources (6,305 passed, 60 failed, 138 ignored).

No S3 staging, image build, real preflight, XCNF export, solver, or replay
was run in this amendment. The proposed 3,600-second charged K600 allowance
and exact astronomy-store writer/destination remain awaiting the owner's
answer. Therefore there is no measured relation yield, rank, or cold total
runtime to compare with the frozen IC baseline or matched rho. No canonical
performance chart changed because this amendment records implementation and
local correctness checks only.
