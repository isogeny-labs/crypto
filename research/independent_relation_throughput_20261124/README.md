# Independent relation throughput

The pool-128 discriminator has advanced to a frozen paired wall/rank panel.
See `P128_PROTOCOL.md`, `p128-panel-spec.json`, and
`analyze_p128_panel.py`. The panel charges full solve setup, uses 16 public
targets and an A/A control, and requires exact rank 3 with zero dependent rows
on every measured execution before wall time is interpreted. No panel result
has been recorded yet. [pcc:unverified]

Producer attempt `45723097479` compiled the frozen source and expanded all 384
planned executions, then stopped before measurement because the workflow tested
the `executions` array as an integer. The retry changes only that assertion to
test the array length. It does not change the frozen protocol, spec, analyzer,
source, arms, seeds, or thresholds. [pcc:unverified]

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
| H-GEN-CACHE | cache each parent's seed-coefficient residues and pre-plan minus coefficients once | at least 5% fewer solve instructions in both frozen pool arms with identical semantic controls | source-exact Callgrind replay of sequences 242/243 | rejected: 3.17% / 3.54% instruction reductions miss the 5% gate |
| H-GEN-SAMPLE | packed fixed-width draws plus duplicate rejection replace shrinking-range Fisher-Yates work for power-of-two pools | at least 5% fewer solve instructions in both pool 64 and 256 with rank and semantic controls intact | source-exact three-pool profile, then paired wall/rank only on a pass | rejected: changed RNG trajectory regressed p64 by 55.78% despite an 8.75% p256 gain |
| H-YIELD-POOL | a larger jump pool reduces collision clustering enough to raise independent rank per generated child | pool 256 beats pool 64 on rank/child and rank/second, with duplicate and zero-row rates no worse | frozen k = 28 and 32 Stage 1 panels, 240 verified runs per arm and size | rejected at the primary metric: fewer children, but lower point-estimate rank/second |
| H-YIELD-P128 | pool 128 retains some child-count gain without pool 256's full setup cost | fewer second points and inserts than pool 64 and fewer solve instructions, at the same full rank | one matched three-pool Callgrind discriminator, then a multi-target panel only on a pass | passed one-workload discriminator; larger paired panel required |
| H-YIELD-LINEAR | accept `(a,b)` when a fixed linear coordinate `u*a + v*b` passes a cheap predicate | conditioning biases the residual scalar `a + d*b` toward relation-bearing points | Lean equivalence and exact fibre cardinality for the nonparallel case | disproved structurally unless `u*d-v=0`, which encodes the unknown `d` |
| H-YIELD-FILTER | a cheap nonlinear point-aware prefilter can enrich rank-contributing collisions | held-out rank/second improves over a matched-random filter with the same acceptance rate | train-only predicate freeze, matched random negative control | queued; fixed linear coefficient filters are now closed by H-YIELD-LINEAR |

## Profile attempts

The first host-capability probe is preserved at
`profiles/probe-macos/probe.json`. It found neither supported CPU sampling nor
Callgrind on the arm64 macOS host. This is an unsupported profiling environment,
not evidence for or against H-GEN-HOT. The in-progress local release build was
stopped after that determination because it could not produce the preregistered
profile on this host.

The Linux fallback uses the repository's measured-child Callgrind mechanism on
two executions from the already frozen k = 28 session plan. Sequence 242 is the
pool-256 arm and sequence 243 is the pool-64 arm; both are workload 0, measured
round 1, and algorithm seed `5769128089488788324`. Each profiled child must
match its unprofiled child on the recovered scalar and deterministic work and
counter fields before its instruction profile is interpreted.

Workflow run `45558180700` passed those controls. Pool 256 used 7,883,046 solve
instructions versus 5,210,850 for pool 64 (+51.3%). The difference is almost
entirely in `generic_ic_precompute`: 6,757,351 versus 4,084,179. Within that
phase, `sib_expand` accounts for 66.31% / 65.82% inclusive and 40.37% / 40.10%
self instructions; `sibling_pairs_batch` accounts for 22.72% / 22.54%.
Pool 256 expands 719 parents and takes 11,473 steps to reach rank 3, while pool
64 expands 433 parents and takes 6,854 steps. Thus P1 holds, while P2's proposed
pool-size-specific bookkeeping explanation is rejected: the hot-path shares are
nearly unchanged, and pool 256's excess comes primarily from doing more parent
expansions on this matched target rather than a distinct pool-256-only hotspot.
This one matched execution is diagnostic evidence, not the held-out pool-size
decision. [pcc:unverified]

## Parent-residue cache decision

Workflow run `45697281271` tested the source candidate frozen in
`CACHE_PROTOCOL.md`. Its profiled and unprofiled outputs were byte-identical on
the frozen semantic projection in both arms. Both retained recovered scalar
`24622491`, rank 3 from three rows, and zero dependent rows. Pool 64 used
5,045,748 solve instructions, a 3.1684% reduction from the frozen 5,210,850
baseline. Pool 256 used 7,604,279, a 3.5363% reduction from 7,883,046.
[pcc:unverified]

The candidate therefore fails the preregistered requirement of at least 5% in
both arms and is rejected before wall/rank measurement. The exact failure mode
is insufficient hot-path coverage: caching the parent residues removes some
repeated modular reductions without changing relation semantics, but the saved
work is too small a share of solve cost. The source change has been removed.
This is not a rank-collapse or correctness failure. The write-once artifact is
under `profiles/cache-run-45697281271/`; GitHub artifact `15062663186` reported
archive digest
`sha256:ed13f71a89369bfc1b68b2758919db7c5d00047053b682de7b4c9a0b94d8eef4`.

## Packed-sampler baseline and pool-128 decision

Workflow run `45707517171` is the accepted unchanged-source baseline for
`SAMPLER_PROTOCOL.md`, pinned to commit
`8b7eded2e2de00e3af5d67e7338799f10722e047`, spec
`ECS1h89a67963d8ae`, and workload `W636dac909917`. The profiled and
unprofiled semantic projections are byte-identical in all three arms. Every arm
recovered scalar `24622491`, reached rank 3 from three rows, and recorded zero
dependent rows. All per-arm hashes in the downloaded artifact were
independently recomputed. [pcc:unverified]

Pool 128 passes its frozen one-workload discriminator against pool 64. Solve
instructions fell from 4,363,387 to 3,609,963 (17.267%); sibling second points
fell from 2,652 to 1,982 (25.264%); and table insert attempts fell from 5,355
to 3,968 (25.901%). Pool 256 was the negative boundary control and used
6,569,285 solve instructions, 4,563 second points, and 9,128 insert attempts.
This pass admits pool 128 to a larger paired wall/rank panel; a single public
workload does not establish a general rank/second improvement.
[pcc:unverified]

The write-once baseline is under
`profiles/sampler-baseline-run-45707517171/`. GitHub artifact `15069106210`
reported archive digest
`sha256:f3cb399fe9e9d87a119b4de1d7be2be43d05f7c179af42cb1c3816e02498e2f8`.
The packed-sampler candidate is tested as an exact child of the baseline commit
so an unrelated synchronization of PR #5175 with `main` cannot enter the A/B.

Workflow run `45713894485` tested that exact-child candidate at commit
`2fcef9ef67aefc2539cc6fccb79e8dbebfd74231`. All three profiled outputs
matched their unprofiled semantic projections; every arm recovered scalar
`24622491`, reached rank 3 from three rows, and recorded zero dependent rows.
Pool 64 nevertheless increased from 4,363,387 to 6,797,296 solve instructions,
a 55.780% regression, while expanding 284 more parents and taking 4,496 more
steps to rank 3. Pool 256 decreased from 6,569,285 to 5,994,183 instructions,
an 8.754% improvement, while expanding 54 fewer parents and taking 865 fewer
steps. [pcc:unverified]

H-GEN-SAMPLE therefore fails the preregistered requirement of at least 5%
improvement in both p64 and p256 and is rejected before paired wall/rank
measurement. The exact failure mode is not correctness or rank collapse: packed
sampling preserves the ordered-subset law, but consumes the random stream
differently, so the deterministic walk follows a different trajectory. That
trajectory variance dominated the local generation saving and reversed sign
between the two boundary arms. A future generation micro-optimization must
either preserve the subset stream exactly or use a sufficiently large paired
panel to estimate through trajectory variance; this candidate does neither at
its frozen admission gate. The source change has been removed.

The write-once candidate is under
`profiles/sampler-candidate-run-45713894485/`. GitHub artifact `15070909657`
reported archive digest
`sha256:bb1633738d5daa3830974458895ab334318f75a3d667c652bd17eb9831445651`.

## Pool-size yield decision

The already frozen Stage 1 panels provide a controlled discriminator for
H-YIELD-POOL: both pool arms used the same 240 verified workloads at each field
size and reached the same required rank. Relative to pool 64, pool 256 used
0.94078 times as many sibling second points and 0.93445 times as many table
insert attempts at k = 28; at k = 32 the corresponding ratios were 0.94065 and
0.93437. Thus the larger pool did reduce generated-child cost per full-rank
result by about six percent. [pcc:unverified]

That component gain did not survive the primary metric. Dividing each arm's
paired wall ratio by the common plain-walk denominator gives pool-256 / pool-64
wall ratios of 1.00173 at k = 28 and 1.01235 at k = 32. Because every admitted
run reached the same rank, the corresponding verified-rank/second ratios are
0.99828 and 0.98780. These point estimates do not establish a statistically
resolved regression, but they fail the preregistered directional requirement
that pool 256 improve rank/second at both sizes. H-YIELD-POOL is therefore
rejected as an end-to-end mechanism; its exact failure mode is that the roughly
six-percent child-count reduction is consumed by the larger pool's setup and
bookkeeping overhead. The source records are the `tw2cj-lazy` versus
`tw2cj-lazy-sib8-p64` and `tw2cj-lazy-sib8-p256` comparisons under
`research/curve25519_autolab_20261104/siblings/stage1-eks/sessions/{k28,k32}`.
[pcc:unverified]

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

The fixed-linear-filter conjecture has been discharged structurally in
`crypto-lean` PR #43 (merged as `4b4c7adaa6c8936d2109a49ceee287e7d4e17e78`).
For a field `F`, hidden scalar `d`, and fixed filter coordinate
`u*a + v*b`, `linearFilterResidualEquiv` proves that

`(a,b) -> (u*a + v*b, a + d*b)`

is an equivalence whenever `u*d-v != 0`. `linear_filtered_set_card` then
factors the exact cardinality of arbitrary predicates on the filter and
residual coordinates, while `linear_filtered_fiber_card` proves that fixing
one admitted filter value leaves exactly one coefficient pair for every
accepted residual scalar. Hence a fixed nonparallel linear coefficient filter
cannot enrich relation density. The excluded parallel direction is proportional
to the residual itself and therefore requires the unknown `d`; it is not an
implementable selector for the search. The module and aggregate `CryptoLean`
target compile under the pinned Lean 4.35.0 environment; the declarations use
only Mathlib's standard `propext`, `Classical.choice`, and `Quot.sound` axioms.

This proof is an algebraic obstruction, not benchmark arithmetic. A winning
nonlinear point-aware mechanism would still require a structural proof that its
coefficient tracking preserves the group invariant and does not collapse the
intended row space. Benchmark ratios and static timing arithmetic remain
explicitly outside the Lean scope.

## Artifacts

- `PROFILE_PROTOCOL.md`: diagnostic profile frozen before collection.
- `CACHE_PROTOCOL.md`: source-exact parent-residue cache discriminator and its
  now-closed rejection gate.
- `SAMPLER_PROTOCOL.md`: frozen packed-sampler and intermediate-pool gates.
- `sampler-pool-spec.json`: one-workload matched p64/p128/p256 discriminator.
- `profiles/`: new, write-once profile outputs.
- A separately frozen `experiment.protocol/v1` record will govern the paired
  wall/rank comparison after the profile identifies one implementation change.

