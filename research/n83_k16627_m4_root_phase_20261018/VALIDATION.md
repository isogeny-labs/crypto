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
| Full release library suite | pre-stack source | pending | pending raw log |
| Final stacked-head release and example checks | review branch | pending | pending raw logs |

The root-phase degree-83 arm was launched only after the source/input
hashes, Linux worker build, and targeted native tests above passed. Its
eventual outcome and phase receipts are separate from these correctness
gates. The two failed worker builds and the interrupted first test command
are retained in this study and are not counted as passing validation.
