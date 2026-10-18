# Search-progress validation ledger

| Gate | Source | Observed status | Receipt |
| --- | --- | --- | --- |
| Frozen protocol | commit `c158b99cba` before solver edits | committed | [protocol](PROTOCOL.md) |
| Six compiled source files | source commit `2e1ba2e476` | six SHA-256 matches in checkout and read-only snapshot | [manifest](source-files.sha256), [checkout](source-check.log), [snapshot](source-snapshot-check.log) |
| Five retained inputs | exact K=16,627 panel | five SHA-256 matches | [manifest](inputs-before-run.sha256), [check](input-check.log) |
| Optional observer SAT/UNSAT parity | final source | 1 passed, 0 failed; test requires actual decisions and a contradictory conflict | [raw log](progress-test.log) |
| Exporter example suite | final source | running | [raw log](example-tests.log) |
| Offline Linux arm64 release worker | final source | exit 0, no OOM; binary SHA-256 retained | [build receipt](BUILD_RECEIPT.md) |
| Capped-outcome derivation | committed fixture | synthetic 100-sample cap accepted; nonmonotone decision count rejected with exit 5 | [check](DERIVER_CHECK.md) |
| Full release library suite | final source | pending | pending raw log |
| Degree-83 positive arm | final source | pending | pending run records |

The first focused build encountered missing tracked files in a sparse
checkout and was interrupted after reporting them; its raw output is
retained in `progress-test-sparse-missing.log`. A prior focused test on the
unstrengthened fixture passed and is retained in `progress-test-initial.log`.
The earlier 19/19 exporter pass before the final test-only source change is
retained in `example-tests-pre-final-source.log`. These attempts do not
replace the final-source gates above.
