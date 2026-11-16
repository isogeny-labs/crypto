# Native Rust API qualifier

This small test crate links the actual root `crypto_lib` library and the
shared algebra crate. It tests the public F4 polynomial adapter and calls
the root F4 engine directly as a basis control. It contains no copied F4
implementation and no replacement numerical solver.

The separate manifest keeps Cargo from building all unrelated command-line
binaries while qualifying this library API. The direct root-library unit-test
build and direct root integration-target build are preserved separately in
`../integration-development/`; missing sparse-checkout historical fixtures and
an unrelated binary source blocked those initial build targets. This target
is not a pass of the entire root test suite.

Build with `cargo test --locked --manifest-path Cargo.toml --no-run`.
For a recorded qualification, copy the emitted `native_api` executable, hash
that immutable copy before calls, then run it with `--test-threads=1 --nocapture`.
Keep separate build, pre-call and post-call manifests. The tests perform
development correctness checks; their elapsed times are not benchmark results.

F4 control comparison is restricted to its documented small-prime range.
The 53-bit case tests exact coefficients, certificates and native normal-form
checks, without claiming general wide-field F4 support. A six-variable case
reports its checked success or explicit proof-budget limit, without replacing
inputs. No performance conclusion is drawn here `[pcc:unverified]`.

## Actual-root comparison worker

`src/bin/native-root-worker.rs` calls the actual F4/certified APIs on the same
bounded System JSON. Three separate CLI tests and immutable worker/test copies
are recorded in ../root-worker-development/. Read ../ROOT_WORKER_CONTRACT.md
for paid scope, incomplete/unsupported handling and the still-pending runtime
gate. Worker clocks are diagnostics, not speed ratios.
