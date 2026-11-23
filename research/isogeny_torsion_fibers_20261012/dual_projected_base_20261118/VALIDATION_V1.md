# Validation and build receipt

The frozen experiment source was committed at
`0077f9d154724ca7ecbf9914040dc95a9eb3ab28` before the first batch
child. The release runner and independent checker binaries were built from
that commit with Rust 1.99.1 on Apple M4 Pro/macOS 27.2. Their source and
binary snapshots and SHA-256 hashes are in [`RUN_V1`](RUN_V1/).

## Commands and outcomes

1. `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc rustup run 1.99.1 cargo test --release --example isogeny_fiber_dual_projected --example isogeny_fiber_dual_projected_check` — **pass**: two runner tests; checker example built with zero tests. The final post-run output is [`VALIDATION_EXAMPLES_V1.stdout`](VALIDATION_EXAMPLES_V1.stdout).
2. `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc rustup run 1.99.1 cargo build --release --example isogeny_fiber_dual_projected --example isogeny_fiber_dual_projected_check` — **pass** before target freeze and first child.
3. `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc rustup run 1.99.1 cargo test --release --lib` — **first attempt failed** because the new sparse worktree lacked tracked `include_str!` fixtures. The full compiler output is preserved in [`VALIDATION_LIB_SPARSE_ATTEMPT_V1.stderr`](VALIDATION_LIB_SPARSE_ATTEMPT_V1.stderr). The missing tracked files were materialized; no source test was edited.
4. The same release library command after fixture materialization — **failed at compilation**, `E0428`, because `the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` is declared twice in [`src/cryptanalysis/ic_framework/plugins.rs`](../../../src/cryptanalysis/ic_framework/plugins.rs), at lines 5285 and 5322. Both declarations are present in `origin/main` at `ad2a014c0022`; this branch has no diff to that file. The exact output is [`VALIDATION_LIB_BASE_FAILURE_V1.stderr`](VALIDATION_LIB_BASE_FAILURE_V1.stderr). The full library test count is therefore unset for this branch.
5. `rustfmt --check --edition 2021 examples/isogeny_fiber_dual_projected.rs examples/isogeny_fiber_dual_projected_check.rs`, `bash -n research/isogeny_torsion_fibers_20261012/dual_projected_base_20261118/run_panel.sh`, and source `git diff --check` — **pass**.
6. `research/isogeny_torsion_fibers_20261012/dual_projected_base_20261118/run_panel.sh` — **pass**: 18 children, 18 replays, pair audit and summary. [`RUN_V1/RUN_STATUS.txt`](RUN_V1/RUN_STATUS.txt) records zero failures. `sha256sum --quiet -c ARTIFACT_FILES.sha256` from within `RUN_V1` — **pass**.
7. `typst compile` on the run and result `.typ` sources — **pass**; the resulting PDFs were rendered and visually inspected. The two PDFs and chart are in [`reports/run-20261118T170106Z`](reports/run-20261118T170106Z/).

The duplicate base test blocks the required full-library validation gate for
this PR. A separate base-branch repair or reviewed merge of a repair is
needed; the frozen run does not depend on that test declaration. The
experiment's primitive price, independent primitive-counter checker, L2
wall, and larger-order comparison remain separate open research gates.
