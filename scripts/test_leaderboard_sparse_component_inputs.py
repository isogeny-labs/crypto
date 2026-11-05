#!/usr/bin/env python3
"""Regression: sparse component evidence must not silently demote a valid result."""
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import build_ic_leaderboard as builder


class SparseComponentInputs(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name).resolve()
        self.directory = builder.COMPONENT_EVIDENCE_DIRS[0]
        self.evidence = self.root / self.directory
        self.evidence.mkdir(parents=True)
        self.repo = patch.object(builder, "REPO", self.root)
        self.repo.start()
        self.addCleanup(self.repo.stop)

    def manifest(self):
        (self.evidence / "VALIDATION.json").write_text(json.dumps({
            "validator_report": {"path": "validator.json"},
            "replay_certificate": {"path": "replay.json"},
        }))
        (self.evidence / "validator.json").write_text(json.dumps({
            "raw_data": [{"path": "raw.json"}],
        }))
        (self.evidence / "replay.json").write_text("{}")

    def check(self, sparse):
        index = b"".join(b"S " + p.encode() + b"\0" for p in sparse)
        with patch.object(builder.subprocess, "check_output", return_value=index):
            builder.require_materialized_gate_inputs([])

    def test_component_manifest_is_required_even_without_a_table_row(self):
        path = self.directory + "/VALIDATION.json"
        with self.assertRaisesRegex(SystemExit, path):
            self.check([path])

    def test_component_certificate_cannot_be_silently_omitted(self):
        self.manifest()
        (self.evidence / "replay.json").unlink()
        path = self.directory + "/replay.json"
        with self.assertRaisesRegex(SystemExit, path):
            self.check([path])

    def test_component_raw_data_cannot_be_silently_omitted(self):
        self.manifest()
        path = self.directory + "/raw.json"
        with self.assertRaisesRegex(SystemExit, path):
            self.check([path])

    def test_restored_bytes_are_accepted_even_before_index_flag_refresh(self):
        self.manifest()
        (self.evidence / "raw.json").write_text("{}")
        self.check([self.directory + "/" + name for name in
                    ("VALIDATION.json", "validator.json", "replay.json", "raw.json")])


if __name__ == "__main__":
    unittest.main()
