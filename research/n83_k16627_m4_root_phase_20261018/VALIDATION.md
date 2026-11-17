# Root-phase validation ledger

| Gate | Source | Observed status | Receipt |
| --- | --- | --- | --- |
| Five retained inputs | exact K=16,627 panel | five SHA-256 matches | [manifest](inputs-before-run.sha256), [check](input-check.log) |
| Six compiled source files | clean pre-stack commit `43c9850a73f1d960f65ed470fdb8adac981a5ffe` | six SHA-256 matches in checkout and read-only snapshot | [manifest](source-files.sha256), [checkout](source-check.log), [snapshot](source-snapshot-check.log) |
| Root SAT/UNSAT continuation | pre-stack source | 2 passed, 0 failed | [raw log](root-tests.log) |
| Small-field four-summand witness | pre-stack source | 1 passed, 0 failed, original-field model checked | [raw log](field-witness-test.log) |
| Touched exporter example suite | pre-stack source | 19 passed, 0 failed | [raw log](example-tests.log) |
| Offline Linux arm64 release worker | pre-stack source | build exit 0, no OOM; two earlier compiler failures retained | [build receipt](BUILD_RECEIPT.md), [state](linux-build-state.json) |
| Four touched Rust files formatting | pre-stack source | passed `rustfmt --check --edition 2021` on the accepted source | [log](rustfmt-final.log) |
| Frozen degree-83 positive arm | accepted Linux worker | `UNKNOWN_wall_cap`, exit 124, no OOM; root returned `Fixpoint` in 185.023 ms; post-root solver result unknown | [outcome](positive/outcome.json), [phase records](positive/worker.json.phases.jsonl), [raw-file hashes](positive-files.sha256) |
| Visual report and benchmark copy | accepted run receipts | two one-page PDFs; 2500×1292 graph PNG at 150 dpi; SVG/PNG/PDF copies byte-identical | [report](RUN_REPORT.md), [benchmark copy](../../benchmarks/20261018-n83-k16627-m4-root-phase/) |
| Full release library suite, sandboxed attempt | pre-stack source | 4,210 passed, 9 failed on loopback socket bind with `Operation not permitted`, 117 ignored; exit 101 | [raw log](cargo-release-lib-prestack.log) |
| Loopback socket control, outside sandbox | pre-stack source | 1 passed, 0 failed | [raw log](socket-control-unsandboxed.log) |
| Full release library suite, outside sandbox | pre-stack source | 4,219 passed, 0 failed, 117 ignored; exit 0, including all nine loopback tests that failed in the sandboxed attempt | [raw log](cargo-release-lib-prestack-unsandboxed.log) |
| Final stacked-head release library | review branch | exit 101 before tests: 26 inherited ecbench/Gaudry compile errors | [raw log](cargo-release-lib-stacked.log) |
| Final stacked-head exporter example | review branch | exit 101 before tests: 25 inherited ecbench/Gaudry compile errors | [raw log](example-tests-stacked.log) |
| Four touched Rust files formatting | review branch | passed direct `rustfmt --check --edition 2021 --config skip_children=true` | [raw log](rustfmt-stacked-touched.log) |
| Repository-wide formatting | review branch | exit 1 on four untouched files | [raw log](cargo-fmt-stacked.log) |
| CI scope script | review branch | exit 1: duplicate `root` declaration at `.github/scripts/ci-scope.mjs:50` | [raw log](ci-scope-stacked.log) |

The root-phase degree-83 arm was launched only after the source/input
hashes, Linux worker build, and targeted native tests above passed. Its
capped outcome and phase receipts are separate from these correctness
gates. The two failed worker builds and the interrupted first test command
are retained in this study and are not counted as passing validation.

The run's accepted source is the clean pre-stack commit
`43c9850a73f1d960f65ed470fdb8adac981a5ffe`, recoverable from the
branch history and the six-file snapshot manifest. Five of those six files
are byte-identical at this stacked review head. The stacked parent carries
additional changes in `src/cryptanalysis/koblitz_index_calculus.rs`; its
bytes differ from the measured worker. The run receipt applies to the
accepted source commit, while stacked-head compilation is reported
separately in the table. The merge kept the parent file instead of rewriting it
to match the earlier experiment.

The same ecbench/Gaudry compile failures and CI-scope syntax error were
recorded on the preceding [complete-witness branch](../n83_k16627_m4_witness_20261018/VALIDATION.md).
They prevent stacked-head test execution and PR CI admission; they do not
change the completed pre-stack tests or the frozen worker receipt.
