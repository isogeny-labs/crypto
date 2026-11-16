# Validation of the `main` merge, 2026-11-16 UTC

The merge joins PR head `d990aeeb4b0e46faac0f41a4e3ab6ec61814647b` and
`main` `d2f6c9e3f51cd0a7ea226df1d1a16c695225bd6f`. The one textual
conflict was in `examples/koblitz_n83_factor_base_export.rs`: the resolution
keeps the PR's v3 store commands and `main`'s Karatsuba command variants. A
merged `primary_adapter.rs` test call also needed the two new kernel flags
introduced by `main`. No frozen evidence file conflicted.

The following commands used Rust 1.99.1, `--locked --offline -j 1`, a separate
Cargo target and temporary directory under `/Volumes/SSD990/tmp/`, and a
2.5 GB boot-volume stop guard. The boot guard was not tripped in the completed
tests. The first exporter test attempt stopped for boot space before a verdict;
the second compiled far enough to expose the stale test call; after that
two-flag repair, the completed results were:

| Gate | Result |
| --- | --- |
| `cargo check --release --example koblitz_n83_factor_base_export --locked --offline -j 1` | Exit 0. |
| `cargo test --release --example koblitz_n83_factor_base_export --locked --offline -j 1` | Exit 0; 27 passed, 0 failed. |
| `cargo test --release --example n83_k600_m6_budget --locked --offline -j 1` | Exit 0; 6 passed, 0 failed. The macOS strip helper warned about missing `libLLVM.dylib`, but the test binary ran. |
| `cargo test --release --lib --locked --offline -j 1` | Exit 101; 6,413 passed, 17 failed, 135 ignored in 583.72 test seconds. This required gate is **red**. |

The full library run used the tracked GPU paths and the exact archived
targetwalk fixture that were absent from the earlier sparse run. Its 17
failures comprise one `ChainedS3` test assertion (`capped.trials` was 16,
expected 0), twelve report/restore tests that require Linux `/proc`, two
wide-prime tests that reached `Deadline`, and two wide-prime tests reporting
`No such file or directory (os error 2)`. The cause of the final two missing
files has not been isolated. `git diff --name-only origin/main -- src
Cargo.toml Cargo.lock tests gpu tools` returned zero paths on the merged
tree; this PR's resolution changed an exporter and a test call in
`research/`, not the library or its tests. These failures are recorded as
inherited or host-dependent observations, not counted as passed validation.
No test was disabled or skipped to obtain a green result.

The raw full-library log has 620,688 bytes and SHA-256
`daa725e16436ccb3f4ddb244524644c1c42a7a9c5ea45833efb58053eda6cbde`.
Its deterministic `gzip -n` copy is named `merge-lib-attempt1.log.gz`:
119,445 bytes, SHA-256
`8a15039f6fbf582e6d515b937461c099ffb6a6e67946342e1f187b58ef9d46a0`.
Artifact migration commit `d948ea32fe88` replaced the Git blob with the
[manifest](.artifacts.json). A fresh read-only GetObject from
`s3://setec-astronomy-artifacts/store/sha256/8a/8a15039f6fbf582e6d515b937461c099ffb6a6e67946342e1f187b58ef9d46a0`
returned 119,445 bytes with the expected compressed SHA-256; decompression
reproduced the raw SHA-256. This read-back proves log custody, not a passing
test result or authorization for a new factor-base publication.

No K600 arm, relation search, rank calculation, or factor-base S3 write was
performed by this merge validation. A full-fixture Linux run of the exact
merged head and green applicable CI remain necessary before merging the PR.
