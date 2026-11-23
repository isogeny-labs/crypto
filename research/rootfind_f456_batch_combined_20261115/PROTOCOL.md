# K13 combined batched rootfinding versus F4/F5/F6 panel

Status: before-data design. No execution or outcome row exists for this panel.
This is a new experiment, not an amendment to either unrun predecessor:
`../rootfind_f456_paired_20261115/` and
`../rootfind_negfold_binary_batch_20261115/`. Their records and publication
receipts retain their own scope. The two predecessor panels cannot be pooled
to decide the batch-versus-F4 question: neither contains both methods in one
same-source, same-host, same-target comparison.

## Question and prediction

On the eight fixed public targets of the registered Koblitz E0 curve over
`F_(2^13)`, does `rootfind-negfold-batch` at m=3 have lower complete cold
wall time than matrix F4, matrix F5, inherited F4 and F6-IC? The candidate's diagonal
pair-table peel, shared inversions and signed-pair scan are the proposed
mechanism. A complete, valid comparison whose candidate does not meet the
decision rule fails the fixed-panel prediction. A wrong answer or censored
comparison makes its verdict invalid or inconclusive, rather than evidence
against the mathematical mechanism. Scalar rootfinding m=3 is a fifth
decisive comparison; m=2 is descriptive as a separate arity.

The paired x86 path is not assumed to execute merely because source code
exists or cross-compiles. Record the host's AVX-512F/VPCLMULQDQ capability,
field-fold condition, selected kernel and fallback counts. A missing or
rejected SIMD route is a measured implementation condition, not a discarded
row.

## Fixed inputs and controls

`SPEC-K13.json` fixes E0 n13, the identical `binary-standard-subspace`
dimension-five base for every IC arm, eight public targets from seed
`202611151301`, six measured rounds, one warmup and shared per-workload-round
algorithm seeds from `202611151302`. The ten arms are matched rho,
`matrix-f4`, `matrix-f5`, `inherited-f4`, `f6-ic`, scalar rootfinding m=3 and
m=2, batch rootfinding m=3 and m=2, and an independent batch m=3 A/A arm.
Every IC arm uses `max_trials=200000`; the four Gröbner arms use m=3,
degree three and `node_budget=8192`. All arms use the same release binary,
source revision, four threads, host, placement and execution caps. Arm order
alternates. The only decisive varied factor is solver choice; no cross-run
or cross-host pooling is allowed.

The planted public scalar is the positive control: every reported solution
must satisfy `[d]G=Q`, checked by ecbench and independent replay. A null
population is not part of this fixed-instance cost question, so there is no
negative population arm. The duplicate batch m=3 arm is the A/A wall-noise
control. Arm labels are visible because method-specific counters identify
them; the decision rule, seed law and analysis commands are fixed before data.

## Primary and secondary measures

Primary: `time.process_wall_ns` from a fresh process through an answer
passing `[d]G=Q`, including factor-base and pair-table setup, unsuccessful relation
probes, rank, descent, recovery and final verification. A pair is admitted
only when both executions verify, all required phases are present, the wall
clock is complete, and both have L2 or higher isolation on the same host and
binary. A wrong answer, timeout, interruption, missing phase or sub-L2 row
is retained and censors the affected comparison. Peak RSS, CPU features,
foreign load, PSI, context switches, source and binary hashes accompany it.

The existing `bitop/v1` and GAE bands are secondary. Earlier K13 F4/F5/F6
records in `../f4_f5_f6_cross_family_20261011/sessions/K13/`, repriced
with the current `ecbench work` command, report
`work.complete=false` on all eight records in each of the four IC arms
because native work remains unpriced. That does not
invalidate a complete process-wall observation. Report every unpriced native
counter and withhold any counted-cost ranking where `work.complete=false`.
Do not call a component timing a complete solve.

For each of five declared wall comparisons, require lower candidate wall on
all 8 × 6 matched measured pairs; its target-cluster bootstrap 99% ratio
interval must lie wholly below one. The duplicate batch A/A 95% ratio
interval must contain one, and each decisive interval's upper endpoint must
be below the A/A interval's lower endpoint. This is a strict fixed-panel
rule with Bonferroni adjustment over the five comparisons, not a population
or larger-curve inference. Report every ratio, target range, failed/censored
pair and phase total even when the rule fails. The four F4/F5/F6 comparisons
jointly decide the user's requested fixed K13 comparison; the scalar m=3
comparison is reported separately.

The bootstrap samples eight target IDs with replacement, preserving all six
rounds of a sampled target as a cluster. Use 20,000 resamples from seed
`202611151603`; form each ratio from the sum of candidate walls divided by
the sum of baseline walls in the sampled clusters. Use percentile endpoints
0.5% and 99.5% for a decisive 99% interval; A/A uses 2.5% and 97.5%.
This uncertainty describes these fixed targets and host noise only.

## Execution and stopping

The native Iso Lab EKS `cpu-l2-spot` runner uses one `c8a.2xlarge` placement,
four CPUs and a fresh process per execution. The fixed 10 × 8 × 7 plan has
560 executions and a 4,800-second job deadline; each execution retains the
1,800-second cap. The planning estimate is 4,220 seconds, derived from the
prior K13 panel's seven-second average plus 300 seconds fixed; it is not a
measurement of this panel. Stop at this one cell. No automatic measurement
retry; one infrastructure-only retry requires a new record. Preserve every
failed, timed-out, interrupted and censored row. A USD 5 new-job ceiling
includes build, queue, failure, storage and transfer; actual daily and monthly
headroom must cover it before a new build or job. The current monthly gate
prevents submission.

Before launch, commit and push the frozen protocol, spec, analysis script and
source, publish an exact `check-experiment-git` receipt, bind the OCI image
and binary SHA-256, inspect existing requests and Jobs for this spec/source,
then verify live pool capacity and budget headroom. Do not start from an
ambiguous submission result without reconciling the original request/Job UID.
Afterward retain raw `ecbench` records, `verify --replay-all`, per-record
`work`, `compare`, `band`, complete wall analysis, A/A, host telemetry,
flow, graph and PDF. Independent replay on a second host class is required
before promoting a performance claim. A future quantitative claim requires
`claims/*.json`, matched evidence and independent verification.

The JSON protocol holds the machine-readable variable ledger, comparisons,
metrics, invalidation and freeze fields. The source and spec hashes in the
publication receipt are authoritative for launch; an updated source requires
a new receipt. A timing result here is limited to K13 and this fixed panel.

## Pre-data analyzer amendment

[Amendment 1](AMENDMENT-1.md) and [protocol v2](protocol-v2.json) correct the
analyzer's admission rule before any panel data. NUMA node zero is valid;
every row must also identify the frozen registered curve, target seed and
index, with the same workload hash and target point across all arms and
rounds. The [original protocol](protocol-v1.json) remains available by its
frozen hash. The sample count, metric and comparison rules above are unchanged.
[Amendment 2](AMENDMENT-2.md) and [protocol v3](protocol.json) require one
per-record work result for every execution and use the frozen bootstrap seed
for all six intervals. The earlier protocol versions remain preserved.
