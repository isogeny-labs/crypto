#!/usr/bin/env python3
"""Tests for tools/validate_validation_json.py (stdlib unittest)."""
from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

TOOL = Path(__file__).with_name("validate_validation_json.py")
HEX = "a" * 64
COMMIT = "b" * 40

VALID = {
    "schema": "validation/v1",
    "claim_tier": "toy",
    "status": "validated",
    "certificate": {
        "kind": "replay",
        "digest": f"sha256:{HEX}",
        "verifier": "ecbench verify --replay 12",
    },
    "reference": {
        "kind": "matched-rho",
        "point_sha256": f"sha256:{HEX}",
        "band": ["0", "0.1", "1"],
    },
    "inputs": {
        "curve_uid": f"urn:ec-record:1:sha256:{HEX}",
        "code_commit": COMMIT,
        "seed": 7,
    },
}


class ValidateValidationJson(unittest.TestCase):
    def run_tool(self, record: dict, *extra: str) -> subprocess.CompletedProcess:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "VALIDATION.json"
            path.write_text(json.dumps(record))
            return subprocess.run(
                [sys.executable, str(TOOL), *extra, str(path)],
                capture_output=True,
                text=True,
            )

    def test_valid_record(self) -> None:
        result = self.run_tool(VALID)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_none_certificate_must_be_empty(self) -> None:
        record = copy.deepcopy(VALID)
        record["certificate"] = {
            "kind": "none",
            "digest": f"sha256:{HEX}",
            "verifier": "x",
        }
        result = self.run_tool(record)
        self.assertEqual(result.returncode, 1)
        self.assertIn("kind=none", result.stderr)

    def test_band_and_tier_are_enforced(self) -> None:
        record = copy.deepcopy(VALID)
        record["claim_tier"] = "huge"
        record["reference"]["band"] = ["0", "1", "10"]
        result = self.run_tool(record)
        self.assertEqual(result.returncode, 1)
        self.assertIn("claim_tier", result.stderr)
        self.assertIn("band", result.stderr)

    def test_legacy_schema_needs_the_flag(self) -> None:
        record = {"schema": "ecdlp.binary-genus2-validation/v1"}
        self.assertEqual(self.run_tool(record).returncode, 1)
        self.assertEqual(self.run_tool(record, "--legacy-ok").returncode, 0)


if __name__ == "__main__":
    unittest.main()
