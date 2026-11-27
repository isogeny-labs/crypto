# Frozen protocol: matched p+1 versus ordinary factor base

Date: 2026-11-26
Lane: draft PR #5229
Status: frozen before implementation outcomes or benchmark execution

## Question

On the production Prime31 curve, does the p+1 norm-one-torus Dickson factor
base make the existing generic five-term `2+3` decomposition algorithm cheaper
than an equally sized ordinary factor base when the two-sum side is streamed
and never retained between targets?

This is a generic-decomposition component comparison. It is not an algebraic
PKM solver, an exact sublinear pair-spectrum oracle, a full relation pipeline,
or a scalar-recovery experiment.

## Prior information and prediction

The decomposition-count identity fixes the uniform-target mean at matched
signed-point cardinality. The existing Dickson-torus/random solution-count
census follows that prediction. Factor-base geometry can redistribute hits and
collisions, but cannot increase the mean number of decompositions.

Prediction: after cardinality and algorithm are matched, the full-scan group
and field operation counts will be equal or nearly equal. Any local wall-time
difference can come from table occupancy, hashing/cache behavior, exceptional
group-law branches, or noise; it is not a yield or asymptotic improvement.

Sources checked are recorded in `docs/session-logs/2026-11-26.md`. The shared
research KB was unavailable in this runtime, so repository primary notes and
source were used as the documented fallback.

## Fixed instance and factor bases

- Curve: `fp_p31_subexp_mersenne_instance()` over `p = 2^31 - 1`.
- Arity: five, split as a streamed two-sum and a retained three-sum table.
- Torus arm: production `dickson_factor_base(instance, 6, 0, 0)`.
- Ordinary arm: production `prime_factor_base(instance, 34)`.
- Match: both arms must have exactly 34 abscissae and 68 signed points; both
  must be closed under the recorded negation permutation. A mismatch aborts.
- Three-sum table: canonical quotient by elliptic negation, keyed by abscissa,
  with projective mixed addition and Montgomery batch normalization.
- Two-sum stream: enumerate the same unordered signed-point pairs in each arm.
  Form each pair and its target complement projectively, normalize complements
  in fixed blocks, probe the three-sum table, and discard the block. No pair
  vector or pair hash table survives a target query.

## Inputs and public randomness

A manifest conforming to `isogeny-labs/experiment-seeds/v1` will be committed
and pushed before its target stream is executed. It will bind:

- one 256-bit master seed and SHA-256 domain separation;
- 256 common random targets, identical point-for-point between arms;
- 8 planted targets per arm, derived from arm-separated streams;
- 9 counterbalanced rounds and query block size 1024;
- source hashes for the runner, verifier, and shared comparison support.

Random targets are the paired comparison. Arm-specific planted targets are
positive controls only and are not used for a same-target speed ratio. No
outcome-dependent seed replacement is allowed. Failed executions are retained.

## Endpoints

Primary deterministic endpoint, on the 256 shared random targets:

- complete full-scan operation vector: additions, doublings,
  multiplications, squarings, inversions, pair candidates and table probes;
- retained three-sum states and distinct abscissa keys;
- complete hit/miss mask, independently recomputed in affine arithmetic.

The full-scan arm continues after the first witness so the operation vector is
not changed by early exit. A separate first-hit pass measures practical early
exit on the same common targets. The planted-target pass verifies the hit path.

Secondary diagnostics:

- counterbalanced local setup/query wall times;
- production factor-base construction counters;
- result and factor-base digests.

Local wall time is non-isolated producer evidence and will not be promoted as
an admitted speed claim. The repository-required `isolab-eks` skill and EKS
route are unavailable in this runtime; if they become available, the exact
precommitted manifest is replayed there without retuning.

## Decision rule

- `torus faster` at deterministic component scope requires a strictly smaller
  primary operation vector for the same full scan without increasing another
  charged operation class, or a separately admitted isolated paired timing
  result under the repository measurement policy.
- Equal primary counts mean no generic streaming-query advantage, even if a
  noisy local wall median differs.
- A finite-sample hit-mask difference is reported as redistribution, not a
  mean-yield advantage.
- A loss is retained and reported as a loss.

## Correctness and independence boundary

A separate Rust verifier will rebuild both production factor bases and target
streams, enumerate affine three-sums, stream affine two-sums, compare every
hit/miss decision, and replay every returned five-index witness. It will not
call the candidate Jacobian arithmetic or streaming solver.

The manifest, transparent witnesses, hashes, and affine replay are
reproducibility/correctness evidence. They are not zero knowledge and do not
prove which host executed the benchmark.
