# B677 execution admission: read-only inspection

Inspection date: 2026-11-23 UTC. This records capability and source inspection
plus the subsequent failed checkpoint-gate receipt. It is not a research
execution, an authentication test, a build receipt, or a submitted job. No
credential values were displayed, no checkpoint was uploaded, and no EKS or
CodeBuild job was submitted during this inspection.

## Standing

The frozen B677 research calls do not yet have an admitted execution route from
this workspace. The repository supports the required class of arbitrary native
CPU command job, but this source revision still needs a tested, digest-pinned
RR5 payload, an authenticated submission route, a successful checkpoint gate,
publication checks, capacity admission, and the finite spending authorization
required by the EKS workflow. Local unit tests and static checks are development
work under the documented rules; they do not substitute for these research
calls.

The zero-budget native-resource test is an admission control. It uses the public
generator as its target and forbids algebraic search through zero native limits.
It is separate from the registered positive, H0, and W0 research requests.

## Documentation followed

The route was selected from `AGENTS.md`, sections “Standing workflow rules,”
“Resume checkpoints in S3 before every run,” and “9a. Experiments default to Iso
Lab on EKS Spot.” The shared skill was read at its documented public location:

- [isolab-eks/SKILL.md](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/SKILL.md)
- [isolab-eks/references/submission.md](https://github.com/isogeny-labs/workspace/blob/main/.agents/skills/isolab-eks/references/submission.md)
- `taskq/deploy/eks-karpenter/ACCESS.md`
- `taskq/deploy/eks-karpenter/remote/launch-remote-command`
- `taskq/deploy/eks-karpenter/build-bundle`
- `.github/workflows/isolab-experiment.yml`
- `scripts/checkpoint.sh`

The current main-branch workflow input block was also fetched through the
connected GitHub reader. Historical examples in ACCESS.md and the submission
reference still mention `aburan28/crypto`; the current task repository is
`isogeny-labs/crypto`. The documented skill requires verifying current workflow
support and OIDC trust rather than inferring authority from an old example.

`python3 tools/lessons.py brief --scope isolab-eks` was read as administrative
guidance. No lesson ledger was changed by this inspection.

## Observed workspace capabilities

At the initial read-only inspection, only executable availability and
configuration-presence booleans were read.
Secret values and credential-file contents were not read or printed.

| Capability | Observation | Consequence |
|---|---|---|
| Connected EKS MCP tools | None exposed in the tool registry | The direct MCP route is unavailable in this session. |
| AWS CLI | Not on PATH | STS and the portable CodeBuild client cannot run as currently configured. |
| kubectl | Not on PATH | No local direct Kubernetes admission or capacity check is available. |
| AWS credential/profile/web-identity environment settings | None of the inspected standard settings present | No authenticated AWS caller has been established. This is not an IAM permission denial. |
| Kubernetes service environment and projected service-account token | Absent | No observed in-cluster submission route. |
| GitHub | `gh` executable present; connected GitHub reads succeeded | Repository read access is established; Actions dispatch permission was not tested. |
| jq, GNU timeout, OpenSSL | Present | Auxiliary portable-client prerequisites exist. |
| Checkpoint upload token environment/file | Absent at the documented location | The token-based checkpoint transport is unavailable as configured. |
| Resume note and current push receipt at initial inspection | Missing/stale | The checkpoint requirement was unsatisfied; the subsequent attempt is recorded below. |

The inspected AWS environment names were `AWS_ACCESS_KEY_ID`,
`AWS_SECRET_ACCESS_KEY`, `AWS_SESSION_TOKEN`, `AWS_PROFILE`, `AWS_ROLE_ARN`, and
`AWS_WEB_IDENTITY_TOKEN_FILE`. This check did not enumerate or copy possible
credentials from other sources. It therefore establishes missing demonstrated
authentication, rather than a claim that no identity could ever be configured.

## Checkpoint observation

The read-only command `bash scripts/checkpoint.sh status` reported:

```text
RESUME.md : NOT READY
  missing .resume/RESUME.md (run: scripts/checkpoint.sh init, then fill it in)
last push : stale or none
```

The script's `gate` implementation first rejects an incomplete resume note and
otherwise calls `cmd_push`; it is not a read-only status command. The initial
inspection therefore used `status` only. The documented token file and
`CHECKPOINT_UPLOAD_TOKEN` were both absent, and the AWS CLI fallback was
unavailable.

The parent session subsequently completed the resume note and attempted the
actual `scripts/checkpoint.sh gate`. The immutable receipt is
`scaling-b677/evidence/admission-01/`, including the note at admission, command,
start/end times, stdout, stderr and exit status. The command exited **1** with:

```text
checkpoint: no way to upload: no AWS profile 'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN, no /home/agent/.config/checkpoint/upload-token
checkpoint: cannot upload, so nothing was checkpointed
```

Thus the retained gate failed on upload transport after the resume note was
completed. A later successful checkpoint must bind the final source, protocol,
launch request, and current handoff before a research run begins.

## Supported command route and present blockers

For this custom RR5 qualification executable, the documented arbitrary-command
route is `taskq/deploy/eks-karpenter/remote/launch-remote-command` using
`isolab.eks.command-request/v1`. It submits through the CodeBuild bridge and
requires the authorized caller's AWS identity, a digest-pinned task image,
explicit argv, CPU/RAM, deadline and bounded observation. The command must
persist its outputs to the assigned results location; a temporary `/work` file
or a successful Pod alone is insufficient. Command-job timings are diagnostic
and do not confer ecbench L2, replay, or A/A admission.

The current main-branch `isolab-experiment.yml` accepts an immutable
`source_revision` and a `spec_path`, but its maintained build bundle builds
`ecbench` and `isolab-fresh-owner`. Its one additional custom command payload is
the explicitly selected norm-branch census. The inspected workflow has no RR5
payload build or generic arbitrary-command input. Changing `spec_path` alone
would not build or run `rr5-b677-gate`; a caller source label also does not change
an executable in an existing image.

Consequently, neither an unrelated default fixture nor the norm-branch job is
a substitute for this protocol. The next supported route requires one of:

1. An authorized AWS/CodeBuild client plus a verified image containing this
   exact published RR5 revision and launch payload; or
2. Reviewed main-branch workflow support that builds, publishes, and invokes
   that payload under the repository's existing trusted identity.

Both still require the successful checkpoint, publication receipt, original
fixed requests and limits, live capacity admission, finite dollar ceiling,
and immutable stdout/stderr/exit/hash receipts. No quota, account, provider,
OIDC trust, or retry policy was changed during this inspection.

## Interpretation

This is a capability gap, not a solver result. It gives no evidence that the
positive control, H0, or W0 completes, times out, or is empty. The source remains
reviewable and the fixed CLI requests remain runnable after admission. The
extractor's `phase_accounting_complete` flag remains separate from extraction
correctness and must not be promoted by a successful admission control.
