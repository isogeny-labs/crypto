#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
#set heading(numbering: none)
#align(center)[#text(size: 18pt, weight: "bold")[L4 early-stop recovery validation]#linebreak()
Recovered source and retained measurements · 19 October 2026]

= Verified outcome
The full release-library run passed: *4,400 passed, zero failed, 118 ignored*.
The F4 controls passed 44 tests; cover controls passed 15, with one ignored.
Both examples built successfully and contain zero example tests.
The full run finished at 22:12:15 UTC (593.41 seconds reported by the harness).

#image("validation.svg", width: 100%)

= Exact source and execution
Base: #raw("abee07ee7337d04a7e734333669ca1bacd6c763a").
The recovered modifications are committed with this report. All six source and
manifest SHA-256 values match the passing receipt. The receipt preserves the
commands, exit statuses, time limits and timestamps.

Native tests ran on a shared Linux EC2 worker, cores 24–25, with
#raw("RAYON_NUM_THREADS=1"). The full suite used two test threads.
The 27 historical raw files were retained and hashed; measurements were not rerun.

= Evidence to inspect
- #raw("receipt.json"): identities and bounded command receipts.
- #raw("full-library.log.gz"): complete full-suite output.
- #raw("f4-focused.log.gz") and #raw("cover-focused.log.gz"): focused controls.
- #raw("examples.log.gz"): both example builds and zero-test results.
- #raw("retained-input-manifest.json"): hashes for all original raw files.
- #raw("../run.sh"): original measurement commands.

= Scope and remaining work
The implementation adds opt-in guarded Hilbert stopping and diagnostic reporting.
The original aggregation script is a historical no-op placeholder. Saved parity
records are producer evidence; the p=251 cover receipts contain no independent
oracle checks. These tests do not retroactively establish independent replay of
every historical measurement. Ignored tests remain unexecuted.

This package records correctness validation and source recovery. Performance
rankings, complete DLP cost comparisons, and leaderboard promotion remain open.
No canonical performance graph was changed. Integration review remains pending.
