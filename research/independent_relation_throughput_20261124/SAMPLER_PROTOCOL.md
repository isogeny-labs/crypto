# Packed subset sampler and pool-128 discriminator

Frozen on 2026-11-25 before implementing the packed sampler. This is a small
source-exact discriminator. It can reject either mechanism or admit it to a
larger paired panel; one workload cannot establish an end-to-end winner.
[pcc:unverified]

## H-GEN-SAMPLE: packed power-of-two subset sampling

The current sibling walk selects an ordered `m`-subset by making `m` shrinking
range draws, swapping a persistent Fisher-Yates index vector, and copying its
first `m` entries. For the active pools (`n = 64, 128, 256`, `m = 8`), the
candidate will instead consume non-overlapping fixed-width chunks from random
words, reject indices already selected for that parent using a four-word
bitset, and append accepted indices directly to the current subset. Non-power-
of-two pools and unsupported sizes retain the existing path.

Under an independent uniform-bit model, fixed-width chunks are independent
uniform pool indices. Rejection of repeats therefore yields the same uniform
law on ordered distinct `m`-tuples as partial Fisher-Yates. The stream is
point-independent. This distributional argument is a hypothesis boundary, not
a Lean result: formalization is admitted only after an empirical throughput
win.

Prediction: relative to the source-exact baseline produced by this protocol,
the candidate reduces measured-child solve Callgrind instructions by at least
5% in both the pool-64 and pool-256 arms. Every semantic projection must remain
internally identical between profiled and unprofiled executions; every arm must
recover the same scalar with rank 3 from three rows and zero dependent rows.

Decision: failure of either 5% arm, any semantic mismatch, any lost rank, or
any dependent row rejects the candidate. Passing both arms only admits a
separately frozen paired wall/rank panel. Setup, duplicate rejections, and all
fallback work remain charged. The pool-128 arm is reported diagnostically but
is not part of this implementation gate.

## H-YIELD-P128: intermediate pool size

The frozen unchanged-source baseline also tests whether pool 128 retains enough
of pool 256's lower child count without paying its full setup and bookkeeping
cost. All three arms use the same public curve, target, measurement seed,
algorithm seed, arity, factor base, and rank requirement. Their full setup is
inside the profiled solve interval.

Prediction: pool 128 reaches rank 3 with zero dependent rows, uses fewer
`sibling_second_points` and `table_insert_attempts` than pool 64, and uses fewer
solve instructions than pool 64. Pool 256 is a negative-boundary control: prior
panels showed a child-count gain that did not survive end-to-end wall cost.

Decision: pool 128 advances to a multi-target paired wall/rank panel only if all
three directional requirements hold on this workload with no semantic failure.
Otherwise this intermediate-pool lead is rejected at the small discriminator,
with the failed component named. Even a pass is only an admission result; it is
not a relation-throughput claim.

## Frozen execution and artifacts

- Spec: `sampler-pool-spec.json`.
- One k=28 public target on the first frozen Stage 1 curve; one measured round,
  no warmup, alternate order, L0 development evidence.
- The workflow plans the spec, resolves each arm's sequence number from the
  plan rather than hard-coding it, runs unprofiled and Callgrind children, and
  compares the frozen semantic projection byte-for-byte.
- Primary implementation unit: solve Callgrind instructions. Yield diagnostics:
  matrix rank/rows/dependence, sibling second points, insert attempts, parents,
  steps to rank three, zero rows, and collisions.
- Raw plan, inputs, semantic projections, summaries, Callgrind parts,
  provenance, and hashes are uploaded together. Artifacts are copied into the
  study before any decision is recorded.

Lean is excluded from benchmark arithmetic and from this preregistration. If
H-GEN-SAMPLE wins the paired throughput panel, Lean may prove equality of the
ordered-subset distribution and preservation of the coefficient/group action.
If a structural obstruction is discovered first, Lean may close the mechanism
without a performance theorem.
