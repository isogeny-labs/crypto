# CT26 — defer and restrict the reference quadratic root table

Prospective engineering work after CT25 publication. No implementation,
timing or cost improvement is accepted by this plan. [pcc:unverified]

## Retained negative assessment

Reading exact43a nonsplit_map.rs and closed_fibres.rs shows that the
lower-degree fibre/infinity methods already use polynomial formulas.
Degree0/1 evaluation does not need nontrivial source-root lookup. The
accepted R0/R1/R2 prime-input graphs have239 lower-degree/12078 quadratic
routes and zero fallback each way. Rewriting those lower-degree formulas
as a claimed removal of root finding would change no computation.
The source-audit record preserves this null candidate. [pcc:unverified]

The actual constructor eagerly enumerates allp² Fp2 elements and inserts
their squares into a BTreeMap. Only the old pointwise evaluator's degree2
source-root path reads that table. The completed closed degree2 chart and
degree0/1 routes leave it unused on the accepted prime-input domains.
This is a static path/property observation,not a measured speedup.
[pcc:unverified]

## Discriminating implementation attempt

CT25 is now65aabe34879630ad4d973021fb21307e1d856dc1,ready1496 against1494.
Focused43136703928 starts on that exact head;next observation~02:37.
Its fresh child is research/chained-transfer-root-cache-20261112.
Use a lazy std initializer owned by the same immutable map context.
No new field,polynomial,group or square-root engine is allowed.

Before implementation,the refined hypothesis is that the typed FpPoly
root API only queries base-field discriminants. For an Fp2 element a+b*s
with s² a nonsquare,the square's second coefficient is2ab. Over an odd
field a base-field square therefore forces a0 or b0. Predict that filtering
the old lexicographic root enumeration to these axes preserves its FIRST
root for every queried discriminant. The candidate enumerates(0,b) for
b0..p-1,then(a,0) for a1..p-1:2p-1 candidate squares. The original p²
builder remains a test reference;no cost or correctness result is accepted
until the native controls and written/formal proof map are recorded.

Preserve canonical root choice,field arithmetic,pointwise evaluator,
fallbacks and all global map formulas. Compare candidate and original
tables for EVERY base discriminant at p17 and101,and check all returned
roots really square to their keys. Nonreal keys are outside the typed
query scope;do not claim a general Fp2 square-root table.

Require a complete closed production evaluation to leave the quadratic
table uninitialized;degree0/1 old reference calls should do the same.
The first degree2 reference/fallback call must initialize the restricted
table and produce identical roots/images. For allsix R0/R1/R2 full input
domains use SEPARATE fresh production/reference map instances,so every
production call can assert its cache remains uninitialized while genuine
pointwise reference calls initialize theirs. Keep dual/group/source/
geometric/exception controls and original fallback behavior. Do not weaken
a reference to avoid its initialization or add an ignored test.

Retain source snapshots/raw receipts/failures,PCC,source-linked report/
diagram/inspectedPDF and ready stackedPR/exact-headCI. If the table is
already needed by any accepted closed path,preserve that finding and
reconsider the proposal;do not silently narrow the evaluator's scope.

This can establish avoided eager allocation/precomputation on a specific
production path. A faster stage,pipeline or ECDLP algorithm still requires
an admitted frozen matched protocol against the actual baseline and rho,
including constructor/setup/conversion/recovery costs,A/A controls and
verified answers. Current S3/livebudget admission remains unavailable.
[pcc:unverified]

Independent isogeny-map/kernel-membership replay,arbitrary-class second
engine coverage,large-prime/catalog support,full geometric Lean and frozen
ledger/vault integration remain open. No further toy hop replaces them.
