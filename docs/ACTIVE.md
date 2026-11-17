# Active goal and items

Owner-pinned work queue. `tools/goal_brief.py` prints this file at the start
of every Claude and Codex session (SessionStart hooks) and for `/goal`
(AGENTS.md, "Goal: what to work on next"). When this file has no
unchecked item, sessions fall back to the newest "Open threads" in
`docs/session-logs/`.

Keep it short: one goal, a few ordered items. Sessions check items off and add
follow-ups here; history belongs in the session log, not in this file.

## Current goal

Run the index-calculus autolab (`ic-autolab` skill, Curve25519-family loop `research/curve25519_autolab_20261104`): round 38 on the generic track, frozen before data, development evidence only.

## Active items (top = next)

- [x] Reconcile upstream with the local checkpoint: ledger at round 37, no autolab changes on `main` since; the generic incumbents sit at the K = 3 collision floor (`ideas/OBSTRUCTION.md`); sibling points already measured (`siblings/`, Stages 1 to 5).
- [x] Build `ecbench` and `ic_autolab` from `main`: the standalone `hashed-ecdl-tensor` crate fails to compile at `origin/main` (8 E0433 errors, `crate::metrics` undeclared in `lib.rs`); use the binary already built on Nov 14 for provenance, or report and fix the crate separately.
- [x] Choose and freeze round 38 (`propose`, `freeze`): sizes and arms with a falsifiable prediction from the cost model, not a knob re-sweep.
- [x] Resume checkpoint and benchmark record before any `run`: S3 gate waived by the owner for round 38 (no upload token in this session); no benchrec record filed.
- [x] Train, select, assess, ratchet, report and PDF for round 38: no challenger at k = 20 or 24 (no holdout); PR with the result and `ratchet.svg`.
- [ ] Next lever (code, not a re-sweep): the generic configuration space is exhausted at k = 20, 24, 28, 32; add a family (fewer unknowns, cheaper price per point, or a construction change per `ideas/OBSTRUCTION.md`) to `families.json` and its oracle in ecbench, then round 39.
- [ ] Fix or report the `hashed-ecdl-tensor` standalone build failure on `main` (`crate::metrics` undeclared in `lib.rs`).
- [ ] Derive exponent-moving Curve25519 mechanism contracts (Codex, started 2026-11-17, branch codex/curve25519-exponent-ideas-20261117; in progress)
- [x] Track every task first in docs/ACTIVE.md and a draft PR; roll /goal out to isogeny-labs repos (claude, started 2026-11-17, branch track-first-active, PR #4499)

## Blocked / questions for the owner
