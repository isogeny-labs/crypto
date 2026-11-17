#!/usr/bin/env bash
# Claude Code Stop hook: append this session's token usage to
# benchmarks/token-usage/ledger.jsonl (tools/token_usage.py session --ledger).
# Stop fires after every turn, so the ledger keeps one line per session: the
# newest, cumulative record replaces the earlier one for the same session_id.
# Never blocks: any failure, or no transcript, exits 0 and writes nothing to
# the ledger. Failures are logged to .claude/hooks/token-usage.log (gitignored).
set -uo pipefail

input="$(cat 2>/dev/null || true)"
root="${CLAUDE_PROJECT_DIR:-}"
[ -n "$root" ] || root="$(git -C "$(pwd)" rev-parse --show-toplevel 2>/dev/null)" || exit 0
tool="$root/tools/token_usage.py"
[ -f "$tool" ] || exit 0
command -v python3 >/dev/null 2>&1 || exit 0

log="$root/.claude/hooks/token-usage.log"
ledger="$root/benchmarks/token-usage/ledger.jsonl"
note() { { printf '%s %s\n' "$(date -u +%FT%TZ)" "$*" >>"$log"; } 2>/dev/null || true; }

transcript="$(printf '%s' "$input" | python3 -c '
import json, sys
try:
    print(json.load(sys.stdin).get("transcript_path") or "")
except Exception:
    print("")' 2>/dev/null || true)"
if [ -n "$transcript" ] && [ ! -e "$transcript" ]; then
  note "transcript_path $transcript does not exist; nothing recorded"
  exit 0
fi

tmp="$(mktemp 2>/dev/null)" || exit 0
trap 'rm -f "$tmp" "$tmp.json"' EXIT
args=(session --json "$tmp.json" --ledger "$tmp")
[ -n "$transcript" ] && args+=("$transcript")
if ! python3 "$tool" "${args[@]}" >/dev/null 2>"$tmp.err"; then
  note "token_usage.py failed: $(head -c 300 "$tmp.err" | tr '\n' ' ')"
  rm -f "$tmp.err"
  exit 0
fi
rm -f "$tmp.err"
[ -s "$tmp" ] || { note "no record produced; nothing recorded"; exit 0; }

mkdir -p "$(dirname "$ledger")" 2>/dev/null || { note "cannot create ledger dir"; exit 0; }
python3 - "$tmp" "$ledger" <<'PY' 2>>"$log" || note "ledger update failed"
import json, os, sys
new_path, ledger = sys.argv[1:3]
new = open(new_path).read().strip().splitlines()[-1]
sid = (json.loads(new).get("source") or {}).get("session_id")
keep = []
if os.path.exists(ledger):
    for ln in open(ledger).read().splitlines():
        try:
            if sid and (json.loads(ln).get("source") or {}).get("session_id") == sid:
                continue
        except Exception:
            pass
        if ln.strip():
            keep.append(ln)
keep.append(new)
tmp = ledger + ".tmp"
open(tmp, "w").write("\n".join(keep) + "\n")
os.replace(tmp, ledger)
PY
note "recorded ledger line (transcript ${transcript:-newest})"
exit 0
