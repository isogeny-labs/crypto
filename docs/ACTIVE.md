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
- [ ] Build `ecbench` and `ic_autolab` from `main`: the standalone `hashed-ecdl-tensor` crate fails to compile at `origin/main` (8 E0433 errors, `crate::metrics` undeclared in `lib.rs`); use the binary already built on Nov 14 for provenance, or report and fix the crate separately.
- [ ] Choose and freeze round 38 (`propose`, `freeze`): sizes and arms with a falsifiable prediction from the cost model, not a knob re-sweep.
- [ ] Resume checkpoint and benchmark record before any `run` (S3 gate: owner waiver needed, the repository AWS credential is invalid).
- [ ] Train, select, fresh holdout, assess, ratchet, report and PDF; PR with the result and `ratchet.svg`.

## Blocked / questions for the owner
