---
description: Pick up the next piece of work from docs/ACTIVE.md and the newest session-log open threads, or pin a new goal
argument-hint: "[new goal to pin, or empty to resume]"
allowed-tools: Bash(python3 tools/goal_brief.py:*)
---
Goal brief:

!`python3 tools/goal_brief.py`

Arguments: "$ARGUMENTS"

1. If the arguments are non-empty, make them the "Current goal" in
   `docs/ACTIVE.md`, break the goal into 3–7 ordered, checkable items under
   "Active items", and record the change in today's session log.
2. Otherwise take the first unchecked item in `docs/ACTIVE.md`. If there is
   none, take the most recent open thread in the brief that no other live
   session owns (`conductor check --summary … --scope path:…`).
3. State the choice and the first concrete step in at most three lines, then
   start. Do not ask which item to take; ask only for a decision the brief
   marks as the owner's.
4. Follow every CLAUDE.md / AGENTS.md rule that applies to the work itself
   (literature and knowledge base first, conductor, checkpoints, records).
5. As items finish, check them off in `docs/ACTIVE.md`, add follow-ups, and
   append the session-log entry. Blocked items move to "Blocked / questions".
