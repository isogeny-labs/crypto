# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

Achieve a measured 2–6× speedup in the Rust Vieta evaluator over Euclid/Newton on matched complete construction-plus-query workloads, starting with word62/m16384. Preserve exact complete relation sets, reference methods and prior frozen evidence; use a corrected, preflight-validated frozen protocol.

## Active items (top = next)

- [ ] Chase measured 2–6× Vieta speedup over Euclid/Newton (codex, started 2026-11-17, branch lane/vieta-speedup-20261117, PR #4535; in progress).
- [x] Profile unchanged baseline and record limited attribution; test the modular-reduction hypothesis (same task/branch).
- [x] Implement native-field NTT/half-GCD and exact small roots; check relations against schoolbook/lookup (same task/branch).
- [ ] Validate canonical fixtures (done) and publish/freeze the fresh primary comparison before data (same task/branch).
- [ ] Measure complete/query costs at larger sizes with A/A and positive controls; replay all successful calls (same task/branch).
- [ ] Report gains/regressions and limits, then follow required publication checks (same task/branch).

- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)

<!-- One line per task, added as the task's first step with its draft PR:
- [ ] <task> (<runtime>, started YYYY-MM-DD, branch <branch>, PR #<n>) -->
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)

## Blocked / questions for the owner
