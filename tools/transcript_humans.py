#!/usr/bin/env python3
"""Extract human-typed messages from saved Claude Code Remote `list_events` pages.

The autonomy baseline (benchmarks/agent-autonomy/README.md) needs only the
messages a person typed into a session. `list_events` pages are mostly tool
results, so a long session spans many pages; the runtime saves each large page
to a file. This tool reads those files and keeps:

  human  a user event whose `internal_anthropic_catchall` carries
         `client_platform` (set by a client when a person sends a message)
         and whose content is not a tool result
  hook   a user event whose text starts with "Stop hook feedback" or
         "[command hook]" (automated hook feedback; class `hook`)

Everything else (tool results, task notifications, agent hand-backs, local
command output, system reminders) is dropped. Events are de-duplicated by uuid
and sorted by time.

    transcript_humans.py PAGE_FILE [PAGE_FILE ...]             # JSONL: ts, uuid, kind, sha256, chars
    transcript_humans.py --with-text PAGE_FILE ...              # adds "text" (scratch use only)

Never commit `--with-text` output: the repository stores only hashes.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

HOOK_PREFIXES = ("Stop hook feedback", "[command hook]")


def load_page(path: Path) -> dict:
    text = path.read_text(errors="replace")
    start = text.find('{"ccr"')
    if start >= 0:
        end = text.rindex("}") + 1
        return json.loads(text[start:end])["ccr"]
    data = json.loads(text)
    return data.get("ccr", data)


def message_text(content) -> tuple[str, bool]:
    """Return (text, is_tool_result)."""
    if isinstance(content, str):
        return content, False
    if isinstance(content, list):
        if any(isinstance(b, dict) and b.get("type") == "tool_result" for b in content):
            return "", True
        return "".join(b.get("text", "") for b in content if isinstance(b, dict)), False
    return "", False


def classify(event: dict) -> dict | None:
    user = event.get("user")
    if not isinstance(user, dict):
        return None
    catchall = user.get("internal_anthropic_catchall") or {}
    message = catchall.get("message") or user.get("message") or {}
    text, tool_result = message_text(message.get("content"))
    if tool_result or not text.strip():
        return None
    if "client_platform" in catchall:
        kind = "human"
    elif text.lstrip().startswith(HOOK_PREFIXES):
        kind = "hook"
    else:
        return None
    return {"ts": event.get("created_at", ""), "uuid": user.get("uuid", ""), "kind": kind,
            "sha256": hashlib.sha256(text.encode()).hexdigest(), "chars": len(text), "text": text}


def extract(paths: list[Path]) -> tuple[list[dict], dict]:
    seen: dict[str, dict] = {}
    meta = {"pages": 0, "events": 0, "has_more_last": None}
    for p in paths:
        page = load_page(p)
        meta["pages"] += 1
        meta["events"] += len(page.get("data", []))
        meta["has_more_last"] = page.get("has_more")
        for ev in page.get("data", []):
            row = classify(ev)
            if row and row["uuid"] not in seen:
                seen[row["uuid"] or f"{row['ts']}:{row['sha256']}"] = row
    rows = sorted(seen.values(), key=lambda r: r["ts"])
    return rows, meta


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("pages", nargs="+", type=Path)
    ap.add_argument("--with-text", action="store_true", help="include message text (never commit this output)")
    a = ap.parse_args(argv)
    rows, meta = extract(a.pages)
    for r in rows:
        if not a.with_text:
            r = {k: v for k, v in r.items() if k != "text"}
        print(json.dumps(r, ensure_ascii=False))
    print(json.dumps({"_meta": meta, "human": sum(r["kind"] == "human" for r in rows),
                      "hook": sum(r["kind"] == "hook" for r in rows)}), file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
