# Setup-charged signed points preserve every actual S3 lift

2026-10-25 · run `signed-cache-20261025` · PR2381 ·
[protocol](../../SIGNED_CACHE_20261025_PROTOCOL.md) · [PDF](report.pdf) ·
[all-target graph](lifting.svg) · [diagram](bridge.svg) ·
[native records/audit](../../native/signed-cache-20261025/) ·
[Lean proof](../../lean-signed-cache/SignedCache.lean) ·
[validation and reproduction](../../validation/signed-cache-20261025/README.md).

The C++ producer and independent Rust implementation execute **80 paired setups
and 800 separate nonidentity main-source recoveries**:40/400 per arm, ten frozen
F43 size-two factor-base policies and four direct/transported roles. All 800
scalars verify. The reference is the previous checked-root candidate, with its
binary ±1 scalar chain retained exactly. Candidate and reference have identical
ordered signed lifts, roots, rows/ranks, complete solver certificates, operational
and expanded witness degrees, promotion traces, failed shifts, base logs and
source answers. An additional batch-RREF/witness checker audits all 100 distinct
solver certificates independently of the request replay.

## Observed requests, including setup

The candidate constructs two signed points per base column. It charges two
negations, four stored entries, four finite/canonical/on-curve checks, four
sign/source bindings and **two inverse-validation additions** once per setup.
Each tested lift then reads two cached points and still computes its actual
sum. It tests every sign, including those after the first successful lift in
a query. It stores neither relations nor target/scalar answers.

| Prefix policy, separate Q=P; request unit | Binary main | Cached main | Binary twist | Cached twist |
| --- | ---: | ---: | ---: | ---: |
| Setup point-add requests |79|33|73|27|
| Online point-add requests |44|12|61|13|
| Cold point-add requests |125|47|136|42|
| Cold negation requests |20|2|24|2|
| Cold scalar-loop iterations |54|14|56|8|
| Cold cache reads |0|40|0|48|
| Tested signed lifts |20|20|24|24|
| Represented matrix slots |540|540|36|36|

Each cached cold record additionally contains4 built entries,4 point checks
and4 bindings. All 400 candidate cold records use fewer point-add requests than
their own reference, including the two setup additions. Exact checked deltas
for a phase with L signed tests are: additions−4L (plus2 in setup/cold),
negations−L (plus2 in setup/cold), scalar-loop iterations−2L, cache reads+2L.
Every other old component is unchanged. Component ratios125/47 and136/42
are **point-add request ratios, not speedups**.

Across 100 unmatched main/twist policy-target records, the cached twist has
lower/equal/higher point-add requests in **43/1/56** cases, and represented
matrix slots in **44/15/41**. Scalar-loop iterations give33/10/57. All cold
negation counts tie. The first policy now has fewer twist additions than main,
but the broader population retains more twist losses than wins on this counter.
Summed hypothetical cold main/twist additions are 4398/4638, versus 11750/11990
for the frozen reference; each sum represents100 separate one-target records,
not a batch throughput or independent curves. Only40 setups per arm actually
execute. Matched and inverse-transported bases reproduce every component except
the explicit two coordinate-map multiplications per nonidentity source.

This is a real lifting-interface change with a verified request reduction
against the specified reference. **Calibrated complete-work improvement remains
unestablished.** The old ±1 binary loop includes an unused final doubling and
an identity addition per operand. A compiler may optimize unused arithmetic;
the ledger records algorithm requests, not retired instructions or runtime.
Direct uncached sign handling is a stronger untested reference and the next
discriminating control. The 36 counters differ in weight and may be nested;
cache storage/reads, curve-check coordinate primitives, field-table setup,
hashing, allocation, sorting/traversal and independent audit remain unpriced.
Do not sum them into a scalar time/GAE. No timing, S, rho ratio, intrinsic
regularity or fastest-external claim is made.

## Structural proof and actual source recovery

For a cache C of a group point P, validation requires C(plus)=P and
C(minus)+P=O. Lean derives C(minus)=−P from the latter, rather than assuming
the cached scalar action. It proves cached sums equal the original ±1 scalar
sums and **ordered filtered lift lists are identical** for any candidate list,
preserving duplicates and order. It then proves shifted source-scalar recovery
through an injective additive map, explicitly using rP=O for reduction modulo r.
Indices i,j are arbitrary and may repeat; the proof covers diagonal root pairs.

The concrete F43 cache contains twist base points(15,9),(17,6) and negatives
(15,34),(17,37). Mathlib nonsingular points, both inverse checks and actual
base-log equations B0=5U,B1=7U are kernel checked. The concrete recovery theorem
therefore exposes no unproved cache or base-log hypothesis. It still requires
an actual cached relation to the target and recorded shift, which the native
audit checks for every recovery.

| Statement | Declaration in `SignedCache` |
| --- | --- |
| Sign branch equals integer scalar action | `signed_eq_zsmul` |
| Validation determines the cache value | `validated_cache_value` |
| All cached pair sums equal old signed sums | `cached_sum_eq` |
| Exact ordered list of accepted lifts | `ordered_lifts_eq` |
| General annihilator-aware shifted recovery | `cached_relation_recovers_mod` |
| Actual F43 cache, base logs and source recovery | `F43.actual_cache_valid`, `F43.actual_base_logs`, `F43.actual_cached_recovery` |
| Coordinate inverse polynomial | `coordinate_inverse` |

Nine new public theorems pass. The unmodified scoped gate passes **191 public
named theorems /249 audited declarations**, zero failing or extra trusted
axioms; the new11 audited declarations include two generated helpers. Only
propext, Classical.choice and Quot.sound occur. Native request code is
independently replayed, not claimed universally formalized. The new source-bound
**IDC1h202bb532af10c977** checks y+(−y)=0 at 32+16 native points with false
identity/source controls; it certifies only that coordinate polynomial.
Group/list/recovery implications rest on Lean. All15 previous identities and
their sources remain unchanged, making16 study identity records.

## Coverage, controls and limits

Main `icv1-fp6-t0-74779950` is E/F43:y²=x³+x, P=(31,18); twist
`icv1-fp6-t0-c6a8a47c` is T:Y²=X³+4X, U=(24,26), both selected subgroups r11.
The existing actual map phi(x,y)=(41x,11y) is additive/injective with P mapped
to U. This is an exceptional base-field-isomorphic fixture. U has no registered
EC1 representation; no older generator alias is reused. The earlier first-query
useful-rank0/2 and reduced operational-degree3/2 facts are preserved, as are the
equal homogeneous cutoff2 and all-target losses. Setup eventually reaches
rank2 on both arms; the main's first-query rank0 is not its final setup rank.
Recovery receives only Q, with labels used outside it to generate/check public
known-answer targets.

The exact fresh replay and seven rehashed corruptions pass: false cached sign,
false cached point, missing entry, false read demand, stale source, omitted failed
query and false scalar all reject101/no receipt. The false-point control uses
a valid on-curve positive point in the wrong negative slot; this is not an
invalid-curve experiment.22 backend tests and workflow metadata checks pass.
Full library/proof/metadata receipts are linked in validation. The initial and
fresh native runs both pass; no arithmetic failure/cap is concealed. An earlier
compiled proof draft covered fixed columns; before execution it was generalized
to arbitrary/repeated columns and checked actual base logs. Both drafts/logs stay.

The profile probe still lacks gawk and supplies no cost evidence. Actual shared
Conductor exits127, no task lease or replacement coordinator. No uploads, paid
resources or credential reuse occurred. The next draft compares binary, direct
uncached and cached signs on identical targets, preserving this round as frozen
evidence. General geometry, stationary useful yield, intrinsic regularity,
calibrated complete work, review, CI access and integration remain open.

Canonical scoreboard/leaderboard/browser graphs were checked for applicability.
No calibrated performance/claim row, registered model, ecbench session or
benchmark changes here; their axes cannot represent these request vectors and
are left unchanged. The new study graph retains all ten separate source targets.
