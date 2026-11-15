# Native K600 m6 arm-input staging component

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

The same example now also exposes `freeze STAGED_DIR EXPORTER_REPLAYER_BINARY
SOLVER_BINARY IMAGE_ID NEW_LOCK_JSON`. It writes a new external lock only
after binding a successful staging receipt to the current clean commit,
the exact committed `pilot-01` metadata root, the unchanged astronomy-store
index, all five panel metadata hashes, and the full 3,025,798-byte object
hash. It hashes the compiled supervisor, exporter/replayer, and solver bytes, checks an
exact Docker image ID, then rechecks the source, staging receipt, panel and
binary bytes before writing the lock. The lock records the frozen variable,
clause, conflict, model, CPU, memory, swap, PID and network caps; it explicitly
marks worker launch and both arms unexecuted. The executable has not yet run
`freeze` on a real staged panel. The image's architecture, compiled source
provenance, actual cgroup isolation, solver behavior, output receipt, and
model replay remain checks for the outer runner. A lock alone does not admit
the positive or natural arm.

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
