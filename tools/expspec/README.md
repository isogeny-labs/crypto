# expspec: one YAML file per experiment, compiled into task cards

An **experiment spec** (`experiments/specs/<id>.yaml`, schema
`crypto.experiment/v1`) is the machine-readable form of a pre-registration:
question, cells, steps, gates, hosts, budget and the rules that will read the
result. `tools/expspec/expspec.py` tracks the spec's run in
`experiments/specs/<id>.state.yaml` and compiles it into
`tools/task_card.py` frozen-command cards, one step at a time. It never runs
an experiment command itself and never interprets a result.

The spec fits the model tiers in AGENTS.md. The reasoning tier writes and
approves the spec, and reads the outcome. `expspec next` does the director's
*compile* step mechanically for anything the spec already decides. The
executor (`executor-mechanical`, Haiku) runs each card through
`task_card.py run` and relays the output verbatim. Haiku never reads the
spec.

```
  frontier/core          expspec.py (no model)            executor-mechanical (Haiku)
  ┌──────────────┐  approve  ┌────────────────────┐  card   ┌──────────────────────────┐
  │ spec.yaml    │ ────────► │ next: ingest       │ ──────► │ python3 tools/task_card  │
  │ PROTOCOL.md  │           │  receipts, gates,  │         │   .py run <card>         │
  │ decision     │ ◄──────── │  budget, hosts →   │ ◄────── │ (receipt, verbatim text) │
  └──────────────┘  facts    │  one action        │ receipt └──────────────────────────┘
                             └────────────────────┘
```

## The loop (main session)

```bash
python3 tools/expspec/expspec.py validate experiments/specs/EXP-FOO-001.yaml
python3 tools/expspec/expspec.py next experiments/specs/EXP-FOO-001.yaml
```

`next` reads any receipts that have appeared, updates the state file and
prints exactly one JSON action:

| action | what the main session does |
|---|---|
| `run_card` | Relay the output of `relay` (`task_card.py prompt <card>`) to executor-mechanical, unchanged. Run it in the background when `background` is true. When `assess` is true (an evidence step), dispatch experiment-director in assess mode on the returned text. Then call `next` again. |
| `ask` | Ask the owner `question` word for word. Record the answer with `expspec.py answer <spec> --gate <gate> --yes/--no --by <name>`, then call `next`. |
| `escalate` | Hand `reason` to the reasoning tier. If it decides to continue, record that with `answer --yes`; to stop, `answer --no`. A `spec_drift` escalation means the spec or the checkout must be fixed: it is not a question. |
| `wait` | Schedule a check-in at `until` and do nothing else. Never poll. |
| `done` | The reasoning tier runs `expspec.py facts <spec>`, applies `outcome.decision_rules`, and writes `reporting.results_file`. |
| `stopped` | Report `reason`. Any open batch host has already been given a teardown card. |
| `refused` | The spec does not validate or is not approved. Fix the spec. |

`expspec.py ls` lists every spec in `experiments/specs/` with its spec
status, run status, steps done, spend against budget, and what it is blocked
on. Use it as the experiment tracker.

## Field reference

Everything below is enforced by `experiment.v1.schema.json`. `validate` adds
cross-reference checks: ids exist, needs come earlier, templates resolve,
every cell fits some host, frozen hashes match, and the approval decision
approves this id and is committed.

| field | meaning |
|---|---|
| `id`, `version`, `amends` | `EXP-<AREA>-<NNN>`. The file name is `<id>.yaml`. An amendment bumps `version` and says whether it was written before any run. A state file is bound to one spec sha256, so editing a running spec stops the run with `spec_drift`. |
| `status` | `draft` (plan only), `approved` (frozen, runnable), `retired`. Run progress is in the state file. |
| `approval` | `decision` is the committed `coordinator_decision` YAML that approves this id. Every card names it and `task_card.py lint` re-checks it. |
| `question` | Questions and predictions with confidences, written before any run. |
| `source` | `repo`, `ref` (the branch hosts clone), `commit` (where the code comes from) and `frozen` (path + sha256, bound into every card). HEAD must equal or descend from `commit`. Without `allow_drift`, the commits in between may touch only specs, the decision and run products. |
| `paths` | Repo-relative directories: `root`, `out`, plus `cards`, `collected` and `ckpt` (defaults under root) and any others. expspec creates all of them before it writes the first card. |
| `cells` / `matrix` | Parameter points. Each cell has a `role` (measure, control, calibration, smoke), a `priority`, an `optional` flag and a `requires` minimum host. A cell no host can run is recorded `not_attempted`, never `failed`. |
| `steps` | Ordered. `scope` is once, per_host (again on each new host) or per_cell. `runs_on` is controller or host. Each step also has `needs`, argv (no shell), `timeout_s`, `outputs`, `must_not_exist`, `retry_exit_codes` (infrastructure only), `censored_exit_codes`, `capture` (stdout values into `{{vars.*}}`), `done_when` (resume by skipping), `gates.before/after`, `evidence` and `on_failure`. |
| `gates` | `checks` (now), `wait_until` (re-checked on each `next` until `max_wait_s`, optionally after a `refresh` card) and `human` (an `ask` action). `on_fail` is stop, skip_cell, void_cell or escalate. |
| checks | `file_exists`, `sha256`, `sha256_manifest` (a sha256sum file), `json` (a dotted path into .json/.jsonl with eq/ne/lt/le/gt/ge/in/exists/truthy/all_true/count_ge), `status_token` (a line of `collected/STATUS`) and `command` (read-only argv, exit 0). |
| `hosts` | Candidates, taken `ordered` or by `cheapest_fit`. `backend` is local, ssh (host steps become `ssh <host> 'cd <workdir> && …'` cards) or batch (a launched machine driven only through `lifecycle`: provision, which must capture `handle`, then poll, collect and teardown). Each candidate also records `timing_valid`, `preemptible`, `paid`, `est_hourly_usd`, `cells` and the launcher. |
| `checkpoint` | `resume` (none, rerun_step, from_ckpt) and `transport`. |
| `budget` | `max_cost_usd` and `max_wall_hours` are hard stops. Crossing `escalate_at_fraction` asks the owner. Spend is estimated as hourly price × host hours, capped at `lifetime_s`. |
| `metrics` | counted, timing, memory, derived or boolean, each with a JSON source. `facts` labels timing metrics descriptive unless a `timing_valid` host ran. |
| `outcome` | `decision_rules` (in words, optionally with `checks` whose results `facts` prints), `invalidation` and `censoring`. Verdicts stay with the reasoning tier. |
| `reporting` | `results_file`, `sections`, `required_artifacts` (checked before `done`; `{{cell.*}}` entries are required per completed cell), `visuals` and `benchmarks_dir`. |
| `agent` | Executor and director models; `autonomy` (never/ask/allowed for launching paid hosts, terminating own hosts, retries, commit, push, PR, plus `spend_without_asking_usd`); `escalate_on`. |

### Templates

`{{exp.id}}`, `{{exp.version}}`, `{{paths.<name>}}`, `{{cell.id}}`,
`{{cell.role}}`, `{{cell.<param>}}`, `{{host.id}}`, `{{host.index}}`,
`{{host.<config key>}}`, `{{vars.<captured>}}` (`handle` is the batch host's
id) and `{{env.<NAME>}}`. An unknown variable is a validation error.

### Instance and state statuses

Instances are `pending`, then `carded` (a card is waiting for its receipt) or
`running_remote` (a batch host is running it). They end as `done`,
`skipped_done`, `failed`, `censored` (an infrastructure exit; never
evidence), `not_attempted` (no host fits, a gate skipped it, or the budget
dropped it) or `void` (a control or gate invalidated the cell). The run
status is `not_started`, `running`, `waiting`, `blocked`, `completed` or
`stopped`.

## Batch hosts (EC2 spot, RunPod, spotlab, isolab)

A batch host runs the experiment's host steps itself. For EC2 spot, PR
aburan28/crypto#2228's `aws_spot.py run` starts `boot.sh`, which writes
`<UTC> <TOKEN>` lines to `STATUS` and serves its outputs on :8000. The
controller only:

1. **provision**: one card. Its stdout's JSON `id` becomes `{{vars.handle}}`. If the host is paid and `launch_paid_host` is not `allowed`, or the projected spend exceeds `spend_without_asking_usd`, `next` first asks the owner.
2. **poll**: a fetch card every `poll_every_s` (at least 300 s). A host step is done when its `status_done` token appears in `collected/STATUS`, and failed on a `status_failed` token.
3. **collect** when every host step is terminal, then **teardown**.

A batch host's own steps cannot carry `gates`: the host runs them by
itself, so `validate` rejects them. Gate the controller step that reads the
collected outputs, or rely on the launch ask.

A cell that does not fit the current host waits. After teardown the next
candidate that fits it is provisioned. See the `a1`/`c2` split in
`experiments/specs/EXP-DREG-M4L6-001.yaml`.

## What v1 does not do

These limits are deliberate; each is a place where a human or the reasoning
tier stays in the loop.

- **One host at a time.** No parallel fan-out across hosts.
- **No preemption recovery for batch hosts.** A poll card that fails escalates ("the host may be gone"). Re-provisioning and re-running a `rerun_step` instance is the owner's call.
- **Spend is an estimate**, not a bill: launch price × hours. Live spot prices are not read, so `cheapest_fit` uses `est_hourly_usd`.
- **Per-cell progress on a batch host needs per-cell STATUS tokens.** Today's `boot.sh` emits per-phase tokens (`GEN_DONE`, `A_DONE`, `CLOSURE_DONE`), so those host steps are `scope: once`.
- **Wait gates and polls advance only when `next` is called.** The main session schedules check-ins at the `until` it is given.
- **No verdicts, no PRs, no visuals.** `facts` prints values and check results. The results file, graphs, PDF and PR stay with the reasoning tier (AGENTS.md "Research searches must leave visual reports").

## Tests

```bash
python3 -m unittest discover -s tools/expspec/tests -v
```

Each test builds a throwaway git repository with `task_card.py`, expspec and
fixtures, approves a spec with a committed decision, and plays the main
session until `done`. Every card must pass `task_card.py lint` and run. The
local run covers resume through `done_when`, spec drift and facts. The batch
run uses a fake launcher with `aws_spot.py`'s verbs and covers two hosts, the
paid-launch ask, the human gate, STATUS tokens, collect and teardown; it also
checks that answering no to the launch stops the run. CI:
`.github/workflows/expspec.yml`.

## Examples

- `experiments/specs/EXP-EXPSPEC-SMOKE-001.yaml`: local, three-cell matrix, a per_host step, an after-gate, a once step. Draft; approve it to run.
- `experiments/specs/EXP-DREG-M4L6-001.yaml`: `research/dreg_closure_m4l6_20261018/PROTOCOL.md` mapped onto the EC2 spot launcher. Draft; values marked `CONFIRM` must be checked first.
