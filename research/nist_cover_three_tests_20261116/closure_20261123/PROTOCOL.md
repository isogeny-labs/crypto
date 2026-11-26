# Pencil closure: bounded static/native qualification

Preserve the proposal and all earlier source, protocol, reports and data.
No new P-192 point search, line factorization, relation collection, timing
campaign, unknown-scalar recovery or paid compute is authorized by this file.
This is a development audit of necessary rank conditions on existing data
and explicitly synthetic matrices, under the original goal's static-check
scope. No performance claim follows. [pcc:unverified]

Question: when can proposed pencil rows survive elimination of all columns
outside the original projected base? A distinct line through the same R
has no other geometric intersection with another such line. Thus simple
residual supports from a single pencil are disjoint on the quartic.
Projection through the degree6 elliptic map can still merge images, with
signs; disjoint geometric support does not establish disjoint projected
columns. Do not infer an actual P-192 collision result from this geometry.

Before data, fix six synthetic matrix controls over the standard P-192
prime subgroup order: private new columns (closed-rank gain0), a shared
new column (gain1), opposite signed alias (gain1), unsigned three-cycle
(gain0), signed three-cycle (gain1), and an already-canonicalized row with
zero new coefficients (gain1). Every case has the same original rank1
row on three original columns. Rank is recomputed by native GMP elimination
both before and after restricting to new columns. Record both ranks and the
dimension of combinations with zero new-column part; compare explicit
expected gains. These are algebra fixtures, not curve principal relations.

Generate a peeling certificate listing active singleton new columns and
their owning rows. Its checker must reject four corruptions: wrong owner,
shared column falsely named singleton, an original column named new, and
a repeated deleted owner. Reconstruct the original matrix at every check;
nonzero means nonzero modulo the declared subgroup order.

Read only the frozen original relation JSON at
replay_20261123/evidence/ci-replay/native/relations/result.json, reconstruct
its sparse225-column matrix and require its reported rank6. Do not append
synthetic rows to that actual matrix or describe a synthetic case as a
new curve observation. The original target/elliptic checks remain unchanged.

Emit an evaluated identity.certificate/v1 for the cleared singleton step:
lambda*(w*v-1)=lambda*w*v-lambda. If w*v=1 and the private-column equation
lambda*w=0 holds, then lambda=0. The identity record concerns that polynomial
step over its declared field2^61-1, not a formal rank proof or an independent
P-192 verification. All case results retain producer standing.

Native worker:30second/1GiB cap, fresh write-once output directory, strict
C++17 compiler warnings, frozen source/input/binary hashes, raw output and
exit. Only one qualification call unless an actual source/build failure
requires a fresh successor. Preserve failure. No statistical measurements.

Read-only lab check: once, run the changed main telemetry workflow deployed
by merged PR4983; preserve exact source/run/error/outcomes. Step conclusion
success under continue-on-error does not establish credential success;
inspect actual outcomes and the final gate. Never publish credential values,
signed artifact URLs or private budget amounts. No new trust, IAM or budget.

Sources: frozen pencil protocol and existing RELATION_PIPELINE/README
negative torsion controls, native exact-rank checker, and ordinary linear
algebra's row-space projection and singleton elimination. KB wrapper remains
unavailable. This audit proposes no new general graph algorithm.
