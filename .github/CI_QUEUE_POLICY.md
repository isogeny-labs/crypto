# Automatic CI queue policy

The owner requested a 15-minute limit for routine Actions jobs on 2026-10-19,
with expensive work removed from push/PR queues and scheduled where useful.
The limit is execution time per job: runner queue time and multi-job workflow
elapsed time are separate. Scheduled and explicitly dispatched work retain
their longer execution budgets.

## Routing

- 79 workflows have automatic job budgets bounded at 15 minutes. Their previous
  manual/scheduled budgets are preserved, including event-dependent values.
- 65 long research workflows formerly triggered by pushes to experiment
  branches now require `workflow_dispatch`. Existing inputs and execution steps
  remain available; select the intended research branch when dispatching.
- Nine expensive workflows run on the schedules below and remain manually
  dispatchable. Their push, PR, and merge-group triggers are removed.
- Routine stateless CI uses a separate concurrency group per workflow, event,
  and PR/ref, cancelling obsolete automatic runs. Existing global research and
  publishing serialization is retained. Manual scientific matrix limits and the
  single-writer autofix queue are retained.

| Workflow | Schedule (UTC) |
| --- | --- |
| CodeQL | Wednesday 02:41 |
| ecbench, including exhaustive historical replay | Sunday 01:37 |
| IC autolab integration | Daily 04:17 |
| Curve cover certificates | Daily 05:37 |
| Koblitz relation-yield bridge | Daily 06:53 |
| Performance history | Daily 08:19 |
| Polynomial reuse correctness | Daily 09:43 |
| Koblitz SOTA-gate reproduction | Saturday 10:23 |
| Release builds and publication | Daily 11:47 |

Release publication is now daily from `main`, or explicitly dispatched on
`main`, instead of publishing after every merge. Performance history records
daily snapshots instead of one snapshot for every main-branch push. These are
intentional cadence changes. A scheduled check failure must still be investigated;
moving the check does not establish that it passed.

## Live disablement and migration

The nine scheduled workflows were disabled through the Actions API during this
change to stop new expensive automatic runs before the trigger changes land.
After merging this PR, re-enable those nine workflows so their schedules and
manual dispatch are active. Do not enable them while main still has the old
push/PR triggers.

Six other workflows must remain disabled pending a separate migration:

| Workflow | Reason its source was preserved |
| --- | --- |
| `compact-s3-batch.yml` | Already disabled; reserved by T-4279 |
| `compact-s3-prefilter.yml` | Already disabled; reserved by T-4279 |
| `compact-swap-quotient.yml` | Already disabled; reserved by T-4279 |
| `weak-curve-database.yml` | Reserved by T-4258; disabled during this audit |
| `weak-curve-lean.yml` | Workflow bytes are part of a historical evidence seal; disabled during this audit |
| `weak-curve-conjectures.yml` | Workflow bytes are part of a historical evidence seal; disabled during this audit |

Their retained timeout declarations are not exceptions authorizing active long
PR checks. Re-enabling any of these six requires migrating automatic routing or
budgets first, while preserving the historical evidence. The existing
`check-pr-budgets.py` still implements the older 25-minute regression policy;
its test file is reserved by T-4258. This patch verifies the new 15-minute
configuration separately. Tightening that guard remains follow-up work; a pass
from the older guard alone does not prove compliance with this document.

Three additional active workflows were found in the queued/in-progress snapshot
on branches, without a workflow file on this `main` revision, and disabled:

| Workflow | Observed configuration |
| --- | --- |
| `boolean-single-write-runtime.yml` | Three PR jobs allow 25 minutes; source `6f0b8518f68cb51815039bf719bc384c0fe15074` |
| `koblitz-modular-carriers.yml` | PR/push replay allows 25 minutes; source `539325b5e66719f4cea7ee1315659808103d5bcb` |
| `n83-padd-rank.yml` | Branch-push panel allows 330 minutes; source `4b2b3efa6210ce3724f1350c3a9b1cc2df1309fd`; its sampled running job had already executed for 114 minutes |

Their branch contents are unchanged. They require an explicit routing/budget
migration before re-enabling. The other queued branch-only workflow inspected,
`validation-gate.yml`, already has a 10-minute budget and remains enabled.
The registered-workflow inventory included 524 entries (including historical
branches and two GitHub-managed dynamic workflows); the source configuration
audit covers all 431 workflow files on the checked main revision, plus these
four branch-only workflows seen in the live queue/running snapshot. It does
not claim to have audited every inactive branch revision.

The live cleanup selected 152 queued automatic runs in the disabled lanes or
superseded automatic groups. Immediately before each cancellation it re-read
the run state: eight had already left `queued` and were preserved; 144 were
cancelled. All 144 were subsequently verified `completed` / `cancelled`, and
all 18 workflows listed above were verified `disabled_manually` (15 disabled
in this task and three already disabled). The queue snapshots fell from 356
to 215 during cleanup; concurrent arrivals and completions mean that snapshot
delta is distinct from the 144 verified cancellations. Existing artifacts and
run records were retained.

## Evidence and limits

The initial live snapshot on 2026-10-19 found 354 queued runs and 15 in-progress
workflow runs. The queued set included 48 ecbench, 42 CodeQL, 40 Rust lint,
37 Rust all-targets, and 21 Release runs. The oldest queued run was created at
16:36 UTC, almost five hours before the snapshot. This is a snapshot of runs,
not a count of individual queued jobs or a measured queue reduction.

Successful job samples, using job `started_at` and `completed_at` rather than
workflow creation/update timestamps:

| Workflow/job | Minutes | Run |
| --- | ---: | --- |
| IC autolab controls | 22.5 | [39292988546](https://github.com/aburan28/crypto/actions/runs/39292988546) |
| IC autolab release integration | 15.9 | [39294211133](https://github.com/aburan28/crypto/actions/runs/39294211133) |
| Cover certificates | 22.0 | [39308571261](https://github.com/aburan28/crypto/actions/runs/39308571261) |
| Relation-yield smoke/custody | 18.2 | [39308514590](https://github.com/aburan28/crypto/actions/runs/39308514590) |
| Performance history check | 15.5 | [39308514484](https://github.com/aburan28/crypto/actions/runs/39308514484) |
| Polynomial reuse correctness | 23.3 | [39294972443](https://github.com/aburan28/crypto/actions/runs/39294972443) |
| SOTA reproduction | 38.0 | [39306190011](https://github.com/aburan28/crypto/actions/runs/39306190011) |
| Release isogeny Windows package | 18.7 | [39294211255](https://github.com/aburan28/crypto/actions/runs/39294211255) |

These samples establish that the named jobs exceeded 15 minutes. The other
budget changes are preventive configuration changes, not claims that every
affected job was measured above 15 minutes. CodeQL had a 60-minute routine
exception and sequential Rust preparation jobs; its latest sampled successful
runs skipped substantive analysis, so those samples are not scan-duration
measurements.

The repository reported zero self-hosted runners. No numeric matrix limit was
restricting the high-volume routine workflows: GitHub already maximizes matrix
parallelism subject to available capacity. Account-level hosted-runner limits
cannot be raised by increasing a workflow concurrency group or its queue depth.
See [GitHub concurrency documentation](https://docs.github.com/en/actions/concepts/workflows-and-actions/concurrency).

Validation includes Actionlint over all 431 workflows, the existing budget
checker and its 15 tests, 25 CI-scope/frozen-workflow helper tests, and a parsed
before/after comparison of 232 jobs across 153 changed workflows. Job steps,
matrix definitions, permissions, dependencies, and execution commands are
preserved; the Release publish condition additionally admits the new schedule.
Automatic timeout expressions and preserved manual/scheduled values were
evaluated independently. GitHub execution after merge is still needed to
measure the resulting queue and identify checks that need splitting to fit
the new limit.
