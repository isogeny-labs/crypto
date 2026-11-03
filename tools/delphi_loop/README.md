# delphi-loop: a continuous learning loop for the task-card lane

`tools/task_card.py lane` sends the director's two calls, **card-compile** and
**assess**, to Delphi. The reply to each is judged by `ingest`, a mechanical
referee. `lane --route` asks `delphi route -family F -json` which model each
call should go to. Delphi has no capability registry yet (no `route` or
`calibrate` subcommand in aburan28/conductor at `343e7c3`), so `--route` has
not been usable (session log 2026-11-01).

This loop learns that registry from the lane's own verdicts and answers
`route` from it. LangSmith is the shared, durable store, so what one ephemeral
container learns, the next one starts from.

```
 lane ──► <card>.lane/lane.json ──harvest──► episodes ──fit──► registry ──route──► lane --route
                                               │  ▲
                              push (traces +   │  │ pull (other containers' runs,
                              ingest feedback) ▼  │       human `review` feedback)
                                              LangSmith ◄── dataset (accepted pairs)
                                                  │
                                eval (replay a dataset on a candidate target,
                                      scored by the same ingest) ──► episodes
```

## The signal

Each director call becomes one **episode**, labelled by what `ingest` decided:

| label | meaning | counts against the model |
| --- | --- | --- |
| `accepted` | ingest wrote the card or the assessment | no (a success) |
| `rejected` | ingest refused the reply (lint findings, a wrong outcome or recommendation, malformed blocks) | yes |
| `refused` | the model declined (`delphi` status refused) | yes |
| `spec_error` | the compile reply listed `specification_errors`, which is the design's gap | no (excluded) |

A human can override `accepted` or `rejected` in LangSmith with feedback key
`review` (score 1 or 0), for example from an annotation queue; `pull` brings
the override back. No model grades another model.

## The policy

`fit` puts a Beta prior on each family's targets (`targets.yaml`, cheapest
first) and updates it with the episodes. It records the posterior mean, a
one-sided 90% lower bound, and list-price cost per call and per accepted
reply, at the batch discount (`../token_prices.json`).

- **`exploit`** (default): routes to the cheapest target whose lower bound
  clears `threshold` (0.8) on at least `min_n` (5) episodes. Otherwise it keeps
  the incumbent, the lane's previous fixed director `claude-sonnet-5-5`. An
  unmeasured model never takes over.
- **`thompson`** (`DELPHI_LOOP_POLICY=thompson`): draws each acceptance rate
  from its posterior and takes the cheapest draw over the threshold, so cheaper
  targets keep being tried while their evidence is thin. The draw is in the
  route's `basis`, which `lane` writes into `lane.json`, so every exploratory
  call is labelled as one.
- **Escalation**: every target after the choice. A refused step always moves to
  a stronger model, which is the chain `lane` already walks.
- `DELPHI_FREE` restricts routing to targets marked `free: true`.

## Use

```sh
# once per container; push, pull and dataset only (harvest, fit, route and eval do not need LangSmith)
pip install -r tools/delphi_loop/requirements.txt
export LANGSMITH_API_KEY=...                 # LANGSMITH_PROJECT defaults to delphi-harness

python3 tools/delphi_loop/loop.py pull       # start from what other containers learned
python3 tools/delphi_loop/loop.py fit        # print and write .delphi-loop/registry.json

# run lanes, routed by the registry (other delphi subcommands reach the real binary)
DELPHI=tools/delphi_loop/delphi DELPHI_REAL=delphi \
  python3 tools/task_card.py lane --design <spec> --card <card.yaml> --assessment <a.yaml> --route

python3 tools/delphi_loop/loop.py harvest    # lane records -> episodes
python3 tools/delphi_loop/loop.py push       # traces and verdicts (hashes only)
```

Before promoting a cheaper model, measure it offline on prompts that already
have accepted replies:

```sh
python3 tools/delphi_loop/loop.py eval --family assess --target anthropic:claude-haiku-5-5@high --limit 10
python3 tools/delphi_loop/loop.py fit
```

`eval` makes paid model calls through `delphi run`. Each replay is a
lane-shaped record under `.delphi-loop/eval/`, labelled `source: eval`, and it
counts toward the registry like any lane.

## What leaves the machine

Only `push`, `pull`, `dataset` and `eval` talk to anything.

- **`push`** sends sha256s, labels, targets, usage and route bases by default.
  `--include-text` also uploads prompts and replies.
- **`dataset`** always uploads text and needs `--yes-upload-text`.

A design can be unpublished research, so send text only for public or
synthetic designs, or to a self-hosted LangSmith (`LANGSMITH_ENDPOINT`).

## State

`.delphi-loop/` (gitignored; `DELPHI_LOOP_HOME` moves it) holds:
- `episodes.jsonl` (`crypto.delphi-loop.episode.v1`)
- `registry.json` (`crypto.delphi-loop.registry.v1`)
- `pushed.json`
- `eval/`

Run, trace and feedback ids are uuid5 of the lane path, the call name and the
reply hash, so `harvest` and `push` are idempotent.

## Scope and limits

- `task_card.py` is the canonical copy from crypto-autoresearcher. This tool
  imports it and does not edit it, so the mirror rule and the shared-code lock
  are untouched.
- This is agent tooling, not cryptographic research or performance work, so
  AGENTS.md "Implementation language: no Python" does not apply. It sits beside
  `task_card.py` and `token_usage.py`.
- Verified: unit tests on synthetic lane records, run with and without the SDK;
  `route` read back through `task_card.route_for`; and a `push`→`pull`
  round-trip through the real `langsmith` 0.14.17 client against a local mock
  endpoint.
- Not yet verified: a real LangSmith workspace, and lanes with real Delphi
  calls. No lane records exist in this repository yet, so the registry starts at
  the prior.
- `pull` uses `Client.list_runs`, which the SDK says it will remove after
  2027-01-31 in favour of `client.runs.query()`.
- When Delphi gains its own `route` and `calibrate`, the episodes here are its
  calibration data. Point `DELPHI` back at the real binary.
