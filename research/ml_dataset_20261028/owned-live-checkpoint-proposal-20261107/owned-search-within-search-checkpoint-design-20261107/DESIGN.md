# Within-search owned capture checkpoints

## Existing interruption gap

original_boolean_search.rs continue_models invokes Session::next(), which synchronously runs a whole Boolean search and its audit. The only intermediate owning checkpoint is after Session::exclude succeeds. dynamic_boolean_continuation.rs next calls search::solve and installs p.search only after it returns. dynamic_boolean_search.rs solve similarly calls scheduler::run and installs node.native only after it returns. Therefore an expensive first node has no durable native rows until both calls return. Existing encoder already serializes partial continuation/search/native state, including batches, basis, queues, stage and cursors; capture publication already retains per-chunk acknowledgements and failure state.

## Minimal useful construction

Add internal callback variants next_with_checkpoint, solve_with_checkpoint and run_with_checkpoint, retaining existing APIs as no-op callback wrappers. Do not introduce from_attempt, resume, deserialization or a new owner constructor. The live function stacks remain the only controllers and retain their existing meters, limits, absolute deadline and task serial counter.

The first native hook goes immediately after the scheduler's move-only batch commit: after basis/pending append, completed/counters update, stage=Committed and active=None. Release the local mutable batch borrow before calling the hook. This is a coherent committed basis and captures progress inside the FIRST search node; simply checkpointing after solve returns would not address the main gap.

Optional finer hooks use the same interface after a complete symbolic-row transition and after a complete elimination-row transition, following cursor update. Those captures are diagnostics of an unfinished batch; only the already committed basis rows are eligible for the present semantic checker. Do not insert into the prepaid atomic commit or between polynomial/proof mutation and its associated retained metadata.

## Immutable overlay, without copying state

The child scheduler attempt is local until run returns, and the child search attempt is local until solve returns. Moving either into its parent early would require a larger state-machine refactor. Instead add a private borrowed encoder view identifying active partition, active node, borrowed search attempt and borrowed native attempt. Encoding substitutes ONLY these exact child references at the known active slots. Other state comes from the authentic immutable parent, and uses existing native/search/continued serialization functions. The wire schema stays unchanged.

At the search call site, avoid retaining a mutable Node reference while run_with_checkpoint executes: set node status Running first; borrow node equations immutably; the callback receives &native and borrows &search immutably. At the continuation call site do the equivalent: set partition Running, drop the mutable partition borrow, borrow its equations immutably, and capture &continuation in the callback. After the synchronous call finishes, install the returned native/search attempt exactly as today before propagating any error.

An EncodingView/ContinuationView private constructor must check that the designated partition/node equals the current active indices, corresponding status is Running and ordinary child slot is absent. It must not accept arbitrary indices or caller-supplied source polynomials. The owning source, map, normalized equations, records, sink and publication remain the existing split from capture_encoding::with_continuation_parts. No raw pointer or unsafe alias is necessary: callbacks receive immutable state plus the separately borrowed owner.

Before each hook, refresh visible charge fields from the live local meters. Use borrowed scalar charge overrides for outer attempts whose normal charge field is written only on return; do not mutate borrowed parents to make counters look current. Clarify in the view that publication cost remains in the original parent ledger, as with existing checkpoint callbacks, rather than silently adding it to kernel-work counters.

## Admission and failure

Each wrapper checks its same live meter immediately before and after calling the hook. Internally unwrap only the existing adapter's mutable owner borrow to pass the SAME original ledger to publication; no reservation is minted. Publication uses existing checkpoint_parts, original CaptureEmitter, original capture limits and original deadline. Both capture byte/trace reservations and nested resource checks remain active. The callback returns a sticky stop; retain its return value and committed acknowledgements before postcheck. On refusal, unwind normally and install every local native/search attempt in the parent before propagating failure. Never retry a failed publication or continue to another node after its stop.

Use a finite deterministic cadence (for example committed batch count interval) and finite maximum snapshots supplied by original policy. Start conservatively with a committed-batch hook; avoid checkpointing every arithmetic operation. Frequency is a policy extension and must not silently change frozen run configuration or grant. A due snapshot that cannot be admitted stops honestly. Do not promise a final snapshot after owner exhaustion or a killed process: only already acknowledged chunks survive, and incomplete last snapshots must remain incomplete under existing replay.

## Required controls

1. Actual first-node multi-batch fixture publishes at least one snapshot before solve returns; decoded node Running/native complete=false and positive basis rows are replayable.
2. No-op hooks preserve native ordering, rows, task serials and final output for original fixtures.
3. Same original owner charge includes publication; tight work/bytes/trace caps stop without resetting any meter or deadline.
4. Failure before chunk publication, after first chunk acknowledgement, and during postcheck preserves committed receipts, partial snapshot and child attempts, and prevents further work.
5. Expired deadline calls no sink. A post-publication expired deadline preserves acknowledgement but refuses continuation.
6. Incorrect overlay partition/node, preexisting child slot, or non-Running status refuses before encoding; no arbitrary detached state is accepted.
7. Snapshot encoder failure during symbolic/elimination state never exports staged matrix rows as checked committed basis.
8. Central replay accepts valid latest complete snapshot rows under current identity/descent/lineage gates; trailing partial snapshot cannot fall back to older success.

## Scope

This is source-only design. It saves bounded partial arithmetic search evidence for ML; it does not implement restart, rehydration, complete causal operation checking or an additional compute allowance. The next implementation should begin with the committed-batch hook and private borrowed overlay, followed by actual interruption fixtures. No published source, build or cloud state was changed.
