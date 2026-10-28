# E12b On-Demand authorization preflight

Observed 2026-10-28 UTC (2026-10-27 America/Los_Angeles). This is an
infrastructure preflight, not an experiment run or an amendment.

**Blocked:** the current AWS worker role can read the deployed ephemeral
configuration and source bucket, but `ec2:RunInstances --dry-run` for the
registered On-Demand host returned `UnauthorizedOperation`, with an explicit
deny from `SpotOnlyEC2Launch`. The shell exit status was 254. No instance,
launcher job, or new experimental draw was created.

The source of the continuation is [Amendment 12](../../PREREGISTRATION.md),
merged in [PR #2728](https://github.com/isogeny-labs/crypto/pull/2728). The
inspected revision is `26cdc819c908fffdb0cca14f677ed7485a836d64`.
The exact dry-run command is [command.sh](command.sh); the structured
observation, including the informative stderr excerpt, is [receipt.json](receipt.json).
The opaque encoded AWS authorization message is omitted from the excerpt.

## Reconciliation

- E9/E10 already completed and were merged in
  [PR #2700](https://github.com/isogeny-labs/crypto/pull/2700). No duplicate was run.
- E12b r2 remains interrupted after 13 completed draws across its four cells,
  including only draw 0 of `3:24:7`; see
  [PR #2727](https://github.com/isogeny-labs/crypto/pull/2727).
- Read-only EC2 enumeration found no pending, running, stopping, or stopped
  instance in account `104186811093`, region `us-east-1`.
- The S3 `jobs/20261027-` study prefixes ended at E12b r2; no matching study
  prefix existed under `jobs/20261028-`. No E12b On-Demand job was found.
  These are point-in-time observations, not a claim about other accounts.

## Resume requirement

The [open problem](../../../../problems/fall-degree-e12b-on-demand-launch.md)
requires an administrator-authorized environment that permits the registered
On-Demand launch. Do not change IAM policy or switch to Spot to bypass this
denial. An allowed launch dry run is necessary but does not guarantee instance
capacity or a successful experiment.

Before a real launch, reconcile EC2 and S3 again. Preserve the original archive
`fd-src-371035216942.tgz`, SHA-256
`4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f`, and the
registered `hky_par newcells` run: cell `3:24:7`, seed `20261012`, `keep=4`,
`cmax=4`, from scratch, without the Lean step. Pass gates 0–3 and compare draw 0
against r1/r2 on every non-timing field (gate 4). Apply the lower median to all
four draws; at least three must be censored at 5 or higher for the registered
"consistent with H_ell" reading. Any interruption or failure of the actual run
ends it as not completed, with no further relaunch under Amendment 12.

This failed dry run did not launch that experimental attempt. E12a and E12c
remain outside Amendment 12; their exhausted retry allowances are unchanged.
No mathematical verdict, timing comparison, or research-state promotion is
made by this record.

## Validation

The command is checked with `bash -n` only; it is not rerun as a validation
step. The JSON parses with `jq`, local links resolve in the Git tree, and
`git diff --check` passes. No solver or library source changes, so no new
algorithm test or experiment is warranted by this documentation record.

Conductor task `T-654` reserved these paths after `conductor check` reported
no overlap. The optional `.conductor/WORKFLOW.md` was absent in the inspected
Git tree and working directory.
