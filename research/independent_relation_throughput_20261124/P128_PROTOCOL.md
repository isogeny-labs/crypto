# Pool-128 paired independent-rank throughput panel

Frozen on 2026-11-25 before any panel execution. This follows the unchanged-
source discriminator in `SAMPLER_PROTOCOL.md`: pool 128 reduced instructions,
second-point construction, and table insert attempts on one public workload.
That result admitted this larger wall/rank experiment; it did not establish a
throughput improvement. [pcc:unverified]

## Question and hypothesis

Question: on the frozen public k=28 curve and target distribution, does changing
only the sibling pool from 64 to 128 increase fully charged, verified independent
relation rank per second without losing rank or introducing dependent rows?

H-P128-WALL predicts that pool 128 achieves at least 1.05 times the aggregate
independent-rank throughput of pool 64. The mechanism is the discriminator's
observed reduction in sibling second-point construction and table insert work;
the competing explanation is that larger-pool setup and target-dependent walk
variance erase the component gain in wall time.

## Frozen design

- Source baseline: commit `9efc89c6761a965aa1449881e4b2f15f7784e482`.
  The workflow refuses relevant source changes relative to that commit.
- Spec: `p128-panel-spec.json`, schema `ecbench.spec/v2`, diagnostic policy
  because the single explicit k=28 curve is outside the canonical scaling panel.
- Workloads: 16 independently derived public targets on the exact curve used by
  the discriminator. Public targets carry no planted scalar in the record.
- Arms: pool 64 baseline, pool 128 candidate, and a duplicate pool 64 A/A
  control. Every arm shares the same algorithm seed within a workload/round.
- Schedule: two unmeasured warmup rounds followed by six measured rounds,
  `alternate_measured` order, 96 measured executions per arm.
- Cost boundary: `SolveReport.solve_wall_ns`, which includes factor-base,
  oracle/pool setup, relation search, linear algebra, and verification. No setup
  is amortized or subtracted.
- Isolation: every measured record must earn L2 or L3. The native runner and
  verifier retain process timing, placement, seals, and replay evidence.

## Primary observable and controls

For an arm, aggregate throughput is

`sum(matrix_rank) / sum(solve_wall_ns)`.

The primary effect is candidate aggregate throughput divided by baseline
aggregate throughput. Uncertainty is a deterministic 20,000-resample paired
bootstrap over the 96 `(workload_id, measured_round)` pairs; every resample
recomputes the ratio of summed ranks to summed solve time. The A/A control is
analyzed identically against the baseline. The geometric mean of per-pair
throughput ratios is also reported as a drift diagnostic, not substituted for
the primary aggregate.

Before timing is interpreted, every measured record must be `verified`, match
its public target, contain a positive solve clock, and report exactly rank 3
from three rows with zero dependent rows. The session must contain exactly 96
measured records per arm and the native audit must replay every deterministic
measured execution.

## Frozen decision

H-P128-WALL passes only if all of the following hold:

1. all semantic, count, replay, and L2/L3 requirements above pass;
2. candidate aggregate rank/sec ratio is at least 1.05;
3. the candidate paired-bootstrap 95% lower bound is greater than 1.0;
4. the A/A paired-bootstrap interval contains 1.0; and
5. the A/A geometric-mean ratio differs from 1.0 by at most 0.03.

Otherwise the candidate is rejected for this distribution, with separate
failure labels for semantics, rank collapse/dependence, isolation, insufficient
effect, uncertain sign, or failed A/A noise control. A pass is bounded to this
curve, target law, implementation, and host class. A reject closes pool 128 as
the next baseline under this protocol; it does not prove a universal statement
about sibling pools.

Lean is deliberately downstream of this gate. A pass may justify proving a
structural property of the winning sampler or rank-support mechanism. A wall
result or threshold comparison will not be formalized in Lean.
