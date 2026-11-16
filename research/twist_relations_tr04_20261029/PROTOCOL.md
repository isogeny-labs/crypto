# TR-04: exhaustive bounded relation-finding protocol

This is the human-readable contract for active [protocol version3](protocol.json),
frozen at **2026-10-29T23:15:13Z** against code
`f270363df37a17b30210a4671cd31c03d3c20b3a`, before primary comparison data. Its canonical hash is
`a142ce49dbc23c78d49b10572aaf6228960f43803472abb9e735536677d2c0ce`.
Native metadata/hash and complete canonical JSON Schema checks pass.
[Version history](versions/README.md) preserves v1 and the failed v2 filing;
only metadata and validation gates change, not numerical inputs, algorithms,
budgets, cost accounting or decision rules. The public-input census and known
development fixtures are separate in [PREFLIGHT.md](PREFLIGHT.md). No primary
selector/rank/work comparison data has been produced.

## Requested work and evidence

| Requested part | Executable contract | Required evidence |
| --- | --- | --- |
| 1. Bounded twist-guided sampling | Try at most eight candidates, stop at B distinct accepted original-curve points; test following twist runs at fixed L=1,2,3,4 | All candidate-word multiplicities, accepted/duplicate/rejected decisions and paid selector counters; incomplete bases retained |
| 2. Matched equal-size controls | Random, progression, collision-avoiding, and four threshold arms, each at B=2 and B=3 | Same curve, canonical signs/order, target and query-word populations; separate distributions and failed mass |
| 3. Identical original-curve relation finder | Public queries aG+bQ against the canonical pair table Pi+Pj, i<=j; fixed first-pair ties | Every emitted relation verified by public group equality, independent replay, coefficient-rank trajectory |
| 4. Paid work to independent rank | First passage to each rank, or all paid work through the budget when the rank is missing | Exact rank distributions and unconditional work sums/means, phase reconciliation, explicit unpriced physical costs and no survivor-only means |

This belongs to **generic decomposition (bounded by sqrt(r); cannot beat rho)**
under AGENTS.md's track definitions. It is not an algebraic elimination study,
a scalar-recovery campaign or an ECDLP speedup claim.

## Complete finite domain

Enumerate every pair (a,b) over p in {7,11,19}: 49+121+361 = **531 model
dispositions**. Retain singular, composite-order and prime-order-below-seven
dispositions explicitly. Execute the comparison on every nonsingular model
whose original group has prime order n>=7. Isomorphic coefficient models remain
separate named models; they are not independent statistical draws.

For each eligible model, select the smallest canonical original-curve point
as G and inspect B in {2,3}. A point has canonical sign
`y=min(root,p-root)`; bases are sorted by x then y. The fields satisfy
p=3 mod 4, matching the implemented square-root formula. Curve discovery and
construction of the public input population are common audit setup, outside
the selector/relation algorithm's modeled work and recorded as such.

The target population is **every Q!=O** for this fixed G, enumerated by public
additions. Known scalar labels and oracle base logs never enter the selector,
pair-table lookup, query generation or rank calculation. This is exhaustive
for the stated generator/target population, not for every possible generator
policy or larger field.

### Selector inputs

The IID arms (`random`, `twist_l1` through `twist_l4`, `collision_avoid`)
exhaust every word in Fp^8 with equal weight. The implementation uses an exact
subset-state dynamic program: states are canonical accepted bases, and carry
both stream multiplicity and summed work. An absorbing B-complete state is
extended by each unused suffix symbol in the census without reading it or
charging work. Consequently the sum of outcome multiplicities is exactly
p^8, including budget failures. This is an exhaustive finite distribution,
not a collection of PRNG seeds.

`progression` separately exhausts every x0 in Fp and every nonzero delta in Fp,
using the eight coordinates x0+j*delta modulo p. Its denominator is p*(p-1).
Candidate generation/update costs are included. This changes the candidate
stream family as well as the strategy and is therefore a contextual control;
it does not isolate the causal effect of the twist predicate. The clean
comparisons use the shared IID domain.

The random baseline accepts the first B distinct original-curve coordinates.
A threshold arm additionally requires the next L coordinates to have
quadratic character -1. The order of character tests, duplicate checks,
early rejection and canonical insertion is fixed by the frozen interpreter.
The sampler cannot use a free full-field character table or a free
feasibility oracle. L=1,2,3,4 and budget eight are never changed after seeing
outcomes. Structural infeasibility is diagnosed after the paid selector
execution; the failed probability mass is retained.

The collision-avoiding control accepts a candidate only if its canonical
trial pair table has no collisions. All candidate/trial table work is paid.
The final accepted table may be reused by the relation finder; it is not
charged twice. Other arms pay for a cold pair table. Reconstructing that
already-paid table inside the accelerated audit is audit bookkeeping, not a
free algorithmic preprocessing step.

### Relation inputs and stopping

For every completed base and every public Q, exhaust all 256 words in
{G,Q}^8, starting with a=b=0 and current point O. A G symbol increments a and
adds G; a Q symbol increments b and adds Q, with coefficients modulo n.
The pair table retains the first lexicographic canonical (i,j), i<=j, for
each sum. Changing that tie policy would change the experiment.

A hit gives the public identity `aG+bQ=Pi+Pj` and the coefficient row
`e_i+e_j-b*e_k`, with right-hand side a. Rank is computed over F_n in the
B base-log columns plus the k column. It is **coefficient rank**, not rank
of an augmented right-hand side, number of hits, or number of distinct sums.
No scalar recovery is required for this endpoint.

Stop at full coefficient rank B+1 or after eight query symbols. Full rank is
absorbing; unread suffixes retain census multiplicity but incur no paid work.
At every prefix and every rank threshold retain attained and missing counts.
A milestone's work sum includes every observation: work to first attainment
when attained, otherwise full-budget work. A failed selector contributes
rank zero and its full paid selection cost, without fictitious relation work.

The prefix trie and repeated-base caches accelerate the exact audit. The
conceptual cold algorithm still pays the declared costs for every represented
candidate/target/query-word combination. DP or trie elapsed time is not a
selector, relation finder or cold solver timing.

## Weighting and decisions

Within each eligible model/B/method cell, use the uniform exposure measure
over its declared candidate words, nonzero public targets and query words.
Retain exact integer numerators and denominators, using checked u128 arithmetic
and decimal strings. Do not pool fields or B values to select a threshold.
The report preserves all four threshold outcomes and every non-improving,
failed or mixed cell.

The decision rule is exact and componentwise: for an explicitly identified
matched model/B cell, a candidate can be called `within_model_pareto` only
when its unconditional P(rank>=r) is no lower for every r=1..B+1 at every
prefix 0..8, and every component of its unconditional expected total and
rank-milestone paid-work vectors is no greater, with at least one strict improvement.
Otherwise report a tie, worse cell or mixed tradeoff.
Work expectations are exact sum/denominator ratios; the audit does not retain
a distribution over individual work values or claim stochastic dominance of
work. No p-value, confidence interval, generic-curve generalization or universal
obstruction follows from this census.

Counters remain in their separate native units: reads/draws, character tests,
roots, point additions, field arithmetic and exponent bits, duplicate/order
comparisons, table operations/collisions, and modular elimination primitives.
Allocator, cache/branch and RNG-internal physical costs, byte-level peak
memory, serialization/evidence IO, DP/trie acceleration and independent audit
work are not a complete physical-time or GAE price. They remain explicit
cost-model limits; unlike units are not silently summed into a speed claim.
Any sensitivity illustration must show the declared weights and cannot
replace the componentwise primary result.

## Independent verification and controls

The independent verifier must use square/root enumeration instead of the
production Euler predicate, extended-Euclid inversion and its own affine
addition, and full batch rank recomputation instead of the incremental
production reducer. It independently verifies selector multiplicities,
canonical point choices, pair tables, public relation identities and rank
census. The receipt states whether physical-operation counters themselves
were independently recomputed; structural replay must not be advertised as
independent counter validation if it does not cover them.

Each field job emits two actual instrument-control receipts, separate from
the seven selector algorithms:

- `planted_rank`: p=7,a=0,b=3,n=13, G=Q=(1,2), base [(1,2),(2,2)], and public
  word `GGQQGGGG`. The intended coefficient rows at prefixes 2,4,6 are
  [2,0,0], [1,1,11], [0,2,11] modulo 13; the fixture must attain rank three
  by prefix six and independently verify the public point equalities.
- `b2_negative`: the same nonvacuous fixed B2 fixture must have three
  distinct pair sums. Every completed B2 audit also checks this property.
  A failure invalidates the field instead of becoming an apparent twist gain.

Exact deterministic quantities have no stochastic A/A noise interval and
no primary clock. Deterministic development tests/replay and the independent
verifier must agree exactly. Development runtime calibration before freeze
is labelled separately and does not compare selector quality or tune L.

[PREFLIGHT.md](PREFLIGHT.md) records the native public-input census and the
explicit heuristic estimate: 219 fixed planning seconds plus a maximum
1,000-second field-work allowance for each of the three field instances,
3,219 serial planning seconds in total. These are extrapolations from the
known-answer development timing, not measured primary runtimes or guarantees.
The existing 1,800-second field job deadline is unchanged. The helper, commands,
raw observations, exact input/cache-size ceilings and assumptions are retained.

## Parallel execution, checkpoint and export

Use the existing registered `twist-relations.yml` workflow, `suite=tr04`,
on existing `crypto-eks-linux` ephemeral ARC runners on Iso Lab EKS Spot.
Run a source/protocol/checkpoint preflight first, then the three field shards
with `max-parallel=3` and `fail-fast=false`. Every shard includes every
selector, both B values and its two instrument-control receipts. Retain the
immutable source and binary hashes, protocol hash, workflow/job attempt and
runner/host identities. The ordinary ARC route earns L0 operation-count
evidence, not L2 timing admission.

The 1,800-second per-job cap includes the workflow's declared job window;
there is no automatic retry. A missing checkpoint secret, unavailable runner,
failed source/protocol validation or failed S3 checkpoint stops launch and
is reported as infrastructure failure. Do not switch to a hosted/local
research backend or change IAM, OIDC trust, runner registration or capacity
settings to conceal it.

The required preflight and per-field start S3 checkpoint gates preserve source,
protocol, leads and resume instructions. Every field also attempts a finish
checkpoint, retaining the actual native outcome and any partial outputs; a
checkpoint failure remains a job failure and never replaces the original
failure status. Per-run/attempt/field/phase prefixes prevent simultaneous
checkpoint name collisions. These source/evidence checkpoints do not guarantee
retention after abrupt Spot pod loss, and the maintained transport's file-size
and total-size caps still apply. The aggregate job attempts another evidence
checkpoint after reporting, including partial failures. They are distinct from
process-memory recovery. Execution has no
CRIU or application-resume contract; interrupted prefixes are incomplete,
not passes or final clocks. A later rerun is a new execution, not a resume.

Before primary execution the workflow also checks that its existing artifact
store credential bindings are present. This is a prerequisite check, not
proof of remote authorization. After analysis and aggregate checkpointing,
`tools/twist-relations/run-tr04-delivery.sh` deterministically archives actual
preflight/field/aggregate records, protocol and fingerprints outside the raw
input directories. It uses the maintained artifact-store tool and existing
`s3://setec-astronomy-artifacts/store` configuration, then performs a fresh-cache
store pull into an empty verification directory and compares the downloaded
bytes. Source/evidence checkpoints do not replace this durable raw archive.

The delivery receipt records the actual archive hash/bytes, store pointer,
source/tool/policy hashes, stage exits and original failed outcomes. Its
GitHub artifact retains hidden `.artifacts.json`/ignore rules and receipts
while excluding the raw archive and verification temporary payloads. Original
raw GitHub artifacts remain separate even on delivery failure. Actual remote
upload, independent fresh readback and runner readiness remain unverified until
that run succeeds; local transport stubs establish none of them. Failed store
delivery leaves end-to-end completion open even if the finite census completed.

The canonical `experiment.export/v1` has **27 logical rows**: nine arms
(seven selectors plus two auxiliary fixture controls), three identical field
instance IDs (`p7`,`p11`,`p19`), one replicate, no warmups. Each of the 21
selector rows retains B2 and B3 under separate measurement/counter keys and
raw records: **42 comparison cells**, never an average across B. The other
six rows are the actual field-job fixture receipts. Physical backend job IDs
identify the three shared field executions; a logical export row does not
claim its own isolated clock or separate pod. The raw census additionally
reconciles all 531 model dispositions and every represented stream/target/
query-word denominator.

Record planned, attempted, completed, not-completed, interrupted, resumed and
excluded flow from the actual executions. Missing/failing shards and all
records already produced remain visible. Bind the export to the frozen
protocol hash, source/binary identity, per-file hashes, independent verifier
receipt and six actual control receipts. A short or invalid export is not a
completed experiment.

The 27-row table is the complete-acceptance requirement, not permission to
invent attempts. A partial export contains only actual execution receipts;
an unattempted missing shard has no fabricated `attempts=1` row. Retain the
expected 27-row domain and missing instance/arm identities separately in the
backend record and observations, reconcile canonical flow to the rows that
actually exist, and exit nonzero for the protocol shortfall.

## Literature, reports and remaining scope

The source review uses SEC 1 v2 section 2.2.1 for the group law, the existing
Diem primary-source review for summation-polynomial context, and the native
TR01-03 contracts and endomorphism rules. Semaev's primary PDF fetch returned
HTTP 403 and that gap is preserved. No novelty or new general theorem is
claimed by these bounded diagnostic algorithms.

A completed or failed run leaves a source-linked Markdown report, a
quantitative graph of recorded exact values (or an execution diagram when
there are fewer than two comparable values), editable visual source and an
inspected PDF. Preserve negative/inconclusive findings and incomplete
infrastructure outcomes. No leaderboard, admitted benchmark, fastest-tool,
matched-rho or algebraic-track claim is promoted by this protocol.
