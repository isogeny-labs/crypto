# Comparison admission reconciliation — 2026-11-03

Named reads of both NodePools succeeded under the unchanged worker role. This
supersedes the broad statement that NodePool reads are denied: earlier list
access and current named get access differ. The saved projections preserve
resource versions and source hashes; they are observations, not live guarantees.

| Pool | CPU cap | Memory cap | Node cap | Reported nodes |
| --- | ---: | ---: | ---: | ---: |
| cpu-spot | 16 | 128Gi | 1 | 0 |
| cpu-l2-spot | 8 | 16Gi | 1 | 0 |

The stricter L2 cap remains unchanged. The overview tool's compiled 32+32 CPU
pool defaults are not authoritative live limits. NodePool status is not an EC2
billing inventory or a guarantee against concurrent provisioning.

A fresh named get of namespace `isolab` ResourceQuota `jobs` returned Forbidden.
The saved quota capture is empty and is not a valid quota snapshot. No role,
quota, NodePool, controller or existing Job was changed. The existing completed
hashed-ECDLP Job and suspended qualification Job were reconciled by UID; no
prime-field comparison Job was found in the overview.

The saved AWS budget observations show both daily alert limits at USD200 and
the monthly limit at USD1000. Their reported spends are USD131.13, USD48.79 and
USD350.379 respectively, with distinct update timestamps preserved in JSON.
These are not additive budgets, current available funds, or lane reservations.
No usable unreserved remainder has been established. Conductor's token-budget
commands do not establish cloud-dollar allowance.

## Sources and decision

Reviewed repository `taskq/deploy/eks-karpenter/COST_CONTROLS.md` at source
02ed831d898f0fd603a98a48921d32fa65d369a6 and the existing homogeneous comparison
draft. AWS documents billing/notification delay and updates typically 8–12 hours
apart: https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-managing-costs.html
(read 2026-11-03). This supports retaining timestamps and refusing to interpret
alert headroom as an unreserved balance.

The comparison remains unlaunched. Obtain the exact quota readback through an
already-authorized operator path and reconcile charged/reserved allowance.
Then finish pinned execution delivery and a bounded per-arm preflight, bind its
runtime estimate, and freeze/publish the protocol before comparative data.
The existing runner has only describe and full-panel modes; a distinct
preflight entry point or wrapper is still required. Preserve the original
4563-case cohort and the separate eight-target component-panel scope.

## Handoff (T-1562)

### What was done
Recovered saved NodePool/budget JSON, projected only relevant fields with source
hashes, reconciled current Jobs and repeated the exact named quota read.

### Decisions made
Keep successful named get evidence separate from prior denied list operations;
retain stricter observed limits and do not infer allowance from alert snapshots.

### Open threads
Quota readback, lane reservation evidence, pinned preflight delivery and freeze.

### Errors
Quota get returned Forbidden. A request to add the daily-log scope using task
claim returned `task already claimed`; this component-local handoff preserves
facts without modifying an unreserved shared log. The migration receipt remains
unreadable; Diem is deadline-expired with pending=false and was not restarted.
