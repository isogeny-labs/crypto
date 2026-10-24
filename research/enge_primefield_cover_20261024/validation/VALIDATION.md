# Validation record

Date: 2026-10-24. Checkout: branch
`codex/enge-primefield-cover-20261024`, commit `4dffaf899a`.
Host: shared x86-64 EC2 host with 32 logical CPUs. The root checkout is
sparse; the first root-library attempts lacked unrelated fixture paths.
Those failures and their raw outputs are retained as
[`enge-root-lib-test.log.gz`](enge-root-lib-test.log.gz),
[`enge-root-lib-test-2.log.gz`](enge-root-lib-test-2.log.gz), and
[`enge-root-lib-test-3.log.gz`](enge-root-lib-test-3.log.gz).
The missing tracked fixture paths were materialized with
`git sparse-checkout add`; no repository source was changed to fix them.

| Check | Command | Observed result | Raw output |
| --- | --- | --- | --- |
| Study crate | `cargo test --release --offline --locked --manifest-path research/enge_primefield_cover_20261024/Cargo.toml` | 2 passed | [study-tests.log](study-tests.log) |
| Identity certificate | `target/release/issue_cover_identity --check research/enge_primefield_cover_20261024/target_norm_identity.certificate.json` | `IDC1h4255f156b69df1c2` accepted with 32 extra checker-seed points | [certificate-check.log](certificate-check.log) |
| Root library, parallel | `cargo test --release --lib --offline` | 4,625 passed, 2 failed, 119 ignored; F4 basis assertion and GPU lock collision | [enge-root-lib-test-final.log.gz](enge-root-lib-test-final.log.gz) |
| F4 isolated | `cargo test --release --lib --offline cryptanalysis::f4_fp::tests::hilbert_stop_gives_the_same_reduced_basis_as_the_plain_run -- --exact` | 1 passed | [f4-isolated.log](f4-isolated.log) |
| GPU isolated | `cargo test --release --lib --offline cryptanalysis::ecbench::gpu::lock_and_capsule_tests::a_gpu_stage_writes_a_host_capsule_the_receipt_binds -- --exact` | 1 passed | [gpu-isolated.log](gpu-isolated.log) |
| WDSat isolated | `cargo test --release --lib --offline cryptanalysis::wdsat_oracle::tests::mock_child_exit_and_empty_output_are_not_refutations -- --exact` | 1 passed after a two-second timeout in the earlier sparse-checkout run | [wdsat-isolated.log](wdsat-isolated.log) |

The parallel failures concern existing F4 and GPU test paths that this PR
does not modify. The GPU failure reported another benchmark holding a
`/tmp/ecbench-gpu-capsule-*/lock`; the F4 failure reported assertion value
`19`. Both focused replays passed. A serial root-suite replay was also
launched to test the concurrency hypothesis; its result is recorded below
when complete.

The Rust run record is
[`../reports/run-20261024T2315Z/REPORT.md`](../reports/run-20261024T2315Z/REPORT.md).
The native JSON matched every field of the earlier out-of-repository
Python diagnostic under `jq -S`. The committed PDF reports were rendered
with `pdflatex`, checked for LaTeX warnings, and inspected page by page.
The pinned Lean project was attempted but did not finish its mathlib checkout;
the report marks its proof source unverified.
