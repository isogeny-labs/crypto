# Current-head release-library rerun: incomplete terminal record

**Status: `INCOMPLETE_MISSING_EXIT_RECEIPT_WITH_DISK_PRESSURE`.** This is a
validation attempt on clean commit
`8e7a65eeaa71c17787f7f7e7ffef44555e7b8907`, not a library-suite pass,
K600 solver run, or IC runtime sample. The worktree was clean; the PR diff
from `origin/main` changes no `src/` file or root Cargo manifest/lockfile.

The exact [wrapper](root-lib-current-head-wrapper-20261115.sh) launched in a
detached tmux session at approximately 20:11 UTC used macOS arm64 Rust 1.99.1,
an independent `/private/tmp/n83-ci-20261115` target, and:

```sh
RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc \
CARGO_TARGET_DIR=/private/tmp/n83-ci-20261115 \
gtimeout 2400 cargo test --release --lib --locked --offline -j 2
```

The root test binary compiled and tests started. The
[356,346-byte raw log](root-lib-current-head-incomplete-20261115.log.gz)
has uncompressed SHA-256
`596c17625741f1fa7222264ec4022246f608aaa51defaceab71e3be93f8ef33a`
and gzip size 70,075 bytes, SHA-256
`bba0df98d47672fd9cf98a85f5ad312516e5fa7ed57152196ec4af6411f3647c`.
It ends with the bare bytes `error` after partial test output and has no
overall `test result` summary for the library suite. There were 39
individual `FAILED` lines before the stop: 38 names occur in the earlier
[complete sparse-fixture failure set](../k600-m6-native-supervisor-20261115/lib-test-current-failures.txt).
The additional line names
`cryptanalysis::koblitz_groebner::checkpoint::tests::exact_pivot_state_survives_binary_roundtrip_at_every_column`.
Its failure details were not emitted before the log ended. The sole
`test result: ok. 1 passed` line belongs to a separately spawned
`solver_checkpoint_child`, with 6,502 other tests filtered out; it is not
the library-suite verdict. The new checkpoint failure requires an isolated
replay on stable storage before attribution.

The tmux session had ended by 20:43:41 UTC. The wrapper used `status=$?`,
but `status` is a read-only zsh parameter; a direct `zsh -c 'status=7'`
check exits 1 with `read-only variable: status`. Thus the intended child
exit file was never written, and the child exit code cannot be recovered
from this wrapper. This receipt defect is independent of whether the cargo
child failed, was terminated, or lost its output stream. The boot volume
had about 13 GiB free before launch and only 115 MiB when the incomplete
log was inspected. That contemporaneous disk pressure is observed, but the
log does not prove it was the child termination cause [pcc:unverified].

The disposable target occupied 887 MiB at inspection. `cargo clean
--target-dir /private/tmp/n83-ci-20261115` then removed 2,030 files and
881.1 MiB; the raw log and compressed copy were preserved. Other concurrent
host storage activity continued, so free-space change after cleanup is not
an isolated measurement of this target's effect. No library test count or
completion ratio is promoted from this partial attempt.

**Next gate:** use a wrapper with a non-reserved exit variable and an
independent terminal receipt, first verify stable free storage, then rerun
the full library suite. If the new checkpoint failure recurs, inspect its
captured failure details and replay it alone on the same committed source.
The prior [explicit LLVM disk-link failure](ROOT_LIB_RESOURCE_FAILURE_20261115.md)
remains a separate attempt. Applicable GitHub CI and review must pass on
the exact final PR head before merge. No canonical IC graph changes because
this attempt measured no IC workload.
