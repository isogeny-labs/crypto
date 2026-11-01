# LEAD-15 / evaluator e5 — prospective Diem–Thomé plane-quartic harvest

Protocol ID: `LEAD15-DT-E5-20261101`
Status: **DRAFT — TARGET DATA EMBARGOED UNTIL THE FREEZE BLOCK IN `protocol.json` IS COMPLETE**
Owner: `codex/lead15`
Evaluator: `research/pdp_strategy_evaluation_20261031/cards/e5.md`

## Question and scope

On elliptic curves over `F_(q^3)` that have both a cubic-GHS `m=3` hyperelliptic
genus-3 cover and an `m=2` non-hyperelliptic plane-quartic cover, does a complete
Diem–Thomé two-large-prime line harvest produce enough independent, sufficiently
sparse relations to preserve the card's projected advantage over the matched
hyperelliptic incumbent?

This is a prospective structural/relation/linear-algebra measurement. It is not an
end-to-end DLP, does not include individual-log descent, and makes no claim about a
deployed curve. The rigorous `O~(q)` algorithm of Diem 2012 is not the measured arm;
the measured arm is the heuristic practical Diem–Thomé graph algorithm and its
assumptions are reported as such.

## Frozen population

- Exact prime sizes: `q in {257, 2053, 32771, 4194301}`.
- The fourth value resolves evaluator e5's impossible phrase “prime `q=2^22`.”
  `4194301` is the largest prime below `2^22`; it retains the stated approximately
  `1.7e7` factor-base pairs and keeps an exact-cofactor-4 subgroup in `u64`. The next
  prime (`4194319`) is not silently substituted.
- Six distinct curves per size, 24 total.
- Base seed: `20261023`. The exact per-stream derivation is the `seed_for` function in
  `examples/lead15_diem_quartic.rs`; it mixes `q`, curve index `0..5`, and a duplicate
  retry counter with the literal constants committed there.
- A stream accepts the first candidate, within `200000` candidates, satisfying every
  inclusion condition below. A duplicate quartic within a size is rejected and the
  next deterministically derived retry stream is used. Candidate counts, rejection
  classes, retry number, and seed are recorded.
- No replacement is allowed for timeout, arithmetic failure, verification failure, or
  exhaustion of the candidate cap. Such a cell is missing/invalid and is reported.

### Inclusion conditions

1. `q` is the frozen odd prime and `K=F_q[g]/(g^3+a g+b)` uses the builder's
   lexicographically first irreducible depressed cubic.
2. There are distinct `a,b in F_q` and `r in K\F_q`; with `r_q=r^q`, the branch tuple
   `{a,b,r,r_q}` certifies the `m=3, r=5` hyperelliptic cubic-GHS type under the prior
   branch-set construction.
3. One of the 24 source orderings has cross-ratio `lambda` for which
   `s^2=lambda/(lambda-1)` has a trace-zero square root. The linearized equation
   `z-z^(q^2)=s` is solved and a Möbius matrix is verified on all four points against
   `(z,z+1,z^(q^2),z^(q^2)+1)`. This is the common-branch certificate.
4. `u=(x-z)(x-z-1)` builds a smooth descended non-hyperelliptic plane quartic through
   the existing `m=2` construction and independent descent checks.
5. The transferred elliptic curve order is exactly `4*ell`, with `ell` prime and
   representable as an odd `u64`. The full order is stored as decimal `u128`.

The common branch locus is invariant under quadratic twist. The measured instance is
the `m=2` target model; its verified branch divisor, not an unverified equality of
Weierstrass coefficients, is the certificate that the same instance admits the `m=3`
cover type.

## Historical development checks (not target observations)

Before freeze, unit tests may use only:

- general square roots in `F_257` and wide Montgomery arithmetic;
- the recorded `q=5`, order-124 branch tuple from
  `research/ghs_cubic_hyperelliptic_genus3_20261013/results/run-20261014T0027Z.jsonl`;
- a finite `q=7` common-branch census in the local cubic basis.
- a synthetic `q=11` all-pairs collector smoke using the same historical toy-field
  construction, solely to check pair/counter/verification invariants.

No generator, factor base, yield, graph, row, rank, or cost value at any frozen target
size may be observed before the protocol hash, commit, and S3 pre-run receipt exist.

The four exact non-target analyzer/schema regressions are currently blocked by the
local 16-GiB Cargo test-profile build envelope, as recorded in
[`problems/lead15-analyzer-test-profile-memory.md`](../../problems/lead15-analyzer-test-profile-memory.md).
This is a pre-freeze validation dependency, not a scientific outcome; the smaller
standalone graph-certificate harness does not substitute for those analyzer tests.

## Harvest, exactly

For every accepted curve:

1. Set `N` to the least integer satisfying
   `10000*N^2 >= 283^2*(q+1)`, i.e. `ceil(2.83*sqrt(q+1))`.
2. Sample `N` distinct affine rational quartic points (`W2 != 0`) with the frozen
   rejection stream. Conditional on this affine chart the sampler is uniform; points at
   infinity are deliberately outside the factor-base population. Point keys are injective
   `u128` encodings of normalized projective points. Count every SplitMix draw, including
   rejected slices and root-selection draws.
3. Visit every one of the `N(N-1)/2` unordered pairs in lexicographic order. There is
   no early success stop.
4. Intersect the pair's line with the quartic, solve the residual quadratic, normalize
   both residual points, and classify the section before duplicate suppression:
   `Full` (zero large primes), `FP` (one distinct large-prime point), or `PP` (two).
   Record raw counts. Use the exact normalized projective line coefficients, injectively
   packed into 66 bits, to identify a section; independently verify every first-seen
   factorable section against the quartic, suppress duplicates, and record accepted
   unique counts.
5. Add FP/PP relations to the parity large-prime graph. Record vertices, edges,
   `E-V+B`, usable relations `Full+E-V+B`, and the largest-component fraction among
   large-prime vertices.
6. For bounded memory retain every spanning-forest edge and the first `8N+1024`
   combined full/non-tree relations. All later pairs still update every count and graph
   statistic. Materialize the forest paths supporting retained cycles. If this frozen
   cap cannot produce at least `N+2` FB-only rows although the full graph can, label an
   implementation failure; do not raise the cap after seeing data.
7. Eliminate every large-prime column by the committed minimum-weight sparse Gaussian
   elimination. The decisive row weight `w` is the arithmetic mean number of nonzero
   coefficients (including the target column) in these FB-only rows. Also record row
   count, maximum weight, and mean pivot-row weight from the subsequent rank SGE.
8. Compute exact rank over `F_ell` with committed sparse Gaussian elimination over all
   materialized FB-only rows. There are `N` columns: target class plus `N-1` factor-base
   logs after fixing the first factor-base log. Report `rank` and `nullity=N-rank`.

## Counts and complete cost

No clock value is a scientific metric. Lifecycle timestamps may be retained only as
execution provenance, and every canonical export row has `wall_valid=false`. Primary
costs are deterministic operation counts in `F_q`-multiplication equivalents; the
3600-second native-child deadline is only a frozen terminal censoring rule. The outer
Kubernetes Job deadline is 7200 seconds and is not a scientific runtime allowance.

- `F_q` multiplication: 1.
- inversion: `ceil(log2(q))`; implemented as `64-leading_zeros(q)`.
- square root request: `ceil(1.5*log2(q))`; the implementation suppresses the
  internal Tonelli--Shanks multiplication counter so the request is not double-priced.
- lookup: 4.
- modular operation: both `M_ell=1` and `M_ell=9`.
- Within sparse elimination, each modular multiplication and addition/subtraction is one
  emitted operation; Fermat inversions contribute the exact number of Montgomery
  multiplications performed by the committed binary (`bitlength(ell-2)+popcount(ell-2)`).
- native counters: `hash_lookups + hash_inserts + rng_draws + uf_ops + root_calls +
  normalizations + lp_sge_entries + rank_entries + sort_items`; price at each of `0`,
  `0.1`, and `1` group-addition equivalent under repository §5a. Sorting is counted by
  the exact number of items presented to a committed sort, not an inferred comparator
  count. At each q, one explicit plane-quartic addition is priced as
  `163 M_q + 2 I_q`, hence `181`, `187`, `195`, and `207 M_q` at the four frozen sizes.
- Independent verification arithmetic is emitted and priced but excluded from attack
  totals. Instance search/order certification is an auditable setup cost, reported
  separately from the attack projection with canonical field-construction arithmetic,
  primality modular work, field operations, BSGS hash/sort, SplitMix, and native-band
  receipts; duplicate retry streams are accumulated rather than discarded.

Each curve therefore emits six complete attack receipts (`M_ell` crossed with the
native band), raw counters, and verification cost. A comparison is rankable only if its
order is stable over the complete band on both sides. The historical rho record lacks
the native counters required by §5a, so the same-instance rho formula below is labelled
a lower-bound relation-only reference, not a complete-cost win. The canonical export
retains both the attack receipt and the attack-plus-instance-builder sum for every band;
the latter does not silently convert setup into an uncharged cost.

## Matched references

- Same-instance rho relation-only model:
  `sqrt(pi*ell/2)*74.5 M_q`, using each measured `ell`. This is distinct from the
  evaluator's cross-instance Item-7 mean-cofactor value.
- q-matched round-4 aggregate hyperelliptic incumbent at `2^15`:
  `9.967e10*(2^15/2^22)^1.408 = 1.0755e8 M_q` (minor display rounding only).
  This is the evaluator's frozen aggregate formula; no hyperelliptic binary is rerun on
  each exact prospective elliptic curve, so it is not represented as an exact-E
  measurement.
- The current C1+C2 hyperelliptic row is reported as context, not substituted into the
  evaluator's named incumbent gate.
- Quartic projection at `2^15`: `7.3e6 + 0.53e6*w`, explicitly an extrapolation using
  the measured post-LP-SGE `w`.

## Decision rules (K1–K4)

K1--K3 use exact integers. K4 forms the six-curve mean as a reduced rational and
compares by integer cross-products. To avoid a platform-dependent `powf` at analysis
time, the high-precision evaluation of the named hyperelliptic formula is frozen as the
canonical decimal `107550595.89844453 = 10755059589844453/10^8`; its exact `0.6`
threshold is `64530357.539066718 = 32265178769533359/(5*10^8)`. Display floats are
descriptive only. `k4-reference.json` retains the 80-digit pre-data evaluation,
rounding rule, canonical rationals, and integer-only verdict rule.

- **K1 (kill):** for any size, the arithmetic mean over its six curves of
  `unique_PP/pairs` is below `0.20`.
- **K2 (kill):** any valid curve has
  `usable_cycles_including_full < N+2` after the complete pair harvest.
- **K3 (kill):** any valid curve has exact `nullity != 1`.
- **K4 (kill):** at `q=32771`, with `w` the six-curve arithmetic mean decisive row
  weight, `7.3e6+0.53e6*w` exceeds
  the frozen canonical evaluation of
  `0.6 * 9.967e10*(2^15/2^22)^1.408`, exactly `64530357.539066718`.

Verdict vocabulary:

- `advance`: all K gates survive, all 24 cells are valid, and complete evidence exists;
- `new_row_or_reject`: at least one K gate fires, with the exact gate named;
- `invalid_or_incomplete`: missing curves, verification failures, protocol deviations,
  or absent required artifacts.

There is no optional stopping, no curve deletion, no post-hoc seed change, no smaller
substitute for the `q=4194301` harvest, and no inference from partial cells.

## Execution and artifacts

The exact byte-level launch and recovery interface is specified in `LINEAGE.md`.

- Build the committed `lead15_diem_quartic` example in release mode once per immutable
  image; record commit and binary SHA-256.
- Backend: the supported non-resumable `isolab_k8s_run` custom-job route on Iso Lab EKS
  Spot, with a digest-pinned image, the tracked wrapper as the sole container command,
  exact argv, `retries=0`, one scientific thread per size cell,
  durable S3 upload, and four size cells submitted concurrently if governance/quotas
  permit. Each Job requests 4 CPU and 16 GiB on Spot; those are scheduler requests, not
  a delivered-hardware, affinity, cache, instance-type, or isolation claim. The exact
  wrapper-enforced scientific-child deadline is `3600` seconds per size; it is a
  censoring rule and is not the runtime estimate. The Job uses
  `activeDeadlineSeconds=7200` and a 60-second termination grace period so scheduling,
  wrapper startup, and durable final upload do not consume the scientific child budget.
  The launcher injects `RAYON_NUM_THREADS=4` and `OMP_NUM_THREADS=4` because the Job
  requests four CPUs; the tracked wrapper is the sole command and overrides both to
  `1` for the native scientific child, recording that child environment in
  `worker-start.json`. Timeout is a prespecified
  terminal, censored/incomplete cell outcome: it receives no
  replacement or retry and cannot be described as an algorithmic failure or as zero
  cost or a performance datum. The serial resumable controller is not substituted.
- Evaluator e5 gives the explicitly unvalidated desk estimate “run under 1 CPU-hour.”
  For canonical preflight metadata only, this is encoded as a conservative 3600-second
  total planning ceiling and 150 seconds per one of 24 curves. It was not measured,
  is not a performance claim, and is independent of the 3600-second per-size censor.
- The application checkpoint policy is `none`. JSONL is sync-written after every
  instance/curve. An infrastructure failure before the scientific deadline is preserved
  as partial evidence and may be reconciled only under the original request/Job identity;
  it does not authorize another scientific measurement. Scientific failures and
  deadline expirations are never retried.
- Before submission, checkpoint the frozen source commit, canonical protocol, manifest,
  image digest, and exact request bytes to private S3 and verify their versioned
  read-back receipts. Workers write JSONL, environment, stdout/stderr, resource, and
  terminal/partial manifests to the exact injected
  `s3://isolab-results-104186811093-us-east-2/runs/<job>/<pod>/` prefix. Collection
  retains and locally hashes the exact request, request head, submission response, raw
  Job, raw Pod when created, terminal status, complete paginated result listing,
  per-object heads/version IDs, downloaded worker artifacts, and payload bytes.
- Required derived artifacts: validated raw JSONL, instance manifest, analysis JSON,
  K1–K4 receipt, complete-cost table, report source/PDF, one explanatory SVG, source
  manifest, and session-log update.
- `source-manifest.json` binds its frozen status, instrument/freeze commits, canonical
  and raw-file protocol hashes, driver/analysis and library-source hashes,
  release-binary/build-info/build-schema hash, worker and collector hashes,
  externally bound image URI/digest,
  the exact K4-reference and evidence-schema hashes, exact argv template (whose only
  post-freeze substitution is the manifest byte hash), fixed Job name/request URI/result
  prefix, both deadlines, retry count, checkpoint receipt, requested resources, and
  durable results root. The request hash is deliberately not embedded in the manifest
  or request argv, which would create a self-referential hash cycle. Each
  immutable completion receipt binds that
  manifest hash and a versioned, read-back-verified request artifact to locally rehashed
  raw Job/Pod/status/result/head/worker bytes, Job/Pod UIDs, node, restart count,
  terminal class, and the payload S3 URI/SHA-256/byte count/ETag/version ID.
  Strict analysis accepts the protocol, manifest, exactly four completion receipts and
  four JSONL payloads. It parses only the newline-terminated prefix and preserves any
  trailing fragment by digest. Any size other than `complete_valid` makes the whole study
  `invalid_or_incomplete` and K1--K4 `not_evaluated`; partial records never determine the
  terminal cause.
- Canonical flow records `planned=24` and emits up to 24 evidenced attempted candidate
  rows, all with `wall_valid=false`. A terminal cell attempt may fan out only to the
  durable per-instance identities actually authenticated for that attempt. A zero-Pod
  cell, a missing per-instance identity, or an unauthenticated record produces no
  fictional run row; the cell-level classification and planned-minus-observed count
  preserve the incompleteness. The paired formula-reference values are analysis-only
  columns/receipts attached to evidenced candidate rows; they are not synthetic
  executions. Actual CPU, memory, kernel, node, Job and Pod facts are provenance only.

## Invalidation and forbidden analyses

Invalidate the affected curve or size for: wrong pair count; duplicate target quartic;
failed common-branch check; non-prime `ell`; cofactor other than four; line-verification
failure; retention-cap materialization below `N+2` when the complete graph has at least
`N+2` usable relations; arithmetic panic; output failing `result.schema.json`; missing or
invalid `LEAD15_COMMIT`, `LEAD15_PROTOCOL_SHA256`, `LEAD15_BINARY_SHA256`, or
`LEAD15_IMAGE_DIGEST`;
binary/source/protocol mismatch; missing completion record; or any replacement/retry.

Forbidden: choosing a more favourable q near `2^22`; changing `N`, retention cap, seed,
candidate cap, curve count, cost weights, or K rules after observations; reporting raw
PP as unique PP; treating wall time as the headline; calling the historical rho formula
a complete reference; combining the unfinished 2026-10-18 quartic run; or describing a
projection as a measurement.

## Primary sources and prior evidence

1. C. Diem and E. Thomé, *Index calculus in class groups of non-hyperelliptic curves of
   genus three*, J. Cryptology 21 (2008), 593–611. Archived full text:
   <https://web.archive.org/web/20220101000000id_/https://hal.inria.fr/inria-00107290/document>.
2. C. Diem, *On the discrete logarithm problem for plane curves*, JTNB 24(3) (2012),
   639–667, DOI 10.5802/jtnb.815:
   <https://jtnb.centre-mersenne.org/item/10.5802/jtnb.815.pdf>.
3. C. Diem, preprint of the plane-curve algorithm:
   <https://arxiv.org/abs/math/0606607>.
4. S. Flon, R. Oyono and C. Ritzenthaler, explicit genus-3 arithmetic:
   <https://eprint.iacr.org/2005/276.pdf>.
5. B. Smith, genus-3 non-hyperelliptic Jacobian arithmetic:
   <https://arxiv.org/abs/0806.2995>.
6. Repository evaluator and provenance:
   `research/pdp_strategy_evaluation_20261031/cards/e5.md`,
   `research/pdp_literature_leads_20261022/leads/LEAD-15.md`,
   `research/ghs_cubic_cover_census_20261012/`,
   `research/ghs_cubic_hyperelliptic_genus3_20261013/`, and
   `research/ic_improve4_20261015/01-genus3-relations/`.
