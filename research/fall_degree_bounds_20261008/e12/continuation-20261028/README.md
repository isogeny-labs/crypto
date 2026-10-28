# E12 continuation launch record

[Amendment 13](../../PREREGISTRATION.md) permits one additional E12a and E12c
attempt after the user's explicit launch request. The exact source, mathematical
parameters, and acceptance gates remain frozen. E12b awaits the user's choice
about retaining On-Demand or registering a new Spot attempt.

| Part | Cell | Draws | Instance | Memory | Lifetime |
| --- | --- | --- | --- | --- | --- |
| a | `3:21:7`, degree cap 5 | 2 | `r7i.8xlarge` | 256 GiB | 48 h |
| c | `4:28:7`, window `w1x` | 4 | `r7i.4xlarge` | 128 GiB | 48 h |

The registered engines cannot resume an interrupted closure. Every new attempt
starts from scratch; a failure leaves the part incomplete with no automatic
relaunch. Output preservation is not checkpoint/restore support. Both machines
self-terminate and have the existing deadline/reaper protections.

## Submission

After committing and pushing the amendment and scripts, from the repository root:

```sh
bash research/fall_degree_bounds_20261008/e12/continuation-20261028/launch.sh a
bash research/fall_degree_bounds_20261008/e12/continuation-20261028/launch.sh c
```

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
its new draws. Conductor task: `T-655`.
