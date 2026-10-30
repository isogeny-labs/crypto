# Clean dataset 01 terminal-result qualification

This is post-execution evidence qualification, dated 2026-10-30. It does not
change the frozen population, expected counts, test command, source, or raw
receipt, and does not rerun tests.

The original wrapper exited1 at coverage_check after Cargo exited0 and both
source/binary guards passed. Its assumption of exactly one libtest summary was
wrong: the existing FIFO safety test launches the same test executable with
`--exact ...fifo_replacement_is_rejected_without_waiting_for_writer --nocapture`
and requires that child's successful exit. The log therefore includes a child
1-pass summary before the parent 479-pass/2-ignored summary. This is visible in
the frozen `src/ml_dataset/telemetry/checked/tests.rs`, lines300–329.

Qualify only this retained command result after checking the original receipt,
exact raw-log SHA08aa2d5a61103280807326084d235d99521f65fa6477e73defa5719f9c06195c,
source/binary identities, successful Cargo exit, and both source guards. Require
exactly the known child and parent summaries in order, the481-test parent start,
the1-test child start, and the named child success. Preserve the original
wrapper exit1 separately from the qualified Cargo test outcome. Record source
evidence for the child spawn; no independent-review claim.

The general wrapper is corrected to distinguish this known nested summary from
the parent result. Future unexpected result counts still fail. Existing raw
receipts are never rewritten. Once this qualification passes, the next planned
phase is maintained clippy on the same unchanged source package; no test retry
is needed merely to repair the wrapper's summary-count assertion.
