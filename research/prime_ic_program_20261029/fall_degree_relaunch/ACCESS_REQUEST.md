# Access request: let an agent submit an argv + image job to Iso Lab without the harness host

**Status: request (plan). Nothing here has been performed.** Broadening lab access is a separate
administrative action (`taskq/deploy/eks-karpenter/ACCESS.md`; `.claude/skills/isolab-eks/SKILL.md`).
The 2026-10-29 session declined to work around it with CodeBuild overrides
(`docs/session-logs/2026-10-29.md`).

## The gap (measured, from the contracts)

| route | runs arbitrary argv? | why not |
|:--|:--|:--|
| `isolab.eks.request/v1` via `remote/launch-remote` (CodeBuild) | no | schema has no command field; `run-request` calls only `isolab_k8s_benchmark`; `seconds <= 3600`; CPU fixed at 4 |
| `isolab-experiment.yml` (GitHub OIDC) | no | `build-bundle` builds only `--bin ecbench`; the publisher image holds ecbench |
| `isolab.eks.resumable/v1` | no | ecbench spec only; `c8a.2xlarge` only |
| `isolab_k8s_run` (MCP) | **yes** | only where `isolab-eks-mcp` has a kubeconfig with `isolab:submitters` rights and a network route to the private EKS API: the harness host, or a VPC host an administrator has mapped (`ACCESS.md` §1, §4) |

So the only path today is a session that runs on the harness host.

## Option 0: no change (already authorized)

Run the card (`CARD_DRAFT.yaml`) in a session on the harness host. The owner chose this route on
2026-10-28. It needs one of the following:

- the owner starts a Claude Code session there;
- a Remote Control folder on the harness host is offered to this account. It must be a Git repository
  root, not inside a hidden directory, and on Linux, per the Remote Control rules.

This changes no permission. It also needs the image pushed to ECR (`isolab/tasks`) by someone holding ECR
push rights on that host.

## Option 1: the minimal code change, a command request through the existing CodeBuild launcher

The launcher already ships `isolab-eks-mcp`, which implements `isolab_k8s_run`. The smallest change that
lets a caller with today's AWS bridge permissions (S3 request upload plus `StartBuild` on
`isolab-experiment-launcher`) submit an argv job is a second request schema that `run-request` dispatches
to `isolab_k8s_run`.

1. **`contracts/command-request.schema.json`** (new): `schema: "isolab.eks.command/v1"`, and a `job` with
   exactly `isolab_k8s_run`'s input fields:
   - `job_name`;
   - `command`: argv, 1-256 strings;
   - `image`: required, with the same pattern as `build.image_uri`, `^104186811093\.dkr\.ecr\.us-east-2\.amazonaws\.com/isolab/tasks@sha256:[a-f0-9]{64}$`.
     That keeps images inside the lab's own ECR repository;
   - `cpu`: 2-28, even;
   - `memory`;
   - `seconds`;
   - `retries`: fixed at 0.

   `provenance` takes the request-v1 fields plus `binary_sha256` (a map from path to hash).
   `additionalProperties: false`. No `env` field, no shell, and no host mounts.
2. **`remote/validate-request`**: accept the new schema. Reject any image outside `isolab/tasks`, and reject
   `command[0]` unless it is an absolute path.
3. **`remote/run-request`**: on `isolab.eks.command/v1`, run `isolab_k8s_overview`, then
   `isolab_k8s_run "$job"`, and write `overview.json` and `submission.json` (with the Job UID) to the receipt.
   It **returns without waiting**: command jobs run for hours, and a CodeBuild build is limited to 70 minutes
   (`HARNESS.md`).
4. **A read-only status kind**, `isolab.eks.command-status/v1` with `{job_name, job_uid}`. It runs
   `isolab_k8s_status` (rejecting a UID mismatch), `isolab_k8s_logs` and `isolab_k8s_results` (following
   `next_token`). It copies the job's `runs/<job_name>/<pod_uid>/` objects into the caller's receipt prefix,
   because callers' S3 grants cover launcher receipts, not `runs/` (`HARNESS.md`).
5. **Tests:** cases in `remote/test-contract.sh` for both kinds. Redeploy with `remote/deploy-remote`
   (`HARNESS.md`: changing orchestration scripts requires repackaging).

This adds no new IAM permission for callers. CodeBuild's role already creates Jobs. Pod security stays
exactly as `submit-job` sets it: UID 1000, read-only root, `/work` emptyDir, no privilege escalation,
`ISOLAB_RESULTS_URI` scoped per job and pod. The administrator should know one thing: it does widen *what* an
already-authorized caller can run, from audited ecbench to any argv in a lab-ECR image. That is the reason this
needs an explicit owner decision, not an implicit one.

## Option 1b: what is also needed to publish the image

Option 1 requires the image to be in `isolab/tasks` already. Today the commit-image path builds only ecbench
(`build-bundle`). Two choices:

- **(i)** Grant the caller's role `ecr:InitiateLayerUpload`, `UploadLayerPart`, `CompleteLayerUpload`,
  `PutImage` and `BatchCheckLayerAvailability` on the single repository `isolab/tasks`. It needs nothing else.
- **(ii)** Add a `dockerfile_path` input to `isolab-experiment.yml`. The build job builds the reviewed
  Dockerfile at the tested commit without AWS identity, and the trusted main-branch publisher pushes it by
  digest. That keeps the current trust model (the publisher never runs the tested executable), and it is the
  larger change.

## What the request does not ask for

- No raised pool caps (`cpu-spot` 32 vCPU / 128 GiB) and no quota increase.
- No resumable command jobs. CRIU/EBS qualification covers ecbench on `c8a.2xlarge` only.
- No permanent keys, no OIDC loosening and no shared service account.
