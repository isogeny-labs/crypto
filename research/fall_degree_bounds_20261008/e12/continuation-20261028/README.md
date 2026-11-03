# E12 continuation launch record

[Dataset persistence](S3_PERSISTENCE.md) records S3 destinations, periodic live
archives, checksums, retention, and the distinction from solver-memory recovery.

**E12b update, October 27 Pacific:** the user selected one new Spot attempt,
which has now launched. See [its receipt and handoff](E12B_STATUS.md).
Amendment 15 permits `launch.sh b-spot`: cell `3:24:7`, four draws, cap 4,
`m7i.4xlarge` (16 vCPU / 64 GiB), 24-hour lifetime. Source and gates remain
frozen. The post-run checker requires four complete draws and exact draw-0
agreement with the r2 reference after removing only `secs` fields. One attempt
only; no automatic relaunch. Receipts are under `receipts/b-spot/`.

[Earlier launch snapshot and handoff](STATUS.md): E12a and E12c were admitted on
2026-10-28 UTC, with startup source checks verified. E12b was pending at that
snapshot. These are launch observations, not completed scientific results.

[Amendment 13](../../PREREGISTRATION.md) permits one additional E12a and E12c
attempt after the user's explicit launch request. The exact source, mathematical
parameters, and acceptance gates remain frozen. Amendment 15 records the user's
subsequent authorization of one E12b Spot attempt.

| Part | Cell | Draws | Instance | Memory | Lifetime |
| --- | --- | --- | --- | --- | --- |
| a | `3:21:7`, degree cap 5 | 2 | `r7i.8xlarge` | 256 GiB | 48 h |
| c | `4:28:7`, window `w1x` | 4 | `r7i.4xlarge` | 128 GiB | 48 h |

The registered engines cannot resume an interrupted closure. Every new attempt
starts from scratch; a failure leaves the part incomplete with no automatic
relaunch. Output preservation is not checkpoint/restore support. Both machines
self-terminate and have the existing deadline/reaper protections.

**Capacity update:** the E12a `r7i.8xlarge` submission created no instance because
all configured zones lacked capacity or support. Amendment 14 permits the fixed
fallback sequence `r6a.8xlarge`, then `r6i.8xlarge`, only after a durable capacity
error and a fresh check finding no prior instance. Both have 32 vCPU / 256 GiB.
There can still be only one admitted E12a solver attempt. E12c's successful
submission is not repeated.

## Submission

After committing and pushing the amendment and scripts, from the repository root:

```sh
bash research/fall_degree_bounds_20261008/e12/continuation-20261028/launch.sh a
bash research/fall_degree_bounds_20261008/e12/continuation-20261028/launch.sh c
```

The Amendment 14 alternatives are `launch.sh a-r6a`, then `launch.sh a-r6i`
only if the prior alternative fails before creating any instance. Keep each
prior durable status under its receipt directory before invoking the next;
the script rechecks that status against S3 and queries EC2 before proceeding.

Each command atomically creates its own receipt directory before calling the
existing ephemeral launcher. It refuses to reuse that directory. A failed or
ambiguous submission is reconciled from its receipt and S3/EC2 state, never
repeated by deleting the directory. Jobs are detached and can run concurrently.
The command retains a protocol hash, source hash, frozen commit, and launcher
hashes. A submitted or running job is not a completed experiment.

## Follow-up

Read the job ID from `receipts/<part>/launch.stdout`. Use the existing launcher
with `EPHEMERAL_REGION=us-east-1` and `status JOB`, `logs JOB`, or `fetch JOB DIR`.
For example:

```sh
EPHEMERAL_REGION=us-east-1 taskq/deploy/ephemeral/ephemeral status JOB
```

The current runner does not JSON-escape its `reason` field. While these multiline
commands run, `status JOB` can fail to parse `status.json` despite an active
instance. Read the raw status, `job.log`, and EC2 state in that case. Never treat
a JSON parse error as permission to submit a replacement. Startup snapshots
preserve the raw status bytes with a `.raw` extension.

On completion, retain original raw files and verify the exact expected draw
count, `cells_status`, gate logs, source/binary identities, and the Amendment 11
criteria. E12a's degree-4 dimensions must match E10 draws 0 and 1. E12c can certify
an upper bound from a successful window closure, but a failed window cannot
establish a lower bound of 5. Publish results and a report separately from these
launch observations. No new scientific reading is claimed here.

## Prelaunch checks

Current account: `104186811093`; region: `us-east-1`. Read-only reconciliation
found no active study job. The registered source object exists (21,048,713 bytes).
AWS reports 32 vCPU / 256 GiB and 16 vCPU / 128 GiB for the selected types, and a
64-vCPU standard-Spot quota. The E12a Spot dry run returned `DryRunOperation`;
this establishes permission for the checked request, not capacity or completion.

Local validation consists of shell syntax, source/launcher identity checks,
case-selection and duplicate-submission refusal checks, and `git diff --check`.
The exact frozen engine tests and replay gates run on each launched host before
its new draws. Launch task: `T-655`; receipt/handoff task: `T-661`.
