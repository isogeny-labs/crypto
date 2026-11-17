#!/usr/bin/env bash
# Install shared session-log guidance before existing instructions.
set -euo pipefail
session_temp=''
trap 'if [ -n "$session_temp" ]; then rm -f -- "$session_temp"; fi' EXIT

session_mode=global
session_root=$HOME
if [ "$#" -gt 0 ]; then
  case "$1" in
    --workspace) session_mode=workspace ;;
    --home) session_mode=global ;;
    *) printf 'Usage: %s [--workspace DIRECTORY | --home DIRECTORY]\n' "$0" >&2; exit 2 ;;
  esac
  [ "$#" -eq 2 ] || { printf 'Expected one directory.\n' >&2; exit 2; }
  session_root=$2
fi
case "$session_root" in
  /*) ;;
  *) printf 'Use an absolute directory path.\n' >&2; exit 2 ;;
esac

if [ "$session_mode" = global ]; then
  session_log_dir=$session_root/.claude/session-logs
  session_display_dir=$session_log_dir
  mkdir -p "$session_log_dir"
  mkdir -p "$session_root/.codex"
  session_settings=("$session_root/.claude/CLAUDE.md" "$session_root/.codex/AGENTS.md")
else
  session_log_dir=$session_root/docs/session-logs
  session_display_dir=docs/session-logs
  mkdir -p "$session_log_dir"
  session_settings=("$session_root/CLAUDE.md" "$session_root/AGENTS.md")
fi

session_begin='<!-- BEGIN SHARED SESSION LOG -->'
session_end='<!-- END SHARED SESSION LOG -->'
for session_file in "${session_settings[@]}"; do
  if [ -f "$session_file" ] && grep -Fq -- "$session_begin" "$session_file"; then
    grep -Fq -- "$session_end" "$session_file" || {
      printf 'Incomplete managed section; inspect %s before continuing.\n' "$session_file" >&2
      exit 1
    }
    printf 'Already configured: %s\n' "$session_file"
    continue
  fi
  session_temp=$(mktemp "${session_file}.session-log.XXXXXX")
  {
    printf '%s\n\n' "$session_begin"
    printf '## Session log\n\n'
    printf 'At the start of every session, read the newest dated Markdown file in\n`%s/`.\n\n' "$session_display_dir"
    printf "Write today's log at \140%s/YYYY-MM-DD.md\140, using the current date.\n" "$session_display_dir"
    if [ "$session_mode" = workspace ]; then
      printf 'These paths are relative to the repository or workspace root.\n'
    fi
    cat <<'GUIDANCE'
Create the folder and file if missing. Claude and Codex share this directory.

Do not wait for the user to end the session. Append a timestamped entry after
every finished piece of work, after every decision, and whenever we switch
tasks. Preserve previous entries. Record concrete facts from commands and
logs; distinguish verified results from plans or assumptions.

Every entry has four headings:

- What was done (commands, files, features)
- Decisions made (and why)
- Open threads (unfinished, pick up next time)
- Errors (what went wrong, how it was handled)
GUIDANCE
    if [ "$session_mode" = workspace ]; then
      printf '\nCommit project handoffs with the related PR so they survive across sessions.\n'
      printf 'Keep entries about this project; do not copy private global logs or chat transcripts.\n'
    fi
    printf '\n%s\n' "$session_end"
  } > "$session_temp"
  if [ -f "$session_file" ]; then
    printf '\n' >> "$session_temp"
    cat "$session_file" >> "$session_temp"
  fi
  cat "$session_temp" > "$session_file"
  rm -f -- "$session_temp"
  session_temp=''
  printf 'Configured: %s\n' "$session_file"
done

session_today=$session_log_dir/$(date +%F).md
if [ ! -e "$session_today" ]; then
  printf '# Session log — %s\n' "$(date +%F)" > "$session_today"
fi
printf 'Daily log: %s\n' "$session_today"
