#!/usr/bin/env python3
"""Tests for tools/transcript_humans.py.

    python3 tools/test_transcript_humans.py
"""
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import transcript_humans as th  # noqa: E402


def ev(uuid, ts, content, platform=True):
    ca = {"message": {"role": "user", "content": content}}
    if platform:
        ca["client_platform"] = "web"
    return {"created_at": ts, "user": {"uuid": uuid, "internal_anthropic_catchall": ca}}


class T(unittest.TestCase):
    def page(self, events, wrap=True):
        d = {"data": events, "has_more": False, "first_id": "a", "last_id": "b"}
        f = Path(tempfile.mkdtemp()) / "p.txt"
        f.write_text(("<other-session>\nheader\n" + json.dumps({"ccr": d}) + "\n</other-session>") if wrap else json.dumps(d))
        return f

    def test_filters(self):
        p = self.page([
            ev("1", "2026-11-01T00:00:02Z", "fix the CI"),
            ev("2", "2026-11-01T00:00:01Z", [{"type": "tool_result", "content": "x"}], platform=False),
            ev("3", "2026-11-01T00:00:03Z", "Stop hook feedback: [command hook]: commit", platform=False),
            ev("4", "2026-11-01T00:00:04Z", "<task-notification>done</task-notification>", platform=False),
            ev("5", "2026-11-01T00:00:00Z", [{"type": "text", "text": "start here"}]),
        ])
        rows, meta = th.extract([p, p])  # duplicate page de-duplicated
        self.assertEqual([r["kind"] for r in rows], ["human", "human", "hook"])
        self.assertEqual(rows[0]["uuid"], "5")
        self.assertEqual(meta["pages"], 2)

    def test_unwrapped_page(self):
        rows, _ = th.extract([self.page([ev("1", "t", "hi")], wrap=False)])
        self.assertEqual(len(rows), 1)


if __name__ == "__main__":
    unittest.main()
