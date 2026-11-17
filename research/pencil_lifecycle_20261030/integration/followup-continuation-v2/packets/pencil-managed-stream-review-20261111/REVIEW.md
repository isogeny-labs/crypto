# Independent source review: managed stream integration

Reviewed at `2026-11-11T00:32:36Z` against repository head
`f3c1b8cc7663ca7d15374ae5e9a16c86839c9d18`.

## Outcome

The managed solver path is a concrete, source-bound integration. The final
overlay manifest is internally consistent, every original file matches the
bound repository head, and `integration.patch` passes `git apply -p1 --check`
against an exact private copy of those originals. The trace-disabled relation
path preserves the original collector insertion and answer logic; the traced
path emits the same donor Header, Candidate, Relation, and Terminal fields in
the donor order. Progress output remains nonfatal, trace output fails the
attempt without falling back, and the online interval is protected by an RAII
guard on every `Result` return after `begin_online`.

The retained-attempt exporter is not yet a fail-closed full-cohort admission
boundary. Two blockers remain before it can certify the advertised
1,260-target / 22,680-slot / 6,300-pair join.

## Findings

### B1 — The exporter counts rows but does not verify the prepared-plan slot bijection

`retained_attempt_join.rs:211-292` accepts seven caller-supplied 64-hex strings
as the plan set and then validates aggregate counts. It does not open or parse
the seven prepared plans. `AttemptJoin` has no plan sequence, warmup round,
measured round, or arm-role slot identity. Consequently it cannot prove:

- exactly 1,260 prepared workloads;
- 18 primary slots per workload, of which 3 are warmups and 15 measured;
- that each row's workload, method configuration, seed, arm, and measured flag
  equal its prepared-plan slot;
- that a measured triplet uses the same workload, plan, method/source/binary/
  protocol identity, and scheduled round.

The map at lines 253-283 checks only `pair_id -> (arm, algorithm_seed)`. Rows
from different workloads or plans with the same seed can therefore form a
passing triplet. Rows with `attempt > 0` are retained but, apart from global
identity uniqueness, are not required to map to an existing primary or retain
that primary's scientific identity. An arbitrary retry can pass validation and
cannot be distinguished from a retry substitution.

Minimal correction: make the exporter consume hash-bound prepared-plan files,
recompute their exact slot inventory, join every primary to one plan slot, join
each retry to its primary without allowing substitution, and recompute the
1,260 / 22,680 / 18,900 / 3,780 / 6,300 geometry from those slots. Pair checks
must compare the complete scheduled identity, not only arm and seed.

### B2 — The CLI trusts FilePins, eligibility, and raw costs supplied inside its request

`ecbench_retained_attempt_export.rs:22-62` deserializes `AttemptJoin` objects
and writes them after structural validation. It does not open, hash, or parse
any referenced context, ordinary record, relation stream/prefix, progress,
resource, process, controller, or cost file. `FilePin::checked_file` exists at
`retained_attempt_join.rs:38-69` but is not used by the CLI or cohort validator.
`raw_costs` is an unconstrained JSON `Value`; it may be null even while
`eligible_for_pair` is true. Complete versus prefix status is inferred from
which caller-supplied pin is non-null, rather than from a checked contiguous
stream ending in a valid Terminal.

A request with invented unique paths, syntactically valid digest strings,
`terminal_status=returned`, `eligible_for_pair=true`, `raw_costs=null`, and
synthetic record/resource pins can therefore pass. The exported file correctly
labels `owner_authentication_verified=false`, but it is not yet an independent
custody/completeness/cost validation.

Minimal correction: either accept only a hash-bound output from the existing
retained-session and custody validators with an exact schema and validator
receipt, or open and validate every pin in this native CLI. Recompute stream
completeness, terminal status, resource/process/controller consistency, raw
cost presence and types, and eligibility reasons before serializing the
cohort. Absent facts must stay null.

## Checks that passed at source scope

- The corrected `overlay-manifest.json` binds the final overlay and patch.
  The earlier overlapping manifest mismatch is preserved by
  `actual-byte-snapshot.json`, `overlay-manifest.pre-supersession.json`, and
  `overlay-manifest-supersession.json`; it is not silently rewritten.
- All seven original file hashes match both the packet and `git show` at the
  bound repository head. All ten final overlay hashes and the patch hash match
  the replacement manifest.
- `integration.patch` passes `git apply -p1 --check` against an exact private
  copy. It was not applied to the canonical worktree.
- `GraphSolver::relation` delegates to `relation_capture(..., None)`. In that
  branch the collector insert, cap failures, pivot lookup, scalar extraction,
  and return ordering are equivalent to the original body.
- The traced branch captures the submitted row, admission, post-insertion
  pending constraint, residual rank, and candidate scalar. The unmetered
  `diagnostic_entries` accessor preserves BTree order and avoids adding finite
  map-visit charges.
- Candidate dispositions and Header/Relation/Terminal fields match the bound
  donor sources. Rejected candidates have no Relation; verified candidates
  emit Candidate then Relation; Terminal follows the source scalar check.
- Progress `Sink::emit` records its own failure and returns `()`. Trace emit
  errors propagate as `relation_trace_failure`; there is no ordinary-solver
  fallback after trace dispatch.
- The online guard is created immediately after `begin_online`; every later
  `?` return drops it. Header failure occurs before online measurement begins.
- The relation writer is create-new, flushes and `sync_data`s each record,
  performs final file and directory sync, hashes the file, and cross-checks
  writer/Trace byte and record totals. A missing Terminal remains incomplete.
- The 40-hex lowercase Git revision is an explicit caller input in
  `DiagnosticLaunch.source_receipt.source_revision`; it is not derived from a
  SHA-256 source manifest. It is required only for managed pencil tracing.

## Accounting boundary

Finite arithmetic and semantic meters are unchanged in the trace-disabled
path. The traced path deliberately performs diagnostic allocations,
serialization, writes, flushes, and syncs; writer `write_ns` and `sync_ns` are
retained, but trace work is also inside the candidate wall/allocation envelope.
Therefore no wall-time or allocation parity is established by this source
review. Runtime parity, compilation, tests, payload execution, and scientific
ratios remain unchecked.

`managed-stream.patch` is the historical semantic sketch.
`integration.patch` is the applyable artifact reviewed here.

