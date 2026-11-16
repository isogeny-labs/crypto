# Amendment 30: defer impossible graph rank checkpoints

Freeze trial35 at p1009, fresh seed347. The paired baseline and candidate
run the same copied Rust executable and public-only fixture. Both use75%
small base, full p² B budget, exact collection rank stride128 and the same
bounded affine descent. The candidate uses `--structural-rank-gate`.
One cold process per arm, no warmups or automatic retries. [pcc:unverified]

An incremental graph helper tracks omitted-point vertices, components and
single-large-row anchored components. Its intended large-column rank lower
bound is V−C+A, giving an upper bound N−(V−C+A) on independent small-only
equations. The candidate defers a checkpoint when that bound is below
small_base−1. It advances the same source-stride schedule as the baseline
even when skipping, rather than choosing new check locations. Weighted
cycles can increase the true large-column rank: the bound is conservative,
not an exact rank estimate or readiness test. [pcc:unverified]

All remaining full graph filtering, original-relation checks, combination
certificate replay, exact rank, affine target solve and elliptic verification
remain required. The default entrypoint/options/serialized output remain
available. No RNG calls are added by the gate. Require exact original
source/descent/collection endpoint/checkpoint schedule identity between arms;
permitted full-checkpoint records must match the baseline. [pcc:unverified]

The separately copied Rust qualifier uses graph traversal and independent
dense large-column elimination, without the production union-find helper,
on every gate prefix. Reconstruct structural state and verify the bound
against actual large-column rank. Require all primitive phases to equal
baseline minus the skipped full-checkpoint vectors plus the gate's known
charged word events. Reject unexplained changes in any phase or an incorrect
gate charge. Parent-probe counts remain source emissions; the qualifier does
not turn them into a signed execution proof. [pcc:unverified]

Charge three initialized state arrays, support identity/coefficient reads,
actual parent probes and bound reads. Explicitly retain the remaining DSU
scalar/comparison/allocation/scheduling gap. All complete totals, comparative
eligibility, bands and ratios stay null/false. This is a graph-checkpoint
component comparison, distinct from T0 against the best IC pipeline and T1
against rho. A null result or increased component cost is retained.
[pcc:unverified]

Use a dedicated Runpod CPU4vCPU/8GiB at at most0.40USD/hour, pinned official
Rust image, first allowed CPU. Bounds:60s fixture,900s worker,1900s whole,
2700s runner. Install `procps` and finish all builds before timing. Native
transport controls check the actual two-arm wrapper's separate stdout/raw
capture. Publish source/spec/manifests before one immutable dispatch.
Preserve every status, failed/censored record and exact source/binary/raw
custody. Collect and hash-check before terminating the owned pod, using the
official v2 API/GraphQL fallback if the legacy REST host is inaccessible.
[pcc:unverified]

Sources checked(KB): unavailable exact/negative structural-rank receipts in
the session journal. Read actual rank-stop/filter/partial-support sources,
the JV primary paper sections3.2/3.3 and prior31/34checkpoint evidence.
Independent native controls cover anchored merges, weighted cycles where
the bound is strict, malformed support atomicity and generated matrix-rank
prefix comparisons. No new asymptotic, general-curve or novelty claim.
[pcc:unverified]
