# Rust extension and hardening validation

This record accompanies PR #2101's Rust extension/hardening update on parent
`459eceffd8d356e0f4d9feaea1d0e28936f1df3c` (main at `c6d64436`). It records
correctness and integration checks, not timing, performance or cryptanalytic
experiments. No research campaign or paid worker was launched.

The environment is Linux 6.18.55 x86_64, with `CARGO_BUILD_JOBS=2` and the
repository's locked dependency graph. Toolchains:

- Rust 1.98.0 (`88d9e12ae`, 2026-08-18), matching CI.
- Rust 1.99.1 (`58cf8f9c3`, 2026-10-13), installed stable.

The final code is identified by `source-files.sha256` (run
`sha256sum -c docs/validation/rust-extensions-20261017/source-files.sha256`
from the repository root). `source-patch.sha256` also records the binary diff
against parent `459eceff` for `src`, `tests` and `examples`. No source changes
are permitted between final validation and commit without rerunning the affected
checks. The one late binary-only change restored ecbench's missing FBM import;
it does not change the library test executable's source graph.

## Final commands and results

```sh
cargo +1.98.0 test --locked --release --lib
cargo +1.98.0 test --locked --release --bin ic --test ic_framework --test ic_progress --test rust_extension_security --example ic_custom_pipeline
cargo +1.98.0 check --locked --bins
RUSTUP_TOOLCHAIN=1.98.0 bash scripts/clippy-maintained.sh
RUSTUP_TOOLCHAIN=stable bash scripts/clippy-maintained.sh
cargo +1.98.0 clippy --locked --lib --bin ic --test ic_framework --test ic_progress --test rust_extension_security --example ic_custom_pipeline -- -D warnings
cargo +stable clippy --locked --lib --bin ic --test ic_framework --test ic_progress --test rust_extension_security --example ic_custom_pipeline -- -D warnings
cargo +1.98.0 test --locked --doc utils::random::try_random_bytes_with_rng
cargo +1.98.0 test --offline --manifest-path crates/crypto-analysis-core/Cargo.toml --lib
```

The final release commands run with network permission so unit tests can bind
synthetic loopback peers. No tests or verification gates are disabled.

| Final gate | Observed result |
| --- | --- |
| Release library | **4,387 passed, 4 failed, 117 ignored**; exit 101. Four unchanged research failures remain. |
| Selected release consumers | **49 passed, 0 failed**; exit 0: 10 IC binary, 25 CLI, 5 progress, 7 extension/security, 2 embedding-example tests. |
| All binaries type-check | Pass, exit 0. |
| Changed-target Clippy, Rust 1.98 / 1.99 | Both pass with `-D warnings`, exit 0. |
| Full maintained-target Clippy, Rust 1.98 | Exit 101: unchanged volcano driver `drop_non_drop` and N83 example `manual_is_multiple_of`. |
| Full maintained-target Clippy, Rust 1.99 | Exit 101: unchanged N83 example `manual_is_multiple_of`. |
| CI-pinned changed-file rustfmt | Pass, exit 0. |
| Existing analysis-core crate | 10 passed, 0 failed, exit 0; source unchanged. |
| CSPRNG compile-fail doctest | 1 passed; separate E0277 diagnostic verifies the intended rejection reason. |

`cargo +1.98.0 run --locked --release --example ic_custom_pipeline` also passes:
`verified=true, matrix=application-matrix, rank=25, submitted_rows=58`.

Changed-target strict Clippy on both toolchains, the all-binary type check,
CI-pinned formatting, the existing lightweight analysis crate's 10 tests,
and the CSPRNG compile-fail doctest pass. The full maintained-target script
reports unrelated `drop_non_drop` in the volcano driver (Rust 1.98) and
`manual_is_multiple_of` in the N83 example (both toolchains); see the retained
full lint logs. The changed-target passes do not establish green whole-tree
gates. `csprng-bound-diagnostic.log.gz` independently confirms
that the rejection is specifically `SmallRng: CryptoRng` not being satisfied
(E0277), using the included `reject_non_csprng.rs` diagnostic source.

The exact shared GF module was also checked directly before the full build:

```sh
rustc +1.98.0 --edition=2021 --test src/symmetric/gf128.rs -L dependency=target/debug/deps --extern subtle=target/debug/deps/libsubtle-63aa32c0c164fbf9.rlib --extern hex=target/debug/deps/libhex-522fe46b32976465.rlib -o /tmp/rust-gf128-tests
/tmp/rust-gf128-tests
```

Both tests pass: NIST SP 800-38B subkeys and all polynomial basis elements /
endian conventions. The dependency filenames above are this build's artifacts;
the portable equivalent is `cargo +1.98.0 test --locked --release --lib symmetric::gf128`.
Arithmetic equality tests and compile-time trait checks are not timing proofs.

## Retained earlier failures and controls

| Artifact | Observed outcome and scope |
| --- | --- |
| `baseline-library-check.log.gz` | Initial main/trace-head library build fails with 25 existing quartic/ecbench integration errors. |
| `initial-interface-check.log.gz` | An implementation iteration imported the canonical matrix trait under a conflicting legacy name; fixed with the `RelationMatrix` alias. Not a baseline failure. |
| `initial-ic-lint.log.gz` | After library repair, the existing IC `prime` references lacked a module declaration; restored. |
| `initial-all-bins-check.log.gz` | The ecbench executable also lacked its FBM import; restored. |
| `prefreeze-release-library.log.gz` | Before the final shared-field/runtime-provider refactor: 4,376 passed, 13 failed, 117 ignored; exit 101. This is a diagnostic iteration, not validation of final source. |
| `local-network-denied.log.gz` | Isolated localhost test fails with `PermissionDenied: Operation not permitted` in the default sandbox. |
| `local-network-control.log.gz` | With network permission, all 38 unchanged collaboration tests pass; no failures. This distinguishes the nine socket failures from code failures. |
| `curve-traits-replay.log.gz` | A committed traits record lacks required `levels`, reproducing the three unchanged query-test failures. |
| `jacobian-replay.log.gz` | Unchanged `c2pnb304w1` route classification returns `Boundary`, expecting `FormulaMarginal`. |
| `prefreeze-consumer-incomplete.log.gz` | An earlier consumer build was stopped while the implementation evolved; exit 130. No test pass or result is claimed. |

A queued release rerun without network permission was cancelled before it
built or ran (exit 130), then replaced with the complete network-enabled gate.
Earlier baseline failures were not hidden by fixture edits, assertion changes,
ignored tests or weakened schemas. See
[the research regression record](../../../problems/current-main-research-regressions.md).

Conductor checks were attempted before editing and scope expansion; the
`conductor` executable and `.conductor/WORKFLOW.md` are unavailable in this
runtime/checkout. No coordination hook or validation check was bypassed.

Logs use reproducible gzip packaging (`gzip -n -9`), preserving the original
bytes and blank lines. Read one with `gzip -dc <name>.log.gz`.
`SHA256SUMS` covers the compressed logs and diagnostic inputs.
These checks do not establish whole-crate side-channel resistance; that remains
tracked in [the timing coverage record](../../../problems/rust-secret-api-timing-coverage.md).
