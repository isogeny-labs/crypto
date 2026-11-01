# Cloud cost controls and experiment custody

Owner instruction, 2026-11-01: check EKS/IsoLab experiments into GitHub,
budget aggressively, and restrict vCPU/RAM through Karpenter limits. This
supersedes older advisory/unlimited-spending guidance for cloud execution.

## Desired limits

| Control | CPU | RAM |
| --- | ---: | ---: |
| `cpu-spot` | 16 vCPU | 128 GiB |
| `cpu-l2-spot` | 8 vCPU | 32 GiB |
| Combined research NodePools | 24 vCPU | 160 GiB |
| `isolab` ResourceQuota `jobs`, both requests and limits | 16 CPU | 128 GiB |
| Four ARC/CI NodePools | 0 | 0 GiB |

The namespace also allows eight active pods and 80 GiB of requested/limited
ephemeral storage. The existing 200-Job object limit retains completed receipts
until TTL cleanup. Every checked-in ARC scale set has `minRunners: 0` and
`maxRunners: 0`; routine CI uses hosted runners. Any temporary activation needs
an explicit finite allowance and restoration to zero. Do not activate a
different pool, account, region, provider, bare-metal host or On-Demand fallback
to avoid these controls. Keep the existing system-node group separate.

The ordinary pool accommodates the retained 96-GiB Semaev run on a 16-vCPU,
128-GiB worker. L2 work queues behind one 8-vCPU worker. Excess work waits;
do not truncate samples or remove controls to fit. Do not evict active jobs or
delete their checkpoints. Existing empty-only consolidation remains enabled.

Karpenter `spec.limits` bounds provisioned node resources. Its parallel
provisioning uses eventually consistent limit checks and can briefly overshoot;
this is not a dollar cap. Namespace ResourceQuota separately rejects excess
pod admission. Quota reductions do not evict existing pods. AWS Budget alerts
are delayed notifications, not a hard spending stop. Source review:
[Karpenter limits](https://karpenter.sh/docs/concepts/nodepools/#speclimits),
[Kubernetes ResourceQuota](https://kubernetes.io/docs/concepts/policy/resource-quotas/).

## Before spending

Commit and push the exact experiment code, protocol, inputs or hashed input
manifest, configuration, seeds and launch command to a focused GitHub branch/PR
before a cloud build or trial. A local commit is insufficient. From the source
repository root, enumerate every launch input with:

```bash
python3 /path/to/crypto/taskq/deploy/eks-karpenter/check-experiment-git \
  --repository OWNER/REPOSITORY \
  --path research/EXPERIMENT/protocol.json \
  --path research/EXPERIMENT/run.py \
  --path research/EXPERIMENT/request.json > publication-receipt.json
```

This checks listed files against HEAD and the index, then verifies the exact
commit through GitHub. Missing files, dirty inputs, unpublished revisions and
unavailable GitHub verification stop launch. It is an agent-side prerequisite;
the existing deployed MCP/bridge does not yet enforce it server-side. Preserve
the receipt and bind its SHA to the actual image, downloaded scripts and input
hashes. A caller SHA alone does not establish which code executed.

Record a finite dollar ceiling, maximum CPU/RAM, concurrency, original absolute
deadline and total retry allowance. Include builds, queue/idle node time,
failed attempts, Spot restarts, storage and transfer. Consult the current
account budget plus the lane's charged and reserved cost before submission.
If the remainder is unknown or exhausted, do not start a new build/job. Record
the impediment and preserve the work. Do not reset the ledger on retries.
Reconcile the original Job/build/request ID after an ambiguous timeout.

Publish results promptly on the same branch/PR, including failures, OOMs and
timeouts: command, source/image hashes, Job/Pod UIDs, resources, wall time,
actual/estimated cost with telemetry timestamp, checks and conclusions at their
checked scope. Large data belongs in durable storage with committed hash,
byte count, URI and replay/extraction instructions. Never rewrite sealed runs.

## Apply and verify

Use an operator identity already authorized to list/get/patch NodePools and
get/patch `isolab/jobs` ResourceQuota. Do not grant yourself access or launch a
privileged job as a workaround. The reduction tool checks the real EKS endpoint,
inventories all pools, refuses unknown/static/missing required pools, preserves
stricter existing caps, and dry-runs every patch before changing any resource.
It uses resourceVersion tests and does not blindly retry partial changes.

```bash
python3 taskq/deploy/eks-karpenter/apply-cost-controls.py \
  --output /tmp/isolab-cost-dry-run-UNIQUE
python3 taskq/deploy/eks-karpenter/apply-cost-controls.py --apply \
  --output /tmp/isolab-cost-applied-UNIQUE
```

Retain the before snapshots, patch plan and each post-apply readback. Audit
`status.resources`, Nodes and EC2 instances separately: verified desired limits
do not prove actual capacity has drained. The tool changes limits only; it
does not roll workers, rewrite NodeClasses or change running jobs. ARC runner
settings in Git still need normal operator deployment if live values differ.
Never claim application from checked-in YAML or the MCP overview, whose pool
numbers are compiled-in defaults. Review any new pool against the aggregate
budget before creating it. Restore larger limits only under explicit owner
authorization, using preserved snapshots and a new reviewed change.

## 2026-11-01 inspection

At 20:20–20:35 UTC (12:20–12:35 PST), 23 retained Jobs were inspected. Every
Job had a deadline, CPU/RAM requests equal to limits, and zero automatic retries.
Three were active: two L2 jobs at 4 CPU / 4 GiB each, and Semaev at 2 CPU /
96 GiB. One separate bare-metal qualification Job remained suspended. The
retained Jobs did not carry GitHub source annotations; their commit custody
cannot be established from Kubernetes metadata alone. Inspecting commands
also found four source commits referenced by six GitHub-launched Jobs and one
commit referenced by three Semaev Jobs; all five commits were verified on
GitHub. This does not certify image/source equivalence or that every historical
result was published. The inspection JSON records the exact associations.

EC2 reported two running Karpenter workers: `c8a.2xlarge` in `cpu-l2-spot`
and `r8i.4xlarge` in `cpu-spot`, both Spot. The current worker role was denied
NodePool list/patch and namespace quota access. No live quota reduction is
claimed. Apply using the authorized operator path above, then verify readback.

Both daily AWS Budget thresholds, `isolab-daily-cost-100` (name retained) and
`spot-experiments-daily-25`, were lowered from USD 100 to USD 25 and verified
by `DescribeBudget`. Existing recipients and filters were preserved. Both
reported spend already above USD 25 in their delayed snapshots; the new
policy therefore admits no new work until recorded headroom exists. This
threshold change does not terminate jobs or enforce a billing cap. Exact
before/after values and billing timestamps are in
[cost-inspection-20261101.json](cost-inspection-20261101.json).
