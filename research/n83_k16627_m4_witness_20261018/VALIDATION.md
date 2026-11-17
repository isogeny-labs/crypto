# Complete-witness validation ledger

| Gate | Source | Observed status | Receipt |
| --- | --- | --- | --- |
| Retained object and five staged inputs | exact K=16,627 panel | five SHA-256 matches; prior generic object/point replay retained by the construction study | [manifest](inputs-before-run.sha256), [check](input-check.log) |
| Five compiled source files | clean pre-stack commit `f68b9a94ada669360f88be4d8c8a5288cca55d73` | five SHA-256 matches in checkout and read-only snapshot | [manifest](source-files.sha256), [checkout](source-check.log), [snapshot](source-snapshot-check.log) |
| Rust exporter check | pre-stack source | passed in 25.39 s | [log](cargo-check-prestack.log) |
| Repository formatting | pre-stack source | passed | [log](cargo-fmt-prestack.log) |
| New seven-block core test | pre-stack source | 1 passed, 0 failed in 0.07 s | [matched log](core-test-matched.log) |
| First exact-name test filter | pre-stack source | 0 selected; rerun above is the actual test | [raw log](core-test.log) |
| Touched exporter example suite | pre-stack source | 19 passed, 0 failed in 63.85 s | [log](example-test.log) |
| Offline Linux arm64 release worker | pre-stack source | build exit 0, no OOM, 2m22s | [state](linux-build.container-state.json), [log](linux-build.stderr.log), [binary digest](worker-binary.sha256) |
| Current stacked-head exporter check | stacked review branch | exit 101 before example compilation: 25 inherited ecbench/Gaudry library errors | [log](cargo-check-stacked.log) |
| Full release library suite | pre-stack source | exit 0; 4,217 passed, 0 failed, 117 ignored, 0 filtered in 2,093.09 s | [raw log](cargo-release-lib-prestack.log) |
| Full release library suite | stacked review head | exit 101 before tests: 26 compiler errors in unchanged ecbench/Gaudry code | [raw log](cargo-release-lib-stacked.log) |
| Touched exporter example suite | stacked review head | exit 101 before tests: 25 compiler errors in unchanged ecbench/Gaudry code | [raw log](example-test-stacked.log) |
| Three touched Rust files formatting | stacked review head | passed `rustfmt --check --edition 2021` | [log](rustfmt-touched.log) |
| Whole-repository formatting | stacked review head | exit 1 on four unchanged files | [raw log](cargo-fmt-stacked.log) |
| CI scope script syntax | stacked review head | exit 1: duplicate `root` declaration at line 50 in unchanged script | [raw log](node-ci-scope-stacked.log) |

The positive run itself is a bounded `UNKNOWN_wall_cap` outcome, not a
passing SAT/model/group-lift validation. Its source is the clean pre-stack
commit above. The three touched Rust files have the same bytes on the
stacked review branch, but that branch cannot currently pass a whole-crate
compile because of unchanged upstream files. The final-head release test
also exited before tests with the same inherited compiler errors. These
branch-level failures are not a passing final-head gate and remain open
on the PR stack. The raw suite logs retain their trailing blank lines;
`git diff --check` flags those receipt lines without a source-code issue.
