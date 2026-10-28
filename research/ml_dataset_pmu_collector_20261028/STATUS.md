# Collector implementation status

2026-10-28: native implementation authored after design commit3c7964ae0a7.
This is an unvalidated implementation checkpoint, not a measurement result.

The isolated branch adds isolation::checked beside the unchanged HwCounters API.
The native backend is limited explicitly to Linux x86_64/aarch64; other targets
return unsupported. Per-event records preserve full requested attrs, event IDs,
open/control/read/close outcomes, raw tuples and checked estimates. Policy can
require cycles or allow a separately recorded standalone-instruction fallback
before work. A cleanup failure prevents that fallback. Active state is thread
bound and RAII owns descriptor cleanup, including preparation unwind.

Nineteen mock test functions cover syscall failures, inheritance scope, optional
cycles, ID substitution, tuple math, repeated start, read lengths, cleanup and
unwind. They have been authored but have not yet been compiled or run. Scoped
rustfmt and git diff --check passed. Parent owns Cargo scheduling and final
required native gates. No new live PMU test has been introduced.

Dataset phase schemas and existing historical records are untouched. The new
collector record keeps platform admission, independent hardware verification,
complete execution, descendant lifetime coverage, whole-worker coverage and
comparative clocks false. Kernel control success is reported evidence, not an
independent proof of hardware truth. Cleanup failures remain separate from a
successfully obtained raw tuple; consumers must inspect both.

No PR, cloud worker, fresh scientific measurement or privilege change was made
for this checkpoint. Original maximum USD200 total campaign cap remains in force.
