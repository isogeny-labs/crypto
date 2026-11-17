# Observed progress and failure flow

The matched producer previously emitted raw counts only after all arms finished. The new main.rs reuses the identical generated arms and meter but adds stderr progress markers outside counter scopes: arm_started, query_started, query_verified, arm_finished after teardown, and panel_finished after complete output. Existing arithmetic, fixtures, forward checks and A/A equality checks are retained. The additional output is harness overhead, excluded from helper counters; it is not wall-time neutral.

The planned three-arm20-target schedule has127 markers. failure_flow.py accepts only an exact prefix, checks marker field types and ordering, and distinguishes started queries, observed verified returns, active/incomplete calls, targets not yet started, and completed arm teardown. It keeps partial final-marker status. It never asserts a complete measurement. A verified-return marker follows recovery and public-point assertions; it does not imply the entire panel or fixed-cost accounting completed.

The CLI checks captured raw/stderr hashes, protocol/binary bindings and the pinned progress-enabled release digest. It refuses successful processes, which belong to the strict success exporter. It exclusively writes failure-flow.json with schema monomial.failure-flow/v1. The receipt and marker hashes remain producer evidence, not independent signatures. If the recorder/host dies before writing a process receipt, reconcile actual process/Job state; this parser does not infer terminal status from a partial file.

## Canonical export boundary

The preserved experiment.export/v1 schema requires attempts>=1 for every row; its linter also expects all60 planned target/arm rows. It cannot represent a genuinely unstarted row honestly. This module deliberately retains a separate failure-flow record. No zero-work cell is relabeled as attempted, and no incomplete accounting is promoted to a successful comparison. Canonical handling of unstarted cells remains an integration task; schema-compatible fabricated rows are not a solution.

## Checks

One selected native correctness test passes;414 inherited tests filtered. The actual two-target development trace has18 markers, six started/verified queries and three completed arm teardowns, with no full-panel-finished marker. Four synthetic methods test all128 prefix lengths, partial final markers, reordered/duplicate/wrong-arm events and receipt mismatches. The release binary compiles; describe mode executes no kernel. No full20-target comparison was run.

Run-record.py overwrites evidence: replay only in a scratch copy. failure_flow.py requires an actual failed/incomplete attempt directory plus expected protocol and binary hashes. The matched/capture draft still pins the earlier binary: amend and bind this new release before freeze or submission. Preflight and authoritative remaining budget/live capacity are outstanding. Original4563-case cohort unchanged. No cloud action.

Sources: matched producer, bounded capture wrapper and preserved export schema/linter. No new mathematical or performance claim.
