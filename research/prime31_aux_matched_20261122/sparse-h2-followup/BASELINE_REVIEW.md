# Same-base pair-table control and incumbent review

Date: 2026-11-23. Scope: the sparse-H2 follow-up on
`p = 2147483647`, `a = 847514846`, `b = 2087444724`, and
prime group order `n = 2147446831`. This note records source review and the
implemented control. It contains no new Prime31 timing or recovery result.

## Implemented control

The follow-up includes a working first-witness `m=3` pair-table adapter in
`native/src/pair_control.rs`. It consumes the **same materialized signed factor
base** as the polynomial arm. It does not choose its own base, receive base
logarithms, choose queries from known relations, or change the public target.
The common collector supplies the same seeded `alpha*G + beta*Q` query stream,
independently replays returned points and signed rows, maintains the target
coefficient in its linear system, and verifies a recovered scalar against the
public points. The adapter has no scalar-recovery shortcut of its own.

The panel uses `PairMode::Batched`. `PairMode::Scalar` is retained as a focused
correctness comparator, rather than an additional selected performance arm.

| Operation | Implementation and provenance |
| --- | --- |
| Scalar folded-table setup | Existing production `PairTable::build_negation_folded` |
| Batched folded-table setup | Existing production `PairTable::build_negation_folded_batched` |
| Pair lookup and sign resolution | Existing production `PairTable::probe` |
| Batched `R-P_i` evaluation | Existing production `CountedGroup::add_each_batched`, specialized by `PrimeCurve` |
| Signed-index enumeration, cooperative deadlines, cancellation prefix, report assembly | New, explicitly identified study adapter |
| Production scalar reference in contracts | Existing `decompose_mitm(..., m=3, ...)` |
| Independent point/row and recovered-scalar replay | Existing study collector and independent affine verifier |

The production code is selected **without rewriting its bytes** from
`isogeny-choice/upstream/ic_boundary.rs`, pinned to
[`33b3bc3c7485df0fe80aae41e2093cf940d87ff3`](https://github.com/isogeny-labs/crypto/blob/33b3bc3c7485df0fe80aae41e2093cf940d87ff3/src/cryptanalysis/ic_boundary.rs).
The retained complete-file SHA-256 is
`1f94ec22dbf7f865fd309e4fb770478dd555afd3282808143526fa775b81726d`,
and its Git blob is `9d3bb77a9ae3b4c57efa054517a20650f4389d79`.
The generated build manifest records exact source ranges and dependency hashes.
The prior documented omission of malformed, unused sibling-method spans remains
a limitation of that selected closure; this adapter calls neither omitted
method and does not claim the entire upstream file compiles unmodified.

### Exact witness contract

The adapter accepts a finite, canonical, explicitly indexed base closed under
negation, with one signed column per abscissa. It checks point membership,
column/sign consistency, the point index, the abscissa index and the negation
involution. The common public-target and base certification supplies primality
and the claim that the declared order annihilates the base. The adapter does
not mislabel its metadata checks as a new group-order certificate.

The shared first-witness policy is
`cancellation_first_then_signed_index_order`:

1. If `R` belongs to the base, return the literal witness
   `[index(R), 0, neg_index[0]]`, representing `R + P_0 - P_0`.
2. Otherwise visit the signed points in their original index order, form
   `R-P_i`, and invoke the production table probe.
3. Replay the three returned points with counted production arithmetic and
   construct their exact integer signed-column row. The collector subsequently
   performs independent replay before inserting that row.

The cancellation prefix is explicit because the production table omits pairs
whose sum is infinity. It is shared by scalar and batched modes. The resulting
first witness agrees with the original production scalar decomposition outside
that prefix; it need not agree when `R` itself belongs to the base.

Completeness of a negative result follows directly from folded-table coverage.
For every pair with finite nonzero sum, simultaneous negation lets its first
point be the chosen representative above the smaller abscissa. The production
builder enumerates this pair, including equal-abscissa doubles. A retained pair
with the same sum abscissa represents either the wanted sum or its negative;
the production probe resolves the sign using the explicit negation indices.
If a three-term witness instead uses an identity pair, its remaining point is
`R`, and the cancellation prefix covers it. Repeated points, order-two points
in small-field contracts, and an infinite target are therefore part of the
contract rather than generic-position assumptions.

The adapter returns `FoundFirst` with `complete=false` on a hit. It returns
`CompleteNegative` only after the cancellation test and the entire signed
complement scan. Zero solution capacity or a request for all solutions returns
`UnknownLimit`: this table retains one witness per folded key and is not an
all-relations enumerator. Deadline exhaustion returns `UnknownTimeout`.
Timeout checks occur before validation and before each block of at most 128
signed points. A started block and a successful witness replay can finish after
the cooperative deadline; the outer process cap remains authoritative.

### Work and memory receipts

`choice_pair_setup` contains the serialized `PairSetupReport`. Its mode, columns,
signed point count and abscissa count bind setup to the already checked selected
base. It records the production table's retained entries, insertion attempts,
index lookups, native work counters, and group/field ledger. Adapter validation
has a separate ledger and explicit point/index/negation counts. Both setup
durations are diagnostics unless collected by the admitted matched campaign.

For the study's odd-order, fully signed base with `C` columns:

- There are `2C` signed points and `C` abscissae.
- The production folded builder performs `C(C+1)` abstract group additions.
- Exactly `C` of those sums are the explicitly encountered `P+(-P)` pairs.
- The builder records `C^2` nonidentity insertion attempts and `C` abscissa-index
  lookups. Different attempts can share a retained table key.
- The requested hash-entry capacity is `C^2+1`. This is the constructor argument,
  not the allocator's returned capacity.

These formulas are not imposed on the general adapter's small two-torsion
fixtures. For example, the singleton order-two base has an empty pair table but
still has a valid three-term cancellation witness.

Every returned query result has `backend_work.backend="pair_table"` and its
mode, table kind and policy. It reports completed complements, submitted batch
inputs, identity complements, table probes, pair hits, cancellation lookups and
hits, deadline checks, blocks, replay additions, and the actual delta of the
production group/field ledger. Prepared batched inputs can exceed completed
complements on the last early-hit block: their inversion preparation is already
charged by production code. Field counters remain visible even though legacy
`GroupOps` serialization omits them. The polynomial/F4 counters remain zero
because that solver was not called.

The memory receipt provides `16 * retained_entries` for logical key/witness
payload and `16 * requested_capacity` for the corresponding requested payload.
Neither is total allocated memory or RSS. Hash control bytes, allocation
rounding, row vectors and retained thread-local arithmetic scratch are excluded;
the campaign must retain independently observed peak process memory. Native
counters likewise do not purport to count every machine instruction, comparison,
hash-table internal operation, or allocator action.

### Qualification to be run by the parent session

`native/tests/pair_control_contract.rs` adds six focused contracts:

- Exhaustive `F17`, order 19 targets and literal signed triples, with independent
  point replay and exact production scalar witness comparison outside the
  documented cancellation prefix.
- Repeated-point triples and doubling, plus a complete negative at infinity.
- Singleton and full rational two-torsion fixtures, including a valid infinite
  triple and a zero-entry table.
- Scalar/batched production table key and first-witness equality on a base with
  more than 128 signed points, exercising multi-block production table setup.
- Unknown statuses for caps, backend identity on every returned status, query
  ledger deltas, and rejection of malformed targets or factor-base metadata.
- The existing natural-query collector recovering an order 13 public target
  with independently replayed rows and the same seeded query stream.

No test or benchmark was executed by the adapter author. The parent session
owns the immutable copied qualifier, pre-call source hashes and recorded test
results. A passing small-field contract is not a Prime31 complete-recovery or
performance observation.

## Applicable maintained controls and limits of the review

The repository and cross-repository review looked for the exact mechanism,
negative results, current prime-field collection controls, and sparse/doubling
auxiliary-isogeny work. **Sources checked (KB): unavailable.** No research-KB MCP
was exposed, and the repository wrapper could not execute the absent backend.
The fallback was pinned production source, research reports and merged PRs.
No absence or novelty conclusion is drawn from missing search hits.

### Same-base pair tables and complete IC

The working adapter above reuses the production folded table and batched
addition primitives rather than a toy replacement. The repository's broader
`rootfind-mitm-prime-batch` framework provides a related maintained control.
In `cryptanalysis`, merged
[PR #1891](https://github.com/isogeny-labs/cryptanalysis/pull/1891) adds the
batched `m=3` pair-table oracle in
[`suite/src/cryptanalysis/ic_framework/prime_batch_mitm.rs`](https://github.com/isogeny-labs/cryptanalysis/blob/71313c2f538165fd0ae5418753f131d5c397939d/suite/src/cryptanalysis/ic_framework/prime_batch_mitm.rs)
(blob `b77edb2569cf0bdde83f0456434d559ea84460fd`). Its setup/probe batching and
first-witness equivalence motivate the narrow reuse here. Its published
small-to-mid-bit results are different fixtures, and its wall-calibrated native
pricing is not the present exact field-operation ledger. Integrating that whole
framework remains a separately registered broader control, not a silently
claimed execution of it.

`prime_fast::ic_solve_fast` is also a maintained complete IC entry point. Its
single-target wrapper uses the multi-target implementation with one target and
internal batched setup walks. It chooses its own lowest-x factor base, however,
so it belongs to an unrestricted cold-control comparison rather than a same-V
membership comparison. Its published wrapper reports wall and walk quantities
without the full field ledger required here; its setup guard is distinct from
the target-walk cap. See the pinned
[batched-default report](https://github.com/isogeny-labs/crypto/blob/175da45202645ef95da55b52dd152ed77309d98a/research/ic_batched_default_20261117/README.md)
and [collection-frontier report](https://github.com/isogeny-labs/crypto/blob/6be5e512f3bcc5f81e10823893f864d45a891aa5/research/prime_ic_collection_frontier_20261117/README.md).
The present adapter does not claim this distinct pipeline ran.

The three registered rho controls remain relevant: historical counted negation,
the production `RhoV3::v6` negation configuration, and the one-thread tuned
`prime_fast::rho_parallel` wall reference. The maintained
[optimization registry](https://github.com/isogeny-labs/crypto/blob/175da45202645ef95da55b52dd152ed77309d98a/src/cryptanalysis/ecbench/optimization.rs)
distinguishes eligible variants from diagnostic lanes. The
[tuned-reference note](https://github.com/isogeny-labs/crypto/blob/6be5e512f3bcc5f81e10823893f864d45a891aa5/research/tuned_gae_20261020/README.md)
explains the wall reference. A variant name, eligibility label or a count in a
different unit does not establish the fastest wall implementation on this
target. No such ranking is asserted here.

### Adjacent auxiliary-isogeny and sparse-representation work

The review found concrete related work, which must remain distinct from the
new full-two-torsion-first H2 factorization:

- Merged crypto [PR #4810](https://github.com/isogeny-labs/crypto/pull/4810)
  (source `49e762880eead5c47ce8407d9f7ad6502fac8557`) studies doubling maps on
  an ordinary target and its twist with odd rational group orders. In that
  setting the rational x-map of `[16]` permutes `F_p`; its geometric degree 256
  does not produce 256 rational preimages. This is a different rational-fiber
  situation from an auxiliary curve with the H2 kernel.
- Merged cryptanalysis
  [PR #1845](https://github.com/isogeny-labs/cryptanalysis/pull/1845)
  (source `e062a097c43549b5c02e3ada64cd448ff27f0ab3`) retains division-free
  split-S3 resultant circuits, sparse elimination and replay, with complete
  small and Prime31 fixtures. Its order/curve fixtures differ from this study;
  its complete-cost observations cannot be transferred to this target.
- Merged crypto [PR #4920](https://github.com/isogeny-labs/crypto/pull/4920),
  [PR #4924](https://github.com/isogeny-labs/crypto/pull/4924), and
  [PR #4928](https://github.com/isogeny-labs/crypto/pull/4928) cover signed S3
  root intersections, sparse membership representations, and rational quadratic
  chain/Dickson fiber questions. A representation identity or input-degree
  reduction is not a measured complete solver cost. Their merged status also
  means stale ACTIVE entries should not be treated as independent live claims.
- The cryptanalysis
  [CT136 report](https://github.com/isogeny-labs/cryptanalysis/blob/71313c2f538165fd0ae5418753f131d5c397939d/experiments/chained-transfer-public-target/ct130-prime31/CT136_REPORT.md)
  uses a different CM-neighbor fixture (`p=2147483033`, `q=536857417`). It is
  useful adjacent evidence, not an applicable fourfold symmetry certificate for
  this target.

The `crypto-autoresearcher` repository metadata was accessible during the review,
but some requested text content was not readable through the available source
tool. Those inaccessible records are a coverage gap. No hidden ongoing lane,
absence of prior work, or stronger baseline is inferred from that gap. The
implemented component is reuse and controlled comparison, not a novelty claim.

## Campaign boundary

Screening and complete-recovery attempts are separate stages. The latter's
logical query ceiling is 65536 under the unchanged 110-second collection,
120-second process and 4-GiB memory limits. A finite ceiling is not evidence that
enough useful relations will be found. Natural first-witness success, duplicate
rows, useful dependencies, recovery and public-point replay must come from the
actual receipts. Full factor-base rank is not required when an earlier useful
target-bearing dependency already determines the scalar.

All base construction, pair-table setup, query work, replay, linear algebra and
scalar verification belong to a cold one-target comparison. Reused setup can
be reported separately only with its reuse assumption explicit. The implemented
batched pair-table control closes a concrete missing same-base comparator; any
claim that one isogeny presentation is faster still requires the admitted,
matched full-pipeline observations.
