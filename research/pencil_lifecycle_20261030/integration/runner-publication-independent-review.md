# Independent source review of runner publication v2 proposal

Read-only 2026-10-30. Reviewed private patch190c91eb1fdfde5328278912d7fd9c129f3d0b57228d98b92df3ae6144e9a2d6 and runner-v2.rs bdecb504eca0edfb0fd7572cfb55d3063c67d7b0ea52c0419eafd2202ea2b724. No application, compilation, fixture execution, solver or cloud activity. Child output fields are a dependency of the separate dispatch patch.

## Required correction: successful function return is not successful reap

The proposal uses child_result.is_ok() both to record ChildReaped and to authorize sidecar reads. Existing Unix run_child only sets Reaped.exit when wait4 returns the expected pid; wait4 failure still reaches Ok(Reaped) with exit=None. Therefore the new gate can assert reap without an observed successful wait. Require an explicit successful-reap observation, or a carefully documented exit.is_some() predicate under the current contract, for both the mark and read gate. On absent reap retain the partial prefix and do not inspect sidecars as quiescent. A fake Ok(Reaped{exit:None,...}) fixture should cover this exact case without spawning a process.

## Capacity and cumulative publication

The proposed journal stores every prior tail and writes the whole pretty JSON after every execution. A minimal valid compact tail occupies560bytes: four64-character SHA256 fields, one-character nonempty IDs, and all numeric fields0. For22680slots one final journal therefore requires at least12,700,800bytes before array separators/header, exceeding8MiB even before pretty formatting. Repeated snapshots contain n(n+1)/2=257,202,540 tail occurrences and at least144,033,422,400bytes of tail serialization for that single-session size. These are static schema arithmetic bounds, not observed IO/runtime/RSS.

For partitioned sessions the formula applies separately, sum_i n_i(n_i+1)/2; do not assume the full22680slots are necessarily one session. Nevertheless capacity and repeated-snapshot overhead must be explicitly admitted per planned session. A bounded linked-chunk journal or immutable per-execution tail plus final index can avoid quadratic copying, but requires separately reviewed custody semantics. The current cap fails closed and retains older files; it is not full-cohort capacity readiness.

## Other inspected behavior and limits

The spawn callback occurs after successful spawn. Absent diagnostics uses the original run_child wrapper. No timeout argument changes. ResourceTracking freezes contiguous advancement after failed observation; later existing record/attachment bytes remain retained even when their mark cannot be represented. Exact record hash, full workload/method JSON hashes, session/sequence/seed/binary comparisons bind v2 ingestion. Missing/invalid sidecars are explicit diagnostic errors, not mathematical results or zero costs.

File publication uses exclusive temp creation, file sync, exclusive hard-link publication and directory sync. A later sync failure leaves files without claiming durability. Session snapshot bytes are immutable per execution. Final resource journal publication has no external closure and cannot establish active-worker totals. Fatal IO/process failure may prevent final prefix publication; retained prior evidence is the limit.

The clock begins at execute_session, excluding earlier process startup, lock/spec/plan/host setup. Per-execution envelope begins after launch.document parsing in the resource-tracker initializer; that tiny setup is outside the execution prefix but within the session clock. Neither is complete cold process cost. Resource status Returned describes complete resource stages, not a completed mathematical solve.

Compile review: missing Value import is fixed in v2. Closure captures and fallible stage/publication paths appear plausible by source inspection, not verified by a compiler. A later build must also account for separate child output field/literal changes. Existing fake fixtures cover publication and missing spawn but not the concrete failed-wait case or integrated per-stage IO failures.

No full-cost, timing, scientific execution or launch qualification is granted by this review.
