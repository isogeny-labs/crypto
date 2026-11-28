# Prospective comparison: shared S3 elimination

**Status:** design prepared before study performance data; canonical freeze and
execution admission pending. This file is not a substitute for the required
`experiment.protocol/v1` record, complete-cost integration, or a run receipt.

## Question and claims

Can reuse of the common finite-degree Macaulay row space reduce complete
relation-solving cost on the same ordinary-prime-field instances, and can exact
floating-point kernels improve that computation on an admitted native host?

Separate claims and contrasts:

| Contrast | Held fixed | Question |
| --- | --- | --- |
| Fresh integer vs shared integer | Family, degree, target order, row reduction, F4, domains | Cost of numerical reuse |
| Shared integer vs shared FMA | All algebra and cached envelope | Effect of exact FMA backend |
| Shared integer vs shared split16 | All algebra and cached envelope | Effect of bounded native split backend |
| Original F4 vs preconditioned F4 | Original equations, domains, answers | Complete solver-stage effect |
| Complete IC incumbent vs integrated candidate | One DLP, curve/base/target, accounting contract | Cold complete-pipeline effect |

The primary candidate is shared integer, fixed in advance. FMA and split16 are
separate secondary engineering comparisons, not a post-hoc best-of-three
headline. Multiplicity treatment follows the canonical protocol before launch.
Passing exact tests establishes neither a statistical speed gain nor a general
ECDLP complexity improvement.

## Workload to bind before launch

1. Principal field `p=2^31-1`; exact existing curve and generator from REPORT.
   Keep coordinate modulus and subgroup modulus distinct.
2. Algebraic relations: original S3 chain plus all factor-base constraints.
   Use the production chain adapter and all required charts from #4920 once
   available. Preserve signed witnesses and final `[k]G=Q` verification.
3. Deterministic factor-base construction, chain size, Macaulay degree, solver
   options, row/cell/work caps, target list, holdouts, and seeds must be recorded
   as exact input artifacts. Target generation is independent of arm; the
   solver never receives a known-answer scalar.
4. The binary/WDSat corpus is not an odd-prime-field S3 system. Freeze an
   equivalent matched solver suite under the repository's parent accounting
   contract: 60 exact inputs, three repetitions, separate untouched holdouts.
5. Bind the strongest current applicable algebraic IC incumbent by source and
   input hash; compare the existing tower/parameter and certified-trace reuse
   paths, not only a deliberately uncached implementation. The current source
   audit identifies #4820 and #4859 as relevant prior paths, not an independently
   reproduced winner. Do not substitute a pair table for an algebraic oracle.

The small-field development fixtures are correctness controls. They do not
discharge this principal-field matched suite or its full-DLP requirement.

## Controls and correctness admission

- A/A: two independently executed copies of the identical integer reference,
  to measure execution noise with the same inputs and process isolation.
- Positive control: planted valid relations, independently checked against
  the original polynomial equations and curve group arithmetic.
- Negative controls: malformed inputs, impossible canonical assignments,
  parameter-coefficient cancellation, and rank/pivot drops. A bounded solver
  refusal or timeout must remain incomplete, never become a negative solution.
- Exact comparison: canonical polynomial solution and signed-witness sets,
  not just equal rank, equal row counts, or a checksum of a different problem.
- Every recovered scalar must pass `[k]G=Q` through an independent existing
  group implementation. Include order/cofactor and point validity checks.
- Keep all original equations and factor-base restrictions even when their
  degree exceeds the chosen preconditioner envelope.

## Accounting boundary

The headline interval is one cold public DLP. Charge all work once to that DLP:
curve/base setup; target generation; family metadata; static RREF; all
coefficient projections; allocations; every attempted relation target; residual
combination and elimination; downstream F4; extraction; lifting/signs;
verification; filtering; relation linear algebra modulo subgroup order; final
scalar recovery and verification. Include failed attempts and refusal checks.

For a fixed target list within that one DLP, the candidate cost contains
`shared_setup + sum(target_work)`. The fresh arm contains
`metadata_setup + sum(fresh_target_work)` and never pays for candidate-only
projections. Candidate construction is not placed outside its cold clock.
Warm timings and batches across distinct DLPs are secondary and name their
target count; no cross-DLP setup amortization enters the cold headline.

The new counters cover named algebraic/kernel components and explicitly mark
coverage incomplete. They do not supply complete calibrated machine work.
Collect the repository's complete phase vectors and all unpriced work before
computing a complete-cost ratio. Keep absent costs null. Exact field-operation
counts alone cannot demonstrate that one exact arithmetic backend is faster.

## Native execution and stopping rules

- Build the frozen reference and candidate revisions; record source SHA,
  binary SHA-256, immutable image digest, rustc version, flags, thread count,
  CPU features, and actual worker identity separately.
- EKS Spot through the maintained authenticated harness. Publish code,
  protocol, and launch configuration before a paid build. Verify checkpoint,
  live budget balance, pool capacity, and the source-publication receipt.
- A finite dollar ceiling and original deadline must be entered and checked
  against current headroom before submission. Do not infer current budget
  from historical limits or failed runs. Unknown headroom stops launches.
- Admit wall time only with the required L2 and replay receipts, one warmup,
  A/A controls, and at least five interleaved measured rounds. Fresh processes
  sample layout. No simultaneous build or audit work on the measured worker.
- Preserve every failed, timed-out, capped, interrupted, and incomplete arm.
  Restore preserves work counters; an interrupted interval loses its clock
  and starts a new admitted timing epoch. No replacement jobs by silent retry.
- Stop at the frozen work/time/storage/dollar bounds. No post-hoc degree,
  factor-base, target-count, or acceptance-threshold changes in the same run.

## Analysis and publication

Report paired results and uncertainty for the same completed workloads, with
minimum/median timings and a 95% paired interval where the protocol admits
them. A difference inside the A/A spread is not a speed conclusion. Primary
complete-cost ratio is reference total / candidate total, with all phases
priced in the same unit; show the native-work pricing sensitivity required by
AGENTS.md. Include matched optimized negation rho when reporting the full-DLP
comparison, but not as a substitute for the algebraic IC incumbent.

Preserve raw outputs and a unified `experiment.export/v1`, its flow block,
canonical protocol hash, independent review record, and signed PCC standing.
Only then update benchmark tables and canonical performance graphs. The flow
at design time is: planned performance suite, zero launched study jobs, zero
admitted study measurements. No speed or complete-cost ratio is available.

## Concrete dependencies

The draft's three repetitions per input and minimum five measured rounds are
both retained. Their exact nesting has not yet been bound to an immutable
runner specification; resolving it and its resulting finite budget is required
before canonical freeze. Neither number is an executed sample count.

1. Complete the still-open full-root-library compile/test gate and the #4920
   production adapter interface. Focused development checks are recorded in
   `evidence/development-receipt.json`; they do not close that library gate.
2. Obtain readable canonical protocol skill/checker files and freeze the exact
   machine record; connector and sparse Git reads currently return binary data.
3. Restore the workflow's required manifest-matching archive or use another
   already authorized supported route; do not remove its identity check.
4. Read current budget/capacity and authenticate the runner. Add the workload
   to its immutable source-built image rather than launching a different fixture.
5. Submit and verify the original matched suite, preserve failures, and report
   the measured component and full-pipeline results at their actual scope.
