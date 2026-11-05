# IsoLab accounting and operator discovery

Read-only search requested by the owner on 2026-11-05. No cloud job, IAM/RBAC
change, role assumption into an operator identity, or capacity mutation.

## Findings

1. The cost policy does not name a centralized reservation ledger. It requires
   fresh account telemetry plus charged/reserved lane costs. A specific central
   ledger file is not an additional requirement. The prior request assumed such
   a file existed; this search did not establish one. Do not invent a central
   ledger or interpret missing entries as zero reservations.
2. The accounting policy is in taskq/deploy/eks-karpenter/COST_CONTROLS.md and
   crypto-autoresearcher's docs/research-budget-policy.md. Local copies searched:
   /home/ec2-user/crypto-eks-cost-controls-20261101 and
   /home/ec2-user/auto-cloud-budget-policy-20261101. Main's runbook was also read
   through GitHub. AWS Budget readbacks are attached with LastUpdatedTime;
   reported headroom is delayed, before outstanding reservations.
3. Lane-specific cost records do exist. T-1572's pencil worktree contains
   research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/
   final-fd-candidate-build-plan/budget-plan.json. It records a USD59 existing
   reserve, USD55 shared-delay encumbrance, and a conditional USD2 suballocation;
   funding_available and execution_authorized are false. Its deadline is
   2026-11-04T06:00:00Z. Expiry does not prove that these funds were released.
   This is a pencil-lane record, not an authoritative all-IsoLab ledger.
4. The October30 operator readback is at
   /home/ec2-user/aws-cost-reduction-20261030/verified-controls.json, timestamp
   2026-10-30T23:43:11.106818Z. It recorded ordinary16CPU/128GiB/nodes1 and other
   Karpenter pools zero. The November1 cost-inspection-20261101.json explicitly
   has live_quota_changes_applied=false, nodepool_patch_authorized=false and
   resourcequota_access_authorized=false. Neither proves current desired caps.
5. Live EKS access-policy discovery identifies the existing IAM user coreuser
   with cluster-wide AmazonEKSAdminViewPolicy, AmazonEKSViewPolicy and
   AmazonEKSClusterAdminPolicy. The current spot-experiments WorkerRole has no
   associated EKS access policies. These are authorization metadata, not proof
   that a usable operator credential/session exists here. No operator credential
   was obtained or used. Direct worker cluster reads previously failed.

## Search coverage

Searched the named cost audit/reduction workspaces, the budget-policy worktree,
the EKS deployment/readback directories, relevant pencil accounting paths,
GitHub cost/Karpenter/budget PR metadata and the live main runbook. Queried
Conductor's authenticated local read-only project/tasks API with limit2000:
1599 task records were returned. The CLI list had exposed only200 oldest
records; using it alone missed current tasks. Relevant T-1436 and T-1572 cards
lead to lane-local accounting, not a shared reservation service. Available MCP
tools offer job operations but no live NodePool/quota or spending-reservation
readback. Migration's root-owned Conductor directory was inaccessible; it was
not opened through elevated privileges. This is a scoped unsuccessful search
for a current centralized ledger/readback, not proof none exists anywhere.

## Concrete continuation

Reconcile account totals with lane encumbrances, including whether expired
reservations were released; record the finite round25 reservation without
double-counting either daily AWS Budget threshold. Obtain a fresh resource
readback through an already authorized operator session such as coreuser.
operator-readback.sh is a read-only collector for that existing session: it
does not log in, assume a role, grant access, modify limits or submit work.
Current session restrictions do not authorize switching to that operator.
Then assess worker idle exposure against round25's USD2 ceiling. Frozen round25
inputs and attempt identity remain unchanged; attempts_used=0.
