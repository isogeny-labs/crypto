# CAPACITY_R2: what a 256-512 GB host needs for the `m = 4` cells that do not fit today

Slug `r2pkm`, 2026-11-08. Decision 5 of `research/open_leads_execution_20261108/DECISIONS.md`: a large host is
**allowed in principle**; the change is "prepared and reported separately". **Nothing here was applied**: no
`kubectl`, no `apply-cost-controls.py`, no AWS write. Every number is `[pcc:unverified]`. Classes: *cited* (a file says so),
*measured* (read from a live read-only AWS call today), *derived*, *heuristic*, *plan*.

## 1. What is blocked and how big

Cells with no request (Amendment 8; cited from the study README and `AMENDMENT_8_DRAFT_m4.md` section 3):
`m = 4`, `t = 5`, `c = 7` (356,959 columns; projected 80-510 GB) and `t = 6`, `c = 6` (350,776 columns; 80-490 GB), then
`t = 6`, `c = 7` (> 0.8 TB, out of range even at 512 GB). Derived: entries are 8 bytes each, so a request of `R` GiB gives
`max_entries = floor((R - 8) * 2^30 / 8)`:

| request | max_entries (derived) | covers |
|--:|--:|:--|
| 112Gi (today) | 13,958,643,712 | t=4 c=7; t=5 c=6 (8-67 GB) |
| 232Gi (256 GiB host) | 30,064,771,072 | lower half of the t=5 c=7 / t=6 c=6 band (to ~240 GB) |
| 480Gi (512 GiB host) | 63,350,767,616 | most of the band (to ~506 GB); not t=6 c=7 |

The projection band is wide (8-fold, "low confidence" in the draft), so a 256 GiB host may or may not be enough. The
first row that the t=5 c=6 run writes (its measured entry count) narrows it; request the host size from that count, not before (plan).

## 2. What limits today (cited)

| item | file | today | blocks a 232Gi pod because |
|:--|:--|:--|:--|
| Karpenter `cpu-spot` `limits` | `taskq/deploy/eks-karpenter/nodepool.yaml`, `cost-controls.json` | cpu 16, memory 128Gi | a 256 GiB node would exceed the pool limit, so no node is provisioned |
| `instance-cpu` requirement | `nodepool.yaml` (`cpu-spot`) | `["8","16","32"]` | an r-family 256 GiB instance has 32 vCPU (fits); a 512 GiB one (`.16xlarge`) has 64 vCPU (excluded) |
| `isolab/jobs` ResourceQuota | `workloads.yaml`, `cost-controls.json` `quota` | requests/limits cpu 16, memory 128Gi | the Job's pod (cpu 28, memory 232Gi) is rejected at admission |
| owner cost ceiling | `nodepool.yaml` header, `COST_CONTROLS.md` | 24 vCPU / 160Gi aggregate (16/128 ordinary + 8/32 L2), 2026-11-01 | the ceiling itself has to be restated by the owner; no implicit increase (also `isolab-eks` SKILL) |
| pod pool selector | `submit-job` | `nodeSelector isolab.dev/pool=cpu-spot` fixed; optional `node.kubernetes.io/instance-type` | a new pool name would also need a `submit-job` change; widening `cpu-spot` needs none |
| request validator | `remote/run-command` | `cpu` even 2..28; memory `^[1-9][0-9]*(Mi\|Gi)$`, no upper bound | none for 232Gi; a pod cannot request more than 28 vCPU even on a 64 vCPU host |
| per-pod disk | `submit-job` | `ephemeral-storage` 10Gi, `/work` emptyDir 8Gi; quota ephemeral 80Gi | none: the closure is in RAM; results go to S3 |

## 3. How these are applied (found in the tree)

- `apply-cost-controls.py` can only **reduce**: `reduction_patch` keeps the smaller of the live and the desired value, so it
  **cannot raise** limits, and it refuses unknown or missing pools (`plan`). It is not the path for this change.
- `COST_CONTROLS.md` ("Apply and verify"): changes go through "an operator identity already authorized to list/get/patch NodePools
  and get/patch `isolab/jobs` ResourceQuota", "restore larger limits only under explicit owner authorization ... a new reviewed change".
- `deploy.sh` does `kubectl apply -f workloads.yaml` (the ResourceQuota) and `kubectl apply -f nodepool.yaml` (NodePools); it also
  reconciles CloudFormation, Helm and the cluster, so run only those two `kubectl apply` lines, not the whole script.
- No CI workflow applies them (a grep of `.github/workflows` finds none), and `ACCESS.md` says the submitting agent has no cluster-admin or
  EKS access entry (the launcher has its own submitter identity). So the apply step is an **operator** step with a cluster-admin identity.
  Whether `coreuser` has it was not tested.
- `test_cost_controls.py::test_rendered_manifests_match_policy` asserts `nodepool.yaml` limits == `cost-controls.json` pools, and the
  quota == `cost-controls.json` quota, so the three files change together.

## 4. Instance types and Spot price

Read-only `aws ec2 describe-spot-price-history` for us-east-2, Linux/UNIX, last 6 h on 2026-11-08 (measured; a handful of
price points per type, not a distribution; today's quote, not a forecast):

| type | vCPU / RAM | Spot $/h seen (min-max) | passes today's `instance-category`/`generation` filters |
|:--|:--|:--|:--|
| r6i.8xlarge | 32 / 256 GiB | 0.51 - 0.61 | yes (gen 6) |
| r7i.8xlarge | 32 / 256 GiB | 0.74 - 1.20 | yes |
| r7iz.8xlarge | 32 / 256 GiB | 0.93 - 1.06 | yes |
| r6i.16xlarge | 64 / 512 GiB | 0.82 - 1.72 | yes, but vCPU 64 is not in `instance-cpu` |
| r7i.16xlarge | 64 / 512 GiB | 0.90 - 0.94 | same |
| r5.16xlarge | 64 / 512 GiB | 1.07 - 1.44 | no (generation 5) |

Dollar view against owner decision 4 (at most US$40 of Spot compute per job; derived):
- 176,400 s = 49 h. 49 h x 0.55 = about US$27 on r6i.8xlarge (inside US$40); 49 h x 1.0-1.7 = about US$49-84 on a 16xlarge (**over US$40**).
- The US$40 ceiling therefore buys about 70 h on r6i.8xlarge or 25-50 h on r6i.16xlarge. The draft's own estimate for `t = 5`, `c = 7` is
  "on the order of two weeks" at the N = 16 rate (heuristic, `AMENDMENT_8_DRAFT_m4.md` section 3), which is 336 h, about US$185 (8xlarge) to
  US$340-580 (16xlarge). That exceeds US$40 per job and probably the US$400 study total with the other jobs. **A job-ceiling decision is
  needed before launching it**; the 176,400 s `seconds` of the existing m4 requests is also too short for it (it fits the t = 4 and t = 5 c = 6 cells).
- Spot reclaim (decision 3): a days-long, non-resumable job on Spot is likely to be lost (heuristic; the arity run lost 4 pods in 4-21 min,
  cited). A lost cell is reported as lost, relaunched once at most, both attempts kept.

## 5. Minimal diff (plan; do not apply)

Choice: **widen the existing `cpu-spot` pool**, not a new pool: `submit-job` keeps selecting `cpu-spot`; the large job pins
`instance_type` in its request so that ordinary jobs never land on it by cost choice. Option A is a 256 GiB host (one r-family 8xlarge);
option B adds the 512 GiB `.16xlarge` size.

```diff
--- a/taskq/deploy/eks-karpenter/cost-controls.json
+++ b/taskq/deploy/eks-karpenter/cost-controls.json
-    "cpu-spot": {"cpu": "16", "memory": "128Gi"},
+    "cpu-spot": {"cpu": "32", "memory": "256Gi"},          # option B: {"cpu": "64", "memory": "512Gi"}
@@
-    "requests.cpu": "16", "limits.cpu": "16",
-    "requests.memory": "128Gi", "limits.memory": "128Gi",
+    "requests.cpu": "28", "limits.cpu": "28",              # one 28-vCPU pod; a 14-vCPU job still fits
+    "requests.memory": "240Gi", "limits.memory": "240Gi",   # option B: "496Gi"
--- a/taskq/deploy/eks-karpenter/nodepool.yaml
+++ b/taskq/deploy/eks-karpenter/nodepool.yaml
-# Owner cost ceiling, 2026-11-01: 24 vCPUs / 160Gi in aggregate.
-# Ordinary: 16 / 128Gi; L2: 8 / 32Gi. No implicit cap increases.
+# Owner cost ceiling, <DATE, restated by the owner>: 40 vCPUs / 288Gi in aggregate.
+# Ordinary: 32 / 256Gi; L2: 8 / 32Gi. No implicit cap increases.
@@ (NodePool cpu-spot)
   limits:
-    cpu: '16'
-    memory: 128Gi
+    cpu: '32'
+    memory: 256Gi
@@ option B only
-          values: ["8", "16", "32"]
+          values: ["8", "16", "32", "64"]
--- a/taskq/deploy/eks-karpenter/workloads.yaml
+++ b/taskq/deploy/eks-karpenter/workloads.yaml
-    requests.cpu: '16'  /  limits.cpu: '16'  /  requests.memory: 128Gi  /  limits.memory: 128Gi
+    requests.cpu: '28'  /  limits.cpu: '28'  /  requests.memory: 240Gi  /  limits.memory: 240Gi
```
(`COST_CONTROLS.md` "Desired limits" table changes the same numbers; the `"#"` notes above are for the reader, not JSON.)

Request changes for the large cell: `cpu` 28 (validator maximum), `memory` `232Gi`, `instance_type` `r6i.8xlarge`,
`seconds` set by the job-ceiling decision, command `m4:5:0:7:30064771072:4000000` (and `:1:`). The 232Gi request leaves about 24 GiB
for the node reservation and kernel; the allocatable memory of a real node was not measured (the 112Gi/128 GiB case is by analogy).
28 vCPU on a 32 vCPU node assumes the 2-vCPU housekeeping core is the only reservation; it was not measured at 32 vCPU.

## 6. Side effects the operator should know (derived)

- Raising the quota to 28 vCPU also lets two 14-vCPU jobs run at once if two 16-vCPU nodes fit under the 32-vCPU pool limit. The study
  stays serial by launch discipline, not by quota. Keeping the old quota is impossible with a 28-vCPU pod; a separate namespace would need
  its own IRSA service account and a `submit-job` change, which is a larger diff.
- A memory-heavy pod may drive Karpenter to a bigger node than the 14-vCPU jobs use; with the pool limit at 256Gi it cannot also hold a
  128 GiB node, so large and ordinary jobs cannot overlap.
- Quota or limit changes do not evict running pods; after the study, return `cost-controls.json`, the quota and `nodepool.yaml` to
  16 / 128Gi in a second reviewed change (the old numbers are preserved in git at f13a690df0).
- `ACCESS.md` and the isolab-eks skill state "no implicit cap increases": this needs the owner's written ceiling, not just decision 5.

## 7. Owner inputs still missing

1. Restated aggregate ceiling (40 vCPU / 288Gi for option A, 72 / 544Gi for option B) and its date.
2. A job dollar ceiling for the multi-day `t = 5`, `c = 7` / `t = 6`, `c = 6` cells (US$40 does not cover them).
3. Who applies: an operator with cluster-admin (not the submitting agent).
