# Empirical relation-yield pruning protocol

Status: implementation and synthetic controls. This protocol defines an
instrument for an existing index-calculus tournament; it is not a new ECDLP
benchmark or a speedup result.

## Question

Given several frozen factor-base/collector candidates, which candidates can be
stopped early without turning ordinary sampling noise, matrix saturation, or a
cheap-but-useless relation stream into a false winner?

The analyzer consumes cumulative checkpoints from the existing native
tournament. It never generates relations, changes a factor base, or chooses a
candidate after seeing its outcome.

## Frozen before the first query

Freeze and hash the curve/subgroup, candidate definitions, factor-base columns,
collector and verifier versions, target rank, independent episode seeds, work
unit, maximum trials, checkpoint schedule, maximum verified relations emitted by
one trial, familywise alpha, candidate and episode slot counts, minimum terminal
episodes, minimum acceptable full-rank probability, and dominance margin.

Unused candidate and episode slots stay in the multiplicity budget. A later
checkpoint is a new write-once report. Changing any frozen item starts a new
panel; it does not continue the old one.

## Recorded channels

For every candidate/episode/checkpoint record:

1. probes and probes producing at least one independently verified relation;
2. all independently verified relations, including multiplicity;
3. rows that increase rank and the independently recomputed rank;
4. factor-base columns touched, optionally with a full hit vector;
5. charged work in the panel's frozen unit;
6. terminal status and independently verified full-rank success; and
7. invalid relation, inconsistent relation, and verification-failure counters.

`relation_hit_trials / trials` measures hit probability.
`verified_relations / trials` measures row yield and may exceed the hit rate.
`independent_relations / verified_relations` measures conditional rank utility.
They are not interchangeable.

## Decision algorithm

Let `A` be the familywise error budget. For `C` planned candidates, `L` looks,
and `E` planned episodes, v1 uses

```
a = A / (4 C L (E + 1)).
```

The four slots cover the observed bounded mean, future bounded tail, exact
full-rank interval, and a reserved paired-extension slot. This conservative
Bonferroni allocation remains fixed even when slots go unused.

For an active episode, let each future trial emit `X in [0, M]` verified rows,
where `M` is the frozen hard multiplicity cap. Two one-sided Hoeffding bounds
give an upper bound on the unknown mean and then on the future sum. If current
rank plus that optimistic future row count is still below target rank, stop the
episode as futile. With no valid `M`, v1 fails open and does not issue this stop.

Across terminal independent episodes, compute an exact two-sided
Clopper--Pearson interval for verified full-rank success. A candidate may be
pruned when its upper endpoint is below the frozen minimum, or when that endpoint
plus the frozen margin is below the incumbent's lower endpoint. Fewer than the
frozen minimum terminal episodes means insufficient evidence.

Correctness failures prune immediately. A deterministic factor-base rank ceiling
below the target is a structural prune. Coverage, hit entropy, novel-rank rate,
and cost are diagnostics only in v1 because they are dependent and become
non-stationary as the matrix fills.

## Assumptions and controls

- Capped per-trial row counts are independent bounded draws with a stationary
  mean within the projected episode segment. If collector adaptation violates
  this, disable probabilistic futility or start a separately frozen segment.
- Terminal outcomes use independent episode seeds. Paired seeds across
  candidates are allowed, but v1 does not claim a paired-test gain.
- Every relation is replayed with an independent group check, and matrix rank is
  computed over the subgroup scalar field.
- Timeouts, caps, OOM, invalid rows, and verification failures remain distinct.
- A candidate stopped by v1 cannot be called inferior end to end. It is only
  eliminated under the frozen panel and its assumptions.

Unit controls cover dominated terminal outcomes, capped in-episode futility,
the uncapped fail-open path, correctness failure, factor-hit reconciliation, and
write-once report custody. `example-input.json` is a deterministic interface
fixture, not empirical performance evidence.

## Integration and replay

At each frozen checkpoint, export the cumulative observations and run:

```
cargo run --locked --release --bin icprog -- relation-prune \
  --input research/relation_yield_pruning_20261031/example-input.json \
  --out /tmp/relation-pruning-report.json
```

The output path must not exist. Preserve the input, output, source revision,
collector hashes, and verifier receipt together. Continue only candidates whose
episode decision is `continue` and whose candidate decision is `keep` or
`insufficient_evidence`. Candidate-level hard decisions require terminal
episodes except for correctness and deterministic structural failure.

## Admission boundary

This implementation supplies a pruning policy and controls. It has not run a
fresh EKS performance panel, changed the canonical tournament scoreboard, or
measured complete IC/rho cost. Any speedup claim requires a separately frozen
end-to-end experiment on identical public inputs, complete charged costs,
independent scalar replay, and a matched rho baseline.
