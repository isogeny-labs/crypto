# Development checks, not benchmark results

All commands run at the repository root with Rust 1.99.1. Neither compilation
nor test elapsed times establish native speedup. No scientific run was launched.
Unless a later attempt is explicitly named, passing checks below refer to the
original tested feature source. They do not validate subsequent main merges.

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

At the 16:41 UTC check-in, upstream main53f4b531f had repaired that methods parser
error and the lockfile references. Merged it normally, preserving both additive
documentation blocks. The recheck at source4790f74f1 then failed before running
tests on an unclosed delimiter in binary_semaev_chain_sat.rs, byte-identical to
upstream. No unrelated source repair or test suppression was applied. The ordered
and E2E rechecks did not run after that compile failure. See the new immutable
[receipt](recheck-20261030T1641Z/receipt.json), exact gzip compiler log and source
hashes; older failure logs and passing results remain unchanged.

Earlier development attempts are retained, including sparse-checkout omissions
and the first Clippy failure. Tracked fixture files were restored from the
baseline commit; no tests were disabled or rewritten. Full-tree cargo fmt finds
pre-existing differences outside the touched files. No unrelated formatting
changes are included.

`checks.tar.xz` retains the raw development logs locally. At 88 KiB it exceeds
the artifact-store policy's 64 KiB data threshold, so it is ignored and not
committed. S3 upload is pending the same missing access; no remote manifest is
invented. `LOCAL_ARCHIVE.sha256` identifies this local snapshot. Short complete
check logs are retained as loose files; `SHA256SUMS` identifies the original
tested source and its check logs. Later rechecks have separate source manifests.
This is development evidence, not an admitted ecbench
run, replay certificate or performance ledger row. The full-suite archive log
is an explicitly dated pre-completion snapshot. A separate completed snapshot,
`checks-complete.tar.xz`, retains the final parent result and later failed checks;
its local identity is COMPLETE_LOCAL_ARCHIVE.sha256. Both exceed the store
threshold and remain ignored pending authorized upload. Readable short transcripts
normalize trailing blank lines; exact raw bytes stay in these local archives.

The checkpoint-only Actions attempt failed with an absent approved upload secret.
checkpoint-actions-receipt.json binds the workflow, immutable source, spec and
protocol hashes. Its build and experiment jobs were skipped: planned scientific
executions 280, attempted 0, completed 0. This is an administrative failure record,
not an invented canonical experimental export.

The native metadata verifier uses the existing queue module's jsonschema/v6
dependency. Fetch the canonical schema at the Git blob recorded in
protocol-validation.txt, verify its SHA-256, then run from the repository root:

```sh
go -C tools/experiment-queue run "$PWD/research/koblitz_halftrace_20261030/validate_protocol.go" \
  /path/to/experiment-protocol.v1.json \
  "$PWD/research/koblitz_halftrace_20261030/protocol.json" check
```

This checks schema and the prospective canonical hash, not experimental outcomes.
