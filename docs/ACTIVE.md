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

- [ ] Account complete prime-field costs for guarded cubic inverse16 (Codex, started 2026-11-17, branch work/chained-primefield-cost16-source, PR #4544; in progress)

- [x] Prove and test cubic-subfield inverse15 for closed Richelot fibres (Codex, started 2026-11-17, branch work/chained-cubic-inverse15-source, PR #4544; in progress)
- [x] Complete native12 provider ZIP byte custody (Codex, started 2026-11-17, branch work/chained-cubic-inverse15-source, PR #4544; in progress)

- [x] Publish frozen joint-chart Richelot divisor map14 and full evidence (Codex, started 2026-11-17, branch lane/chained-richelot-map14, PR #4544; in progress)

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)

## Blocked / questions for the owner
