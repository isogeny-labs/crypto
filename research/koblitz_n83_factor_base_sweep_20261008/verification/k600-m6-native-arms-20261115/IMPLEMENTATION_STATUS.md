# Native K600 m6 arm-input staging component

The [larger-memory AWS Spot host proposal](AWS_ARM64_SPOT_BUILD_SMOKE_PROTOCOL.md)
was [rejected before allocation](AWS_ARM64_SPOT_LAUNCH_REJECTION.md):
AWS reports that the selected account is blocked. Exact-tag
instance and Spot-request readbacks were empty. No cloud build ran.

The [frozen Linux ARM64 build smoke](LINUX_ARM64_BUILD_SMOKE_PROTOCOL.md)
was executed once at source `3ff510d8a280`. Its
[attempt-1 report and visual](LINUX_ARM64_BUILD_SMOKE_ATTEMPT1.md)
record a 6 GiB Docker OOM while compiling the `crypto` library with
two Cargo jobs; all five requested executables remain absent. This is
a failed engineering build control, not a K600 relation or runtime result.
The separately frozen [single-job retry](LINUX_ARM64_BUILD_SMOKE_ATTEMPT2_PROTOCOL.md)
reused the partial target and also reached a
[recorded Docker OOM](LINUX_ARM64_BUILD_SMOKE_ATTEMPT2.md) while compiling
the same library. A larger-memory Linux arm64 host is required to test
the complete build path under a fresh target; these two runs do not
establish its exact minimum RAM.

The [Linux build producer amendment](LINUX_BUILD_PRODUCER_AMENDMENT.md)
adds a source-bound native build receipt for the existing `charge build`
precharge phase. Its two focused release tests pass. A frozen host-specific
plan, actual Linux arm64 build, image inspection, full-invocation wall receipt,
and subsequent K600 controls remain unexecuted. It reports no new performance
value.

The [runtime image-label correction](RUNTIME_IMAGE_LABEL_CORRECTION.md)
aligns export, solver and replay inspection with the two-revision build
commit. Its release tests pass; a real Linux K600 path remains unrun.

The [native precharge child-interval amendment](PRECHARGE_CHILD_INTERVAL_AMENDMENT.md)
adds bounded wrappers and a four-phase receipt chain. Its tests are
implementation controls; the full-invocation cold runtime and real K600
host preflight remain open.

The [outer arm budget controller amendment](ARM_BUDGET_CONTROLLER_AMENDMENT.md)
records the native nine-phase ordering and its current accounting limit.
It is implementation evidence only; the K600 execution and full cold budget
receipt remain pending.

The [preflight lock/arm commit-cycle problem](../../../../problems/n83-k600-preflight-lock-commit-cycle.md)
records a source-identity admission conflict found in the unexecuted native
path. A versioned two-revision or detached-ref lock contract and negative
controls are required before the Linux zero-trial and arm gates can run.

The [two-revision source amendment](TWO_REVISION_SOURCE_AMENDMENT.md) now
implements the strict `C -> H` one-lock tree-diff contract with native
negative controls. This resolves the static commit cycle in the unit-tested
admission predicate; a qualified Linux preflight and arm freeze have not
yet tested the complete execution path.

**Status: implemented and locally tested, not run against S3 for this arm pair.**
The Rust `examples/n83_k600_m6_native_arms.rs` `stage` command bridges the
already verified read-only astronomy-store K600 object to the original
`pilot-01` panel layout consumed by `primary-k600-m6-xcnf` and
`primary-k600-m6-replay`. It checks the five exact legacy metadata and public
fixture files by SHA-256 and length, selects the unique primary `a=0`,
K600, `public_x_hash`, seed `2026100801` row, checks the configured store
remote and content-addressed index, and requests the object through native
`Store::ensure` into an empty cache. The staged 3,025,798-byte object is
rehash-checked at the legacy object path; source metadata, index, and clean
source commit are rechecked after staging. The output is write-once and sits
outside the source checkout. An error after output creation writes a terminal
`failure.json`. The command makes no S3 PUT and does not execute a solver.

The initial example exposed `freeze STAGED_DIR EXPORTER_REPLAYER_BINARY
SOLVER_BINARY IMAGE_ID NEW_LOCK_JSON`; the current command adds
`COMMITTED_PREFLIGHT_LOCK` before `NEW_LOCK_JSON` as specified in the
[two-revision amendment](TWO_REVISION_SOURCE_AMENDMENT.md). It writes a new external lock only
after binding a successful staging receipt to the current clean commit,
the exact committed `pilot-01` metadata root, the unchanged astronomy-store
index, all five panel metadata hashes, and the full 3,025,798-byte object
hash. It hashes the compiled supervisor, exporter/replayer, and solver bytes, checks an
exact Docker image ID, then rechecks the source, staging receipt, panel and
binary bytes before writing the lock. The lock records the frozen variable,
clause, conflict, model, CPU, memory, swap, PID and network caps; it explicitly
marks solver launch and both arms unexecuted. The executable has not yet run
`freeze` on a real staged panel. The current `freeze` source requires Linux
arm64 ELF exporter and solver bytes plus an image ID whose inspected OS,
architecture, and source-revision label match the clean commit in that
initial version; the amendment binds the label to its direct-parent build
commit. A build
receipt that ties those bytes to that source, actual cgroup isolation, solver
behavior, output receipt, and model replay remain to be checked. A lock alone
does not admit the positive or natural arm.

The release example tests passed **2/2** on local macOS arm64 Rust 1.99.1,
and the production release example build exited 0. The same invocation also
passed the existing exporter **27/27** and native preflight **6/6** suites.
The two staging tests confirm that committed `pilot-01` metadata match all
frozen hashes and the exact K600 row; a changed manifest byte is rejected.
The [compressed raw test log](example-test.log.gz)
retains the result. The actual S3-backed `stage` command was **not** run, so
its fresh-fetch and terminal-receipt paths remain unverified. It has not
exported a K600 XCNF, solved a constructed or natural arm, or replayed a
model. The frozen 3,600-second proposed build/preflight/arm allowance remains
unapproved and unused. A source-locked Linux image, completed native outer
arm runner, same-circuit positive control, then at most one public natural
query remain separate admission gates. No relation yield or total runtime is
measured by these tests.

The later `freeze` source addition passed release example tests **3/3** and
the production release example build with Rust 1.99.1 on local macOS arm64.
Its added negative test gives a syntactically successful staging receipt a
changed compressed object and confirms the input check rejects it before
consulting the S3 index. The separate [test](arm-freeze-test.log.gz) and
[build](arm-freeze-build.log.gz) logs retain the compiler output. No real
K600 `freeze`, image inspection, constructed XCNF export, solver run, or
natural query was performed in this step.

The production CLI's no-search negative control exited 1 with
`source checkout is not clean` while this source change was uncommitted, and
created no lock; its [stderr](arm-freeze-dirty-source.log.gz) is retained. This
checks the source gate only. A separate existing-output negative control
exited 1, retained the existing directory, and wrote no `failure.json` into
it; its [stderr](arm-existing-stage.log.gz) is retained. The default Homebrew
`rustc 1.93.1` could not compile the current AWS dependencies (minimum
1.94.1), so the final commands
selected installed `rustc 1.99.1` explicitly through `RUSTC`. The final
release tests and build then exited 0. No canonical runtime or factor-base
graph changed because this step produced no relation or timing observation.

The added `export ARM_LOCK_JSON PREFLIGHT_LOCK_JSON PREFLIGHT_RUN_DIR
positive-pinned|positive-unpinned NEW_OUTPUT_DIR OUTER_SECONDS` command now
prepares a source-bound XCNF *export stage* in a
Linux arm64 Docker image. It mounts the clean checkout, exact staged panel,
and exporter binary read-only at `/src`, `/base`, and `/exporter`, and mounts
the worktree's Git common directory read-only at its original absolute path
so the `.git` worktree pointer can resolve. `GIT_OPTIONAL_LOCKS=0` avoids
index writes; only a new output directory is writable. The Docker invocation specifies a 4 GiB hard
memory limit, no additional swap, one CPU and pinned CPU 0, 64 PIDs, no
network, no added capabilities, no new privileges, read-only root, and a
64 MiB scratch tmpfs. The explicit outer cap must be positive and at most
3,600 seconds; it bounds the Docker child, and the receipt separately records
the entire export-stage wall interval. The later cumulative controller must
pass only the remaining approved allowance. Before creating output, the
command checks a successful zero-trial native preflight lock, outer receipt,
worker config and summary against the same source commit, image, K600 object,
point set, policy, seed and frozen caps; it rechecks their hashes after export.
The guard kills a timed-out
container, waits for it, and checks removal; terminal wall-cap, resource-exit,
cleanup-failure, and producer-failure receipts remain distinct. Successful
exports must match the frozen K600 source/base/mode, all output bytes and
hashes, a clean-checkout attestation, per-representative circuit shape, and
the 125,000-variable and 200 MB output caps. It rechecks the source, lock,
panel, binaries, and image after the process before writing a success receipt.
The image must be built with its clean source at `/src` for the exporter’s
compiled source attestation to pass; this has not yet been demonstrated.

The Docker export source passed release example tests **6/6** and the
production release build on local macOS arm64 Rust 1.99.1; the new tests check
the hard resource and read-only Docker arguments and reject natural export
before a constructed solver control. A synthetic successful zero-trial
preflight bundle passes the receipt gate; a changed worker summary is rejected.
The [test log](docker-export-test.log.gz)
and [build log](docker-export-build.log.gz) retain compiler output. No Docker
image or real staged panel was available for an export, so the launch,
timeout cleanup, in-container source attestation, and XCNF receipt checks are
implemented but unverified. The 3,600-second proposed charged envelope is
still unapproved and unused. This export step alone does not enforce the
cumulative build/staging/preflight/arm budget. A source-matched image/build receipt, zero-trial
preflight, constructed solver control, natural solver arm, and native model
replay remain open. This command does not run a solver or change the
total-runtime selection. Natural export remains unavailable through this
command until a completed constructed solver control can gate it.

The follow-on preflight gate checks the exact `a=0`, fixture-0,
`artifact_store_v3`, `orbit-chain-s3`, K600/m6, hash-policy/seed selection,
chain limits, 4 GiB cgroup limit, zero swap, no trial/relation/matrix work,
and null total runtime in the worker config and summary. It also checks the
outer status, child exit, absence of cap/failure records, and preflight lock
and worker receipt hashes. The positive synthetic receipt test passes, while
a fixture change is rejected **even after** the test recomputes the outer
summary hash. Release example tests pass **6/6** and the production release
build exits 0 on local macOS arm64 Rust 1.99.1; the new
[test](preflight-gate-test.log.gz) and [build](preflight-gate-build.log.gz)
logs are retained separately from the prior version. A real preflight and
Docker export remain unrun, and this receipt check is not evidence of a
completed constructed control.

The [solver-stage amendment](SOLVER_STAGE_AMENDMENT.md) freezes a separate
CryptoMiniSat XCNF backend and the pinned-control, unpinned-control, natural
order before any such run. The Rust `solve` command currently admits only
`positive-pinned`: it rechecks the arm lock, read-only K600 panel, exact
preflight, constructed XCNF/export hashes and Linux arm64 solver/image before
launch. Its Docker arguments enforce the same hard resource isolation,
one-thread solver, seed `20261115`, 10,000 conflicts, complete model output,
and an internal wall limit five seconds below its outer limit. It captures
stdout/stderr and emits a terminal solver receipt. Exit 10 is explicitly
`SAT_PENDING_XCNF_REPLAY`; no relation is accepted until native replay.
Unpinned solver and natural export remain gated. The result and failure
receipts retain the full stage interval, exit/cap class and input hashes, and
leave total IC runtime null.

The solver-stage release example passed **8/8** and the production release
example build exited 0 on local macOS arm64 Rust 1.99.1. The added tests
check hard Docker and CryptoMiniSat arguments and reject an unpinned solve
before pinned replay. The [test](solver-stage-test.log.gz) and
[build](solver-stage-build.log.gz) logs are retained. This code has not run
the solver, a real preflight or any K600 XCNF. The image/build receipt,
cumulative approved-budget controller, pinned model replay, unpinned and
natural arms, useful rank and full cold runtime selection remain open.

The [pinned replay amendment](PINNED_REPLAY_AMENDMENT.md) adds a source,
input, solver-output and image-bound `replay-pinned` command. Its isolated
native replayer checks the original XCNF, six point lifts, group equation and
signed-Frobenius relation row before a terminal passing stage receipt can be
written. The release example passed **11/11** tests and its production build
exited 0; [test](pinned-replay-test.log.gz) and
[build](pinned-replay-build.log.gz) logs are retained. This component has not
run on the real K600 input or a Docker host. The image/build receipt,
cumulative approved-budget controller, real pinned positive control,
unpinned and natural arms, useful rank and full cold runtime selection remain
open. No performance graph changed because this step produced no K600
measurement.

The [unpinned control amendment](UNPINNED_CONTROL_AMENDMENT.md) adds a
`solve-unpinned` command gated on the exact pinned export, solver and native
replay receipts. Its solver receipt binds the pinned replay-stage SHA-256,
and the command rechecks the control and unpinned XCNF after the process.
The release example remains **11/11** and its production build exits 0;
[test](unpinned-control-test.log.gz) and
[build](unpinned-control-build.log.gz) logs preserve results. This is an
implemented admission gate, not a real unpinned solve or model replay.
Natural export/query, the qualified image and cumulative budget controller,
rank/logs and full cold-runtime selection remain open. No performance graph
changed because no experiment was run in this code step.

The [unpinned replay amendment](UNPINNED_REPLAY_AMENDMENT.md) adds a
`replay-unpinned` command. It requires the exact unpinned SAT receipt bound
to the passing pinned replay, rechecks both controls after isolated native
XCNF/group/row replay, and emits a distinct passing or terminal failure
receipt. The release example remains **11/11** and the production release
build exits 0; [test](unpinned-replay-test.log.gz) and
[build](unpinned-replay-build.log.gz) logs preserve results. The real K600
path is still unrun. A source-matched image/build receipt, cumulative budget
controller, natural arm, rank/logs and full cold-runtime selection remain
open. No graph changed because this revision contains no K600 measurement.

The [natural export amendment](NATURAL_EXPORT_AMENDMENT.md) adds an
`export-natural` command after both terminal passing constructed-control
replays. It checks that pinned and unpinned exports used the same constructed
target, binds the unpinned replay digest into the natural export receipt,
and rechecks control/input hashes after the bounded exporter process. The
release example remains **11/11** and the production release build exits 0;
[test](natural-export-test.log.gz) and [build](natural-export-build.log.gz)
logs preserve the result. No real natural XCNF or solver stage has run.
The image/build receipt, cumulative budget controller, natural solver and
replay, useful rank and full cold-runtime selection remain open. No graph
changed because this revision contains no K600 measurement.

The [natural solver amendment](NATURAL_SOLVER_AMENDMENT.md) adds
`solve-natural`, requiring the natural XCNF receipt and both completed
constructed-control replays before one bounded solver invocation. Its
receipt distinguishes `SAT_PENDING_XCNF_REPLAY`, `UNSAT_NATURAL_QUERY`,
caps and failures; it binds the unpinned replay digest and leaves total IC
runtime null. The release example passed **12/12** tests and the production
release build exited 0; [test](natural-solver-test.log.gz) and
[build](natural-solver-build.log.gz) logs preserve results. No natural
solver ran. The source-matched image/build receipt, cumulative budget
controller, natural model replay, rank/logs and full cold-runtime selection
remain open. No graph changed because this revision contains no K600
measurement.

The [natural model replay amendment](NATURAL_REPLAY_AMENDMENT.md) adds
`replay-natural` after a SAT natural solver receipt, original XCNF export,
and both constructed-control replays. It records a distinct passing row or
terminal failure, while leaving useful rank and total IC runtime unset.
The release example passed **13/13** tests and the production release build
exited 0; [test](natural-replay-test.log.gz) and
[build](natural-replay-build.log.gz) logs preserve results. No K600 natural
model was replayed. A source-matched image/build receipt, cumulative budget
controller, real arm run, rank/logs and full cold-runtime selection remain
open. No graph changed because this revision contains no K600 measurement.

Validation commands from the repository root (stdout and stderr preserved
in the linked logs):

```sh
RUSTC="$(rustup which rustc --toolchain 1.99.1)" cargo test --release --example n83_k600_m6_native_arms
RUSTC="$(rustup which rustc --toolchain 1.99.1)" cargo build --release --example n83_k600_m6_native_arms
```

The last required root release-library run on this branch's unchanged root
library and Cargo sources remains [failed](../k600-m6-native-supervisor-20261115/ROOT_VALIDATION_20261115.md):
6,305 passed, 60 failed, 138 ignored; the extra deadline test passed when
rerun alone. This component changes an example and study files only.
