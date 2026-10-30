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
| `cargo test --release --lib --locked` | still running at 06:47 UTC; 5,446 parent tests passed, 123 ignored by existing annotations, one pre-existing prime-field isogeny-walk test unfinished; no failures observed |
| Final-source `cargo test --release --lib --locked cached_` | recompilation in progress at 06:47 UTC |
| Scientific checkpoint / EKS comparison | blocked; checkpoint-upload.txt and [access record](../../../problems/native-koblitz-measurement-access.md) |

The full-suite executable was built before a lint-only change from unsigned
`% 2` parity checks to `& 1` at two sites. The final-source filtered run recompiles
that change; Clippy has already checked the final source. The full suite is not
reported as passed until its parent process exits successfully. Two nested
child test summaries in its raw log are not the full-suite result.

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
is explicitly a snapshot if its process is still active.
