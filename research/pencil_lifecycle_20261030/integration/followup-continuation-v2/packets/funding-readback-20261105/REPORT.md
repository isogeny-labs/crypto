# Funding and live-cap readback, 2026-11-05

Read-only observations were collected at 15:46–15:47 UTC using existing configured credentials. No role, cluster configuration, resource, job or build was changed.

The shared USD 200 daily allowance has delayed reported spend of USD 104.44 and USD 44.765 in its two nonadditive budget snapshots. Use neither snapshot as an unreserved spending balance. The conservative arithmetic residual is USD 95.56 before commitments and billing delay. The USD 1,000 monthly snapshot reports USD 538.72, leaving USD 461.28 arithmetically before commitments and delay. Actual available funds remain unknown.

The existing kubectl context named `spot-job-submitter` actually invokes the legacy LauncherRole in its exec arguments. The current worker role was denied sts:AssumeRole before any Kubernetes RBAC decision. NodePool, namespace quota, ARC, Jobs and Nodes readbacks therefore failed. Nonzero-exit empty Lists are not evidence of an empty cluster. No identity substitution was attempted.

The separately configured IsoLab MCP overview returned twelve retained jobs: ten failed, one succeeded and one suspended. None of the returned jobs was an active pencil job. Its compiled pool limits do not establish live NodePool or quota enforcement. The returned list is not an account-wide cost or job census.

The requested continuation remains subject to a finite reconciled dollar allocation, live enforcement evidence, publication and runtime/protocol gates. This packet admits no execution and does not renew or reinterpret historical debits. See summary.json and per-command receipts for exact timestamps, exits and hashes.

Sources checked: the owner runbook `crypto-eks-cost-controls-20261101/taskq/deploy/eks-karpenter/COST_CONTROLS.md`; fresh AWS Budgets and STS readbacks; configured kubectl context and command failures; current IsoLab MCP overview. The managed-cloud environment-status tool and network-policy snapshot were unavailable on this migrated host; no readiness assertion was inferred from that absence.
