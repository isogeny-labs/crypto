# Complete-cost comparison gate

**Status: open.** The RR5 implementation and its signed-row consumer are a
development correctness path. `SolveRecord::complete_cost_accounting` and
`Diagnostics::phase_accounting_complete` remain false. A successful finite-field
fixture does not establish a PRIME31 solve, a cost advantage, or an asymptotic
exponent. This document records the comparison obligations; it launches no run.

## Source-bound incumbent inventory

These are existing implementation pointers, not a newly measured ranking. The
strongest applicable IC configuration must be selected from reproducible project
evidence and frozen before the matched comparison. A familiar implementation
must not be called the best merely because its interface is convenient.

| Role | Existing source and entry point | Required interpretation |
| --- | --- | --- |
| Original algebraic IC completion contract | [`prime31-algebraic-ic`](../prime31_ic_20261115/native/lib.rs): `Base::build`, `decompose`, `solve` | Native two-summand membership/F4/root extraction, authentic `aG+bQ` rows and final scalar replay. Reuse its arithmetic and public-input contract; its own counters expressly remain developmental. |
| Production algebraic IC family | [`PkmSemaevOracle`](../../src/cryptanalysis/ic_framework/pkm_semaev.rs), through the existing IC pipeline | Freeze the admissible membership, arity, mode, trace/guess settings and `descent` setting. This source also contains exhaustive/grid recovery paths; every invocation of those paths must retain its classification and full cost. An F4 configuration name alone does not certify nonenumerative extraction. |
| High-throughput generic IC controls | [`prime_fast`](../../src/cryptanalysis/prime_fast.rs): `ic_solve_fast`, and `ic_solve_lockstep` with `LockStepMode::DirectSweep` / `PairTable2` | `ic_solve_fast` runs the batched pair-table path on one target, with automatic or explicit base size. Charge base/table construction and all relation/log recovery work. These controls do not satisfy the RR5 nonenumeration requirement. |
| Optimized prime-field rho wall reference | [`prime_fast`](../../src/cryptanalysis/prime_fast.rs): `rho_parallel` with `RhoConfig::tuned(n)` | The existing tuned configuration uses one thread, negation, look-ahead, a cycle window, short-coefficient jumps and stride starts. Freeze seed, step limit, walkers and resources. Retain the whole run and its setup, restarts, cycle escapes, collisions and verification. |
| Counted rho reference | [`ecbench::methods`](../../src/cryptanalysis/ecbench/methods.rs): `rho.negation`; [`ecbench::work`](../../src/cryptanalysis/ecbench/work.rs) | Preserve the established counted reference alongside the optimized wall reference. Target-dependent rho setup remains a per-target expense; a phase named `setup` is not automatically reusable. |

The [Run72 width result](../prime31_f4_admission_20261121/RUN72_WIDTH_SWEEP_RESULT.md)
and its [K4 source](../prime31_ic_20261115/native/examples/prime31_run72_k4_rhs_width.rs)
remain historical generic controls. There, K4 is four public certificates per
target, not a four-summand algebraic extractor, and the panel has 32 targets.
Its matched BSGS and its width ranking cannot be substituted for a fresh cold
single-target IC/rho comparison. The
[Run75 component floor](../prime31_f4_admission_20261121/RUN75_COLD_PREORACLE_FLOOR_20261123.md)
is explicitly conditional on the old fixed pipeline; it does not bound RR5 or
all index calculus.

## Primary experiment: one public target, cold

The primary unit is a complete verified recovery of one public target from fresh
candidate state. Charge the factor base even though `solve_public` receives it
from its caller. Include construction or loading, curve/base validation, selector
and membership construction, every generated public `aG+bQ`, every unsuccessful
or unsupported extraction, all successful extraction, signed replay, duplicate
handling, exact mod-n rank, target recovery and final `dG=Q`. A supplied positive
decomposition is a qualification fixture, not a natural-target yield sample.
The governing scope is the [development protocol](PROTOCOL.md).

Use the same public targets and resource envelope for the candidate, the frozen
applicable IC incumbent and tuned rho, on at least three eligible ordinary
curves. Retain failure, timeout, interruption and memory-limit outcomes as well
as successes. Report success probability and complete cost distributions with
their censoring policy. Record peak resident memory and complete elapsed work;
do not report only productive queries or elimination multiplications.

Any shared-base or multi-target experiment is secondary and identifies exactly
which work is reusable and how many targets pay for it. The primary cold result
cannot amortize base construction, prepared equations, learned traces, tables or
factor-base logarithms. An exponent claim additionally needs a preregistered
growing-size family and all size-dependent phases, not a fit to one PRIME31
fixture or a symbolic input degree.

## Accounting gaps that still block a cost claim

| Phase | Current diagnostic support | Remaining obligation |
| --- | --- | --- |
| Caller and input setup | `caller_supplied_base`, consumer setup arithmetic | Charge base selection/construction/loading, public input validation, allocation and indexes. Start the cold clock before these operations. |
| Polynomial construction | Symbolic additions/multiplications and input terms/degrees | Include map operations, allocation, clones, normalization, membership construction and transient memory. The symbolic term cap is not an F4 matrix/RSS cap. |
| F4 and finite-field roots | `VisitReport`, root multiplications, traversal/status data | Include all native symbolic work, specialization, matrix creation/reduction, all roots, refused/incomplete paths and output handling. Diagnostic F4 counts do not price the full phase. |
| Sign lifting and group replay | Original `Arithmetic::Counts` | Include coefficient evaluation, inversions/powers outside that ledger, sign/index handling and every rejected assignment. Preserve cap causes together with replay failures. |
| Relation consumer | Exact rank, row dispositions, mod-n work and comparison counts | Charge canonicalization, allocation/tree traversal, transient storage and every spent row. `persistent_coefficient_bytes_floor` excludes substantial memory overhead. Stop useful-row consumption when the target functional is determined. |
| Query generation and termination | Public coefficients/points, attempts, receipts and final replay | Preserve spent queries that expire before extraction and partial transcripts on terminal errors. Freeze query, time, output and memory policies and distinguish cooperative checks from external process limits. |

`consumer_setup_arithmetic` is a subset of the cumulative
`consumer_arithmetic`; summing both double-counts setup. Extractor diagnostics
belong to their respective attempts and must also enter the ledger. Keep group
operations, field operations, mod-n operations and unpriced native work distinct
until the project's specified conversion/calibration has been applied. Neither
the current counters nor a development test's elapsed time provide such a
calibration. [Sources: `solver.rs`](native/solver.rs),
[`extractor.rs`](native/extractor.rs),
[`relation_adapter.rs`](native/relation_adapter.rs).

The remaining qualification requires the frozen source/input/executable
identities, the original completion gate, admitted research resources and the
independent replay required by the project protocol. A focused unchanged-source
closure passing development tests does not satisfy those separate gates.
Until a matched, fully charged panel passes, no claim of being cheaper than the
best applicable IC or optimized negation-map rho follows.
