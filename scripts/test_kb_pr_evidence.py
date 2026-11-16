#!/usr/bin/env python3
"""Tests for kb_pr_evidence.py; stdlib only."""
from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load_module():
    spec = importlib.util.spec_from_file_location("kb_pr_evidence", HERE / "kb_pr_evidence.py")
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


pr = load_module()

PAYLOAD = [
    {"number": 1, "title": "receipt", "mergedAt": "2026-11-05T10:00:00Z",
     "body": "Sources checked (KB): source:aaaaaaaaaaaaaaaa"},
    {"number": 2, "title": "unavailable", "mergedAt": "2026-11-06T10:00:00Z",
     "body": "The research-kb MCP server isn't available in this container."},
    {"number": 3, "title": "mention only", "mergedAt": "2026-11-06T11:00:00Z",
     "body": "Not done: research-kb queries."},
    {"number": 4, "title": "nothing", "mergedAt": "2026-11-06T12:00:00Z", "body": "plain"},
    {"number": 5, "title": "old", "mergedAt": "2026-11-04T12:00:00Z",
     "body": "Sources checked (KB): source:bbbbbbbbbbbbbbbb"},
]


class PrEvidenceTest(unittest.TestCase):
    def test_counts_and_since(self):
        items = pr.rows(PAYLOAD, "2026-11-05")
        stats = pr.summarize(items)
        self.assertEqual(stats["merged"], 4)
        self.assertEqual(stats["receipts"], 1)
        self.assertEqual(stats["source_ids"], 1)
        self.assertEqual(stats["mentions"], 2)
        self.assertEqual(stats["unavailable"], 1)
        self.assertEqual(stats["used_without_record"], 1)

    def test_cli_writes_json(self):
        with tempfile.TemporaryDirectory() as tmp:
            src = Path(tmp) / "prs.json"
            src.write_text(json.dumps(PAYLOAD))
            out = Path(tmp) / "out.json"
            with contextlib.redirect_stdout(io.StringIO()):
                code = pr.main(["--input", str(src), "--since", "2026-11-05", "--json", "--out", str(out)])
            self.assertEqual(code, 0)
            data = json.loads(out.read_text())
            self.assertEqual(data["schema"], "kb-pr-evidence/v1")
            self.assertEqual(data["stats"]["receipts"], 1)

    def test_bad_input(self):
        with tempfile.TemporaryDirectory() as tmp:
            src = Path(tmp) / "prs.json"
            src.write_text("{}")
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(pr.main(["--input", str(src)]), 2)
            with contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(pr.main(["--input", str(Path(tmp) / "missing.json")]), 2)


if __name__ == "__main__":
    unittest.main()
