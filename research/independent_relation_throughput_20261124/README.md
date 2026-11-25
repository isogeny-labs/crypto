# Independent relation throughput

Status: active controlled study. Measurements and conclusions remain
`[pcc:unverified]` until the repository validation and replay gates are complete.

## Objective and metric

Maximize the rate at which the collector adds independently replayed relation
rank. The primary quantity is

`new verified rank / charged relation-collection wall second`. [pcc:unverified]

This metric definition is part of the protocol, not a claim that an arm has
achieved it. [pcc:unverified]

Raw candidate rate, raw table collisions, raw relations, counted group
operations, and decomposition probability are diagnostics. None can promote an
arm when incremental rank per charged second regresses. Setup is reported both
separately and amortized over the frozen workload; failures, duplicate rows,
zero rows, rank-deficient prefixes, verification work, and rejected candidates
remain charged.

## Prior evidence and decisions

- The current collector already generates residuals with a one-addition walk;
  fresh double-scalar multiplication is not its hot-path baseline.
- Fixed and renewed strides are closed for this study. Existing autolab rounds
  29 and 30 show that fixed strides cluster on short seed relations and create
  dependent rows, while renewing the stride removes the observed dependence but
  needs about 2.6 times as many probes per independent hit at k = 32.
- `sib=8,pool=256` is retained as a diagnostic candidate, not a winner. The
  existing Stage 1 local run reduces counted work relative to the plain two-seed
  walk but takes about 1.08 times its end-to-end wall time at k = 28 and k = 32.
  The exact failure mode to investigate is implementation cost per generated
  child, not an accepted rank improvement. [pcc:unverified]
- Uniform-residual factor-base redesign is not a yield hypothesis here. The
  repository's KN-FIND-007 double-count identity fixes mean decomposition count
  at fixed base size and arity. A selection policy must demonstrate held-out
  incremental rank gain after its own filtering cost.

## Hypothesis ledger

| id | mechanism | prediction | smallest discriminator | status |
|:--|:--|:--|:--|:--|
| H-GEN-STRIDE | fixed or relation-renewed affine stride | fewer generation operations without rank loss | existing rounds 29/30 | rejected: dependent clusters or 2.6x probe inflation |
| H-GEN-SIB | sibling generation from one batched inversion | lower counted work and, after implementation repair, lower wall per independent rank | profile pool 64 and 256 on the same curve/seed, then paired A/B | active; existing wall result is negative |
| H-GEN-HOT | per-parent subset selection or key/reference bookkeeping dominates the sibling wall regression | one named software path accounts for at least 20% of samples or instructions unique to the sibling arm | matched CPU sampling and Callgrind profiles | active diagnostic |
| H-YIELD-POOL | a larger jump pool reduces collision clustering enough to raise independent rank per generated child | pool 256 beats pool 64 on held-out rank/child and rank/second, with duplicate and zero-row rates no worse | fresh k = 28 and 32 paired blocks | unconfirmed; counted-cost evidence only |
| H-YIELD-FILTER | a cheap point-only prefilter can enrich rank-contributing collisions | held-out rank/second improves over a matched-random filter with the same acceptance rate | train-only predicate freeze, matched random negative control | queued; no predicate selected before the profile |

## Prior-art receipt

Research-kb was current at revision
`750797a540fb4c38837c67b12df84d5b`. Relevant records reviewed before this
study include:

- `source:74a2644c66631f3730c9a78d2b7a2e93`: finding points is not the same as
  obtaining known-coefficient, rank-contributing relations;
- `source:3ec24fd4e0ba7362b061f0421bc95cf6`: primary throughput metric is verified
  independent relation rank per charged second, with paired workloads because
  run-to-run dispersion is material;
- `source:e808ee120fcb3a6265261193039c6353` and
  `source:883ab40565c1f261353efc03ba04473e`: relation collection must retain orbit,
  duplicate, replay, and rank accounting;
- `source:efa8c53d0306cba66023a6c9b36bb887`: relation probability and collection
  cost must be accounted together rather than optimizing either in isolation.
  [pcc:unverified]

The current checkout controls implementation status. Research-kb hits are leads,
not measurements.

## Lean gate

Lean work begins only after the empirical decision. A winning sampling mechanism
may justify a structural proof that coefficient tracking preserves the group
invariant and that its admitted transformation cannot collapse the intended row
space. A rejected bias mechanism may justify an obstruction such as
permutation-invariance of full-cycle density. Benchmark ratios and static timing
arithmetic are explicitly outside the Lean scope.

## Artifacts

- `PROFILE_PROTOCOL.md`: diagnostic profile frozen before collection.
- `profiles/`: new, write-once profile outputs.
- A separately frozen `experiment.protocol/v1` record will govern the paired
  wall/rank comparison after the profile identifies one implementation change.

