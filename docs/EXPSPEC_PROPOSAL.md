# Proposal: one tracked YAML spec per experiment, run by Haiku through task cards

Status: proposed, 2026-10-20. Implementation: `tools/expspec/` (Go), schema
`crypto.experiment/v1`, PR aburan28/crypto#2352.

## 1. Problem

An experiment in this repository is currently spread across several places:

- the prose pre-registration (`research/<topic>/PROTOCOL.md`);
- a launcher (`pod/launch.py`, PR #2228's `aws_spot.py`, spotlab, isolab);
- a boot script and a `run.sh`;
- the director's hand-written task cards;
- chat history.

Nothing records in one place which experiments exist, which are approved, what
each has spent, and what each is waiting on. Every run is also re-planned by
hand, which costs effort and lets errors in. The protocol's machine-checkable
parts are read by a model each time: which cells, which host, which gate opens
measuring, the budget cap, and what counts as censored rather than evidence.
Examples:

- the dreg m4l6 run needed two mid-run amendments (§8, §10) to change host,
  cell set and draw count;
- its tag gate had to be swapped for a checksum gate after a git-proxy refusal;
- the $50 cap was tracked by hand.

AGENTS.md "Model tiers" already says who may do what:

- frontier/core models decide;
- the Sonnet director compiles a decided experiment into a task card;
- Haiku runs only `tools/task_card.py run <card>` and refuses anything
  underspecified.

What is missing is a standard, machine-readable form of "a decided
experiment" that the compile step can work from without judgment.

## 2. Goals and non-goals

Goals:

1. **One file per experiment, one place to list them all.** `experiments/specs/<id>.yaml` holds the plan, and `expspec ls` shows status, progress, spend and blockers across experiments.
2. **Haiku can run an approved experiment end to end** without making any choice. Every value it needs is in the spec or captured mechanically from earlier steps.
3. **Fit the existing tiers and tools.** Cards stay `crypto.autoresearch.mechanical-task-card.v1`, linted and run by the canonical `task_card.py`. Approval stays a committed `coordinator_decision`. Launchers stay what they are.
4. **Hard limits are enforced, not remembered.** These are budget, wall clock, paid launches, pushes, frozen inputs and the pinned commit.
5. **Pre-registration is mechanical where it can be.** Predictions, decision rules, censoring and invalidation are written before the run, and `facts` evaluates any checks attached to them.

Non-goals:

- **Interpreting results.** Verdicts, claims, the results file, visuals and PRs stay with the reasoning tier.
- **Replacing PROTOCOL.md.** The prose stays the authority; the spec is its index and run plan, and it binds the protocol's hash.
- **Replacing spotlab, isolab or aws_spot.py.** They are wrapped as batch hosts through four lifecycle commands.
- **A scheduler.** The main session calls `next` and schedules its own check-ins.

## 3. Design

### 3.1 Three files and a loop

| file | written by | role |
|---|---|---|
| `experiments/specs/<id>.yaml` | reasoning tier | The plan. Frozen once `status: approved`. Its sha256 is pinned by the state file, so editing a running spec stops the run (`spec_drift`). |
| `decisions/…yaml` (existing `coordinator_decision`) | Coordinator | Approval. The spec names it, and every card names it again for `task_card.py lint`. |
| `experiments/specs/<id>.state.yaml` | `expspec` only | Run state: hosts, instances, attempts, gate results, captured vars and an append-only event log. |

```
reasoning tier writes + approves spec
        │
        ▼
expspec next  ──► one action ──► run_card ──► main session relays `task_card.py prompt` ──► Haiku runs `task_card.py run`
   ▲                  │                                                                         │
   │                  ├─ ask       ──► owner answers (`expspec answer`)                         │
   │                  ├─ wait      ──► main session schedules a check-in                        │
   │                  ├─ escalate  ──► reasoning tier                                           │
   │                  └─ done      ──► reasoning tier: `expspec facts`, decision rules, report  │
   └──────────────── receipt (ingested on the next call) ◄──────────────────────────────────────┘
```

`expspec` is the director's compile step for everything the spec already
decides. The director keeps its *assess* step: `next` marks the cards of
`evidence: true` steps with `assess: true`, and the main session dispatches
experiment-director on those receipts.

### 3.2 The spec

The full field reference is in `tools/expspec/README.md`; the schema is
`tools/expspec/experiment.v1.schema.json`. Its parts:

- **Identity and lifecycle:** `id`, `version`, `amends` (with `before_any_run`), `status`, `approval`.
- **Pre-registration:** `question.questions`, `question.predictions` (with confidence), `outcome.decision_rules` (in words, plus optional checks), `outcome.invalidation`, `outcome.censoring`.
- **Inputs:**
  - `source.commit`: HEAD must equal it or descend from it, and in between only specs, the decision and run products may change unless `allow_drift`.
  - `source.frozen`: path + sha256, bound into every card.
- **The work:**
  - `cells`, or a `matrix` that expands into cells. Each cell has a role (measure, control, calibration, smoke), a priority, an optional flag and a minimum host.
  - Ordered `steps`, with scope once / per_host / per_cell, run on the controller or the host. Each step carries argv (no shell), a timeout, outputs, `done_when` for resume, captures and an `evidence` flag.
  - `gates`: checks, `wait_until`, or a human question.
- **Where:** `hosts.candidates`.
  - local, ssh, or batch.
  - Batch hosts are driven by `provision` (which captures `handle`), `poll`, `collect` and `teardown` commands. They report progress through `boot.sh`'s existing `STATUS` token convention.
- **Limits:** `budget` (cost cap, wall cap, ask at a fraction) and `agent.autonomy` (never / ask / allowed for paid launches, terminating own hosts, retries, commits, pushes and PRs).

### 3.3 Statuses that keep evidence honest

Instance outcomes:

- `failed`: a real failure.
- `censored`: a timeout or a listed infrastructure exit (an OOM kill, a lifetime cap). Never read as evidence.
- `not_attempted`: no host fits, a gate skipped it, or the budget dropped it.
- `void`: a control or gate invalidated the cell.
- `skipped_done`: resume found the output already present.

These encode the repository's existing rule that infrastructure failure is not
evidence. They also keep "deferred" (A2, B1, B2, C2 in the dreg run) distinct
from "failed".

### 3.4 Why Go

The tool is one static binary with the schemas embedded, built with
`go -C tools/expspec build`. It has two dependencies: `gopkg.in/yaml.v3` and
`santhosh-tekuri/jsonschema/v6`. `task_card.py` stays Python because it is
canonical in crypto-autoresearcher and copied byte-identical into every
repository. `expspec` shells out to `python3 tools/task_card.py lint` for every
card it writes, so there is one lint, not two.

## 4. Alternatives considered

| alternative | why not |
|---|---|
| **Haiku reads the YAML and runs it** | It breaks the tier rule that the mechanical executor makes no choices. Choosing the next instance, evaluating a gate and estimating spend are all choices. A tool makes them reproducibly; a model guessing is the failure the tier exists to prevent. |
| **The director (Sonnet) compiles every card by hand from PROTOCOL.md** | This is today's path. It costs effort per card, and nothing tracks state across cards. It stays available for anything the spec cannot express. |
| **Extend crypto-autoresearcher's `experiment.schema.json`** | That schema covers the ledger side (hypothesis id, metrics, stopping and invalidation rules) and generator/verifier run lists. It has no hosts, budget enforcement, gates, resume or state. Where the two overlap the concepts map one to one (`invalidation_rules` → `outcome.invalidation`, `stopping_rules` → `budget`, `success_criterion`/`falsification_criterion` → `outcome.decision_rules`), so a converter is mechanical. |
| **spotlab's `spotlab.job/v1` as the experiment format** | It is a job format: one command plus checkpoints on spot capacity. An experiment is many steps, cells, gates and possibly several hosts. spotlab fits as one batch host candidate. |
| **A workflow engine (Argo, Airflow, GitHub Actions)** | These need infrastructure the repository does not run, and they bypass `task_card.py` receipts and the approval check. |

## 5. What was validated

`go -C tools/expspec test ./...` passes locally and runs in CI
(`.github/workflows/expspec.yml`, path-filtered). Each test builds a throwaway
git repository, approves a spec with a committed decision, and plays the main
session. Every card is linted and run by the real `task_card.py`.

- **Local smoke:** 5 cards in order, metrics read back by `facts`, resume skipping an existing cell, and an edited running spec giving `spec_drift`.
- **Batch:** a fake launcher with `aws_spot.py`'s verbs runs two hosts, each fitting one cell. The run goes through the launch ask, a human gate, STATUS tokens, collect and teardown on both, with no escalations. Answering no to the launch stops the run with no host.
- **Validation:** all committed specs validate, a draft refuses to run, and a gate on a batch host's own step is rejected.

`EXP-DREG-M4L6-001` maps `research/dreg_closure_m4l6_20261018/PROTOCOL.md` onto
the spot launcher as a worked example. It validates, and its warnings
reproduce the protocol's own deferrals: A2, B1, B2 and C2 do not fit, wall times
are descriptive, and the full matrix exceeds $50. It is a draft. Five values
are marked `CONFIRM`: frozen hashes, the spot price, the RunPod launcher verbs,
the run-file JSON layout, and a copy helper that is not written yet.

## 6. Risks and limits

- **Spec quality is now the bottleneck.** A wrong spec runs wrongly and confidently. Mitigations: approval is a committed decision, `validate` runs in CI, and `plan` shows every expanded command before approval.
- **One host at a time.** No parallel fan-out. That suffices for current runs, which are one large-memory host at a time.
- **Spend is an estimate:** `est_hourly_usd` × host hours, capped at the lifetime. A wrong price means a wrong cap; live prices are not read.
- **Batch preemption is not recovered automatically.** A failed poll escalates to the owner.
- **Per-cell tracking on a batch host needs per-cell STATUS tokens.** Today's `boot.sh` emits per-phase tokens, so host steps there are `scope: once`.
- **Two languages.** The card runner is Python (canonical, shared) and the compiler is Go. The interface between them is the card YAML and the receipt JSON, both already versioned.

## 7. Rollout

1. Land the schema, tool, tests and examples (PR #2352). Nothing existing changes.
2. Confirm the `CONFIRM` values in `EXP-DREG-M4L6-001`, add `copyruns.go`, commit a coordinator decision, and run it as the first real experiment. Compare the effort against the hand-run §8/§10 amendments.
3. Add per-cell `STATUS` tokens to `pod/boot.sh` so batch steps can be `per_cell`.
4. Write new pre-registrations as a spec next to PROTOCOL.md. Back-fill only experiments that will run again.

## 8. Open questions for the owner

1. **Location.** Should specs live in `experiments/specs/` (one tracker directory, as now) or next to each study in `research/<topic>/experiment.yaml`?
2. **Approval form.** Is approving with the existing `coordinator_decision` file right, or should approval be a field signed in the spec itself?
3. **Default autonomy.** The examples set `launch_paid_host: ask` and `spend_without_asking_usd: 0`. Is there a per-experiment amount Haiku may launch without asking?
4. **Cross-repo.** Should `crypto.experiment/v1` become canonical in crypto-autoresearcher, like `task_card.py`, so cryptanalysis uses the same tool?
5. **Next host type.** Should native spotlab and isolab lifecycle commands come next, or EC2 via `aws_spot.py` only?
