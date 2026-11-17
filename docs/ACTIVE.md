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

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)
- [x] s56: carry the planted log onto the plane quartic at p = 2^56 - 5 (64-bit Pic^0, wide Psi) (claude, started 2026-11-17, branch claude/explicit-quartic-jacobian-map-6i2dng, PR #4540)

## Blocked / questions for the owner

- main does not compile since after #4438 (`hashed-ecdl-tensor` uses `crate::metrics` with no `mod metrics`; `src/cryptanalysis/ecbench/workload.rs` unclosed delimiter); every Rust check on PR #4540 is red for that reason (claude, 2026-11-17).
