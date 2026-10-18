# Selector-control validation ledger

| Gate | Status | Receipt |
| --- | --- | --- |
| Frozen protocol | committed before implementation on review and clean execution branches | [protocol](PROTOCOL.md) |
| Checked selector/phase wire API | focused release control passed | [small-control-after-api-fix.log](small-control-after-api-fix.log), [orbit-domain-final.log](orbit-domain-final.log) |
| Completed XOR work observer | focused release parity control passed | [observer-parity-final.log](observer-parity-final.log) |
| Small direct/observed chained-field, group-lift and overhead controls | 20 AB/BA pairs of 100 checked solves per arm passed; timing is descriptive L0 only | [raw log](small-control-after-api-fix.log), [summary](small-overhead-summary.json) |
| Exact degree-83 source, inputs and Linux binary | six source files and five inputs match; accepted offline Linux build exited 0 | [build receipt](BUILD_RECEIPT.md), [source](source-files.sha256), [inputs](inputs-before-run.sha256), [binary](worker-binary.sha256) |
| Degree-83 selector-pinned positive arm | `PASS_SELECTOR_PINNED_WITNESS`; exit 0, no OOM; exact x/group/rank checks | [run report](RUN_REPORT.md), [raw receipt](positive/worker.json), [raw hashes](positive-files.sha256) |
| Separate native relation replay | `PASS_RELATION_REPLAY`; changed-x and changed-row receipts each rejected with exit 1 | [replay receipt](replay/receipt.json), [raw negative controls](replay/) |
| Touched exporter example suite | 19 passed, 0 failed | [raw log](exporter-example-tests.log) |
| Full release library suite | pending | pending raw log |

The first focused command exited before cargo because this sparse checkout had not materialized the newly tracked study directory; [the shell error](initial-missing-study-path.log) is retained. After materializing that path, the first compiler run reported eleven other tracked `include_str!` fixtures missing from the sparse checkout; its [raw log](observer-test-initial.log) is retained. Those exact fixtures were materialized and the build was rerun as [observer-test-after-sparse.log](observer-test-after-sparse.log). A sparse-checkout omission is an infrastructure condition, not a model result.

The first combined small-field control compile found an incorrect call to the underlying `BinaryCurve` group API; [the failed compiler log](small-control-first.log) is retained. The corrected control calls `KoblitzCurve::add` and passes in [small-control-after-api-fix.log](small-control-after-api-fix.log). Its full SAT model bits matched with and without the observer, the original S3 equations and group lift passed, and the contradictory unit-clause cases returned UNSAT. The overhead summary covers all 20 pairs and 4,000 checked solves; the shared macOS host produced wide timing spread, so these values are not a degree-83 overhead correction.

The first pinned Linux worker build exited 101 while expanding the already large exporter `json!` receipt after the new selector field pushed it past the macro recursion limit. The [raw compiler error](linux-build-first-stderr.log) and [container state](linux-build-first-state.json) are retained. The field was moved to a typed `Value` assignment after the receipt literal; this changes only receipt construction and requires a new source commit and binary freeze before execution.

The accepted degree-83 worker completed on source `b4beefcb55f05713d1d63468fcbcd21e4339fa50`. The separate Rust verifier reads the exact compressed object and worker receipt, and does not construct or solve the SAT model. It rechecks the original-field chain and group relation. A changed first x block failed with `worker x block differs from independent group addition`; a changed relation-row digest failed with `relation row hash differs from independent phase-zero row`. Both negative invocations exited 1 and left no success output. The worker did not export the SAT assignment bits, so the verifier's successful receipt explicitly records `solver_model_bits_replayed: false`.
