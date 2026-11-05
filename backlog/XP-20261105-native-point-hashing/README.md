# Native point hashing: proposed experiments

Design date: 2026-11-05. Status: **launch requested; blocked before submission**.
The [launch-readiness record](launch-readiness-20261105.json) retains the
read-only admission checks and next actions. The [card](card.json) is unready:
the driver, independent checker,
canonical protocols, power analysis and execution admission remain to be built
or completed. Numbers below specify proposed workloads and limits, not results
or runtime predictions. The initial request was experiment design; the later
launch request authorizes execution once the implementation and admission
prerequisites are satisfied. No paid build or scientific submission was made.

## Question and deliverables

Which uses of point hashing reduce the complete cost of useful relations, and
which survive a cold, independently checked, one-target ECDLP comparison?

| Requested use | Experiment | Deliverable |
|---|---|---|
| Reusable tables for two-point decompositions | 1 | Setup/query/memory curves and reuse break-even |
| Canonical keys and normalization | 2 | Correctness controls and complete key-production costs |
| Membership and decomposition caches | 3 | Natural hit rates and bounded-memory costs |
| Point/relation deduplication | 4 | Label-preserving relation stream and elimination costs |
| Distinguished-point collision matching | 5 | Checked recoveries and coordinator costs at fixed work |

These are separate component studies. Only a selected implementation proceeds
to a new full-pipeline confirmation. A canonical `experiment.protocol/v1` and
matching `experiment.export/v1` are required for each actual study; this README
is not a frozen protocol. The original 1,728-attempt feasibility panel under
`research/hashed_ecdl_ecbench_20261030/` retains its existing inputs and protocol.

## Source-informed baseline

Planning source: `08951ab1b665ccdf8451a2ff2566fc1116a89696`. Inspection of
[`finite.rs`](../../crates/hashed-ecdl-tensor/src/finite.rs) finds that the default
selector oracle already derives the final point as `R - sum`, with a per-query
selector-decode cache. For two slots it scans the first domain and tests the
derived point. A quadratic pair scan would therefore misstate the current
native baseline. `prime_decompose` prepares new domains for each query.
[`reduction.rs`](../../crates/hashed-ecdl-tensor/src/reduction.rs) already
enumerates bases into per-slot `BTreeMap<Point, usize>` column indexes once per
trial. Native `Point` is affine. These are source-inspection observations,
pending the independent source audit. [pcc:unverified]

Experiment 1 must distinguish reusing decoded points and domain preparation,
reusing validated membership facts, and changing the lookup data structure.
Retain the existing matrix indexes during the component comparison. Sharing
one prepared representation with matrix assembly is a separate integration arm:
charge its full preparation once, account for every retained copy, and preserve
the original column ordinals. Do not enumerate the same base twice for free.

## Common controls and accounting

- Freeze the curve/subgroup, factor-base recipe and mathematical affine hash
  (`alpha`, `beta`, threshold and base seeds). The index hash is a separate
  implementation choice. Preserve selector order, slot identity, relation-label
  RNG, query order, deadlines and logical limits across pure lookup variants.
- Generate public subgroup query points without exposing discrete-log labels
  to the oracle. Use the same points for every arm; retain misses, empty bases
  and exhausted attempts. Planted-hit cases are correctness fixtures only.
- Keep exact equality after every fingerprint match, including collision
  buckets. Force a constant index hash, test distinct points with equal
  fingerprints, infinity/rejection policy, and both `P` and `-P`. An x-only
  key cannot silently identify signs. Never hash raw projective coordinates.
- Independently replay every returned witness: canonical/subgroup validity,
  per-slot factor-base membership and the sum equation. Keep final scalar
  verification. A table hit is not a replay certificate.
- Report cold preparation, all successful/failed queries, witness checks,
  matrix assembly/elimination, recovery and runner verification separately and
  together. Component amortization stays secondary to cold one-target cost.
- Record additions/doublings, subgroup scalar checks, field inversions,
  multiplications/squares, root/decoder calls, serialized bytes, hash calls,
  probes/full comparisons, allocations, live/capacity table bytes and peak RSS.
  Existing meter steps are diagnostics until a complete price is calibrated.
- Use a separate instrumented pass for counters and an uninstrumented pass
  for clocks, with identical checked outputs. Price calibration, host state,
  binary/source hashes, external versions and instrument overhead belong in
  the frozen analysis. Do not relabel local development timings as results.
- Include duplicate-baseline A/A arms, fresh processes, seeded randomized arm
  order and paired reverse orders over an even number of measured rounds.
  Identical logical-seed replays assess determinism; fresh processes assess
  timing noise; independent base/target seeds assess workload variation.
- Freeze masking, analysis, primary endpoints, multiplicity handling and the
  complete attempt flow before execution. Resample at independent base-seed
  or target clusters, not individual correlated queries or timing repeats.
  Retain all failures, interruptions, missing measurements and exclusions.

## 1. Prepared two-point oracle and lookup tables — first priority

For each fixed pair of bases, seek `P + Q = R` using these arms:

| Arm | Preparation and membership path | Contrast |
|---|---|---|
| A / A-prime | Current selector oracle, including current per-query cache | A/A control |
| B | Reuse prepared domain inverses and decoded selector points; retain arithmetic membership checks | Preparation/decode reuse |
| C | B plus complete prevalidated membership in a BTree/sorted point index | Reuse of membership facts |
| D | C with full-key hash lookup replacing ordered lookup | Data structure only |
| E | D with 64-bit fingerprint buckets and exact full-point comparison | Key/storage layout |

Keep valid points in original selector order and rejected-selector facts
explicit; a complete index must certify the whole selected domain. Never
iterate a randomized map to select witnesses. On uncapped cases these pure
lookup arms should return the same first witness or complete absence and the
same logical relation stream. Physical work counters may differ. Under a fixed
resource budget, changed completion is a separate, explicitly reported endpoint.

Proposed pilot: registered prime-field sizes **12 and 16**, thresholds **32 and
128**, and **8 independent base seeds** per size/threshold, with **2 warmup and
8 measured rounds**. Record actual per-slot cardinalities `B`; do not resample
bases to reach a preferred size. Each arm-batch starts cold and processes the
same 256 queries. Record cumulative prefixes at **1, 4, 16, 64 and 256** queries.
This is 32 contexts x 6 arms x 10 rounds = **1,920 timed arm-batches**, including
1,536 measured batches. Instrumented passes, checker fixtures and external
controls are additional attempts; their counts must be fixed before readiness.

Plot total `preparation + queries + checks` against query count and memory.
Report the earliest *observed* prefix with lower cumulative cost than A, with uncertainty;
report no crossover if none occurs. Fit `T_setup + Q * T_query` only as a
secondary model and check residuals. Include costs of unused prepared entries.
Record lookup and cold costs even when they move in opposite directions.
Neither lookup throughput nor a warm win establishes
a full-pipeline gain.

For external lookup controls, benchmark pinned Rust standard/hashbrown and an
available competitive native table (for example a pinned SwissTable) on the
identical canonical-key corpus, hash policy, load/capacity settings and host.
Document unavailable references. Do not call one the fastest without that
comparison, or count key serialization on only one side.

## 2. Canonical keys and normalization

On the current affine backend, compare canonical encoding on each use, cached
canonical bytes, and fingerprint-plus-full-payload lookup. Hold group arithmetic
fixed and charge key production, copies, storage and equality checks.

A **separate representation study** compares individual affine normalization
with batch sizes **16, 64 and 256** on identical projective points. Include
Jacobian rescalings `(lambda^2 X, lambda^3 Y, lambda Z)` and verify identical
canonical keys. Include zero-Z/infinity handling and partially filled batches;
count latency as well as throughput. Use certified domains and pin all prime
certificates; do not accidentally time the >64-bit AKS setup path as lookup.
Compare an external normalization implementation only on the same curve and
representation contract. It cannot substitute for the native end-to-end arm.

## 3. Cache value under natural reuse

Compare current per-query caching with prepared-decode reuse, then scoped
membership and successful-witness caches. Primary inputs are the actual native
reduction's query trace, replayed identically in each arm. Controlled **0%, 25%
and 75% repeated-query** traces are secondary diagnostics, disclosed separately.

Propose LRU capacities **0, 256 and 4,096 entries** under a common byte cap;
freeze that cap and eviction policy before running. Namespace entries by curve,
subgroup, encoding version, factor-base/slot, domain and backend/options as
applicable. Test invalidation on every field. Cache a negative answer only after
complete exhaustion of that exact domain. A timeout, limit or interrupted search
is unknown and cannot populate a negative entry. Recheck returned witnesses.

Measure natural hit rate, fills, evictions, bytes, work avoided, replay cost and
cold total. Select a cache only on the natural trace; planted repetitions alone
cannot justify promotion.

## 4. Label-safe relation and point deduplication

Compare the unchanged stream with (a) exact fully-labelled duplicate suppression
and (b) shared LHS storage retaining every relation's labels and provenance.
The identity includes slot/base identities, canonical point tuple or normalized
row, normalized `(u,v)` and all semantics needed for reconstruction.

**Do not discard a relation just because its LHS matches.** From
`sum(P_i) = u G + v T`, two equal LHS values with different labels give
`Delta_u G + Delta_v T = 0`; in a prime-order group, nonzero `Delta_v` can yield
the discrete log. Include a fixture where retaining that pair is necessary.
An early collision-recovery variant is a separate algorithm arm with checked
scalar recovery, not an invisible optimization of the relation stream.

Keep raw-attempt limits and stopping rules fixed. Measure full labelled stream
equivalence, storage, checked useful relations, independent dependency/rank
evidence, elimination work and final recovery. Keep raw attempts as the yield
denominator and report retained relations separately.

## 5. Distinguished-point table coordination

Compare central and sharded exact endpoint maps, then full keys versus
fingerprint buckets, with **1, 2 and 4 workers** inside the same aggregate CPU
allocation (including coordinator CPU). Hold walks, starts, step function,
distinguished-point predicate and total work budgets fixed. Carry walk
coefficient labels through collision handling and verify recovered scalars.

Measure total steps and cold time to a checked solve, endpoint rate, bytes per
message, coordinator CPU, lock/queue delay, duplicate trails, memory and cost.
Use a separate crossed follow-up for distinguished-bit counts **4, 6 and 8**;
changing endpoint density is not a table-implementation effect. The first
study is one-host coordination. Multi-host scaling requires its own matched-host
protocol and existing authorized capacity; this card authorizes no expansion.

## Full native ECBench confirmation and decision rule

Choose at most one candidate on training/pilot inputs, freeze it, then evaluate
fresh public targets in the actual native adapter. Proposed holdout roster:
**10, 12, 14 and 16 bits**, **16 new targets per size**, **1 warmup and 8 measured
rounds**. Compare the unmodified selector backend, selected implementation and
matched strongest single-target `rho.negation`, plus A/A controls. Freeze the
complete method/seed/target manifest and exact attempt count before launch.

Report cold cost per independently checked one-target solve, failures/success probability
and memory at every size. Include all validation, preparation, retries, witness
checks and elimination. Timing-only component wins do not complete native
arithmetic pricing or qualify a native `S` ranking. Any scaling claim needs all
four sizes and its uncertainty, not a fit to two pilot sizes.

A proposed engineering screen is a **5% cold-cost reduction**, with no correctness
regression and an explicitly acceptable memory tradeoff. This is not a power
calculation. Estimate variance from the pilot, set test-specific power and
sample counts before confirmation freeze, and bound multiplicity across
predeclared primary contrasts. If the budget cannot fund the required sample,
keep confirmation unready; a nonsignificant result is not equivalence. Preserve
regressions and null outcomes without promoting them into a speed claim.

## Readiness, cost derivation and stopping

1. Implement a native driver and separate exact checker; qualify forced
   collisions, label-safe duplicates, exhaustive-negative cases, invalidation,
   normalization and accounting. Pin the merged combined source and manifests.
2. Freeze each canonical protocol, input/seed manifests, driver/checker hashes,
   external references, byte/work pricing, analysis and the full attempt flow.
3. Establish current account/lane charged-plus-reserved headroom and prices,
   then fresh exact-source S3 checkpoint **write/readback**, authenticated native
   image identity and execution admission. No chat credentials or substitute
   local fixtures/timings. No scientific execution until these checks succeed.
4. Propose a first-pilot ceiling of one **4-vCPU, 4-GiB** native L2 Spot job,
   concurrency **1**, original absolute deadline **1,800 seconds**, and **zero
   automatic retries/restores**. CPU reservation is at most `4 * 0.5 = 2`
   vCPU-hours for the payload. That is a capacity bound, not a runtime estimate.
   Build, queue, idle, disk and transfer costs are additional and currently
   unestimated; obtain current prices and a whole-job dollar cap before
   readiness. Later studies and confirmation need their own costed admission.
5. Stop on a correctness/control failure, invalid manifest or budget/deadline
   boundary. Retain original IDs/deadlines and partial artifacts; do not resubmit
   an ambiguous or interrupted job as a fresh attempt. Export planned,
   attempted, completed and excluded counts, including all failures.

Funding is proposed as zero while unready. No allocation, lease, build or job
is requested by this design. The protected PCC-ledger proposal on PR #2984
remains a separate decision; this card does not authorize editing that ledger
or the conflicted 2026-11-04 journal. Profiling and the original frozen matched
panel retain their pending status and are not discharged by these new studies.

## Sources checked

- [HAC chapter 3, section 3.6.2, Algorithm 3.56](https://cacr.uwaterloo.ca/hac/about/chap3.pdf):
  established table-based discrete-log matching; charge construction, search
  and storage. This is prior work, not a novelty claim for hashing points.
- [van Oorschot and Wiener, Parallel Collision Search with Cryptanalytic Applications](https://www.scs.carleton.ca/~paulv/papers/JoC97.pdf),
  sections 4 and 5.1: distinguished-point storage/coordination, trails and
  coefficient labels. Separate table throughput from steps to a valid solve.
- [Explicit-Formulas Database, Jacobian coordinates](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html):
  affine scaling and normalization costs motivate the separate representation
  experiment; no batch speed ratio is assumed.
- Native [`finite.rs`](../../crates/hashed-ecdl-tensor/src/finite.rs),
  [`reduction.rs`](../../crates/hashed-ecdl-tensor/src/reduction.rs) and
  [`hashed_native.rs`](../../src/cryptanalysis/ecbench/hashed_native.rs);
  [existing native integration plan](../../research/hashed_ecdl_ecbench_20261030/README.md).
- Repository [controlled-experiment skill](../../.claude/skills/controlled-experiment/SKILL.md),
  [canonical-protocol pointer](../../.agents/skills/experiment-protocol/SKILL.md),
  [ECBench measurement skill](../../.agents/skills/ecbench-measure/SKILL.md) and
  [backlog contract](../README.md).
