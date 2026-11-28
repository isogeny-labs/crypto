# I1/I2 implementation candidate

## Fixed scope

This packet implements the next source seam without changing the scientific
denominator: 1,260 targets, 22,680 execution slots, 18,900 measured executions,
3,780 warmups, 6,300 measured pencil/rho pairs, and all 36 pricing cells. A failed,
capped, interrupted, or trace-incomplete attempt remains enrolled and produces a
null affected ratio. Historical records are not substituted for current attempts.

## I1: managed solver trace

`src/cryptanalysis/ecbench/pencil_progress_native.rs` remains the algorithmic body
used by managed custody. The earlier `managed-stream.patch` is preserved as the
semantic design. `original/`, `overlay/`, and `integration.patch` now provide the
exact source bytes and an applyable unified diff without touching the root files:

1. the exact `pencil-relation-stream/v1` header, candidate, relation, and terminal
   observations at the same mathematical boundaries as the donor;
2. `GraphSolver::relation_capture`, where the ordinary `relation` path delegates to
   the same insertion implementation;
3. a `solve_one_with_progress_and_trace` entry point; progress-only delegates with
   `None`, so its arithmetic path is unchanged;
4. an online-measurement Drop guard so a trace error closes the existing interval;
5. explicit `relation_trace_failure`, with no fallback solve;
6. root adapter plumbing that accepts both the existing progress sink and a caller-
   supplied single-use trace.

The pending-constraint snapshot uses a new output-only `TrackedMap` accessor with
the same unmetered semantics as its existing serde implementation. This avoids
changing the managed finite-operation ledger merely because tracing is enabled.
Its allocation, formatting, write, flush, and sync work remains observer cost and
must be retained separately; wall parity is not asserted by this source candidate.

The patch deliberately does not silently call `native_solver::solve_one_with_trace`.
That would change the managed body and discard its progress milestones.

## I2: retained attempt/cost export

The exporter composition starts from the checked native APIs and the existing
source-built `retained_export.rs.txt`:

- `FreshOwnerPins` and `close_fresh_enclosing_evidence` check owner/resource joins;
- the actual-session file adapter rechecks typed host/session/preparation/launch and
  child-context joins;
- caller custody remains separate and `owner_authentication_verified` remains false;
- unknown worker-allocation, active-worker, and exact-exit values remain null.

`STREAM_SCHEMA.md` and `validation-contract.json` add the per-attempt join required
for the cohort. Every expected execution sequence has exactly one primary attempt
row and zero or more separately identified retries. Each row pins its context,
ordinary record if present, progress prefix/final totals, relation stream or durable
partial stream, resource prefix/tail, process/controller receipt, and raw cost
ledger. The exporter never fills an absent value with zero and never treats a
partial relation stream as a terminal stream.

## Admission sequence

1. Independent source review of exact hashes and patch semantics.
2. Separately authorized, serial source application and formatting only.
3. Separately frozen bounded build/test protocol.
4. Four parity modes on fixed non-scientific fixtures: no sink, progress only,
   trace only, combined.
5. Injected progress I/O, trace I/O, record-cap, byte-cap, interruption, timeout,
   and cap failures.
6. Only after those pass: the prescribed three-arm 18-slot gate, then all 1,260
   targets. No screening subset can satisfy completion.

No step after source review is admitted by this packet.

Before a trace-enabled attempt is admitted, the plan must compute a conservative
record and byte ceiling from the exact p and 16-walk bound, then reserve that disk
and exporter capacity. A smaller cap may fail the attempt and retain its prefix; it
may not truncate the trace while allowing the scientific row to remain eligible.
