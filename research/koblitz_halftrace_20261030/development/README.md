# Development checks, not benchmark results

All commands run at the repository root with Rust 1.99.1. Neither compilation
nor test elapsed times establish native speedup. No scientific run was launched.

| Check | Result |
| --- | --- |
| Unmodified release library and ecbench builds | passed before source edits; see ../BASELINE.txt |
| `cargo test --release --example koblitz_halftrace --locked` | 2 passed; six two/three-summand cases per path plus the public ecbench pipeline |
| `cargo clippy --release --locked --lib --no-deps -- -D warnings` | passed on final Rust source |
| Touched-file `rustfmt --check --edition 2021` | passed |
| `git diff --check` | passed |
| `ecbench plan --spec research/koblitz_halftrace_20261030/spec.json` | validated 8 workloads, 5 arms, 280 executions; no execution requested |
| `cargo test --release --lib --locked` | completed: 5,447 parent tests passed, zero failed, 123 existing ignored; full-library-result.txt |
| Final-source `cargo test --release --lib --locked cached_` | passed: 10 tests including cache, scope and factor-base parity |
| Final-source `cargo test --release --lib --locked fast_base_lifts_preserve_roots_and_order` | passed: 1 test |
| Scientific checkpoint / EKS comparison | blocked; checkpoint-upload.txt and [access record](../../../problems/native-koblitz-measurement-access.md) |

The full-suite executable was built before a lint-only change from unsigned
`% 2` parity checks to `& 1` at two sites. Both final-source filtered runs pass
after recompiling that change; Clippy also checks the final source. The final
parent summary is now retained in full-library-result.txt. Two nested child test
summaries in its raw log were not treated as the full-suite result.

The later merge with main preserves the feature files. Its repeated locked
check first found inherited Cargo.lock dependency-reference damage, normalized by
Cargo, then an unclosed-delimiter error in ecbench/methods.rs that is byte-identical
to the fetched main tip. Those failures are retained in inherited-lock-failure.txt
and current-main-build-failure.txt. The original results are not relabelled as a
pass of this later source. Scientific preparation pins the tested feature source
specified in the prospective protocol.

Earlier development attempts are retained, including sparse-checkout omissions
and the first Clippy failure. Tracked fixture files were restored from the
baseline commit; no tests were disabled or rewritten. Full-tree cargo fmt finds
pre-existing differences outside the touched files. No unrelated formatting
changes are included.

`checks.tar.xz` retains the raw development logs locally. At 88 KiB it exceeds
the artifact-store policy's 64 KiB data threshold, so it is ignored and not
committed. S3 upload is pending the same missing access; no remote manifest is
invented. `LOCAL_ARCHIVE.sha256` identifies this local snapshot. Short complete
check logs are retained as loose files; `SHA256SUMS` identifies the final source
and committed check logs. This is development evidence, not an admitted ecbench
run, replay certificate or performance ledger row. The full-suite archive log
is explicitly a snapshot if its process is still active. Readable short
transcripts normalize trailing blank lines; exact raw bytes stay in the archive.
