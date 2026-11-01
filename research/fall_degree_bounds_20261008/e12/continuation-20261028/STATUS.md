# E12 launch snapshot: 2026-10-28T02:20Z

Two continuation jobs have started. Neither has a completed new draw in this
snapshot. E12b has not launched: Amendment 12 requires On-Demand, and the current
role's `SpotOnlyEC2Launch` policy explicitly denies it. The user's choice between
retaining that requirement and registering a new Spot attempt is pending.

| Part | Job | Instance | Resources | Hard deadline (UTC) |
| --- | --- | --- | --- | --- |
| E12a, `3:21:7`, cap 5, two draws | `20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f` | `i-0f1e979a30bc4f52b` | `r6a.8xlarge`, 32 vCPU, 256 GiB, Spot | 2026-10-30T02:18:31Z |
| E12c, `4:28:7`, N=84, window `w1x`, four draws | `20261028-021036-fd-e12c-20261028-r3-914d46` | `i-0ac1c3ad72a141ac0` | `r7i.4xlarge`, 16 vCPU, 128 GiB, Spot | 2026-10-30T02:10:38Z |

Each job has a maximum lifetime of 48 hours. A Spot reclamation can end it
earlier. The engines do not resume an interrupted closure; no automatic retry is
authorized. The jobs run concurrently and self-terminate.

## What was done (commands, files, features)

- Reconciled prior cloud jobs and results before submitting continuations.
- Committed and pushed Amendments 13 and 14 before their respective submissions.
  E12c binds protocol commit `640b58b211e464939f077ef3722f3f4c00dd8bbb`;
  E12a's admitted fallback binds `f1e7c2d25df` (full SHA in its request receipt).
- Submitted through the existing ephemeral launcher with `--detach --no-source
  --spot --toolchain rust --ttl 48h`. The wrapper atomically reserves one local
  receipt directory per selection and refuses to reuse it.
- Both downloaded the exact frozen archive from
  `s3://ephemeral-runs-104186811093-us-east-1/jobs/inputs/fd-src-371035216942.tgz`.
  Both startup logs record `source.tgz: OK` for SHA-256
  `4ae808ac57920d93f599936c34513167befc93d865495defcae29fb16a18fb8f`.
- EC2 reports both instances `running`. E12a has started its build. E12c has
  completed gate 0 and reproduced gate 1 reference cell `B_3-18-6`; its remaining
  reference gates and all new draws are still pending in this snapshot.
- Preserved request, launcher output, submission outcome, EC2 state, startup
  logs, raw runner status, and protocol/launcher hashes under
  [`receipts/a-r6a/`](receipts/a-r6a/) and [`receipts/c/`](receipts/c/).

Local validation: shell syntax and whitespace checks, case selection,
duplicate-submission refusal, and a stubbed launcher check of resource/TTL flags.
No solver source changed. The frozen scripts run their original Rust tests and
reference gates remotely before new draws; no claim of a complete local Rust
suite is made for this launch-only change.

## Decisions made (and why)

The initial E12a job
`20261028-020921-fd-e12a-20261028-r3-4358b0` failed for lack of `r7i.8xlarge`
Spot capacity and created no instance. Its durable error is retained in
[`receipts/a/status.json`](receipts/a/status.json). Amendment 14 permits a fixed
pre-execution fallback order, with a fresh S3/EC2 reconciliation before each
submission. The first alternative, `r6a.8xlarge`, succeeded, so `r6i.8xlarge`
must not be submitted. Only one E12a solver attempt was admitted.

The archive, seeds, cells, draw counts, and mathematical acceptance gates stay
fixed. Prior study protocols/results and the existing ephemeral runner were the
references for this operational continuation. It introduces no new mathematical
method or performance comparison. A failed window closure in E12c remains
inconclusive about a lower bound.

## Open threads (unfinished, pick up next time)

Read logs under each job's prefix in
`s3://ephemeral-runs-104186811093-us-east-1/jobs/`. Do not relaunch from this
snapshot: reconcile current S3/EC2 state first. On completion, fetch the full
outputs and inspect gates, `cells_status`, exit codes, and actual draw counts.
E12a also requires degree-4 dimensions to match the two E10 reference draws.
A runner exit of zero alone does not establish scientific completion.

E12b is pending the user's response to the explicit On-Demand/Spot choice.
Do not substitute Spot or change IAM silently. Retain the On-Demand denial
evidence already published in PR #2734.

Conductor task T-661 owns this receipt publication. The shared
`docs/session-logs/2026-10-28.md` was exclusively reserved by T-660, so this
timestamped entry is preserved here without editing that task's file. Link or
append it to the daily log when that reservation is released.

## Errors (what went wrong, how it was handled)

The original runner writes its `reason` into JSON with unescaped `printf %s`.
The multiline command therefore makes the running `status.json` invalid JSON.
This is a status serialization failure, not evidence that the experiment
failed. Preserve the original `.raw` snapshots; use EC2 and `job.log` for current
observations. No running instance was replaced because of this parse error.

Releasing T-655 to restore its expired file reservations moved it to `cancelled`
under the Conductor CLI's default policy; reopening was rejected. T-661 records
the explicit successor scope. This bookkeeping error did not change any cloud
job or scientific artifact.
