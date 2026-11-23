# Existing ARC readiness route (read-only audit)

Inspected 2026-10-30. No workflow dispatch, cancellation, identity/permission change,
routing-variable change, credential change, runner registration or cluster mutation
was performed.

## Decision

There is an existing, active, main-branch manual readiness workflow. It is the
repository's documented smoke probe for its existing ARC scale sets, and directly
names the selected EKS runner label. It needs no routing-variable opt-in, no new
workflow, no runner-admin read, and no solver adapter. A future successful run could
establish **current workflow-level availability of that label and its specified
readiness checks**. It would not qualify scientific timing, checkpoint/storage,
xmem capacity, or independently establish Kubernetes node/Spot provenance.

The current GitHub connection can read the workflow and all run/job records. Read
API success does not prove Actions-write permission: that exact capability cannot
be proven in this read-only audit. The dispatch endpoint is concrete; an authorized
POST would either accept it or return the precise token/policy denial. A permission
denial must not be worked around by creating identities or changing permission.

## Exact existing workflow

Repository: `isogeny-labs/crypto`.

Inspected main revision: `50a55d888560684d08598bf371171231aef4aaab`.

Workflow: `.github/workflows/eks-arc-smoke.yml`.

Workflow id: `390779205`; API state: `active`.

Workflow file blob: `8e496e54f5a376996de968b81c82604ee7fb760a`.

Workflow URL:
https://github.com/isogeny-labs/crypto/blob/50a55d888560684d08598bf371171231aef4aaab/.github/workflows/eks-arc-smoke.yml

There is one manual input:

| Input | Type | Choices | Default |
|---|---|---|---|
| `pool` | choice | `linux`, `highmem` | `linux` |

The sole `smoke` job has:

- condition `github.ref == 'refs/heads/main'`;
- exact `runs-on` expression `${{ inputs.pool == 'highmem' && 'crypto-eks-highmem' || 'crypto-eks-linux' }}`;
- `timeout-minutes: 20`;
- workflow token permission `contents: read` (no OIDC, Actions-write or cluster permission requested);
- no concurrency setting, matrix, extra job or retry;
- no checkout, build, artifact upload, S3 write or solver execution.

It runs the following checks in its only shell step, with `set -euo pipefail`:

1. Print `RUNNER_NAME` and `uname -m`; require `x86_64`.
2. Query Docker daemon version with `docker info --format '{{.ServerVersion}}'`.
3. Require writable `RUNNER_TEMP`.
4. Read `/sys/fs/cgroup/memory.max` and `cpu.max`, print both.
5. Require memory limit at least 24 GiB for linux, or 112 GiB for highmem.

CPU quota is printed, not asserted. It does not allocate or touch 24/112 GiB, run a
CPU benchmark, test a native solver, test artifact storage, test a checkpoint, prove
exclusive cores or check interference. It produces the GitHub job log only.

## Configured resources, effects and bounds

At the inspected main, the chart values pin runner image:

`104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/gha-runner@sha256:71692163462dc305dc4a8f312143e704938b5929b84256bec9d30a5b27e6e7bf`

| Label | Runner request | Runner limit | Scale-set max |
|---|---|---|---|
| `crypto-eks-linux` | 2.8 CPU, 24 GiB, 10 GiB ephemeral | 4 CPU, 28 GiB, 80 GiB ephemeral | 7 |
| `crypto-eks-highmem` | 14 CPU, 112 GiB, 20 GiB ephemeral | 16 CPU, 120 GiB, 100 GiB ephemeral | 1 |

Both use Docker-in-Docker and have `minRunners: 0` and
`automountServiceAccountToken: false`. The values are intended configuration,
not an observation that the current deployment matches those files. Linux values
blob: `5654c3c3765b69101085711aac70416c0c3e860f`;
highmem values blob: `81e511f1883d6c071d95a30042a9ced58cfdb49f`.

Dispatch creates one Actions run/job; ARC may create an ephemeral runner pod and
Karpenter may provision a Spot node. Those automatic existing-system scheduling
effects incur EC2, EBS and NAT cost and consume regional Spot quota. The README
says empty nodes consolidate after five minutes. The highmem probe may therefore
need a larger node despite its seconds-long actual shell step. Linux is the
smallest existing documented probe.

The 20-minute timeout bounds **running job execution**, not runner queue time.
The workflow has no explicit queue timeout or automatic cancellation logic. A
bounded operational probe therefore needs monitoring plus a preselected maximum
queue wait; cancellation, if needed, is a separate Actions mutation using the
same existing permission. No cancellation was attempted here. A queued job is not
proof of any specific fault: registration, quota, Spot availability and concurrent
work are among possible causes. Do not repeatedly dispatch to diagnose it.

The README's complete deployment check also requests `kubectl get nodeclaims`
and eventual node consolidation. Those observations are unavailable here. Runner
job metadata and this smoke log can establish the narrower current readiness
without that administrative API, but should not be reported as full cluster
verification or hardware-isolated measurement admission.

## Repository validation and routing policy

Current `taskq/deploy/eks-karpenter/arc/README.md` explicitly prescribes:

> After deployment, dispatch `eks-arc-smoke.yml` once per pool ... Confirm both
> jobs pass ... GitHub's job log identifies the expected runner name.

It also identifies this workflow as one of the four that intentionally name a
scale set directly, exempt from ordinary per-workflow repository-variable routing.
This is an already documented validated probe, not a proposal to invent/enable
new infrastructure. The same README says the scale sets were registered before
the repository transfer and describes administrator rotation/redeployment if
needed. No such rotation/redeployment is implicit in running the probe.

README blob: `de89cc3ff6fab707b0f4344fa4ba2dcc2482b27e`.

## Historical evidence and present uncertainty

The workflow API reports exactly three existing runs, all completed successfully
on October 20. There are no currently queued/in-progress runs for this workflow in
the inspected response:

| Run | Pool label | Runner recorded by job API | Started → completed (UTC) |
|---|---|---|---|
| 39460365188 | `crypto-eks-highmem` | `crypto-eks-highmem-runner-4mwgr` | 16:47:32 → 16:47:35 |
| 39459423179 | `crypto-eks-highmem` | `crypto-eks-highmem-runner-27zdf` | 16:40:21 → 16:40:23 |
| 39459409233 | `crypto-eks-linux` | `crypto-eks-linux-runner-nrkxz` | 16:40:04 → 16:40:07 |

Those runs prove historical execution; they do not establish post-transfer
readiness. A newly accepted main dispatch that remains queued would reveal
non-readiness through the supported job API even without runner-admin access.
A new success with matching job label/name and all checks passed would establish
the narrower current readiness described above.

The repo metadata reports admin/maintain/push access for the connected principal,
and it is neither archived nor disabled. GET workflow succeeds and returns
`X-Accepted-GitHub-Permissions: actions=read`; that header describes the endpoint's
required permission, not the token's full grants. GET Actions policy and runner
administration endpoints return 403 `Resource not accessible by integration`.
Neither those denials nor the repo principal's permissions settle token
Actions-write access. No nonexistent read-only dry-run has been assumed.

## Exact future call, not executed

After reviewing current main for the unchanged workflow blob, the smallest
existing probe is the REST request:

```http
POST /repos/isogeny-labs/crypto/actions/workflows/390779205/dispatches
Content-Type: application/json

{"ref":"main","inputs":{"pool":"linux"}}
```

Equivalent CLI, included as documentation only:

```sh
gh workflow run eks-arc-smoke.yml --repo isogeny-labs/crypto --ref main -f pool=linux
```

Record the accepted run id, workflow `head_sha`, attempt, actual requested labels,
runner name, all outcomes/logs and elapsed queue time. Do not infer a fresh run id
from old logs or treat a queued job as a scientific experiment. Do not dispatch
highmem unless its pool is relevant to the planned workload. This smoke does not
run a candidate solver and cannot alone remove the need for a reviewed native
research workflow or genuine ecbench integration.

Retained copies live in `/workspace/audit/continuation-route/` with `.readiness`
suffixes. No dispatch has been performed by this audit.
