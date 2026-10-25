# Publication continuation — 2026-10-19

The factory was transferred to current upstream main without modifying library
sources. The original implementation and its failed repository gate remain on
`codex/factor-base-factory-20261019` at
`6831216e0c797475780199843bc821b43fb53f68`.

The publication branch is `codex/factor-base-factory-main-20261019`, based on
upstream `d67bb0dac6e8edf907765f6527291508c9e94325`. The three transferred commits
are `e1b094c72`, `4c9ecb640`, and `a605b0e07`. The main branch already includes
registry-model and other library updates absent from the original checkout.

## Retained checks

- The earlier library executable's entire `pollard_collab` test module passed
  with current permissions: 37 passed, zero failed. This resolves the earlier
  local-socket permission failures for that executable. See
  `local/socket-tests-unrestricted-20261019.log`.
- The standalone package passed 17 tests, with the fixture-generation utility
  ignored. Clippy with warnings denied and formatting checks passed. See
  `local/package-main-tests-20261019.log`, `local/package-main-clippy-20261019.log`,
  and `local/package-main-build-20261019.log`.
- All six checked-in manifests regenerated successfully, and the saved binary
  relation receipt replayed. See `local/verify-main-*-20261019.json` and
  `local/replay-main-binary-20261019.json`. Replay construction source digest:
  `7e5e2bc488d06cbb7828393a8dc6429952db7a0b2cd1e5340cc1858b50937e13`.
- The new Linux/macOS workflow passed `actionlint` and the repository's workflow
  budget checker. It pins Rust 1.93.1, runs the standalone package checks and
  archive replay, and needs no AWS credentials. Hosted results are separate
  from these local checks.

## Repository gate

The initial main-branch invocation lacked the sparse path dependency
`tools/volcano-cli`; the next reached the library but lacked three compile-time
fixture inputs. Both failed outputs are retained in
`local/repository-main-lib-test-20261019.log` and
`local/repository-main-lib-test-materialized-20261019.log`.
The repository's `node .github/scripts/ci-scope.mjs --materialize` restored 117
compile-time inputs from the pinned Git tree; its output is
`local/materialize-ci-inputs-20261019.log`. Runtime fixture directories were
also materialized without changing their contents.

The final gate command is:

```sh
CARGO_BUILD_JOBS=2 \
CARGO_TARGET_DIR=/Volumes/SSD990/llm/tmp/factor-base-root-target \
cargo test --release --lib --offline
```

Its output is `local/repository-main-lib-test-complete-inputs-20261019.log`.
The command exited zero: **4,399 passed, zero failed, 118 ignored**. This clears
the local publication gate on the stated main-branch baseline. The original
failed gate remains historical evidence and must not be reported as a pass.
Compilation and test durations in the log are shared-host operational diagnostics,
not qualified performance measurements. No library source, test assertion,
timeout, or ignore marker was changed by this factory branch.

Hosted CI and review remain separate gates. The dedicated package workflow
was validated locally; its Linux/macOS execution status is available on the PR.

The S3 manifests and shards are unchanged. The additional source-bundle/catalog
S3 upload has not been retried. The original producer source snapshot remains
available in this package with its recorded archive digest.
