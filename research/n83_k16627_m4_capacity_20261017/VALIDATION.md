# Validation of the retained K=16,627 m=4 capacity branch

The native macOS release exporter example tests passed 19/19 on the committed adapter; [the example log](example-test.log) is retained with the build receipt. `rustfmt --check` on the touched Rust file and repository-wide `cargo fmt --all -- --check` passed before stacking. The exact native degree-83 source-bound, Frobenius-orbit preflight and chained-S3 relation/rank/target-solve controls passed before the worker; see [CONTROL_RECEIPT.md](CONTROL_RECEIPT.md). The Linux arm64 release worker built offline and both capped worker arms completed with source and input checks.

The first `cargo test --release --lib -- --test-threads=2` attempt ran in the restricted local sandbox. It was intentionally interrupted after network collaboration tests failed with `Operation not permitted` on local socket creation. [Its partial raw log](cargo-release-lib-sandbox-denied.partial.log) preserves the other completed tests, including three unrelated algebra tests that failed under shared-host contention. The exact `cryptanalysis::pollard_collab::net::tests::foreign_job_is_refused` test failed in the sandbox at `src/cryptanalysis/pollard_collab/net.rs:359` with `Os { code: 1, kind: PermissionDenied, message: "Operation not permitted" }` and passed unchanged with local socket access. The exact `cryptanalysis::jv_cover::tests::nagao_finds_planted_six_sums_and_agrees_with_the_oracle` test also passed when rerun alone. Neither test was edited or omitted.

A fresh full release library suite with local socket access completed against
the pre-stack source checkout at `bdf9c8dc45`: **4,216 passed, 0 failed, 117 ignored**, in
3,503.73 seconds. Its complete [raw log](cargo-release-lib-prestack.log)
and process exit code 0 are retained. The suite's last test was the unrelated
randomized 16-bit Barrett parameter-generation check; it passed without a
source change. This establishes the pre-stack library gate. The stacked head
also inherits the current main-branch library build obstruction recorded in
`problems/current-main-library-build-20261017.md`; its exact current-head
validation is recorded separately below.

## Stacked-head validation

`cargo fmt --all -- --check` exited 1 on the stacked head. Its retained
[log](cargo-fmt-stacked.log) names only
`examples/n83_precomputed_rho.rs`,
`examples/n83_precomputed_rho_analyze.rs`, and
`research/conductor_gap_cm_orbits_20261015/native/ecbench_next.rs`.
All three are byte-unchanged from the #2124 base branch. The touched
`primary_adapter.rs` passed `rustfmt --check --edition 2021` separately;
its [log](rustfmt-touched-stacked.log) is empty on success.

`CARGO_TARGET_DIR=/private/tmp/n83-structured-m4-target cargo test --release
--lib -- --test-threads=2` exited **101** on stacked source commit `d6b7360498`
before any library test could execute. The complete [compiler log](cargo-release-lib-stacked.log)
contains **26 errors**, including unresolved `ecbench::fbm` paths, inconsistent
`Fp4` definitions, and Gaudry F4 names. The error locations are confined to
`src/cryptanalysis/ecbench/{db,methods,runner,workload}.rs` and
`src/cryptanalysis/gaudry_quartic.rs`; all five files are byte unchanged
from the #2124 base. This is the same open inherited blocker tracked in
`problems/current-main-library-build-20261017.md`. The touched exporter
cannot be retested at this stacked head until the base library compiles;
its 19/19 release pass above applies to the pre-stack source.

`node --check .github/scripts/ci-scope.mjs` also exited 1 on the stacked
head because `root` is declared twice at line 50; the script is unchanged
from #2124. Its [raw log](ci-scope-stacked.log) is retained as a CI diagnostic.
