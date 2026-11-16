#!/usr/bin/env python3
"""Positive and adversarial controls for the actual prospective report."""
import json
from pathlib import Path
import tempfile
import unittest

from check_numeric_fidelity import check

HERE = Path(__file__).resolve().parent
ORIGINAL = HERE.parent / "validator-report.json"
SUCCESSOR = HERE / "validator-report.json"


class NumericFidelity(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name) / "successor.json"
        self.text = SUCCESSOR.read_text()

    def rejects(self, text, message="inherited scientific JSON values changed"):
        self.path.write_text(text)
        with self.assertRaisesRegex(ValueError, message):
            check(ORIGINAL, self.path)

    def test_exact_report_passes(self):
        check(ORIGINAL, SUCCESSOR)

    def test_observed_default_parser_rounding_is_rejected(self):
        value = "1.0930878076239279"
        self.assertIn(value, self.text)
        self.rejects(self.text.replace(value, "1.093087807623928", 1))

    def test_decimal_drift_below_binary_rounding_is_rejected(self):
        value = "1.0930878076239279"
        self.assertIn(value, self.text)
        self.rejects(self.text.replace(value, "1.09308780762392790000001", 1))

    def test_boolean_cannot_be_replaced_with_one(self):
        value = '"independent": true'
        self.assertEqual(self.text.count(value), 1)
        self.rejects(self.text.replace(value, '"independent": 1', 1))

    def test_registry_pin_and_receipt_must_agree(self):
        value = json.loads(self.text)["current_registry_source_equivalence"]["current_registry_sha256"]
        self.rejects(self.text.replace(value, "0" * 64, 1), "receipt current registry pin differs")

    def test_other_raw_digest_changes_are_rejected(self):
        value = next(r["sha256"] for r in json.loads(self.text)["raw_data"]
                     if r["path"] != "../../docs/curves/registry.json")
        self.rejects(self.text.replace(value, "0" * 64, 1))

    def test_original_report_bytes_are_pinned(self):
        original = Path(self.temp.name) / "original.json"
        original.write_bytes(ORIGINAL.read_bytes() + b" ")
        with self.assertRaisesRegex(ValueError, "original report pin differs"):
            check(original, SUCCESSOR)


if __name__ == "__main__":
    unittest.main()
