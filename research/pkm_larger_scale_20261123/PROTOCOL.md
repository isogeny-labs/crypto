# Larger-scale PKM replication: admission draft, 2026-11-23

Status: NOT LAUNCHED. This is a concrete proposed panel, not an execution receipt
or a frozen executable protocol. User request: test #5067 at much larger scale,
resolve source reproduction, and test transfer on untouched curves before a
Prime31 independent-rank and complete cold IC/rho comparison. [pcc:unverified]

## Proposed population and sequence

1. Build the original `ic` consumer from identified source with locked dependencies
   and Rust 1.99.1. Retain source commit/tree, full build streams, toolchain,
   binary SHA-256 and OCI digest separately. Repair unrelated compilation only
   through its owning lane; do not remove modules to make the benchmark build.
   A newer source revision must explicitly list changes from #5067.
2. Reproduce the original 128-scalar 10-bit panel, using the exact scalar list in
   `../pkm_adaptive_sweep_20261123/replication_height4_random.jsonl`.
   This is source reproduction, not independent confirmation. Preserve both
   successful and failed rows and the old executable's unknown source provenance.
3. Independently test subgroup-size rungs 14, 16, 20, 24, 28 and 31 bits, five
   untouched eligible ordinary prime-field curves per rung, 128 unique uniform
   nonzero subgroup scalars per curve: 30 curves and 3,840 targets per arm/round.
   A separate Prime31 population uses exactly p=2147483647 and at least three
   eligible ordinary curves; it must not be silently identified with the
   generic 31-bit subgroup rung. Prime31 uses 128 targets per curve.
4. Fix candidate families before any timed call: (A) the original degree-16
   ordinary four-step 2-isogeny construction and (B) ordinary 2-isogeny towers
   with t=ceil(log2(r)/3). Family B tests an explicit size-dependent policy;
   family A tests transfer of the original construction. Do not pool them.
   On each curve, inspect auxiliary seeds 0..63 in order, using only construction
   and size information; retain the first exact match to the lifting-abscissa
   count of an interval below 2^t. For A, t=4. If no exact match exists, retain
   the curve as an applicability failure. Never choose a replacement by timing,
   relation yield or scalar recovery. Do not switch seeds after a loss.
5. Compare each candidate with its own exact-size interval, using identical
   m=2 Semaev chain F4, degree bound 8 and per-system five-second cap. Separately
   compare complete cold solving with the strongest reproducible existing IC
   pipeline and optimized single-target negation-map rho on identical targets.
   Identify the incumbent recipe/commit and external solver reference before
   admission. The degree-16 interval is a component control, not automatically
   the best IC incumbent. Do not infer asymptotic behavior from fixed-degree A.

## Workload freeze and identity

Generate curves and scalar panels with a native Rust implementation. Freeze
all p,a,b,r,h,P,Q and scalars plus ICV1 identities and hashes in a new immutable
workload manifest before timing. Use separate deterministic seed namespaces
`pkm-large-v1/curve/<rung>/<index>` and `pkm-large-v1/scalar/<curve-id>`;
record the RNG algorithm/version and use rejection sampling. A known-answer
scalar is validation data and must not enter the solver. Reject duplicate or
previously used curves/scalars according to their exact identities. Do not
claim the new panel is independently sampled until this audit succeeds.

The old `ic bench` couples generated curve identity to seed AND repeats. Its
scalar stream is deterministic consecutive planting. Simply increasing --bits,
--seed or --repeats in the Python runner does not implement this protocol.
Use explicit fixed-instance native APIs or add an audited manifest consumer.
The native replacement must preserve the old panel as historical evidence.

## Controls, cost and acceptance

Use EKS Spot with native L2 admission, fresh warmup, five measured rounds,
balanced randomized arm order and matching A/A control in every epoch. Replay
all answers independently via [k]P=Q. Interruption invalidates that timing epoch;
restore computation only through the supported CRIU/EBS contract. Do not retry
measurement selectively. Freeze exact L2/A-A thresholds from the maintained
harness contract in the executable launch spec before admission.

Retain raw relation rows and report independent relation rank, duplicates,
rank gained per attempt and per charged unit, matrix dimensions, sparse
elimination cost, recovery and verification. Relation count alone is not rank.
Record cold map construction, preflight/selection, calibration, failed attempts,
encoding, F4, extraction/lifting, filtering, matrix work and final recovery as
exclusive phases; separately retain wall, child CPU, F4 operations, peak memory
and GAE pricing-band readings. Include the strongest matched rho in that band.

For each size and family, report candidate/control ratios of summed cold cost
and curve-cluster bootstrap intervals (10,000 draws, resampling curves then
paired targets; five rounds stay in their target cluster). Report per-curve
outcomes too. Proposed familywise alpha=0.01 across 12 size/family contrasts:
use Bonferroni intervals at 1-0.01/12 for decisions and descriptive 99%
intervals alongside. A component success requires complete identical verified
workloads, cost ratio <0.90, corrected upper bound <1, and wall and CPU ratios
<1. Failures/timeouts/OOMs remain in completion denominators; incomplete arms
cannot qualify. Full cold advantage uses the separately named best IC and rho
controls and all phases. No optional stopping or dropping losing curves.

## Proposed finite envelope (not a spending override)

Maximum 4 requested CPUs/16 GiB per serial measurement job, concurrency 1,
zero automatic retries, 4,800-second maximum per job and 2 hours aggregate
worker time including failed attempts; total campaign/build ceiling USD 20,
subject to the smaller live daily/monthly remaining allowance. Freeze an
absolute UTC deadline and split the immutable panel into supported job sizes
before launch. If the whole panel does not fit, retain censored rows and seek
an explicit extension; do not reduce targets or call a smoke test completion.
No build/job is admitted while remaining spend or reservations are unknown.

## Admission checklist and requirement state

| Requirement | State / exact next action |
| --- | --- |
| Original source/executable binding | Blocked: old git_commit=null; identified-source build and native replay required |
| Native fixed-instance consumer | Not implemented; preserve Python history and replace execution path before extending |
| Larger independent panel | Proposed above; exact manifest generation/audit remains |
| Best incumbent and external solver | Not frozen; identify exact reproducible recipes on each curve |
| Complete rank/cold-cost instrumentation | Audit required; missing values stay null |
| Live shared spend and reservations | Blocked: no AWS identity here; latest telemetry skipped budget reads |
| Live EKS capacity/L2/A-A/checkpoint | Not checked; no submission admitted |
| Larger measured outcome | Not attempted; zero new benchmark calls |

No original measurement or confidence interval is changed by this draft.
