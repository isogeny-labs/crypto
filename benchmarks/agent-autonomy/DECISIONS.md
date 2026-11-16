# Autonomy measurement: decisions

Owner delegation, 2026-11-15: "Make decisions". Each decision below was taken by
the main session (Claude Opus) under that delegation, with its reason. The owner
can reverse any of them; a reversal is a dated entry here, and the records are
re-read under the new rule (the ledger is append-only, so nothing is rewritten).
Everything here is `[pcc:unverified]`.

## D1. "Continue" nudges count against autonomy, in their own class

New intervention class `nudge`: the agent stopped while work remained and the
human restarted it ("continue", "resume", "keep going, do X next"). An episode
with a nudge is not an unsupervised success. Reason: AGENTS.md "Persistent
scientific research loop" says finishing a stage is not a reason to stop, so a
stop that needed a human to restart the work is a supervision event. It is
reported apart from `redirect` so that stops caused by infrastructure (a
container restart, a context limit) stay visible and can be argued case by case.

## D2. Second-pass labels by the main session; the owner audit slot stays open

Classifier precedence is human > opus > sonnet > haiku > rule; within a rank the
latest label wins. The main session relabelled all 15 audited messages as `opus`
(it never writes `human`). Result of the second pass:

| first-pass label | count | second-pass label |
| --- | --- | --- |
| redirect | 13 | redirect 2, nudge 5, approve 3, unblock 2, new_task 1 |
| question / new_task | 2 | unchanged |

- The two fragments that authorised a merge ("Just merge lean or", "Pr") are
  `unblock`: merging is an owner decision.
- Replies that pick among options the agent offered ("1+2", "1-3 …", "yes
  Possible next step …") are `approve`.
- "keep going, do X next" and "Continue" are `nudge` (D1).
- "Use Rust for this" (the agent planned Python against AGENTS.md) and
  "You gotta figure this part out … devise novel ideas" (the agent was stuck)
  stay `redirect`.

The owner's own labels, if given, override these (`--classifier human`).

## D3. A session with several PRs is one episode

The unit is a task, not a PR. A session that lands one task in several PRs is
one episode: `--pr` is the last PR merged, every PR is listed as evidence, and
`--ended` is the last merge. The pre-loop session with PRs #3845, #3849 and
#3851 stays out of the baseline. Reading its transcript costs more than the
one data point is worth now that episodes are recorded as they happen.

## D4. The task set is frozen in two stages

`tasks/SELECTION.json`:

- **pilot**: 14 tasks whose check was run here and behaved as specified;
- **needs host validation**: 33 tasks whose check needs Lean, a cargo build or
  a harness fixture, or only exited non-zero without output;
- **dropped**: T-004 (solvability unconfirmed).

The A/B protocol (PROTOCOL.md) is frozen only after a pilot run on one host has
validated the harness and measured cost per task. The pilot is a run on
`isolab-eks` and spends money, so it starts on the owner's explicit go-ahead
(CLAUDE.md: loading instructions alone does not authorize an experiment).

## D5. The open benchmark record on `main` is left to its owner

`benchmarks/records/20261115T0359Z-b0-pdp-sat-baseline.json` (from `dec928b649`)
fails the non-required `records` check on every PR. Closing it would end another
session's measurement, so it is noted on the PRs and left alone.
