#!/usr/bin/env python3
"""Count research-kb evidence in merged pull-request bodies.

The session-log audit cannot see a receipt written in a PR body, and it
cannot see a session that said the catalog was unavailable there. This tool
reads a JSON export from GitHub and reports that second evidence channel
separately, so an availability gap is not misread as a usage gap.

Produce the input with `gh` (no network is used here):

    gh pr list --state merged --limit 300 \
      --json number,title,body,mergedAt > /tmp/prs.json
    python3 scripts/kb_pr_evidence.py --input /tmp/prs.json --since 2026-11-05

A body counts as a receipt when it contains "Sources checked" or a
`source:<hex>` id. An unavailable declaration is recorded separately and is
not added to the log coverage.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from datetime import datetime, timezone
from pathlib import Path

SCHEMA = "kb-pr-evidence/v1"
RECEIPT_RE = re.compile(r"Sources checked", re.IGNORECASE)
SOURCE_RE = re.compile(r"\bsource:[0-9a-fA-F]{8,}\b")
MENTION_RE = re.compile(r"research-kb|knowledge base|search_knowledge|scripts/research-kb", re.IGNORECASE)
UNAVAILABLE_RE = re.compile(
    r"KB unavailable"
    r"|research-kb[^.\n]{0,80}(?:not|isn't|is not|was not|absent|missing|unavailable)"
    r"|no research-kb"
    r"|research-kb[^.\n]{0,40}isn't available"
    r"|research-kb[^.\n]{0,40}not installed",
    re.IGNORECASE,
)


def rows(payload: list[dict], since: str | None) -> list[dict]:
    out = []
    for pr in payload:
        merged = pr.get("mergedAt") or ""
        day = merged[:10] if merged else ""
        if since and day and day < since:
            continue
        body = pr.get("body") or ""
        out.append(
            {
                "number": pr.get("number"),
                "merged": day,
                "title": (pr.get("title") or "")[:80],
                "receipt": bool(RECEIPT_RE.search(body)),
                "source_id": bool(SOURCE_RE.search(body)),
                "mention": bool(MENTION_RE.search(body)),
                "unavailable": bool(UNAVAILABLE_RE.search(body)),
            }
        )
    return out


def summarize(items: list[dict]) -> dict:
    total = len(items)
    return {
        "merged": total,
        "receipts": sum(1 for r in items if r["receipt"]),
        "source_ids": sum(1 for r in items if r["source_id"]),
        "mentions": sum(1 for r in items if r["mention"]),
        # Availability-only sessions: declared unavailable and no receipt.
        "unavailable": sum(1 for r in items if r["unavailable"] and not r["receipt"]),
        "used_without_record": sum(
            1 for r in items
            if r["mention"] and not r["receipt"] and not r["unavailable"]
        ),
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument("--input", type=Path, required=True, help="gh pr list JSON export")
    parser.add_argument("--since", default=None, help="merge date on/after YYYY-MM-DD")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args(argv)

    try:
        payload = json.loads(args.input.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        print(f"kb_pr_evidence: {exc}", file=sys.stderr)
        return 2
    if not isinstance(payload, list):
        print("kb_pr_evidence: expected a JSON array from gh pr list", file=sys.stderr)
        return 2

    items = rows(payload, args.since)
    stats = summarize(items)
    result = {
        "schema": SCHEMA,
        "generated_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "since": args.since,
        "input": str(args.input),
        "stats": stats,
        "rows": items,
    }
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    if args.json:
        print(json.dumps(result, indent=2))
    else:
        print(
            f"merged {stats['merged']}, receipts {stats['receipts']}, "
            f"source ids {stats['source_ids']}, mentions {stats['mentions']}, "
            f"unavailable {stats['unavailable']}, used-without-record {stats['used_without_record']}"
        )
        for row in items:
            flags = (
                ("R" if row["receipt"] else "-")
                + ("S" if row["source_id"] else "-")
                + ("U" if row["unavailable"] else "-")
            )
            if row["mention"]:
                print(f"  {row['merged']} #{row['number']} {flags} {row['title']}")
        print("R = Sources checked, S = source id, U = unavailable declared; separate from log coverage.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
