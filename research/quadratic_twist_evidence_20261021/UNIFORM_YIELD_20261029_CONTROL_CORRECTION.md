# Pre-third-run actual-row control correction — 2026-10-29

At frozen42b72c3a2, the corrected independent audit verifies all96arms,
896source recoveries,210comparisons and536reachable states, with joint counts0/0.
The full runner nevertheless exits2 before controls complete: false-row sets
the first query's already-empty row set to empty, so the strict changed-payload
guard correctly rejects this no-op. Preserve attempt-02 and its positive
receipt; it is not a completed replay/control/backend contract.

Before the next execution, new native/uniform-yield-audit-v3 preserves both
old sources. The false-row control now finds an actually nonempty query and
removes one real row, asserting that removal occurred. No arithmetic, state,
rank, degree, source recovery, comparison or acceptance check is weakened.
Two full-JSON equality assertions use the identical equality predicates with
short diagnostics instead of printing entire payloads on rejection. Exact
payloads themselves remain retained. Other mutations and all13 control classes
are unchanged; missing-failed-query is applicable in the positive receipt.

New run-uniform-yield-v3.sh changes only its versioned runner/auditor/addendum
bindings. Compile and commit these sources before a fresh FULL96/896/210 rerun.
Require all97 producer case files to match both earlier executions byte for
byte; require independent positive replay, every rehashed rejection/no receipt,
backend tests and source checks. Do not accept the positive audit alone as the
full contract. Keep the original iid sampler, complete policies/losses,
600-second command caps/3600-second iteration envelope/two workers, actual
source reductions and all broader proof/registration/cost gates.
