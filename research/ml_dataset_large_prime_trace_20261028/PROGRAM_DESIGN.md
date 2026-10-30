# Bounded sparse-filter row-operation capture

Status: proposal for root review; no source edits, builds, trials or cloud
execution. Prepared2026-10-28 against producer
`d79f658f7a0a83e2f75f4cb6f0d1f7542d0e6b83`.

## Recommended first increment

Add optional bounded capture to `large_prime_filter::filter_large_primes`
and a separate replay checker, then wire only the generic dataset relation
adapter. Preserve the existing API, result ordering, error behavior and all
legacy report counters when tracing is absent or finishes within its bounds.
Keep the signed-Frobenius reducer, modular rank reducer and ecbench callers
unchanged. The `SOURCE_MAP.md` describes those separate paths precisely.

Expose all selected row-level transitions for the admitted prefix, including
discarded inputs, peeled rows and rejected merges. Do not advertise every
scalar arithmetic instruction, queue probe, allocation or CPU operation. This
increment does not implement an incremental reusable filter: the dataset
still calls the batch algorithm repeatedly on cumulative accepted inputs.

## Minimal native API

Proposed types, not implemented signatures:

```rust
pub fn filter_large_primes_with_trace(
    relations: Vec<PartialRelation>,
    options: &LargePrimeFilterOptions,
    limits: FilterTraceLimits,
    sink: &mut dyn FnMut(&FilterEvent) -> Result<(), String>,
) -> Result<FilterRun, LargePrimeFilterError>;

pub fn verify_filter_trace(
    inputs: &[PartialRelation],
    options: &LargePrimeFilterOptions,
    run: &FilterRun,
) -> Result<(), String>;
```

The existing `filter_large_primes` delegates to a shared internal core with
tracing disabled. Do not expose private mutable `WorkRow` or pivot maps to
callbacks. Use a strict owned `RowSnapshot {small, large, rhs, sources}` with
canonical sorted unique nonzero terms; all referenced rows have stable IDs
and monotonically increasing state versions. Input identity is source ID;
a pivot reference is `(large_column, producer_row_id, row_version)`.

`FilterRun` should contain schema, raw-input/options/limits binding, stop,
acknowledged events, at most one attempted unacknowledged event, separately
scoped trace counters, accepted complete-row prefix and report snapshot.
Only `Completed` plus an acknowledged terminal event supplies a final legacy
report and completed batch. A partial snapshot must be labeled partial; do
not present its `complete_relations` or core counts as final native totals.

The old error variants remain errors, including duplicate IDs and
noninvertible pivots. Already emitted prefixes survive an error through the
live sink. Invalid inputs do not become censored mathematical certificates.
Legacy modulus admission remains unchanged; generic dataset replay separately
proves its exact public subgroup is prime. No stronger native primality claim.

## Event vocabulary and exact insertion sites

| Event | Required content and validation |
|---|---|
| `Begin` | Input digest/count, options and limits, counter/event scopes, version; bind raw ordered input bytes, not just normalized rows |
| `InputNormalized` | Input position/source ID and normalized row; checker independently reduces coefficients, merges duplicate columns and drops zeros |
| `DuplicateDropped` | Dropped ID and first retained representative ID; equations exactly equal excluding IDs |
| `Peel` | Degree-one large column, active removed row, affected columns and degree before/after values; checker reconstructs incident active rows |
| `CoreReady` | Surviving ordered row IDs and static column degrees; verify exact sort key and no remaining singleton |
| `RowBegin` | Current core row with initial source coefficient1 and row version |
| `PivotSelected` | Existing/new pivot reason, column, static degree, row weight, score, selected coefficient, stored pivot reference; verify ties from sorted row terms |
| `Reduction` | Before row, pivot reference, inverse, scale and complete candidate after-row; verify inverse*coefficient=1 and every component of row+scale*pivot, including RHS and sources |
| `FillRejected` | Candidate widths and violated original option(s); preserve old pivot table and drop this current row, then continue |
| `PivotStored` | Full raw pivot snapshot and selected column; do not normalize it to1, since the current batch algorithm does not |
| `CompleteRow` | Final row, empty large columns and complete source combination; report cycle only when sources.len()>1 |
| `Finish` | Exact returned row order, final unchanged legacy report and trace totals; acknowledge before declaring completed |

Originally smooth input rows produce `CompleteRow` in their existing order
before core elimination. Record the actual peel order; initial HashMap queue
order is not deterministic. The checker accepts any valid recorded degree-one
sequence yielding the recorded exact two-core; it must not rerun randomized
queue iteration and compare event bytes. No sorting change is needed.

Static core degrees remain static. Keep the existing score and tie behavior;
do not silently substitute a different Markowitz heuristic. Only fill
rejections count toward legacy `rejected_fill`; trace limits have separate
stop reasons and counters. Failed tentative merges are visible even though
they are excluded from legacy successful `merges`.

## Atomic steps, limits and sink failure

Use bounded atomic row transitions. Before computing a transition, check event
admission and a conservative logical-work reservation derived from operand
sizes. Candidate construction is bounded independently by hard row/input
limits. After constructing the candidate, compute exact serialized event size
and call the sink before committing its public state/report changes. Count
computed-but-unacknowledged work separately. On acknowledgement, commit the
state and the same legacy counters as the untraced algorithm. On failure,
return a censored prefix immediately and never call that native sink again.

Proposed first-version hard ceilings, subject to root review before coding:
4096 input rows, 1,000,000 total input terms, 65,536 terms in any captured or
tentative row, 100,000 attempted events, 16MiB compact native event bytes and
1,000,000 logical coefficient/selection visits. They are resource admission
limits, not measured optimal settings. Check size arithmetic with checked
addition and reject oversized requests before cloning event state. The legacy
untraced API does not acquire these new trace caps. Generic dataset inputs are
already limited to4096 queries and32 residual columns; keep their fill4096
rules, point/query caps and original work-limit interpretation intact.

The trace-work counter can count normalization input-term visits, pivot
candidate visits, copied row terms and updated right-hand terms, with inverse
calls informational. Define each charge at its actual loop or atomic-step
boundary; do not call this field-operation cost. Extended-Euclid internals,
BTreeMap comparisons, allocations, hashing, serialization, validation,
independent replay, curve arithmetic and rank reduction are excluded. A
logical-work reservation may stop before a transition that would use fewer
actual visits; report a reservation-bound stop, not an exact exhaustion claim.

`Stop` distinguishes Completed, InputLimit, StateLimit, WorkLimit, EventLimit,
ByteLimit and SinkFailure. Native counts/byte totals are per-run and the
adapter passes the remaining **aggregate** allowance across repeated filter
batches. Never reset to a fresh full allowance for every cumulative refilter.
JSON byte scope must say whether envelopes/newlines are included; use the
binary-root precedent of compact native envelopes and separately retain the
outer capture layer's whole-stream limits.

Legacy fill rejection is not a trace stop: drop that row and continue as
before. A trace stop ends the batch and outer collection immediately. Preserve
already acknowledged complete rows as individually replayable observations,
but initially do not run rank/recovery on an unfinished batch. Failure at
`Finish` also leaves it censored. This conservative rule avoids inventing
partial-batch recovery semantics in the first increment.

Native sink failure returns a stopped report where possible. A dataset disk or
capture I/O error propagates to worker failure/censored-prefix handling rather
than attempting to seal a successful transcript through the same failing
sink. Independent failure provenance can be written by the supervisor. No
sink failure, empty filtered row set, peeled graph, rejected fill or low rank
is an UNSAT/no-solution certificate.

## Independent mathematical and lifecycle checks

Implement checker arithmetic separately from `WorkRow::axpy` and
`replay_complete_relation`; the latter shares producer code. A simple bounded
BTreeMap/vector coefficient accumulator using u128 modular operations is
sufficient. Validate modulus/ranges, unique source IDs, canonical rows,
input digest/order and event sequence before any lookup/arithmetic.

For every captured state, independently recompute

`state = sum(source_coefficient[id] * normalized_input[id]) mod r`

across small columns, large columns and RHS. Require exact equality, not just
the same group point. For a completed row require all large coefficients zero.
Also check each individual row reduction, stored-pivot reference, version,
fill decision, degree-one peel and terminal counter reduction. Reordered or
missing required state transitions cannot silently become a complete run.
Allow no events after a terminal stop. Verify an unacknowledged candidate only
as an attempted transition; never expose it as a committed pivot/complete row.

Dataset verification additionally binds:

1. Full canonical public instance, actual factor base and CaseConfig hashes,
   with explicit family/n/m/twist, field model/modulus, G/Q and subgroup order.
2. Signed small and residual domains, column mapping, actual pair table,
   public query coefficients and m-2 selected small-point indices.
3. Each source ID to the accepted attempt at that sequence. Recheck
   `aG+bQ = sum(small points)+sum(residual points)` and the exact row with
   target coefficient `-b` and RHS `a`. Bind batch index, `through_attempts`,
   source count and the full ordered cumulative source prefix.
4. Completed source combinations and their independent group equations before
   existing rank/recovery handling. Preserve the determined-target requirement
   and final `scalar*G == Q` replay; source-cancellation proof alone is not a
   scalar recovery. Free-column particular values remain unusable labels.

The portable checker proves recorded row identities and accepted source
combinations. It does not prove all potential relations were collected, every
instruction was logged, every operation counted, or every possible dependency
found. Explicit flags remain false for `execution_trace_complete_verified`,
`total_operation_count_verified` and `dependency_basis_complete_verified`.
A narrower `recorded_row_transitions_verified` flag may be true only after
that checker passes. Preserve the existing overall `trace_complete=false`.

## Versioned dataset integration

Do not loosen generic v1 replay's full unresolved query-history requirement.
Introduce explicit generic transcript v2, with exact original public/domain/
attempt fields plus bound trace policy, aggregate counters and typed batches:

- `Completed { legacy_batch, trace_reference }`, where the legacy result and
  report must equal the untraced result and the traced Finish is acknowledged.
- `Censored { through_attempts, source_count, native_prefix, stop }`, allowed
  only for the final scheduled batch, with a provable limit/failure reason.

Scheduled filtering remains every32 accepted sources or the final permitted
attempt. Replay verifies all previous complete batches, exactly when the last
batch began, and why later attempts are absent. Unjustified truncation remains
an error. V1 and admitted Koblitz v1 replay remain unchanged.

Change points are `relations_generic::{batch,solve,verify}`, explicit schema
dispatch in `relations::verify_transcript`, requested-work extraction and
outcome consistency in `ml_dataset::verify_cell`, and the schema/role/path
allowlists in `export::{large_prime_feature_projection,
large_prime_cell_projection}`. Censored v2 keeps scalar/log recovery fields
empty, solver_completed=false, status=budget_exhausted with the precise trace
reason. I/O-failed unsealed prefixes use the existing worker failure path.

Versioned trace artifacts should be restricted solver evidence, for example
`native-large-prime-filter-events`, `native-large-prime-filter-state` and
`large-prime-filter-certificates/v1`. Save `Begin` before native work; stream
events; save completed row/pivot or periodic bounded state payloads before
proceeding. Each payload references public/config/domain/source-prefix hashes,
batch index and acknowledged event range. Artifacts are not process-memory
checkpoints or safe retry/resume tokens. Replaying a prefix for validation is
not authorization to repeat a campaign job.

## Restricted artifacts and model-input boundary

Keep attempt coefficients, residual discoveries, pivots, row operations,
source combinations, rank/RREF, recovered logs/scalars, counters and stop
reasons out of static feature rows. They are solver-dependent or answer-bearing
training targets/provenance even though their algebra starts from public data.
Membership diagnostic logs remain in their existing separately restricted
artifact and are never supplied to the solver.

Only the existing checked static curve/G/Q, exact domain and declared action
projection may enter features. Add certificate references to restricted target
rows and hashes to provenance. A new v2 censored transcript needs explicit
feature-projection validation; missing/partial provenance must remain marked
unavailable, never fall back to a generic unlabeled domain. Test path/schema/
role mismatches and forbid using a target-PDP certificate as a relation-query
domain label.

## Meaningful tests and delivery sequence

1. Native equivalence: all five existing graph/hypergraph tests plus
   hand-constructed normalization/duplicate-ID/duplicate-equation, smooth-row,
   cascading-peel, arbitrary signed coefficients, tie scores, repeated pivots,
   zero rows and both fill guards. With ample limits, traced outputs **and every
   legacy report field** must equal the old call. Include ecbench orbit callers
   in the later parent library gate. Composite-modulus errors remain compatible.
2. Independent arithmetic: construct bounded matrices over several small odd
   primes and independently recompute every emitted source combination. With
   unlimited fill and bounded exhaustive small examples, compare projected
   row spans/dimensions against a separate dense reference, not just two calls
   through the same producer. Label this software validation only.
3. Every boundary: cap before each recorded event and fail its sink, including
   normalization, peel, reduction, rejected fill, pivot store, CompleteRow and
   Finish. Require no later native callback, no unacknowledged row publication,
   retained earlier bytes, exact charged executed prefix, and no false recovery.
   Test byte cap one byte below/exact/above, input/state admission, overflow and
   aggregate exhaustion across two cumulative batches.
4. Tampering: input/order/ID/options/modulus, duplicate representative, peel
   degree, core order/static degrees, pivot choice/score/inverse/scale, row
   values/RHS/source coefficients, stale pivot version, rejected-fill action,
   counters, event omission/reordering, forged terminal/cap and post-stop events.
5. Public adapter fixtures: generic binary/prime and applicable twists, odd/even
   fields, arity2 and at least3 to verify query encoding. Cover no relation,
   unsuccessful/capped batches, a source-verified complete relation, a verified
   target recovery and rank deficiency with an undetermined target. Check
   legacy v1 records still replay and untruncated v2 results match v1.
6. Sealing/export/live capture: rehash tampered manifests, outcome flags,
   certificate roles, config/work limit, source-prefix/domain mappings and
   trace limits; assert replay rejects them and model features contain no
   new trace data. Durable partial records survive an induced sink failure.
7. Independent consumer support and source-bound executable checks, then parent
   full-library/strict-lint/publication gates. Use existing software fixtures;
   no cloud scientific launch, new spend authorization or change to frozen
   historical datasets follows from this design.

Recommended commits after root review: native event API/checker/tests first;
generic v2 adapter and outcome/export binding second; independent consumer and
durability tests third. Signed-Frobenius and modular-rank operation capture
remain explicit later increments. The original USD100–200 total authorization,
maximumUSD200, and all existing external-job ownership/deadlines are unchanged.
