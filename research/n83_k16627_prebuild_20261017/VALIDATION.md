# Validation and branch state

The preregistered construction protocol is commit `246a7ed01b`. The unchanged exporter was built and tested as recorded in [BUILD_RECEIPT.md](BUILD_RECEIPT.md): the offline Linux arm64 release example suite passed 19/19 tests. The first construction, generic replay, storage round trip and precommitted peak-recovery repeat have separate raw receipts in this directory.

Before stacking this branch on the current m=4 core, the following repository checks ran in this worktree:

| Check | Command and outcome | Evidence |
| --- | --- | --- |
| Rust format | `cargo fmt --all -- --check`; exit 0 | [log](cargo-fmt.log) |
| Release library | `CARGO_TARGET_DIR=/private/tmp/n83-structured-m4-target cargo test --release --lib -- --test-threads=2`; exit 0, 4,216 passed, 0 failed, 117 ignored, 501.29 s test execution | [log](cargo-release-lib.log) |
| Initial sparse-checkout attempt | Same release-library command; exit 101 during compile because tracked `include_bytes!` fixtures were absent from the custom sparse profile | [retained failure log](cargo-release-lib-sparse-failed.log) |

The missing tracked fixtures were materialized with `scripts/sparse-checkout.sh apply` before the passing retry. The initial exit 101 is a checkout infrastructure outcome, not a test failure. The passing suite applies to the pre-stack source head.

After merge commit `b04d68910c` stacked this branch on the m=4 core, the same release-library command exited 101 at compilation with 26 errors and five warnings. Its [current-head log](cargo-release-lib-stacked.log) includes duplicate `Fp4`, unresolved `ecbench::fbm`, missing `OnbWeightBase` and `sparsest`, and related incoming ecbench/gaudry errors. The repository-wide format command exited 1 on four incoming Rust files, recorded in [its current-head log](cargo-fmt-stacked.log). `git diff --name-only codex/n83-chained-s3-m4-20261017..b04d68910c -- '*.rs'` returned no paths: this K=16,627 branch changes no Rust source relative to the stacked parent. The current-head build obstruction is tracked in [the open problem](../../problems/current-main-library-build-20261017.md). These failed checks remain failed; the pre-stack pass does not substitute for them.

The study and `benchmarks/20261017-n83-k16627-prebuild/` SVG, PNG and PDF copies matched byte for byte under `cmp`. The PNG's `dpiWidth` and `dpiHeight` are both 150. This validates the report copies, not relation yield or complete IC cost.
