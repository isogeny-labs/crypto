# Agent autonomy

This directory measures whether agent sessions finish tasks without the owner's
supervision. Schema `crypto.autonomy/v1`. Tool: `tools/autonomy_report.py`.
Tests: `tools/test_autonomy_report.py`. All figures are `[pcc:unverified]` until
a pcc claim cites them.

## What is recorded

An **episode** is one task given to an agent session, from the first prompt to
its PR's merge or close (or abandonment). Each human message after the first
prompt is an **intervention**, labelled with one class:

| class | meaning | effect |
|:--|:--|:--|
| `redirect` | the agent was wrong, stuck or off-scope, or the human had to rescue it | counts against autonomy |
| `unblock` | access, credentials, budget, or an owner decision the agent lacked authority for | reported apart |
| `approve`, `new_task`, `question` | neutral messages | neutral |
| `hook` | automated hook feedback, such as a stop hook asking for a commit | reported apart, neutral |

An episode is an **unsupervised success** when all of these hold: result is
`success`, zero `redirect` interventions, the PR is merged, CI was green at
merge, no revert within 14 days, and `validator_blocking` is not true. Each
episode is in one of three states: `unsupervised_success`, `fail`, or
`pending` (PR open, revert window not elapsed, or a check not yet recorded).
The rate leaves pending episodes out and reports them as a separate count.

**Self-recovery** is the number of incidents (CI failures, merge conflicts,
blocked commands) the agent fixed without a human message, over all incidents.

## Files

- `episodes/E-<sha12>.json` is an episode, `I-<sha12>.json` an intervention,
  `C-<sha12>.json` a later check (`reverted`, `validator_blocking`). The latest
  check by `created` wins.
- An id is the SHA-256 of the entry's canonical content. Entries are append-only:
  a correction is a new entry. `verify` checks ids, references, required fields
  and, with `--base REF`, that nothing from REF changed or vanished.
- Only the SHA-256 of a message is stored, never its text.
- `report/AUTONOMY.md` and `report/autonomy.svg` come from `report`.

## How episodes are recorded

1. The session that owns the PR records its episode with `add-episode` when the
   PR merges or closes, with evidence (PR link, CI job link).
2. A Haiku classifier labels the interventions. `classify-prompt` prints its
   instructions. Labels go in with `add-intervention --classifier haiku`.
3. The owner audits a 10% sample: `audit-sample --rate 0.1 --seed N` lists the
   ids. A human label for the same message hash overrides the machine label,
   and the report states how often the classifier agreed with the human on the
   audited sample.
4. After 14 days, anyone records `set-check --check reverted` from `git log`.

## Sabotage caveat

Episodes are self-reported by the session being measured, so the numbers are
claims, not facts. Three things can be checked outside the session: whether the
PR merged, whether a revert landed (both visible in `git log` and on GitHub),
and whether a stored message hash matches the transcript. The audit sample
covers label quality only. A reader who needs more than this reruns the checks
above against the evidence links in each entry.
