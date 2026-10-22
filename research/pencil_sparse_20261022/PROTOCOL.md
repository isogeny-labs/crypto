# Point-driven pencil sparse-order continuation: frozen protocol

This experiment continues the original 21-curve, 60-target-per-curve public
known-answer cohort fixed in `../four_followups_20261021/PROTOCOL.md`. The
source archive and target/workload identities are unchanged. The purpose of
this gate is to test whether a deterministic factor-base variable ordering
reduces exact modular elimination work. It does not redefine the requested
1,260-target end-to-end pencil experiment.

The candidate is a source-checked point-driven pencil solve, with the exact
three-point Jacobian relation verified before entering the sparse matrix.
Every candidate target is paired with same-process source negation-rho and
both recovered scalars are verified against the source generator and target.
Failures, caps and incomplete runs remain in the raw record.

Three variable orders are frozen before execution:

* `identity`: original increasing rational x-column order.
* `reverse`: old column i maps to N-1-i.
* `affine`: old column i maps to `(a*i+b) mod N`, where b is the fixed
  integer `0x5bd1e995 mod N` and a is the first integer at or above
  `0x9e3779b97f4a7c15 mod N` that is coprime to N. This is a bijection.

These alter only the pivot order. The curve, source transfer, walk labels,
sieve, relation verification and scalar checker stay unchanged. The screening
gate uses all 60 targets at p=739, p=1151 and p=1187, then the first target
at p=48383. All raw candidate/rho records and any failed run are retained.
The original identity arm is a reproduction control. Only after source and
counter validation may one order be frozen for continuation of the remaining
cohort. Any future order-selection rule must be written before that run.

The original collector charged full factor-base setup per target, but did not
charge logical variable-order initialization. For the screening gate, compare
the exact `solver_row_updates` and `solver_inversions` counts, plus measured
wall time as a diagnostic. Do not interpret screen row-update changes as a
full-cost gain. A full-cost continuation must explicitly price the new
variable-order work, all existing phases, and the 36 pricing cells against
same-process rho. The reference price is `M+0.5S+10I` and 0.1 source-addition
equivalents per native logical operation, with the full 12-by-3 grid retained.

Each command, binary revision, archive checksum, host, run outcome, raw
JSONL digest, replay certificate, 36-cell grid, graph and PDF is recorded in
`results/manifest.json` and `reports/run-<UTC>/`. A failed or stopped run gets
its own report. Python may render plots and PDFs from native results; it does
not compute recovered scalars or prices.

## Upper-size diagnostic extension (frozen 2026-10-22 05:23 UTC)

After the small-order screen began and before this diagnostic ran, add the
first frozen target of the `p=186379` 32-bit model under identity order. The
same native collector's 10,000,000 stored-coefficient cap remains in force.
The external wall budget is 3,600 seconds and is recorded as a diagnostic
resource limit; a timeout or matrix cap is retained as an unsuccessful run,
not used to infer a cost ratio. The command uses `PENCIL_MAX_TARGETS=1` and
`PENCIL_NODE_ORDER=identity` with `--cohort 32 186379`. No parameter is tuned
from that target before it completes.

## Degree-peeling elimination extension (frozen 2026-10-22 05:28 UTC)

The variable-order screen does not itself remove the 10-million-coefficient
cap. Test an exact alternative on the same relation stream: collect every
verified three-point row of a walk, then after each completed walk perform a
fresh offline elimination over all rows collected so far. Remove degree-one
factor-base variables with their unique row; these rows cannot participate
in a dependency eliminating all factor-base columns. On the remaining core,
choose a live variable of minimum current occurrence count, choose its
shortest incident row as the pivot, and eliminate that variable from all
other incident rows. Check each resulting factor-base-free row for a
nonzero target coefficient and independently verify any recovered scalar on
the source curve. Charge all repeated attempts, indexing, elimination,
relation verification and setup. Record the same-process source rho result.

The first gate is all 60 original p=739 targets followed by frozen target 0
at p=48,383, both with `PENCIL_NODE_ORDER=identity`. Keep an explicit
10-million-coefficient cap and a 3,600-second wall budget on the larger
target. Report failures and every null cost ratio. Do not select parameters
from either gate before the result is recorded.

## Paired small-curve continuation (frozen 2026-10-22 05:35 UTC)

The p=739 degree-peeling result is retained without changing the algorithm.
Run the same `PENCIL_SOLVER=peel`, identity-order collector on all 60 frozen
targets for each of the other eight previously completed pilot curves:
p=1,151, 1,187 (16-bit), 3,371, 3,527, 3,931 (20-bit), and 11,731, 11,887,
12,143 (24-bit). Each curve gets a 3,600-second external wall budget and
keeps the 10-million-coefficient cap. Record every target, failure, process
exit and matched rho result. Compare every fully completed curve to the
original identity-order online collector on exactly the same workload IDs
through all 36 price cells. No curve is dropped based on its outcome.

## Matched 32-bit cap check (frozen 2026-10-22 05:39 UTC)

On the same frozen p=186,379 target 0 that stopped the online collector at
the stored-coefficient cap, run the unchanged degree-peeling alternative with
identity order, a 10-million-coefficient cap, and a 3,600-second external
wall budget. Include the exact candidate work and same-process rho control;
only a recovered and source-verified candidate receives a cost ratio. This
is one target, not a 32-bit curve-slice estimate.

## Original-cohort 28-bit continuation run log

The p=48,823 and p=49,451 online identity first-target jobs began around
05:17 UTC, before the later degree-peeling extensions. These are the next
two 28-bit models in the parent study's already frozen 21-curve cohort;
their target-0 workloads and algorithm seeds were fixed in the archived
source records. They were not used to select the peel solver or its
parameters. This section records the run selection after launch, rather
than claiming a new preregistration. Each is reported as a one-target
observation only, with full cold cost, failed outcomes if any, and matched
same-process rho.
