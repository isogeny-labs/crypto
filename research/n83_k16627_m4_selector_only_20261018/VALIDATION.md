# Selector-only comparison validation ledger

| Gate | Current state | Receipt or remaining work |
| --- | --- | --- |
| Protocol before code | committed as `3ed109bd24` | [frozen protocol](PROTOCOL.md) |
| Four/28-unit arm separation | compiled, small-field control passed; degree-83 behavior pending | [focused release test](focused-test.log), Rust adapter and CLI |
| Full/selector-only/phase-only small SAT and contradiction fixture | one release test passed; direct/observed complete-model parity, original-field group lift and contradictory UNSAT checked in all modes | [focused release test](focused-test.log) |
| Capped metadata filter | synthetic selector-only and phase-only fixtures accepted; decreasing XOR row scans and wrong-arm identity rejected with exit 5 | [deriver controls](DERIVER_CHECK.md) |
| Five retained panel inputs | SHA-256 checks passed before build | [input manifest](inputs-before-run.sha256), [check](input-check.log) |
| Exact source and Linux worker | six source files and five inputs checked; offline Linux build exited 0 without OOM | [build receipt](BUILD_RECEIPT.md), [source](source-files.sha256), [inputs](inputs-before-run.sha256), [binary](worker-binary.sha256) |
| A1/B/A2 and conditional C | A1, B and A2 passed with one checked rank-one relation each; B returned before its cap, so C omitted by protocol | [run report](RUN_REPORT.md), [summary](summary.json), [raw arms](arms/), [hash manifest](raw-files.sha256) |
| Separate native relation replay | all three positives passed; six changed-x/row receipts rejected with exit 1 | [replay receipts and negative checks](replay/NEGATIVE_CHECKS.md), [verifier source](replay-source.sha256), [binary](replay-binary.sha256) |
| Touched exporter example suite | 20 passed, 0 failed in release mode | [raw log](exporter-example-tests.log) |
| Replay example compile | passed in release mode; the example has no embedded tests | [raw build log](replay-example-build.log) |
| Full release library suite on follow-on execution branch | passed: 4,221 passed, 0 failed, 117 ignored in 1,133.66 s | [raw log](cargo-release-lib-followon.log); `CARGO_TARGET_DIR=/Volumes/SSD990-2/.codex/worktrees/ec70/crypto/target/n83-selector-only-build cargo test --release --lib -- --test-threads=2` |
| Full release library suite on stacked review branch | blocked before tests: exit 101, 27 inherited ecbench/quartic source errors after tracked fixtures were materialized | [accepted stacked log](cargo-release-lib-stacked.log), [initial sparse-checkout failure log](cargo-release-lib-stacked-sparse.log), [open build problem](../../problems/current-main-library-build-20261017.md) |
| Touched exporter example on stacked review branch | blocked before tests: exit 101, 26 inherited ecbench/quartic source errors | [raw log](exporter-example-tests-stacked.log) |
| Touched replay example on stacked review branch | blocked before tests: exit 101, the same 26 inherited source errors | [raw log](replay-example-tests-stacked.log) |
| CI scope syntax on stacked review branch | blocked: unchanged script declares `const root` twice | [raw syntax log](ci-scope-stacked.log), [initial sparse-checkout diagnostic](ci-scope-stacked-sparse.log), [open build problem](../../problems/current-main-library-build-20261017.md) |
| Raw, visual and benchmark copies | 70 raw/replay files rehashed; SVG and 150-dpi PNG inspected; result and run PDFs rendered and inspected; benchmark copies byte-equal | [hash check](raw-files-check.log), [renderer log](report-generation.log), [run PDF](RUN_REPORT.pdf), [result PDF](RESULT.pdf) |

The initial report-renderer call failed because its replay receipt names did not match the actual `a1.json`, `b.json` and `a2.json` paths; [the raw error](report-generation-first.log) is retained. The renderer was corrected before generating the accepted report from the same frozen worker and verifier receipts. The degree-83 arms used worker source commit `48ebe3b370810e6da2d48e5234e42ba92bba259b`; reporting-source edits after the arms did not change any of the six compiled worker files. These single constructed-witness arms establish a checked stage result, not natural relation yield or a complete IC performance ratio.

Final review recheck: the six-file manifest passes against the retained worker source snapshot, the 70-file raw manifest passes from the repository root, and the study and benchmark PDF/SVG copies match byte for byte. The merged review head's `src/cryptanalysis/koblitz_index_calculus.rs` differs from the frozen worker snapshot because of parent-branch integration; the arm receipts are bound to the snapshot and binary hashes, not to a rebuilt binary from the merged head. A fresh current-head worker replay remains required after the inherited build errors are repaired.
