# Common signed-base lookup protocol

2026-11-24. Local software qualification, not admitted timing evidence.

## Mechanism and scope

For a public residual R=[a]G+[b]Q that equals a signed factor-base point
sP_i, submit the single coefficient c_i=s to the existing relation consumer.
Its equation is s*x_i-b*d=a modulo the subgroup order, where x_i and d denote
the unknown logs of P_i and Q. The consumer independently replays the group
equality before row reduction. The driver never receives a known target scalar.

This is ordinary signed factor-base membership, not a new map or isogeny.
It changes the recovery policy to accept singletons as well as the original
four-leaf chart. It does not implement the original cofactor builder's missing
target-in-base four-leaf chart. Every arm gets exactly the same public BTreeMap
lookup, before invoking its original oracle. Misses preserve the old solver and
failure behavior; identities and duplicate residuals remain skipped/logged.

The optional `base-lookup` argument follows `account=SEED` or
`account-suite=SEED` and optional `draws=N`, before the explicit method names.
The unflagged path retains format version 1. The flagged path is format version
2 with a separate `base_lookup` interval, checks/hits, one-term rows, and an
explicit singleton-or-four-leaf scope. Its `calls` field counts four-leaf oracle
calls only. The existing `base_setup` interval includes dictionary construction;
lookup, consumer and process overhead remain included in their charged intervals.
The reference still pays its own pair-index setup.

## Hypothesis and falsification

The coverage hypothesis concerns the complete recovery policy:
all five frozen F127 target-in-base residuals should supply valid singleton rows
through a common lookup without invoking the unsupported builder on those hits.
A wrong sign/equality, lost failed query, an uncharged lookup, a changed unflagged
history, or a parent receipt accepting mislabelled counts falsifies the component.
Recovery draw counts are outcomes; fewer oracle calls alone cannot establish
lower complete cost, and no timing ratio is admitted by these local controls.

Before data collection, native tests exercise both signs of every signed base
point on both existing toy curves, wrong signs/right-hand sides, all five old
refusals in the fixed 12-draw mask prefix, misses with an unavailable oracle,
policy/count mutations and the unflagged historical reducer. No algebraic search
is used to find a scalar for these tests: singleton replay uses a public base
point as Q, and the prefix test uses the original public Q and mask generator.

## Fixed controls and bounds

Run two separately recorded suites once each from the same frozen executable:
first unflagged, then `base-lookup`. Each suite has Direct, Refined and
ProductionCofactorMsolve in that order. All arms use F127, selector depth 4,
seed 781815 and maximum 30 draws. The public curve is y²=x³+x+7, subgroup
order 109, G=(1,3), Q=(11,98). Stop each recovery on its first consumer scalar,
then perform the existing fresh final replay.

Refined's remaining four-leaf oracle still has the narrower four-distinct-coarse-
image scope. Direct/cofactor retain four distinct x-coordinates, disjoint from
the target x. The shared singleton prepass does not make those oracle scopes equal.

Each query keeps three seconds and unchanged solver limits. Each child keeps
120 seconds. Each suite has a 400-second outer deadline plus five-second grace,
one worker and 2 GiB virtual memory. Two suites are planned, with no automatic
retries. A failed suite is retained; the other planned suite still runs. A fatal
child lists any unrun arms rather than silently excluding them. Records remain
bounded to 1 MiB each and child stdout/stderr to 32 MiB. Parent polling latency
and its parsing/forwarding boundary are unchanged from COLD_ACCOUNTING.md.

Freeze/push the source, this protocol and exact command before either suite.
Keep both outputs, stderr, exit status and every parent receipt. Compare the
unflagged recorded mask/row/consumer histories to cold_accounting_v1 without
rerunning that historical executable. Inspect all singleton receipts and retain
any unfinished recovery. Report counts and scope, not a local speed ratio.

## Remaining requirements

The user’s new smooth-p+1 gain still requires a new supported mechanism and an
admitted full-cost comparison against the strongest applicable reference.
This component only closes a shared recovery-policy coverage gap. The existing
EKS path does not yet package this driver plus msolve; no cloud submission is
represented as planned or completed by this protocol. Canonical export tooling
was unreadable in the preceding component; local development flow records must
not impersonate a validated experiment.export/v1. All results need independent standing; this protocol supplies no signed verdict.
