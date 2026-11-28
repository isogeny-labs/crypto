# Frozen follow-up: satisfied XOR-row cache

Frozen 2026-10-20 UTC before changing the solver or running this comparison.
The preceding `small-m4-v1/final/audit.json` fixes the observed baseline:
K2048 and D12 target-0 searches cap at 100,000 conflicts, with 476,855,934
and 467,890,580 repivot visits. The accumulated XOR elapsed counter is
97.508–97.561% of worker wall. This motivates testing one implementation
change; it does not establish an improvement.

## Candidate and correctness

Add an explicit opt-in solver setter, default off, caching only a fully
assigned XOR row after its residual is checked false. Invalidate cached
rows on every actual backjump, matrix rebuild/clear, incremental XOR
change and variable-width change. Clear a destination row's cache bit on
row operations. Do not change branching, propagation order, reasons,
learning, budgets, model or base. Record cache setting and skipped row
visits. Test backjump into a contradictory assignment, matrix rebuild,
incremental variables/rows, and mixed CNF/XOR systems against exhaustive
truth tables, with cache-off/on verdicts and returned models checked.

## Matched bounded diagnostic

Use one new native revision/binary for cache off (A) and on (B), exact
K2048/D12 objects and the frozen public corpus from SMALL_M4_PROTOCOL.md.
Target 0 is the observed target. Target 1 is a fresh public holdout for
this m=4 implementation. Keep the validation scalar sidecar outside all
containers. Preserve old cards and binaries unchanged.

The new `small-m4-cache off|on preflight|positive|natural ARM BASE CORPUS
TARGET_INDEX NEW_OUTPUT` mode is limited to targets 0 and 1. Set exactly
10,000 conflicts and one model, retaining 1,790 s internal / 1,800 s outer,
300,000 variables and 8,000,000 domain clauses. This is a separate bounded
event, not a replacement for the prior 100,000-conflict workload.

First run constructed controls for both bases under both settings, each
with a separate general-field replay. All four must pass. Then for each
cell, in order (K2048,0), (D12,0), (K2048,1), (D12,1), run A1,B1,B2,A2
as fresh processes. Sixteen natural attempts are required. Each returned
relation must be independently replayed against its actual public index.
Retain every capped, failed and interrupted attempt. Stop on incorrect
control/model/lift, missing instrumentation, policy drift or Spot loss.
No retry is implicit. Commit source/binary/card hashes before launch.

Use the already authorized SSH Spot host, CPU 31, one CPU quota, hard
4 GiB, zero swap, read-only inputs/root and no network; no new instance.
Record other activity and label shared-host observations L0. Build before
running, never during the card. This diagnostic compares work counters and
correctness; no quiet-host wall-time claim follows from a contended run.

## Acceptance and accounting

Cache-off/on must agree on verdict, exact returned model/lift/row when
present, and decisions, conflicts, restarts, XOR passes, row operations
and row scans for the same cell. Repivot visits and cache-skip counts are
the intended changed work counters. Check cache-on repivot visits plus
skips equal cache-off visits. If timing caps prevent the same endpoint,
keep the pair censored and do not compare it as equal work. Keep all
candidate/selected-best-runtime fields null until a qualifying selection.

Charge every worker stage and failed attempt, archive all receipts, audit
progress and independently replay relations. The original WDSat suite is
inapplicable to this binary-field chained-S3 orbit model; this is a native,
matched N83 component diagnostic with a fresh public holdout. It does not
complete the repository's full performance regression or full IC pipeline
gate. Full useful-rank collection, linear algebra, cold/reused costs,
matched rho, external fastest-solver comparison and quiet-host paired
confidence intervals remain separate open obligations. A reduction in
row visits can be reported at that scope; it cannot select the best base.

If the cache preserves semantics and reduces visits, measure the remaining
propagation work and freeze the next qualified comparison. If it regresses
or changes semantics, retain the result, keep it disabled, and investigate
the failing control before another run.
