# RR9 call-owned F4 and whole-collector accounting

- **Status:** `open`
- **Kind:** implementation gap; measurement and accounting gap
- **Affected work:** the prospective [Prime31 common-support RR9/F4 protocol](../research/prime31_shared_coefficient_membership_20261118/PROTOCOL_RR9_F4_20261120.md) and its [machine-readable v1 protocol](../research/prime31_shared_coefficient_membership_20261118/protocol-rr9-f4-v1.json), the [RR9 and closures development report](../research/prime31_rr9_and_closures_20261119/REPORT.md) and [raw result](../research/prime31_rr9_and_closures_20261119/RESULTS.json), and the separate [B8 full-eight precursor protocol](../research/prime31_ic_20261115/rr9/RR9_FULL8_F4_PROTOCOL_20261122.md)
- **Next action owner:** draft PR #5227, call-owned F4 accounting implementation slice
- **Last updated:** 2026-11-26

## Problem

No RR9 F4 call or Prime31 relation-collection run can yet receive a
fully-charged cost label because the current F4 measurements are partial
process-global counter differences and the outer collector has no reconciled
ledger for all work from public residual generation through final scalar
replay. This blocks the requested multiplicity-aware blinded B4/B8 panel and
any fully charged B31 or six-size comparison against matched shared BSGS and
negation-map rho; it does not block retaining the existing correctness
observations as development evidence.

The principal construction is the ordinary cofactor-one Prime31 curve
`p=2147483647`, `a=847514846`, `b=2087444724`, `n=2147446831`, with the
shared-support predicate `M_(A,V,R) | H_B^8` represented by the frozen cubic
remainder tower. The registered rungs are B4/B8/B12/B16/B24/B31. The
prospective v1 runner requires one Rayon thread, normal strategy and grevlex;
it has 44/52/56/60/68/75 variables and 45/53/57/61/69/76 equations at those
rungs. The `DirectCubicQuotient` and `Reduced4VarQuotient` systems in the later
development report are different constructions and must remain separately
identified rather than being pooled with the 75-variable B31 v1 tower.

The B8 full-eight protocol is also affected when it asks to charge public
equation construction, F4, extraction and checks, but it covers only the
squarefree `M=H_8` stratum. Its accounting cannot substitute for the
multiplicity-aware B4/B8 protocol or for a B31 collector ledger.

## Evidence and uncertainty

- **Observed partial F4 counters:** [`VisitReport`](../src/cryptanalysis/f4_fp.rs)
  exposes completion status, dimensions, F4-call count, `field_ops`, root and
  domain multiplication diagnostics, and optional wall-time buckets.
  `field_ops` counts row-reduction multiplications (reducer application, pivot
  normalization and elimination), not the complete call. The source labels
  `native_ops_total()` a lower bound: it counts selected leading-monomial
  divisibility tests, monomial-table probes and staircase work while explicitly
  omitting pair selection, matrix assembly and sorting. The opt-in supplemental
  `jv_bitops` snapshot contains field-multiplication, field-addition,
  field-inversion, row, word and hash-probe counters, but it is not a coverage
  declaration or a reconciled total.
- **Observed ownership gap:** `FIELD_OPS`, native counters and supplemental
  counters are process-global atomics. Only the row-reduction field counter has
  an opt-in thread-local mode. The v1 RR9 runner snapshots the native and
  supplemental totals around `solve_visit` and uses saturating subtraction;
  it has no call identifier, checked-overflow failure, phase/leaf
  reconciliation, or machine-readable `accounting_complete` blocker. The
  runner's one-thread check reduces one source of interleaving but does not
  make these deltas call-owned. Process peak RSS at the end and elapsed phase
  buckets likewise do not account for allocations, scratch peaks or operations.
- **Draft call-owned foundation:** draft PR #5227 adds opt-in
  `solve_visit_accounted_v1` and traced entry points, checked call-owned phase
  totals, distinct solver dispositions, sticky blockers, provenance/arm
  bindings, legacy-report shape preservation, callback/sink isolation and
  focused ownership/reconciliation controls.  The implementation deliberately
  keeps both operation-coverage and memory-coverage promotion gates false;
  admitted records currently seal with `memory_tracking_gap`, with the
  independent operation-coverage flag also false.  Allocation/scratch,
  remaining operation/branch coverage, the prospective RR9 runner, and the
  outer collector are not complete.  This foundation therefore satisfies no
  checklist item and authorizes no scientific row.
- **Observed partial collector charge:** the development whole-solve code adds
  only `rec.f4_field_ops + rec.root_field_muls` for each RR9 call. Its Prime31
  scaling table derives a lower bound from the smaller observed UNSAT
  `f4_field_ops` counter. Neither expression charges the complete residual
  walk, system construction, decode/replay, relation handling, modular linear
  algebra, extraction, final replay, storage or I/O.
- **Observed B4/B8 evidence retained:** the later development report records
  exact set-level agreement on four B4 fibres (single, four-way collision,
  multiplicity-eight point and UNSAT) and B8 positive, collision and negative
  controls, with no forbidden factor-base tuple-enumeration calls reported. It
  also records a toy `F_257` recovery of `d=173` with `[d]G=Q`, nine independent
  relations and all 15 random walks retained. These are useful correctness and
  mechanism observations under that development source; their cost counters
  remain partial, and they are not completed rows of the separately frozen v1
  protocol, whose `results/` directory has no registered solver rows.
  [pcc:unverified]
- **Observed precursor boundary:** the full-eight lane has a replayed public B8
  positive under its earlier development path and two D9 negative controls,
  while the frozen full-eight successor has no registered three-case result.
  Its own protocol says a pass would not establish multiplicity-aware fibres,
  natural relation yield, six-size scaling or a B31 solve. [pcc:unverified]
- **Observed censored rows:** in the development scaling table, the B12, B16,
  B24 and B31 Prime31 calls are timeout-censored. At B31 the largest recorded
  matrices have 120,696 columns for the 30-variable `DirectCubicQuotient`,
  25,103 for the 27-variable `Reduced4VarQuotient`, and 47,825 for the frozen
  75-variable tower. These
  capped runs and their partial-operation lower bounds are inconclusive: they
  establish neither SAT nor UNSAT, do not measure a complete B31 cost, and do
  not disprove shared-coefficient membership or retire RR9. [pcc:unverified]
- **Derived status:** no complete-cost property or scaling law is
  established by the current counters. A timeout is not a mathematical
  negative, and a lower bound assembled from an incomplete counter is not a
  fully charged solver comparison.
- **Suspected explanation:** matrix growth may dominate the larger systems,
  but incomplete accounting and short censored calls cannot distinguish
  intrinsic algebraic growth from implementation, allocation, ordering or
  instrumentation costs.
- **Missing capability:** source-complete call-owned operation,
  allocation/capacity and scratch accounting (including remaining pivot and
  unowned-branch coverage), prospective runner integration, and a distinct
  whole-collector ledger that covers every attempt and phase in a common,
  source-bound schema.
- **What this evidence does not establish:** it does not invalidate the B4/B8
  fibre observations, supply a B31 cost ratio, refute any unexecuted RR9 stratum,
  or justify abandoning the active RR9 lane. Only a regime ruled out by its
  own frozen acceptance and replay conditions may be closed; resource caps and
  missing accounting remain open work.

## What would unblock it

Implement a call-owned `VisitAccountingV1` for the frozen single-thread,
normal-grevlex F4 path and a separate
`prime31.rr9.whole-collector-accounting/v1` ledger for the enclosing RR9
pipeline. Both records must bind source, executable, protocol, case, system
construction and arm identity, and the analyzer must fail closed unless both
records reconcile.

- [ ] Each `solve_visit` invocation owns one `VisitAccountingV1` value. All
  counters use checked addition. Complete, empty, resource-stop, timeout,
  callback-stop and trace-error outcomes have distinct solver dispositions and
  reconcile all work performed before the stop. Overflow, an uninstrumented
  branch or a reconciliation failure cannot set `accounting_complete=true`;
  an otherwise fully counted timeout may do so but cannot be labeled a
  complete solve. Phase and leaf totals reconcile exactly to the declared
  total without process-global subtraction.
- [ ] For the admitted path, the ledger covers validation and input
  normalization; critical-pair generation, selection and criteria; symbolic
  preprocessing; monomial and column ordering; matrix construction; pivot
  search, row reads/writes and field arithmetic; basis cleanup,
  interreduction, sorting and deduplication; specialization; root extraction
  and verification; and solution-callback delivery. Field operations, native
  word/probe work, allocations/scratch peak, CPU and wall observations remain
  distinct units rather than being silently converted into one another.
- [ ] Until separately instrumented, Rayon execution above one thread, trace
  reuse, sibling batching, checkpoint paths and every other unowned branch are
  rejected or returned with an explicit accounting blocker. No partial record
  may be promoted by filling missing fields with zero.
- [ ] Focused tests hand-count representative primitives and cover complete,
  empty, stopped and timed-out traversals; unrestricted and finite-domain root
  paths; overflow and reconciliation failures; concurrent-call
  noncontamination or fail-closed rejection; and equality of solutions and
  dispositions with accounting disabled.
- [ ] The whole-collector ledger separately charges public input and support
  construction; residual-walk EC arithmetic and rejected/identity attempts;
  RR9 system construction and normalization; every F4/root call; Hermite and
  multiplicity/sign decode; original-equation and EC relation replay;
  canonicalization, deduplication, hashing and relation storage; modular rank,
  base-log solve and target extraction; final `[d]G=Q` replay; serialization
  and I/O; CPU/wall; and current/peak/scratch memory. It records complete,
  timeout, OOM, invalid, unsupported and infrastructure outcomes without
  dropping failed attempts, and states setup sharing or amortization exactly.
- [ ] A source-bound analyzer rejects a B31 `fully_charged` label, a six-size
  scaling row, and an RR9/BSGS/rho ranking unless the per-call and collector
  ledgers are both present, complete, arm-consistent and exactly reconciled.
  Matched comparators retain their own setup, online, verification and memory
  charges.

These conditions resolve the accounting implementation obstruction. They do
not themselves establish B4/B8 correctness under the frozen isolated panel,
B31 feasibility, relation yield, linear-algebra completion, target recovery,
a scaling law or an advantage over BSGS/rho; those downstream obligations
still require prospective executions and independent replay.

## Next discriminating step

- **Action or proof obligation:** finish and independently review the
  `VisitAccountingV1` slice in `src/cryptanalysis/f4_fp.rs`: route every
  admitted allocation/capacity/scratch and remaining operation/branch through
  checked call-owned accounting, stop on the first unowned operation, close
  the remaining phase and hand-count controls, and keep both coverage gates
  false until that review passes. Keep both copied-source bridges compiling
  while completing the ledger, then integrate only through a prospectively
  frozen successor RR9 runner schema.
  Do not combine this slice with a B31 run or the whole-collector
  implementation.
- **Inputs and controls:** use small deterministic polynomial systems to
  exercise each phase, then the existing public B8 known-chart case only as a
  non-registered integration preflight. Compare solutions, completion states
  and existing diagnostic counters before/after instrumentation. Keep the v1
  52-variable tower separate from the later seven- and four-variable
  quotient arms.
- **Execution limits and stop condition:** one Rayon thread, normal strategy,
  grevlex, no trace pool, sibling batching or checkpoint path. Stop fail-closed
  at the first unowned operation, overflow or reconciliation mismatch; retain
  the blocker rather than estimating the missing work. This implementation
  slice authorizes no scientific campaign or additional resource spend.
- **Outcomes and next actions:** a reconciled call-owned ledger advances to an
  independent source review and then the separate whole-collector schema. A
  coverage gap extends the same ledger before any registered run. A correctness
  mismatch is an implementation failure. A timeout remains inconclusive and
  changes only engineering/resource planning. Once both ledgers pass, re-run
  the frozen B4/B8 gates before considering B12+ or B31.
- **Recheck trigger:** merge-ready `VisitAccountingV1` source and tests, a
  reviewed whole-collector schema, or new registered evidence satisfying the
  exit conditions above.

A planned implementation is not a completed experiment. Existing protocol,
review, artifact, budget and replay rules continue to apply.

## Resolution and history

2026-11-25: Opened after recovery of the RR9 lane and audit of PRs #4692,
#4816 and #4919. The Nov. 18 partial accounting work is absent from current
main. B4/B8 development evidence is retained, the larger Prime31 timeout rows
remain censored and inconclusive, and the accounting obstruction is
unresolved. Creating this record does not close a lead or establish a research
result.

2026-11-26: Draft PR #5227 gained the opt-in call-owned accounting
foundation and focused non-scientific controls.  Independent source review
found remaining operation and allocation/scratch coverage, stop-on-blocker,
and whole-collector gaps, so both private promotion gates remain false and
admitted ledgers fail closed.  No registered or scientific case was run; all
B4/B8/B31, full8, collector and six-size obligations remain open.
