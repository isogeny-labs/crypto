# Restart experiments from any agent

The runner emits `state/JOB/run.json` using `experiment.run/v1`. It describes the
execution and recovery contract without referencing a Claude, Codex or OpenCode
session. An agent reads the JSON and submits it to the installed runner. The
background service owns the experiment, so the agent does not need to remain
connected or recreate the original launching session.

On the experiment worker:

```bash
experiment-runner --root /srv/experiments/spot-worker --list-runs
experiment-runner --root /srv/experiments/spot-worker \
  --resume state/search-001/run.json
```

The second command returns JSON with `state: "submitted"`. The installed
`spot-experiments.service` picks up the request. `state: "complete"` means the
verified completion marker already exists; it does not rerun the experiment.
`state: "refused"` includes the reason recovery could not be admitted. The CLI's
nonzero exit status also reports refusal. To execute synchronously when there
is no service, add `--once`; if another supervisor owns the workspace, the
request is submitted instead of creating a duplicate worker.

The same commands can be issued by a Codex tool over SSH, a terminal, Claude or
OpenCode. Existing prompt/SSH wake hooks start the host; the job definition and
checkpoint do not require those clients' session state. Worker IAM credentials
come from the current host. Nonsecret region/activity-table settings are loaded
from the durable `runner-config.json`, so an SSH caller participates in the
same idle/admission fence as the system service.

For managed deployments, submission invokes the stack's wake function with
`managed_work: true`. This explicitly starts the managed worker and clears an old
draining flag under the existing lifecycle lock, even while the separate legacy
source still holds the SSH IP. No agent needs the original launching session or
its environment. Only the managed ASG changes demand; source-IP protection stays
active. The managed worker rechecks admission before starting the application.

## Record contract

[`experiment-run.schema.json`](experiment-run.schema.json) defines the JSON
shape. A record contains:

- `schema`, stable job `id`, and `spec_sha256` bound to durable `spec.json`.
- `job`: exact argument array, working directory, checkpoint contract and only
  explicitly declared `env` settings. Inherited credentials, harness session
  variables and process environment dumps are not included.
- `workspace`: a location hint, and `paths`: checkpoint/output/log/record paths
  relative to that workspace.
- `checkpoint`: validation level and `selection: "latest-at-resume"`.
  `last_selected` is the generation validated at an attempt's start, not a pin
  that prevents choosing a newer valid checkpoint.
- `status`: phase, observation time, attempt id, host, PID and terminal exit
  code when available. It is observational; after a host loss it can be stale.
- `resume`: the stable `experiment-runner` submission command and the
  `resume-only` policy. The CLI validates the record's job; it never evaluates
  shell text from the record or trusts its status/checkpoint hints as proof.

Records are committed atomically as mode 0600 files. They are updated on
preparation, execution, interruption, refusal/failure and completion, with a
30-second running heartbeat. Status write failures do not orphan a running
child. The committed application checkpoint and completion marker remain
authoritative.

An example job entry is:

```json
{
  "id": "search-001",
  "cwd": "work/search",
  "argv": ["./search", "--seed", "20261019"],
  "env": {"RAYON_NUM_THREADS": "16"},
  "checkpoint_contract": "manifest-sha256-v1"
}
```

Use durable inputs, executables and dependencies, or the pinned runtime paths.
The record is control metadata; it does not embed application checkpoint bytes,
copy arbitrary temporary files or install missing research dependencies. Job
arguments remain an array, with no shell interpolation by the runner. The runner
owns `SPOT_CHECKPOINT_DIR`, `SPOT_OUTPUT_DIR`, `SPOT_RESUMED` and
`SPOT_CHECKPOINT_MANIFEST`; a job's `env` cannot replace them.

## Resume validation

The CLI verifies the schema/version, safe job id, specification hash, exact
durable specification and workspace paths. A record with a different command
or a changed existing job is refused. If `jobs.json` is missing or the job is
absent, the validated embedded job reconstructs admission while preserving other
jobs. The workspace's `state/JOB/spec.json`, checkpoint and required files must
still exist; possessing a record alone cannot recreate lost data.

A strict `resume-request.json` is committed before exposing a newly admitted
job to the service. Both the client and executor require recovery state.
Manifest-contract jobs run native discovery again and verify the selected
generation's identity, hashes and paths. Newer corrupt generations can fall
back to a previous valid one. Missing/unusable state cannot silently become a
fresh run. Completed jobs are skipped. Legacy atomic-file jobs retain their
application-managed validation; nonempty files alone do not prove the contents
are recoverable. Prefer the manifest contract for integrity-checked recovery.

`--root` can select a coherently copied workspace; absolute application argv/cwd
paths still require their original locations or a newly identified job. The
format is independent of agent harnesses, not a promise of arbitrary program,
operating-system or CPU compatibility.

## Existing runs

Export descriptors for existing managed runs without restarting them:

```bash
experiment-runner --root /srv/experiments/spot-worker --export-runs
```

This backfills only missing `run.json` files for existing, matching durable
identities, including saved runs no longer listed in `jobs.json`. It never
creates an attempt or starts an application. The returned
index uses `experiment.run-index/v1`.

The separately enrolled legacy CRIU processes use their existing fenced recovery
daemon and private S3 discovery pointers. Their restored executable can retain
a historical `/tmp/claude-...` pathname; restoring its saved bytes does not run
or require Claude. These process-memory images do not become native managed
checkpoints through a format declaration.

## Executed validation

The native kill/restore fixture launches under synthetic Claude session values,
exports a record, loses its worker, removes the original jobs manifest, corrupts
newer generations and resumes under a synthetic Codex session. It restores the
older valid progress and completes, without serializing inherited session or
secret values. Infrastructure tests also cover privacy, explicit settings,
tampered identities/paths, missing state, completed-run idempotence, concurrent
supervision and admission without the original launcher's environment.

The deployed worker role also passed a separate disposable handoff: the first
native process was killed after progress 3, its jobs manifest was removed, and
a new launcher used `run.json` and the durable worker configuration to invoke
managed wake, pass admission, restore progress 3 and finish at 12. Both launcher
labels are synthetic environment fixtures; the proof exercises the actual CLI,
native process and AWS role, without automating either agent's interactive UI.
The existing source experiment and SSH IP were preserved.
