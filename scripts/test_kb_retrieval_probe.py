#!/usr/bin/env python3
"""Tests for kb_retrieval_probe.py; stdlib only."""
from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import os
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent


def load_module():
    spec = importlib.util.spec_from_file_location("kb_retrieval_probe", HERE / "kb_retrieval_probe.py")
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


probe = load_module()

RESULTS = [
    {"origins": [{"repo": "other", "path": "docs/x.md"}]},
    {"origins": [{"repo": "crypto-autoresearcher", "path": "knowledge/a.md"}]},
    {"origins": [{"repo": "crypto-autoresearcher", "path": "knowledge/a.md"}]},
]

FAKE_RUNNER = """#!/usr/bin/env python3
import json, sys
query = sys.argv[2]
rank = 1 if "prime" in query else 4
results = [
    {"origins": [{"repo": "other", "path": "docs/no.md"}]},
] * (rank - 1) + [
    {"origins": [{"repo": "crypto-autoresearcher", "path": "knowledge/a.md"}]},
]
print(json.dumps({"results": results}))
"""


class ProbeTest(unittest.TestCase):
    def test_rank_of_matches_first_origin(self):
        self.assertEqual(probe.rank_of(RESULTS, "crypto-autoresearcher", "knowledge/a.md"), 2)
        self.assertIsNone(probe.rank_of(RESULTS, "crypto-autoresearcher", "knowledge/missing.md"))
        self.assertEqual(probe.rank_of([], "x", "y"), None)

    def test_summarize_counts(self):
        stats = probe.summarize([1, 4, None], top_k=5)
        self.assertEqual(stats["queries"], 3)
        self.assertEqual(stats["top1"], 1)
        self.assertEqual(stats["hits"], 2)
        self.assertAlmostEqual(stats["hit_rate"], 2 / 3)
        self.assertAlmostEqual(stats["mrr"], (1 + 0.25) / 3)

    def test_end_to_end_with_fake_runner(self):
        with tempfile.TemporaryDirectory() as tmp:
            tmpdir = Path(tmp)
            runner = tmpdir / "runner"
            runner.write_text(FAKE_RUNNER)
            os.chmod(runner, 0o755)
            queries = tmpdir / "queries.json"
            queries.write_text(
                json.dumps(
                    {
                        "schema": "kb-retrieval/v1",
                        "top_k": 5,
                        "queries": [
                            {"query": "prime field", "repo": "crypto-autoresearcher", "path": "knowledge/a.md"},
                            {"query": "something else", "repo": "crypto-autoresearcher", "path": "knowledge/a.md"},
                        ],
                    }
                )
            )
            out = tmpdir / "out.json"
            with contextlib.redirect_stdout(io.StringIO()):
                code = probe.main(
                    ["--queries", str(queries), "--runner", str(runner), "--json", "--out", str(out)]
                )
            self.assertEqual(code, 0)
            data = json.loads(out.read_text())
            self.assertEqual(data["schema"], "kb-retrieval-result/v1")
            self.assertEqual([row["rank"] for row in data["rows"]], [1, 4])
            self.assertEqual(data["stats"]["top1"], 1)
            self.assertEqual(data["stats"]["hits"], 2)

            with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                code = probe.main(
                    ["--queries", str(queries), "--runner", str(runner), "--top-k", "3", "--fail-under-top5", "0.5"]
                )
                self.assertEqual(code, 0)
                code = probe.main(
                    ["--queries", str(queries), "--runner", str(runner), "--top-k", "3", "--fail-under-top5", "0.6"]
                )
                self.assertEqual(code, 1)


if __name__ == "__main__":
    unittest.main()
