# Reject signal deaths as negative CLI evidence

Frozen before edits. Review of native fixture06 run.rs found that supervised()
returns -1 for a child killed by a signal. Runner::invoke classified every
nonzero status as an expected negative when success=false; its current timeout
and cancellation checks do not exclude an unsolicited signal death. The copied
control contract expressly requires signal failure to remain a failure.

Extract a small pure exit-admission predicate: reject negative status, timeout,
cancellation, and mismatched success expectation. Accept an ordinary nonzero
exit only as process-level negative admission; a separate control diagnostic
classifier must establish the expected semantic rejection. Wire Runner::invoke
to that predicate. Add a table-based native unit test covering zero, positive,
and -1 statuses in both expectation modes, with timeout and cancellation.
Do not execute processes to simulate the table or alter runner resource limits.

Retain native fixture06 and candidate08. This next revision changes the native
controller only, without changing dataset libraries or their passing results.
The helper02 validation can proceed independently on immutable snapshot08.
Source reference: run.rs supervised() status.code().unwrap_or(-1), invoke()
nonzero comparison, completion-plan01 section on expected CLI rejections.
