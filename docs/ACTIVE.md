# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

_(none pinned — set one with `/goal <text>`)_

## Active items (top = next)

- [ ] Pursue 15x relation-pipeline speedup with qualified Rust integration and matched measurements (Codex, started 2026-11-17, branch lane/relation-fusion-next, PR #4530; in progress)

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)

## Blocked / questions for the owner

- Relation15x / PR #4530: actual-root36-case adapter and saved-proof audits pass. Profile169/179 completes36 calls with full proof replays, no runtime admission. Current source120 tests180 and3 strict actual-root initial tests181 pass; full release unit165 remains incomplete after46 observed failures. Next is complete packed-constructor hypothesis182. Historical benchmark/PCC metadata gates unchanged; no15x admitted, goal in progress. [pcc:unverified]
