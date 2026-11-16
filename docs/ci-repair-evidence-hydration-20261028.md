# Repair evidence hydration and historical report verification

The current Jacobian integration exposed three independent CI failures. The
failed job logs and all validation output are retained in
`reports/ci-hydration-repair-20261028/`, with raw SHA-256 hashes. These changes
repair software validation and preserve the research evidence.

## Hydrate the actual discovered sessions

The ecbench harness and sixteen audit shards discover every committed session
by its schema, including sessions outside the original static checkout globs.
Their Git checkouts contain artifact pointers but not the referenced data.
Job 122570994817 consequently fails on missing records and plans, rather
than reporting a mathematical or integrity discrepancy.

Both jobs now check out the canonical artifact tool and configuration, then
hydrate every directory in the discovered session list before any integrity
or replay checks. The existing tool verifies the stored digests and sizes.
The actual discovery and pull shell commands were executed in an independent
temporary Git repository using a local content-addressed store. Both jobs
hydrate two sessions outside the old static globs and reject a corrupted
object with exit 2. The existing 28 artifact-tool controls also pass.

## Verify the preserved historical report at its own path

Main now retains a newer report revision with lock digest `9a93e422...`,
while the immutable R0 checker correctly requires `5385e70d...`. Checking
the newer revision as R0 made job 122560011181 fail before reproduction.
The existing R0 lock already lives under
`docs/backend-leaderboard/revisions/20261027-pre-method-registry/`.

Complete that archive with all five byte-exact original outputs. Its JSON
keeps the existing content-addressed object; the remaining outputs reuse
their original Git blobs. The checker reads the archived lock and outputs,
still verifies current scientific session inputs against the original lock,
and reproduces the report at snapshot
`91424ecf8cc7db04e7c4a2805c9001a02a6106ca`. The full native reproduction
passes, with `solver_executed: false` and `timing_reproduction: not_tested`.
All seven refusal/positive controls pass, including substitution of the newer
report for the archived report. Neither lock nor the current report is resealed.

## Repair the maintained example lint

Job 122560121805 rejects the manual lowest-set-bit expression in
`ic_next10_03_pkm_degree.rs`. Use the equivalent stable
`u64::isolate_lowest_one` operation. Rust 1.98.0 Clippy for that actual example
passes with `-D warnings`. This check compiles the instrument without running
a research experiment. Initial sparse-checkout omissions were restored from
the exact tracked source and compile-time input blobs before validation.

Actionlint with ShellCheck, artifact workflow wiring and whitespace checks
pass. The initial corruption control reused a valid shared cache and did not
exercise the corrupted object; the final control uses a separate empty cache
and observes the required refusal. Current-head hosted checks and review
remain required. No audit, record, threshold or scientific status is skipped
or promoted, and no new experiment or performance claim is introduced.
