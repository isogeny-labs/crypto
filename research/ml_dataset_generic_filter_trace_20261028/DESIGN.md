# Generic dataset v2 bounded filter integration

Frozen before implementation, 2026-10-28, based on native source
`52ded7eb1e42ab97c2d23cb6a4354e985ddd8540`. Conductor T-755 reserves
`src/ml_dataset/relations_generic.rs`, the necessary `relations.rs` dispatch,
`mod.rs` cell-verification hooks, `export.rs` schema gates, and this research
directory. The new worktree is `crypto-ml-generic-filter-trace-20261028`.
Root retains publication, session-log and Cargo coordination. No changes to
`solvers.rs`, native filter code, existing branches/test copies, Koblitz
Frobenius reduction, cloud jobs, historical datasets, or budget/deadlines.

## Prior work and scope

This implements the Versioned dataset integration section of
`../ml_dataset_large_prime_trace_20261028/PROGRAM_DESIGN.md`, using the actual
native API/checker now present at the base commit. Its SOURCE_MAP.md records
primary readings: Cavallar, CWI MAS-R0012 sections 2.1–2.2,
https://ir.cwi.nl/pub/4456/04456D.pdf, and official CADO-NFS merge history source,
https://raw.githubusercontent.com/cado-nfs/cado-nfs/master/filter/merge.cpp.
Those sources motivate singleton filtering and replayable merging; their NFS
setting does not certify this odd-prime ECDLP code. Existing source replay,
canonical public/domain validation, determined-column recovery, and the native
separate-arithmetic checker are reused without a new algorithm or cost claim.

The existing generic v1 strict replayer and Koblitz v1 path remain intact.
Only new generic solves emit `ml-ecdlp-generic-large-prime-transcript/v2`.
Generic identity/sign collection retains m=2..15 and two residual summands;
this is not higher-m polynomial root support. All unsupported states remain
explicit. No experiment or scientific sample is authorized by this increment.

## Contract and public binding

V2 retains the full original public instance, domains, pair table, attempts,
requested work and derived collection limits. It adds the full CaseConfig,
canonical hashes of public instance, actual factor base, config, signed small
and residual domains/pair table, a versioned trace policy, typed filter batches,
aggregate trace accounting and explicit false completeness/cost flags.
Hashing uses compact JSON with recursively sorted object keys, not source
whitespace or consumer insertion order. Config seeds/indices remain restricted
provenance. Sealed-cell replay compares the full saved config to the frozen
manifest and factor-base recipe. Query coefficients/indices are additionally
replayed from the same frozen RNG stream for v2.

Every scheduled batch binds its index, through_attempts, source_count, ordered
cumulative accepted-source hash and public/config/domain identities. Each
accepted source remains tied to its exact query sequence and checked equation
`aG+bQ = sum(small points)+sum(residual points)`, including target coefficient
-b and RHS a. This binding wraps all native events and the terminal run.

Completed batches contain the original legacy result/report/rank snapshot
and the native run. They require acknowledged native Finish, independent
native checker acceptance, equality of completed rows/report with the existing
untraced filter, independent group replay, and the original modular rank and
determined-target scalar checks. Native source cancellation alone is never
scalar recovery. Replayer checks legacy equality as well as recorded schedule.

Censored batches contain the native prefix and exact scheduled source/attempt
binding, with no rank or recovery snapshot. Only the final batch may be
censored, no later queries or batches may exist, and recovered scalar/log fields
must be absent. Refusal at Finish is censored even if individual complete rows
were already acknowledged. Native invalid-input errors and SinkFailure are
errors, not successful sealed censored records. Input/State/Work/Event/Byte
limits may produce verified censored prefixes, never UNSAT or no-relation
claims. Empty declared domains remain distinct from trace censoring.

## Aggregate admission and persistence

The fixed production policy uses native default hard ceilings: 4096 raw rows,
1000000 raw terms, 65536 per-state terms, 100000 attempted events, 16777216
acknowledged compact-native-event bytes and 1000000 computed logical work.
Row/input/state ceilings are per batch. Event/work/byte allowances are shared
across every cumulative refilter; each call receives only the remaining
allowance. Original collection work_limit/query/pair derivations are unchanged;
filter logical units are a separate bounded software admission budget.

Track acknowledged events/work/bytes and separately attempted events,
computed work, inverse calls, and refused-event bytes. A byte-refused candidate
is computed but never admitted; its serialized size may exceed the remaining
acknowledged-byte allowance and is reported separately. It ends collection,
so it cannot reset or evade the aggregate cap. Native row/state admission and
the existing whole-capture limits bound that candidate and wrapper overhead.
All accounting sums use checked arithmetic. Zero remaining allowance still
produces a precisely checked terminal prefix on the next scheduled call.

Before native work, write a bound batch-start event. Stream each native event
through the existing live sink with batch/public/config/domain/source binding;
acknowledgment precedes native state mutation. Save the typed completed or
censored batch payload under a unique fixed batch-index path, using restricted
role `large-prime-filter-certificates/v1`, before collection proceeds. The final
v2 transcript retains the same exact batch objects; cell verification requires
one matching payload for each batch and rejects role/path/content mismatches,
missing or additional batch payloads. Native events include normalized rows,
pivots and reduction candidates, so acknowledged prefixes survive interruption
via the existing capture mechanism. These are artifacts, not memory checkpoints
or permission to rerun a scientific job.

The existing outer bound capture continues to count wrapper bytes, payloads,
newline/envelopes and other solver events. No outer cap is raised or bypassed.
Any live event/payload I/O failure is propagated as worker failure, including
native callback failure; it must not become a successful sealed transcript.
Already persisted prefixes remain available to the supervisor.

## Cell and export hooks

`relations.rs` dispatches generic v1 and v2 explicitly and exposes helpers for
v2 config/outcome/payload consistency without changing Koblitz v1 replay.
`mod.rs` recognizes v2 requested-work encoding and binds full config, public
and base identities, scalar/censor outcome, aggregate counters, all typed
batch payloads, and false claim flags. Lightweight consistency checks also run
in hash-only mode; independent mathematical replay remains the replay gate.
V1 acceptance and unresolved-history strictness remain unchanged.

`export.rs` accepts v2 only through the same existing generic transcript path,
role and checked signed-domain route. The static feature projection remains
byte-equivalent for equivalent public domains; no query seed/coefficient,
source row, pivot, RREF, recovery, trace count, stop reason or certificate enters
features. Batch payloads stay restricted target/provenance evidence. Existing
missing-transcript/domain behavior remains excluded-from-features, and a target
PDP certificate cannot label a relation-query domain. Consumer support for v2
is a later explicitly validated dependency, not implied by producer acceptance.

Keep `trace_complete`, `execution_trace_complete_verified`,
`total_operation_count_verified`, `dependency_basis_complete_verified` false.
`recorded_row_transitions_verified` can be true only after the independent
native checker accepts every supplied complete/censored run. Native work scope
excludes hashing, serialization, validation, map operations, group/rank work
and arithmetic internals; this is not full field-operation or CPU cost.

## Software validation and delivery

| Requirement | Planned validation |
|---|---|
| Legacy replay | Retain v1 and Koblitz tests; freeze a v1-shaped transcript from unchanged legacy helpers and require original truncation rejection |
| Public correctness | Generic prime/binary, both twists, m2/m3, odd/even fields; full source equation and target replay without private witness |
| Aggregate bounds | Two or more cumulative batches with reduced test allowances; exact remaining policy, no cap reset, zero remainder, refused Finish, no partial-batch rank/recovery |
| Independent trace | Input/source/order/batch/hash/counter/cap/event tampering rejected by separate native checker and adapter binding |
| Lifecycle | Missing/extra/reordered batch, prematurely truncated queries, forged completion, post-censor work and wrong frozen config rejected |
| Persistence | Live native events and batch payloads, induced callback/payload failure propagates error; previously acknowledged prefix retained |
| Cell/export | Rehashed outcome/config/role/path/payload tampering rejected; censored labels remain false; static features contain no new trace data; v1 routes unchanged |
| Delivery | Scoped formatting, targeted checks, strict lint, immutable full-library gate coordinated with root, review and canonical Git/S3 evidence |

Design commit precedes implementation. No Cargo invocation until root grants
the centralized slot. Native 52ded7 full gate remains root-owned and may still
be running; this work does not modify it. Original USD100–200 total, maximum
USD200, existing jobs/owners and original deadlines remain unchanged.
