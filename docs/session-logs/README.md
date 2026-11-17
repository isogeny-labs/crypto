# Shared session logs

Claude and Codex read the newest `YYYY-MM-DD.md` here at the start of a
session. They append to the current day's file after each finished piece
of work, decision, or task switch. Each timestamped entry has four headings:

- What was done (commands, files, features)
- Decisions made (and why)
- Open threads (unfinished, pick up next time)
- Errors (what went wrong, how it was handled)

Use [TEMPLATE.md](TEMPLATE.md) when starting an entry. Record actual commands,
file paths and verified outcomes. Identify plans, assumptions and unavailable
results as such. Preserve older entries and other sessions' work when appending.
Commit project handoffs in the related PR, so the history travels with the
repository. These logs supplement the existing evidence and PR requirements.

The checked-in `AGENTS.md` and `CLAUDE.md` already enable this workflow for
this repository. New dates create new daily files; README and TEMPLATE are
not daily logs. Keep project logs about project work. Global logs can contain
unrelated context: do not copy them or chat transcripts into this public history.

## Install globally or in another workspace

From this repository root:

```sh
# Global Claude and Codex guidance, sharing ~/.claude/session-logs/:
bash scripts/setup-session-logs.sh

# Another home directory (also useful for checking installation):
bash scripts/setup-session-logs.sh --home /absolute/home

# Portable repository/workspace guidance and docs/session-logs/:
bash scripts/setup-session-logs.sh --workspace /absolute/workspace
```

The Bash installer prepends a marked section to existing instruction files,
so the logging rule is visible before lengthy guidance. It preserves their
prior content. A second run skips an existing complete
marked section and preserves logs. An incomplete marked section causes an
error for inspection. It creates today's log only if missing and does not
rewrite old entries. Existing marked sections are not automatically upgraded;
review instruction changes in Git and edit that section deliberately.

Global mode writes `~/.claude/CLAUDE.md` and `~/.codex/AGENTS.md`. Workspace
mode writes `CLAUDE.md` and `AGENTS.md` and uses paths relative to the workspace
root, so checked-in guidance works in a different checkout. In a repository,
its more specific instructions take precedence over global guidance.

A global install requires a writable home directory. The managed environment
used for the initial setup mounted `/home/agent` read-only, so only workspace
installation was possible there. The script does not alter credentials,
change environment variables, upload logs, commit files, or push branches.
