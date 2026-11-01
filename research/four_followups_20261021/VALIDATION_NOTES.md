# Verification log for the four-follow-up update

The study crate passed `cargo fmt --all -- --check` and
`cargo check --release --locked --bins` from this study directory. Its Rust
producers and validators independently replayed 6,300 factor-base target
pairs, 540 complete-curve pencil target pairs, the separate 28-bit pencil
target pair, 324 nine-curve pricing cells and 36 single-target pricing cells
as described in [REPORT.md](REPORT.md). The factor-base archive-input run
files matched byte-for-byte; the pencil reruns matched every non-wall field.
`sha256sum -c SHA256SUMS` and
`python3 scripts/validation_gate.py evidence benchmarks/20261021-four-followups`
both passed, the latter returning `valid`.

The first root-library serial command was
`cargo test --release --lib -- --test-threads=1`. It ended with 4,467 passes,
22 failures and 119 ignored. All 22 failure traces report `os error 28` or
`No space left on device` while creating temporary files; `/tmp` was a
62 GiB tmpfs with about 61 GiB in use, while the root disk had 66 GiB free.
The complete output is retained as
[release-lib-initial-enospc.log.gz](results/release-lib-initial-enospc.log.gz).
This was an environmental failure, not a passing suite.

The rerun used an explicit task-specific directory on the root disk:
`mkdir -p /var/tmp/four-followups-test-tmp` followed by
`TMPDIR=/var/tmp/four-followups-test-tmp cargo test --release --lib -- --test-threads=1`.
It completed in 694.97 seconds with **4,489 passed, 0 failed and 119
ignored**. The complete output is
[release-lib-root-tmp-pass.log.gz](results/release-lib-root-tmp-pass.log.gz).
The root library source was unchanged by this PR update; the standalone
study crate carried the new Rust implementation and passed its own release
checks above. The parallel root-suite attempt had one unrelated F4 test
failure; that test passed alone, and the full serial rerun passed.
