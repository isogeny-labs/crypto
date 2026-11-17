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

- [ ] Prime31 RR9 exceptional x=r residual and exact signed fibre checks (ChatGPT automation, started 2026-11-17, branch research/prime31-rr9-exceptional-residual-20261117)

- [x] Prime31 RR9 B8 signed-fibre linear-factorization falsifier and Rust exact controls (ChatGPT automation, started 2026-11-17, branch research/prime31-rr9-b8-signed-fibres-20261117, PR #4545)
- [x] Prime31 RR9 exact squarefree/radical support reduction and Rust prime31 controls (ChatGPT automation, started 2026-11-17, branch research/prime31-rr9-radical-support-20261117, PR #4541)
- [x] Prime31 nonenumerative RR9/S9 extraction feasibility and Rust-checkable cost gates (ChatGPT automation, started 2026-11-17, branch research/prime31-rr9-algebraic-feasibility-20261117, PR #4539)
- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)

## Blocked / questions for the owner
