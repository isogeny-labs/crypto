# Search-progress validation ledger

| Gate | Source | Observed status | Receipt |
| --- | --- | --- | --- |
| Frozen protocol | commit `c158b99cba` before solver edits | committed | [protocol](PROTOCOL.md) |
| Six compiled source files | source commit `2e1ba2e476` | six SHA-256 matches in checkout and read-only snapshot | [manifest](source-files.sha256), [checkout](source-check.log), [snapshot](source-snapshot-check.log) |
| Post-run source identity | same six files | all six SHA-256 hashes still match after the capped arm | [check](source-check-after-run.log) |
| Five retained inputs | exact K=16,627 panel | five SHA-256 matches | [manifest](inputs-before-run.sha256), [check](input-check.log) |
| Optional observer SAT/UNSAT parity | final source | 1 passed, 0 failed; test requires actual decisions and a contradictory conflict | [raw log](progress-test.log) |
| Exporter example suite | final source | 19 passed, 0 failed | [raw log](example-tests.log) |
| Offline Linux arm64 release worker | final source | exit 0, no OOM; binary SHA-256 retained | [build receipt](BUILD_RECEIPT.md) |
| Post-run worker binary identity | same arm64 binary | SHA-256 still matches the frozen worker receipt | [check](worker-binary-after-run.log) |
| Capped-outcome derivation | committed fixture | synthetic 100-sample cap accepted; nonmonotone decision count rejected with exit 5 | [check](DERIVER_CHECK.md) |
| Full release library suite | exact worker source before stacking, after the degree-83 arm | exit 0; 4,220 passed, 0 failed, 117 ignored; 1,808.27 s test time | [raw log](cargo-release-lib-prestack.log) |
| Degree-83 positive arm | source commit `2e1ba2e476`; exact retained object | `UNKNOWN_wall_cap`; exit 124, no OOM; last of 176 fsynced samples: 322,255 decisions, 360 conflicts, 2 restarts | [outcome](positive/outcome.json), [raw-file hashes](positive-files.sha256), [hash check](positive-files-check.log), [copy check](raw-copy-check.log) |
| Visual record | final raw outcome | run and result PDFs compiled; SVG and 150 dpi PNG rendered from one source and visually inspected | [run PDF](RUN_REPORT.pdf), [result PDF](RESULT.pdf), [graph](search-progress.svg), [PNG](search-progress.png) |
| Benchmark browse copy | final visual files | four files byte-identical to study originals | [copy check](benchmark-copy-check.log), [browse directory](../../benchmarks/20261018-n83-k16627-m4-search-progress/) |
| Stacked review source boundary | review branch above the root-phase PR | five of six frozen worker files match; `src/cryptanalysis/koblitz_index_calculus.rs` differs because the stacked parent carries later work | [check](stacked-source-check.log) |
| Stacked review release library suite | merged review head | exit 101: 26 inherited ecbench/Gaudry compilation errors; no test execution on this head | [raw log](cargo-release-lib-stacked.log) |
| Stacked review exporter example suite | merged review head | exit 101: 25 inherited ecbench/Gaudry compilation errors; no example test execution on this head | [raw log](example-tests-stacked.log) |
| Stacked touched-file Rust formatting | merged review head | passed | [check](rustfmt-stacked-touched.log) |
| Stacked CI scope script | merged review head | inherited duplicate `root` declaration at `.github/scripts/ci-scope.mjs:50`; Node exits 1 before scope evaluation | [raw log](ci-scope-stacked.log) |

The first focused build encountered missing tracked files in a sparse
checkout and was interrupted after reporting them; its raw output is
retained in `progress-test-sparse-missing.log`. A prior focused test on the
unstrengthened fixture passed and is retained in `progress-test-initial.log`.
The earlier 19/19 exporter pass before the final test-only source change is
retained in `example-tests-pre-final-source.log`. These attempts do not
replace the final-source gates above.

The observer parity test covers direct and observed small native SAT/UNSAT
search, including model-bit equality and checked XOR rows. The exporter
suite covers existing chained-field correctness, but it does not yet pair
direct and observed modes on one chained-field fixture with independent
group lifting; sampler overhead on that fixture is also unmeasured. These
remain explicit controls for the next instrumented run, not silently passed
gates for this one. The degree-83 worker source is the clean pre-stack
commit above; the stacked review branch must preserve that boundary rather
than describe the worker as a replay of its entire final head.
