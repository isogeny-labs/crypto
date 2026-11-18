# Ordered stream and attempt join

## Relation stream

The solver emits the existing `pencil-relation-stream/v1` JSONL records in this
order:

1. exactly one `Header`;
2. for each executed walk, either one `Candidate/DegreeSkip`, or one Candidate per
   enumerated lambda in enumeration order;
3. for every verified candidate, its `Candidate/Verified` immediately followed by
   exactly one `Relation` containing the exact submitted row, collector admission,
   pending constraint, residual rank, and candidate scalar;
4. exactly one `Terminal` only after the ordinary recovery check.

Rejected candidates have no relation record. A stream with no terminal is a durable
prefix and is never relabeled complete. Event sequence numbers are contiguous from
zero. The header binds source revision/hash, workload ID, run/attempt ID, p, r,
order, cofactor, seed, source polynomial, generator, target, x-to-column mapping,
small columns, denominator, and coefficient cap.

The managed progress stream remains `pencil-progress/v1`. Its I/O failure is stored
in `IoTotals.failure`; it does not alter the mathematical loop. Relation-trace I/O
failure returns `relation_trace_failure`, closes the online interval through the
guard, retains the durable prefix, and marks the attempt failed. The caller must not
retry under the same attempt identity.

## `ecbench.pencil-attempt-join/v1`

Required fields:

| field | meaning |
|---|---|
| `session_id`, `execution`, `attempt` | unique native attempt identity |
| `plan_sha256`, `pair_id`, `measured` | one of seven externally pinned plans and exact measured-pair membership; warmups have null pair ID |
| `workload_sha256`, `method_config_sha256`, `algorithm_seed` | exact scheduled scientific identity |
| `source_manifest_sha256`, `binary_sha256`, `protocol_sha256` | implementation and protocol pins |
| `context` | exact FilePin for the validated child envelope |
| `ordinary_record` | FilePin or null, never fabricated after timeout |
| `progress_files`, `progress_totals` | ordered diagnostic pins and writer totals/failure |
| `relation_stream` | complete FilePin only when Header and Terminal validate |
| `relation_prefix` | partial FilePin only when terminal is absent |
| `stream_error` | null or exact typed trace error |
| `process` | exit/signal/timeout/spawn observation; absent facts remain null |
| `resources` | resource-prefix and publication-tail pins; active worker fields may be null |
| `controller` | controller request/job/pod/attempt pins and charged allocation ledger |
| `cost_record` | raw finite-operation, calibration, wall/CPU/RSS, serialization, storage, retry, and infrastructure fields in their native units |
| `terminal_status` | `returned`, `solver_error`, `trace_error`, `timeout`, `signal`, `spawn_error`, or `controller_error` |
| `eligible_for_pair` | true only after every existing scientific and cost gate passes |
| `ineligibility_reasons` | complete ordered reasons; never used to remove enrollment |

At most one of `relation_stream` and `relation_prefix` may be non-null. A pencil
attempt normally has exactly one; both may be null only before trace creation or
after a proved solver-validation, spawn, or controller/create failure.
Non-pencil arms require both null. `eligible_for_pair=false` preserves the expected
pair and forces its reported ratio to null.

## Full-session cardinality

The exporter checks the prepared plan rather than trusting row counts. Across the
seven plans it requires 22,680 unique primary `(session, execution, attempt=0)`
identities, including 18,900 measured and 3,780 warmups. It requires exactly 6,300
measured pencil/rho pairs and 6,300 rho_AA/rho control pairs before any complete-
denominator ratio can be emitted. Each pinned plan contributes one primary session,
3,240 primary executions, and 2,700 measured executions. Retry rows are charged and
retained but do not replace primary identities.
