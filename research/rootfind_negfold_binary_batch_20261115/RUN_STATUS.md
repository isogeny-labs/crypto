# K13 binary batch candidate: admission state

**2026-11-15 15:30 PST — protocol and spec prepared before data; no job submitted.**

The native release `ecbench plan` accepted [the fixed spec](SPEC-K13.json)
as `ECS1h4f4e157aa5aa`: six arms, eight registered public Koblitz E0 n13
targets and 336 planned executions. The spec SHA-256 is
`36c579d68884e371143bf7ad07929caa48cf48608ee2b1c2aa8b8e44ee365105`;
the ecbench canonical spec SHA-256 is
`4f4e157aa5aa667b75defcef52ddd899c7494826da5bbcb3f9d99107bbba1783`.
The protocol file SHA-256 is
`9b1db00526d19d4ad3042038ae25b2e6dab40480d66ee56ff37789bf311515ff`.
The [protocol](PROTOCOL.md) specifies same-target scalar/batch comparisons,
cost completeness, relation controls, L2/A/A wall admission, independent
replay, fixed stopping, and the USD 5 new-job ceiling. Its source and protocol
publication [receipt](publication.json) checks the listed source, protocol
and spec paths against pushed commit `2f242cf2728f6091461441d2687688158778f98f`.
The launch request and exact image/binary hashes remain to be bound at
admission. No session directory or outcome row exists here.

**Before-data source amendment:** [`AMENDMENT-1.md`](AMENDMENT-1.md)
selects the lazy-sum scan at source revision
`c30a899bce6e74fe4574f042016ed7b6ddd1e1ac` and clarifies that the
ordinate is completed only after a table-key hit. The original protocol,
spec and receipt remain intact. Run only after the separate amended
publication receipt is committed and checked; do not pool eager-sum and
lazy-sum revisions.

**Second before-data amendment:** [`AMENDMENT-2.md`](AMENDMENT-2.md) applies
an x-only lazy sum to the candidate's binary pair-table build as well. The
original protocol and spec remain fixed. This revision requires its own
`publication-amendment-2.json` receipt and the stated release parity gate
before execution; earlier publication receipts keep their original scope.

**Third before-data amendment:** [`AMENDMENT-3.md`](AMENDMENT-3.md) corrects
the primary decision metric for this batch-implementation question.
`bitop/v1` prices the unchanged abstract binary additions at a fixed
affine weight and cannot recognize shared inversions or omitted ordinate
products. The m=3 complete cold L2 wall comparison, with its A/A and
target-cluster gates, is now primary; m=2 is descriptive because the fixed
spec has no m=2 A/A arm. The [cost-model review](COST_MODEL_REVIEW_20261115.md)
records the derivation. A new publication receipt must bind this amendment
before a study job is admitted.

**Fourth before-data amendment:** [`AMENDMENT-4.md`](AMENDMENT-4.md)
deduplicates adjacent signed-pair denominators in the candidate's `m=3`
lazy scan. The block still pays one field inversion, while an ordinary
full K13 eight-point block has five denominator entries instead of eight
in the batch product chain on the portable path. Its SIMD path retains
the existing kernel. The source-level arithmetic count, correctness
checks and unchanged wall decision are recorded there. A new pushed-source
receipt including Amendment 4 is required before execution.

**Fifth before-data amendment:** [`AMENDMENT-5.md`](AMENDMENT-5.md)
records the post-merge source revision and its cached point-lift integration.
It leaves the six-arm spec and Amendment 3 wall decision unchanged. The
merged release library, point-lift order and candidate first-witness tests
passed; a new pushed-source `publication-amendment-5.json` receipt must bind
this exact source before any scientific execution. Earlier receipts remain
valid only for their listed revisions.

**Sixth before-data amendment:** [`AMENDMENT-6.md`](AMENDMENT-6.md)
aligns the candidate's `m=3` scan to complete signed-pair blocks after its
initial self-negative point, allowing the existing x86 paired SIMD kernel
to run when its CPU, field and input conditions hold. The
[source review](SIMD_PAIR_SCAN_REVIEW_20261115.md) records the route,
fallback and checks. The fixed spec and Amendment 3 wall decision remain
unchanged. A separate `publication-amendment-6.json` receipt must bind
this source before a scientific job; earlier receipts remain historical.

**Seventh before-data amendment:** [`AMENDMENT-7.md`](AMENDMENT-7.md)
peels the exceptional same-abscissa group from each candidate binary
pair-table row, then offers complete signed pairs to the existing x-only
paired SIMD routine. The [source review](PAIR_TABLE_DIAGONAL_REVIEW_20261115.md)
records the route and checks. The fixed spec and Amendment 3 wall decision
remain unchanged. This revision needs a separate pushed-source
`publication-amendment-7.json` receipt before a scientific job.

## Admission obstruction

The read-only [AWS monthly budget response](budget-monthly-readback.json)
from account `104186811093`, profile `ecc-challenges-new`, reports USD 1,982.224
calculated spend against a USD 1,300 limit, with AWS's last-update time
2026-11-15 13:10 PST. The exact command was
`aws budgets describe-budget --account-id 104186811093 --budget-name isolab-monthly-account-cost --profile ecc-challenges-new --query 'Budget.{BudgetName:BudgetName,BudgetLimit:BudgetLimit,CalculatedSpend:CalculatedSpend,LastUpdatedTime:LastUpdatedTime}' --output json`.
`taskq/deploy/eks-karpenter/COST_CONTROLS.md` forbids a new image build or
job while remaining headroom is exhausted or unknown. The monthly balance
alone stops this study; no alternate host or smaller design substitutes for
the stated L2 run. This is a resource admission condition, not evidence
about the candidate's speed.

The existing [complete-cost problem](../../problems/rootfinding-complete-cost-calibration.md)
also tracks native pricing, isolated wall and independent-replay obligations.
Its index row is currently held by Conductor T-6315, so this scoped PR leaves
that shared record and index unchanged. The original [F4/F5/F6 study](../rootfind_f456_paired_20261115/RUN_STATUS.md)
has its own frozen source and spec and remains separately gated.

## Checkable resume

Re-read both daily and monthly AWS balances and charged/reserved costs; admit
the USD 5 ceiling only with known headroom in both. Validate this study's
published source/spec receipt, check for an existing request or Job with this
spec hash and source revision, and run the live `cpu-l2-spot` pool check for
four CPUs and requested memory. Submit one native EKS L2 job only after all
gates pass. Preserve every execution and any censored row, audit all target
answers and costs, then produce the paired report, graph, PDF and independent
second-host replay before an advantage claim is considered. A changed budget
readback affects admission only.
