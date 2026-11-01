# Direct signs expose the signed-cache tradeoff

2026-10-25 · run `direct-sign-20261025` · existing PR2381 ·
[protocol](../../DIRECT_SIGN_20261025_PROTOCOL.md) · [PDF](report.pdf) ·
[graph](signs.svg) · [diagram](bridge.svg) · [proposed route](route.svg) ·
[native cases/audit](../../native/direct-sign-20261025/) ·
[validation/reproduction](../../validation/direct-sign-20261025/README.md).

The C++ producer and independent Rust arithmetic execute **120 setups and1,200
separate nonidentity main-source recoveries**,40/400 per arm: frozen binary
signs, frozen setup-validated signed cache, and new direct uncached signs.
All ten F43 size-two policies and four direct/transported roles remain.
All1,200 scalars verify. Both old arms match their frozen records after removing
only two newly emitted common counters and the new tested-sign trace hash.
The latter binds every signed operand/sum/acceptance test, including rejections.
Every ordered lift, root, row/rank, full certificate, degree/witness/promotion,
base log, failed shift and source answer is identical across all three arms.
The additional batch-RREF/witness checker audits100 distinct solver certificates.

## Observed tradeoff against the stronger reference

Direct signs use P for plus and one native negation for minus. They do not call
the binary scalar loop or build a signed cache. Every lift still performs its
actual point sum and all sign tests, including after a first successful lift
within that query. Cached setup remains charged exactly as in the prior round.

| Prefix policy, separate Q=P; cold request unit | Binary | Direct | Cached |
| --- | ---: | ---: | ---: |
| Main point-add requests |125|45|47|
| Twist point-add requests |136|40|42|
| Main negation requests |20|20|2|
| Twist negation requests |24|24|2|
| Main scalar-loop iterations |54|14|14|
| Twist scalar-loop iterations |56|8|8|
| Main signed-operand reads, all arms |40|40|40|
| Twist signed-operand reads, all arms |48|48|48|

**All400 direct cold records use two fewer point-add requests than cached;
all400 cached records use fewer negations than direct.** The two cached inverse
validation additions explain the first difference. With L signed tests, direct
removes4L addition requests and2L scalar-loop iterations relative to binary,
while retaining its L sign negations. Cached and direct have equal scalar-loop
iterations; caching replaces L negations by2 at setup and adds its validation,
storage and read requests. Neither variant dominates all request components.

Hypothetical cold main/twist addition sums are11750/11990 (binary),4398/4638
(cached) and4198/4438 (direct). Negation sums are1888/1888,200/200 and1888/1888
respectively. Each sum describes100 separate one-target records per role, not
batch throughput,400 repeated cold setups or independent curves. Main/twist
direct addition lower/equal/higher counts remain43/1/56, as for cached.
Matrix slots remain540/36 for the first policy/target, with first-query useful
rank0/2 and reduced operational degree3/2 retained from earlier finite evidence.
Both full setups ultimately reach coefficient rank2. No new intrinsic regularity
or stationary useful-yield result is supplied by changing the sign interface.

The prior125→47 and136→42 component reductions against the binary reference
remain valid. This control shows that those reductions cannot alone establish
the value of caching: uncached direct signs lower additions still further.
The old loop includes identity additions and unused final doublings, which
compilers may optimize. Requests are not retired instructions or timings.
The38 components have different/nested costs, and are not summed as equal-cost
operations. Cache storage/curve checks/reads, field tables, hashing/trace work,
allocation/sorting/traversal, exponentiation and independent audit remain unpriced.
No overall ranking, calibrated engineering gain, speedup, GAE/S/rho or fastest-
external claim is admitted. Exact weighted complete work remains an obligation.

## Correctness and existing proofs

The actual main `icv1-fp6-t0-74779950` is E/F43:y²=x³+x, P=(31,18), and
twist `icv1-fp6-t0-c6a8a47c` is T:Y²=X³+4X, U=(24,26), selected r11.
The existing additive/injective phi=(41x,11y) preserves the source scalar.
This exceptional pair is base-field-isomorphic; U has no registered EC1
representation and no older generator alias is reused. Recovery receives Q
alone; known labels only generate/check public synthetic points outside it.

Direct is exactly the existing `SignedCache.signedPoint` branch. Its unchanged
`signed_eq_zsmul`, `ordered_lifts_eq`, cache-validation and actual F43 recovery
theorems already establish the required structural implications, with the source
annihilator explicit. No duplicate theorem or new polynomial identity is claimed.
The unmodified scoped gate again passes **191 public named theorems /249 audited
declarations**,0 failing or extra trusted axioms. All16 identity records/sources
remain unchanged; the existing coordinate IDC1h202bb532af10c977 is checked again
with its unchanged statement/source and false identity/hash controls.
Only propext/Classical.choice/Quot.sound occur; native counters are independently
replayed, not universally Lean-formalized.

The new common `sign_operand_reads` and `sign_dispatches` counters equal2L
in every arm. Old `cache_reads` is a nested cache-only view, not a second equal-
cost read to add to the common counter. Query hashes are independently rebuilt
from all ordered `(i,j,si,sj,A,B,sum,accepted)` tests and do not feed answers to
the solver. Full certificates are still source-bound by actual target x,
with an independent semantic witness audit beyond their hashes.

Seven rehashed controls reject101/no receipt: false direct trace hash, false
direct demand, on-curve cached point in wrong sign slot, missing direct arm,
missing direct failed query, false direct scalar and stale source. The exact
fresh replay,22 backend tests,4 workflow metadata tests and fresh full release
library checks are retained in validation. No native failure/cap is hidden.

## Next return to the actual transfer/yield question

The next selected lead uses already registered F13 models: source
`icv1-fp4-t0-6bee67e0` (a1,b4), intermediate `icv1-fp4-t0-bb514d71` (a4,b7)
and target `icv1-fp4-t0-097da6b6` (a4,b6), the d2 monic quadratic twist.
A standard degree-two coordinate candidate, after translating the rational
2-torsion point(10,0) and scaling by u5, is

    Phi(x,y)=(12*(x+2/(x-10)),8*y*(1-2/(x-10)^2)).

This route is **proposed**, not executed here. Verify infinity/kernel/poles,
actual curve equations, r7 subgroup transport/injectivity and the exact
non-isomorphism claim before any main-DLP benefit claim. Then freeze all
size-two bases, source targets and matched/pullback controls for useful S3
rank/degree comparisons. All three models and the original prefix case already
exist, so no registry/catalog mutation or new store upload is needed. The atlas
already covers transported-base conservation; no invention or automatic yield
advantage is claimed. The cubic j0 alternative remains a separate unexecuted
draft, deferred behind this registered seed.
The profile probe remains unavailable through missing gawk; no profile/cost
evidence is substituted. Shared Conductor still exits127, no reservation or
replacement claimed. No paid resources, upload or credential reuse. The
scientific, review, CI-access and integration completion holds remain.
Canonical scoreboard/leaderboard/browser axes carry no new calibrated claim,
model or session in this round, so they remain unchanged; this study graph
records every source target of the first policy and the raw data retain all policies.
