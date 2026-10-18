# Selector-control validation ledger

| Gate | Status | Receipt |
| --- | --- | --- |
| Frozen protocol | committed before implementation on review and clean execution branches | [protocol](PROTOCOL.md) |
| Checked selector/phase wire API | focused release control passed | [small-control-after-api-fix.log](small-control-after-api-fix.log), [orbit-domain-final.log](orbit-domain-final.log) |
| Completed XOR work observer | focused release parity control passed | [observer-parity-final.log](observer-parity-final.log) |
| Small direct/observed chained-field, group-lift and overhead controls | 20 AB/BA pairs of 100 checked solves per arm passed; timing is descriptive L0 only | [raw log](small-control-after-api-fix.log), [summary](small-overhead-summary.json) |
| Exact degree-83 source, inputs and Linux binary | pending freeze before run | pending manifests |
| Degree-83 selector-pinned positive arm | not attempted | pending frozen run |
| Full release library and touched exporter suites | pending | pending raw logs |

The first focused command exited before cargo because this sparse checkout had not materialized the newly tracked study directory; [the shell error](initial-missing-study-path.log) is retained. After materializing that path, the first compiler run reported eleven other tracked `include_str!` fixtures missing from the sparse checkout; its [raw log](observer-test-initial.log) is retained. Those exact fixtures were materialized and the build was rerun as [observer-test-after-sparse.log](observer-test-after-sparse.log). A sparse-checkout omission is an infrastructure condition, not a model result.

The first combined small-field control compile found an incorrect call to the underlying `BinaryCurve` group API; [the failed compiler log](small-control-first.log) is retained. The corrected control calls `KoblitzCurve::add` and passes in [small-control-after-api-fix.log](small-control-after-api-fix.log). Its full SAT model bits matched with and without the observer, the original S3 equations and group lift passed, and the contradictory unit-clause cases returned UNSAT. The overhead summary covers all 20 pairs and 4,000 checked solves; the shared macOS host produced wide timing spread, so these values are not a degree-83 overhead correction.
