# Minors panel: first launch unschedulable, relaunch B at 12 vCPU (2026-11-08)

## First launch: `minors-gd-panel-20261107`, never scheduled, deleted

The pod `minors-gd-panel-20261107-m9d6x` stayed `Pending` from launch until it was deleted at about 12:20Z. It produced no data.

The scheduler and Karpenter gave these reasons. Each is from the EKS API-server audit log, `/aws/eks/isolab/cluster`:

| time | reason |
|---|---|
| 11:18Z | 1 node had insufficient cpu and 1 had insufficient memory. The only cpu-spot node was held by the outside job pq2-msolve-n37. |
| 11:55Z onward | No instance type has enough resources for `resources={"cpu":"14180m","memory":"32872Mi",...}`, with requirements `karpenter.sh/nodepool In [cpu-spot]` and `instance-cpu In [16 32 8]`. |

**Cause:** the pod cannot be scheduled on any node cpu-spot can create.

- The nodepool limit is 16 vCPU, so cpu-spot can create at most a 16 vCPU node.
- On that node the kubelet reserves `kubeReserved` 1 CPU plus `systemReserved` 1 CPU (`taskq/deploy/eks-karpenter/nodepool.yaml`), which leaves 14000m allocatable.
- The pod requests 14 CPU, which is 14180m once the daemonsets are added. That is more than 14000m.
- `remote/run-command` accepts cpu 14, so the request validated anyway.

The job was deleted with the request and receipt in `receipts/R2/delete/`, state `deleted`.

## Relaunch B: `minors-gd-panel-20261108-b`

The request is `eks/r2img_request_b.json`. It is VALID under `eks/r2img_validate.sh`, and it equals `eks/r2img_request.json` except in four fields:

| field | first launch | relaunch B |
|---|---|---|
| `job.job_name` | minors-gd-panel-20261107 | minors-gd-panel-20261108-b |
| `job.cpu` | 14 | 12 |
| `job.command` | `entry.sh 14` | `entry.sh 12` |
| `provenance.note` | (original) | sentence appended |

Everything else matches, checked with `jq -S` after removing those four fields: the same image `72934ab9…` and the same frozen `run_panel.sh`, binary and analysis.

**Why this does not change the measurement:**

- The entry argument is the number of parallel shard processes (`xargs -P`).
- PROTOCOL.md (lines 11–13 and 102–103) says the measured quantities are exact counts, with no wall-clock or host confound. Wall time is informational.
- Fewer parallel shards change only the wall time. The planned 25–55 core-hours become about 2–4.6 h on 12 vCPU, inside the 43200 s ceiling.
- No data existed before this change.

**Platform finding for the lab:** a 14 vCPU request passes validation but can never be scheduled on cpu-spot. The PKM requests (14 vCPU) have the same problem. They are handled separately, because PKM runs carry a wall budget.

## Relaunch B outcome: lost to Spot reclamation (recorded 2026-11-08 12:45Z)

Job `minors-gd-panel-20261108-b` (job uid `0f7f5d43…`) ended `failed`.

- **Pod:** the pod's task container ran 12:22:59–12:37:20Z. The node's kubelet reported exit 137 (`Error`), with pod reason `Terminated` and message "Pod was terminated in response to imminent node shutdown." This is from the EKS API-server audit log.
- **Status:** the status route returns `pods: []` (receipts/RC/status/poll-0009…).
- **Data:** the entry script syncs to S3 every 600 s, so a partial upload may exist under `runs/minors-gd-panel-20261108-b/`. `run_panel.sh` never finished, so nothing from this run is admitted or analysed.
- **Accounting:** under owner decision 3 (a lost Spot cell is reported as lost, with at most one relaunch under a new name), this is the study's first Spot loss. The first launch, `minors-gd-panel-20261107`, never ran, so it is not a Spot loss.

## Relaunch C: `minors-gd-panel-20261108-c`

This is the single allowed relaunch.

- **Request:** `eks/r2img_request_c.json` equals `eks/r2img_request_b.json` except `job.job_name` and the appended provenance note, checked with `jq -S`. It is VALID.
- **If C is also lost:** the study is reported as lost at this scale, with no further relaunch.
