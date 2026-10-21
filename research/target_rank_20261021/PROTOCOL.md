# Frozen target-coefficient rank collector protocol

The fixed cohort is the original seven exact-order specs from
`research/five_leads_e2e_20261019/specs/genus2-projective-chart-v1/`:
three curves × 60 targets at each source-order size 16, 20, 24, 28, 32,
36, and 40 bits; **1,260 targets total**. `make_specs.py` copies every
workload and the measurement seed/cap unchanged, selecting only the new
method. `ecbench` derives the same algorithm seed per workload/round as the
frozen projective, affine and rho records. No held-out target is used to tune
the policy after results are seen.

For each continuation, evaluate two distinct seeded jumps. Both additions,
root/split tests and valid new relations are charged and inserted; the cap
counts both prospective divisors. Before each insertion, reduce its signed
edge against the current graph potentials. The exact target-coefficient rank
score is 0 for target-free, 1 for a target-bearing merge of free components,
2 for a target-bearing anchor, and 3 for an immediate target equation. The
larger score determines the next walk state; ties retain the first candidate.
The score is a *heuristic for future relations*, while the coefficient
classification for the current graph is exact. Only the signed graph's
actual equation or a charged group identity can extract a scalar. All
preview `find` operations and scalar arithmetic are counted; each preview
also charges one native logical probe. The same projective numerator chart,
exceptional Cantor fallback, duplicate handling and restart cap apply.

Run all seven specs once, preserve every failure and exhaustion, source-check
every answer via `ecbench`, and independently replay all records. Join by
workload ID with the frozen projective/affine and source negation-rho records;
require byte-identical workloads and algorithm seeds. Compute ratios of sums
of complete cold one-target costs at all 12 arithmetic prices (S/M =
0,.5,1; I/M = 1,10,100,1000) and native logical prices 0,.1,1. Report
per size, per curve and pooled ratios, counts, memory, and any source-check
failures. A stage gain cannot be reported as an end-to-end gain. Wall times
from a busy L0 host are diagnostic only; fastest-external claims require
qualified same-host comparison.

Primary comparison: new rank policy versus the frozen projective two-choice
collector. Secondary comparisons: affine two-choice, incumbent v2 and source
negation rho. The latter are frozen matched controls; rerunning them is not
necessary to establish the new arm's one-target cost. The freeze is this
protocol and the generated specs before the first cohort run.
