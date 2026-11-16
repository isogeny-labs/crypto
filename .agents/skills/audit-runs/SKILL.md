---
name: audit-runs
description: Re-audit finished runbook and ecbench sessions with a nonce-sampled replay and digest checks, and the VALIDATION.json directories of the validation gate. Use for the daily audit, after a suspicious result, or when a row's evidence is in doubt. Reports; never edits sessions or results.
---

# audit-runs

Read `.claude/skills/audit-runs/SKILL.md` from this checkout and follow it.

## Continuous loop: learn, adapt, act on your own

AGENTS.md "Continuous learning loop"; ledger `tools/lessons.py`.

1. **Before acting**: `python3 tools/lessons.py brief --scope audit-runs`. Apply the
   adopted lessons; treat candidates as hypotheses this run can test.
2. **While acting**: resolve a blocker yourself within your authority
   (diagnose the cause, try the documented remedy, then a materially different
   approach) before escalating; AGENTS.md "Persistent scientific research loop".
   Persistence grants no extra execution or spending authority.
3. **After acting**: record the outcome with evidence,
   `python3 tools/lessons.py outcome --scope audit-runs --task "..." --result success|partial|fail|blocked --evidence <receipt|commit|job> [--applied L-...]`,
   and `lessons.py test --lesson L-... --outcome confirm|refute --evidence ...`
   for each lesson you applied. A surprise that would change what the next run
   does becomes `lessons.py add --scope audit-runs --trigger "..." --action "..." --evidence ...`.
4. **Adapt**: when a lesson scoped to `audit-runs` is adopted (two distinct
   confirmations), propose the edit to this file in a PR that cites the lesson
   id; when one is refuted twice more than confirmed, it retires itself.
