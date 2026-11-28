# Identity module test-closure qualification

Date: 2026-11-23. Parent task: crypto PR #5016.
Baseline: `7d61e3b45de6e93d235345a0061dae08db1f8b44`.

## Prestate and purpose

The parent found a concrete integration omission before publication. The
unchanged copied production `identity_certificate.rs` contains a `cfg(test)`
`include_str!("../../docs/identity-certificates/schoenhage-10mult.json")`.
In the standalone study this resolves to `docs/identity-certificates/` under
the study root, where the fixture was missing. The earlier issuer/checker
qualification compiled ordinary binaries, so it did not exercise that test
closure. Its records remain unchanged and do not claim this unit-test pass.

Add only the exact baseline fixture, Git blob
`8a17c8270bbb6c2ffee6c3cddf6ccd5c2093586e`, SHA-256
`84a61d6ccc237b724695e8f1416e9ad48f24c899ae316512bf0b952e46edb293`.
The production module, SHA-256 implementation, prior protocol, expression
witness, Cargo.toml, Cargo.lock and prior evidence must not be changed.
No test implementation or assertion is added or altered.

## Frozen qualification

Before compilation, record this protocol, the new metadata runner, the exact
fixture, all earlier compiled source pins and the prior dependency manifest
in a new source/build record. Verify the fixture directly against the blob
in the baseline Git object tree. Verify the original source/dependency
closure remains unchanged, including the task-private cached registry
archives, extracted sources and sparse-index metadata. Record compiler and
Cargo binary hashes and versions. Refuse Cargo source or compiler overrides.

Use the existing task-private `identity/cargo-home/` and `identity/target/`,
with explicit per-process compiler paths and offline dependency resolution.
No other agent cache, root Cargo manifest or production source is modified.

1. Build the standalone crate's existing tests with `cargo test --release
   --manifest-path identity/Cargo.toml --frozen --offline --locked --no-run
   --message-format=json`, with a 180-second process-group timeout. Capture
   stdout and stderr separately. Identify all test executables from Cargo's
   compiler-artifact records, hash them and retain frozen byte copies.
2. Before running tests, write a separate pre-call record with the Cargo
   command, all actual test-binary hashes, source pins and dependency-closure
   status. Run `cargo test --release --manifest-path identity/Cargo.toml
   --frozen --offline --locked -- --test-threads=1`, with a 60-second
   process-group timeout. Capture raw stdout and stderr.
3. After the test call, record the exit status, timeout status, output hashes,
   actual and frozen test-binary hashes, source/dependency closure status,
   and unchanged earlier certificate bytes. A successful result requires
   exit zero, no timeout, and unchanged inputs and binaries. Report the
   observed test counts directly from Cargo's raw output.

The expected scope is the existing production identity-certificate and
SHA-256 unit tests, plus any default binary/doc test targets selected by
Cargo. This is a new qualification of the completed test closure. It is not
a historical unit-test result, formal identity proof, independent replay,
performance measurement, or a rerun of compatibility tests.

Every new record, raw output and frozen test binary is created once in this
attempt. Keep failures, diagnose their concrete cause, and do not overwrite
the earlier identity evidence or rerun unchanged commands to obtain a pass.
