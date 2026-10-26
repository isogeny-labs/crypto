---
name: queue-experiment
description: Submit a protocol-bound experiment to a queue and get its data back as a unified export - route a measurement job to an isolated host (isolab, the EKS isolab namespace, a taskq cpu-exclusive pool) and a throughput, search or training job to the cheapest spot capacity (spotlab, a taskq spot pool, the spot-system host) with an application or CRIU checkpoint that survives interruption and a restore that is validated before it resumes. Use whenever an experiment should run somewhere other than the current machine, must survive spot preemption, or must land as experiment.export/v1 rows with their isolation tier and checkpoint history.
---

# Queue an experiment

Read `AGENTS.md` §9a, §10 and §12, the `controlled-experiment` skill, and
the design `taskq/deploy/spot-checkpoint/README.md` (what is implemented,
what is designed, what is untested). A queue moves bytes and measures; it
decides nothing. Submit only a frozen protocol (`experiment-protocol`), and
treat a task id as neither approval nor result. Each job is paid capacity:
start the jobs the protocol names and no more, and kill what you keep.

## 1. Classify the job, then pick the backend

The protocol's `execution.measurement_class` decides where it may run:

| class | what is a metric | backend | checkpoint |
|:--|:--|:--|:--|
| `measurement` | a clock or a counter read against an A/A interval | isolab (`ecbench-independent-runner`, tiers A–D), the EKS `isolab` namespace (`isolab_k8s_benchmark`, L2 from the static CPU manager on the Spot pool), a taskq `cpu-exclusive` pool with Guaranteed QoS | `none`; an interrupted row is rerun as a new row, never resumed |
| `throughput`, `search`, `training` | counts, yields, found-or-not, a model | spotlab (cheapest AWS or GCP spot, resumes across clouds), a taskq spot pool, the spot-system host (`experiment-runner --resume`), `ephemeral --spot` for one command | `application` (a manifest generation with SHA-256 per file) or `criu` (a process image); after a restore counts are valid and the clock is not |

Counts never need a level; wall time needs the protocol's
`isolation_required`, which the backend grades from what it observed, never
from a node label (`research/eks_crypto_validation_20261019`: all 72 rows
earned L0 on a Spot VM under an L2 spec, and the report says so).

## 2. Write the job envelope

Fill `experiment-job.v1` (`taskq/deploy/spot-checkpoint/experiment-job.v1.json`)
once per (arm, instance) or per shard: the protocol path and hash, the arm,
the measurement class, the argv (no shell), setup steps, inputs by hash,
placement (CPUs, memory, families, spot allowed, isolation required), limits,
the checkpoint block (policy, interval, grace, store prefix, restore
validation), and the export target. Then compile it to the backend's own
spec by the table in the design README:

- **taskq**: `taskq.task-spec/v1` with `source.commit` pushed,
  `idempotency_key: <protocol_id>/<arm>/<instance>`, labels carrying
  `protocol_sha256`, `verify: {builtin: certificate}` for any solve, and the
  checkpoint fields as the design states (pending in the worker: a spec with
  them is handed back by an older worker, which is the safe failure);
- **isolab / isolab-k8s**: `isolab.job/v1` or `isolab_k8s_benchmark`
  (`fidelity.policy` from the required level; `--cpus auto` for ecbench);
- **spotlab**: `spotlab.job/v1` (`checkpoint.interval_s`, `checkpoint.grace_s`,
  `limits.max_cost_usd`; the command honours `$SPOTLAB_CHECKPOINT_DIR`,
  `$SPOTLAB_RESUMED` and SIGTERM);
- **spot-system**: a `jobs.json` entry with `checkpoint_contract:
  manifest-sha256-v1` and `experiment-runner --resume`.

The command itself must checkpoint as the design's contract says: write a
complete generation, fsync, write the manifest last, keep the previous valid
generation, restore only the generation it is handed, exit within the grace
period on SIGTERM. A program that cannot do this runs only under `criu` on a
pool that permits it, or on an isolated non-spot host.

## 3. Submit, wait, fetch

Use the backend's CLI or MCP tools (`taskq submit|wait|result`,
`isolab_submit`/`isolab_wait`/`isolab_result`, `spotlab run|reconcile|result|fetch`,
`ephemeral run --detach|status|fetch`). Detach and check back at the default
workflow's cadence; never poll in a loop. Under Claude Code the submit, wait
and fetch commands are a card (`experiment-director` compiles, the main
session relays verbatim, `executor-mechanical` runs, the director assesses);
the reading stays here. Record every attempt, decline and requeue the
backend reports: a `not_completed` task is never evidence about the thing
measured.

## 4. Export and land

Convert the backend's result into `experiment.export/v1` beside the raw
records (`experiment_protocol.py from-taskq|from-ecbench|from-rows` in the
crypto-autoresearcher checkout), with each row's `attempts`, `resumed`,
`checkpoint.restored_from_generation`, isolation tier and blockers, and
`wall_valid` false on any resumed, contended or under-level row. Run
`check-export --protocol ... --verify-files`. Land the export, the raw
records (§13a archive or loose), the host manifest and the run report
(`research-visuals`) in the PR; wall time appears only where the export
says it is valid.

## What this skill never does

- Decide a result, rank arms or claim a speedup: `controlled-experiment`,
  `ecbench-measure` and the reasoning tier do that from the export.
- Resume a measurement row, or count a restored throughput row's clock.
- Retry a non-zero exit or a timeout; only infrastructure failures retry,
  under the protocol's `max_attempts`.
- Launch instances by hand, leave a `--keep` host running, or write
  credentials into the tree.
