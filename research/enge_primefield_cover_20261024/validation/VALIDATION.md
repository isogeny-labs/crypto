# Validation record

Date: 2026-10-24 to 2026-10-25. Checkout: branch
`codex/enge-primefield-cover-20261024`; initial implementation commit
`4dffaf899a`, with the final head recorded by PR #2532.
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
| Study crate | `cargo test --release --offline --locked --manifest-path research/enge_primefield_cover_20261024/Cargo.toml` | 2 passed | [study-tests.log.gz](study-tests.log.gz) |
| Identity certificate | `target/release/issue_cover_identity --check research/enge_primefield_cover_20261024/target_norm_identity.certificate.json` | `IDC1h4255f156b69df1c2` accepted with 32 extra checker-seed points | [certificate-check.log.gz](certificate-check.log.gz) |
| Root library, parallel | `cargo test --release --lib --offline` | 4,625 passed, 2 failed, 119 ignored; F4 basis assertion and GPU lock collision | [enge-root-lib-test-final.log.gz](enge-root-lib-test-final.log.gz) |
| F4 isolated | `cargo test --release --lib --offline cryptanalysis::f4_fp::tests::hilbert_stop_gives_the_same_reduced_basis_as_the_plain_run -- --exact` | 1 passed | [f4-isolated.log.gz](f4-isolated.log.gz) |
| GPU isolated | `cargo test --release --lib --offline cryptanalysis::ecbench::gpu::lock_and_capsule_tests::a_gpu_stage_writes_a_host_capsule_the_receipt_binds -- --exact` | 1 passed | [gpu-isolated.log.gz](gpu-isolated.log.gz) |
| WDSat isolated | `cargo test --release --lib --offline cryptanalysis::wdsat_oracle::tests::mock_child_exit_and_empty_output_are_not_refutations -- --exact` | 1 passed after a two-second timeout in the earlier sparse-checkout run | [wdsat-isolated.log.gz](wdsat-isolated.log.gz) |
| Final-head study crate | Same locked release test command after the lint fix | 2 passed | [study-tests-head.log.gz](study-tests-head.log.gz) |
| Final-head study lint | `cargo clippy --release --offline --locked --manifest-path research/enge_primefield_cover_20261024/Cargo.toml --all-targets -- -D warnings` | Passed | [study-clippy-head.log.gz](study-clippy-head.log.gz) |
| Optional issuer-bin lint | Same Clippy command with `--features certificate --bin issue_cover_identity` | Interrupted during dependency checking on the shared host; no lint verdict for this bin | [issuer-clippy-interrupted.log.gz](issuer-clippy-interrupted.log.gz) |

The parallel failures concern existing F4 and GPU test paths that this PR
does not modify. The GPU failure reported another benchmark holding a
`/tmp/ecbench-gpu-capsule-*/lock`; the F4 failure reported assertion value
`19`. Both focused replays passed. A serial root-suite replay
(`cargo test --release --lib --offline -- --test-threads=1`) was stopped
after 865 passing tests and no reported failures because running all 4,746
tests serially on this shared host was taking much longer than the parallel
suite. It exited 130 and is **not a complete pass**; its partial raw output
is [enge-root-lib-serial-interrupted.log.gz](enge-root-lib-serial-interrupted.log.gz).

The Rust run record is
[`../reports/run-20261024T2315Z/REPORT.md`](../reports/run-20261024T2315Z/REPORT.md).
The native JSON matched every field of the earlier out-of-repository
Python diagnostic under `jq -S`. After the lint fix, a fresh native run
matched the committed result byte for byte (`cmp` exit 0); its current source
SHA-256 is `4d5ed2e5ee973bbadffc69c4fb2286ccd5c2b6fad9ac7f8d77b258ea259409c1`.
The committed PDF reports were rendered
with `pdflatex`, checked for LaTeX warnings, and inspected page by page.
The pinned Lean project was attempted but did not finish its mathlib checkout;
the report marks its proof source unverified.
