"""Scratch-only schema/serialization controls; never invokes dataset or freeze."""
import copy
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest

import prepare
from common import canonical, decode, file_bindings, load, sha, write

TOOLING = Path(os.environ.get("PROTOCOL_TOOLING", "/home/ec2-user/ml-dataset-work-20261028/protocol-tooling-f755085ff951"))
spec = importlib.util.spec_from_file_location("official_protocol", TOOLING / "tools/experiment_protocol.py")
checker = importlib.util.module_from_spec(spec)
spec.loader.exec_module(checker)
SOURCE = "8bddd785ff0d44f50b6710649ed403401075f5cb"


class DraftControls(unittest.TestCase):
    def draft(self):
        return prepare.prepare(SOURCE)

    def test_authoritative_schema_and_semantic_lint(self):
        p = self.draft()
        self.assertEqual(checker.schema_errors(p, "experiment.protocol/v1"), [])
        self.assertEqual(checker.lint_protocol(p, prepare.HERE), [])

    def test_absent_measurement_and_binary_stay_null(self):
        p = self.draft()
        self.assertEqual(p["freeze"]["status"], "draft")
        self.assertIsNone(p["freeze"]["protocol_sha256"])
        for field in ("fixed_seconds", "per_sample_seconds", "estimated_total_seconds", "measured_on"):
            self.assertIsNone(p["sample_size"]["preflight_runtime"][field])
        self.assertTrue(all(a["instrument"]["binary_sha256"] is None for a in p["arms"]))
        self.assertTrue(p["extensions"]["ml_dataset_sat_fixture"]["pending_before_freeze"])

    def test_frozen_candidate_with_missing_runtime_fails_real_linter(self):
        # In-memory invalid object only; no freeze command or frozen file exists.
        p = self.draft()
        p["freeze"]["status"] = "frozen"
        errors = checker.lint_protocol(p, prepare.HERE)
        self.assertTrue(any("preflight_runtime.estimated_total_seconds is null" in e for e in errors))

    def test_old_top_level_fixture_rejected_by_official_schema(self):
        p = self.draft()
        p["fixture"] = p["extensions"]["ml_dataset_sat_fixture"]
        self.assertTrue(checker.schema_errors(p, "experiment.protocol/v1"))

    def test_candidate_table_and_control_population_preserved(self):
        p = self.draft()
        f = p["extensions"]["ml_dataset_sat_fixture"]
        self.assertEqual(len(f["candidates"]), 16)
        self.assertEqual(sum(1 << (c["config"]["m"] - 2) for c in f["candidates"]), 24)
        for left, right in zip(f["candidates"][::2], f["candidates"][1::2]):
            l, r = dict(left["config"]), dict(right["config"])
            self.assertFalse(l.pop("twist")); self.assertTrue(r.pop("twist")); self.assertEqual(l, r)
        self.assertEqual(p["controls"]["negative"]["null_population"]["n"], 4)
        self.assertEqual(f["limits"]["attempt_bytes"], 3 * 1024**3)
        self.assertEqual(f["driver_files"], file_bindings())

    def test_canonical_roundtrip_is_exact_and_reproducible(self):
        p = self.draft()
        self.assertEqual(canonical(decode(canonical(p))), canonical(p))
        self.assertEqual(canonical(self.draft()), canonical(p))
        with tempfile.TemporaryDirectory() as d:
            file = Path(d) / "draft.json"
            write(file, p, exclusive=True)
            self.assertEqual(load(file), p)
            with self.assertRaises(FileExistsError):
                write(file, p, exclusive=True)

    def test_duplicate_json_and_nonfinite_timing_rejected(self):
        with self.assertRaises(ValueError): decode(b'{"x":1,"x":2}')
        with self.assertRaises(ValueError): decode(b'{"fixed_seconds":NaN}')

    def test_invalid_source_and_binary_rejected(self):
        with self.assertRaises(ValueError): prepare.prepare("short")
        with self.assertRaises(ValueError): prepare.prepare(SOURCE, "not-a-digest")

    def test_runtime_receipt_requires_source_binary_and_two_arms(self):
        with tempfile.TemporaryDirectory() as d:
            file = Path(d) / "invalid-runtime.json"
            write(file, {"schema": "ml-dataset.sat-fixture-runtime-preflight/v1",
                         "source_commit": SOURCE, "binary_sha256": "0" * 64,
                         "threads": 1, "excluded_from_candidate_selection": True,
                         "measured_on": "unit test rejection input; not execution evidence",
                         "arm_measurements": []})
            with self.assertRaisesRegex(ValueError, "final binary"):
                prepare.prepare(SOURCE, None, file)
            with self.assertRaisesRegex(ValueError, "source/binary mismatch"):
                prepare.prepare(SOURCE, "1" * 64, file)
            with self.assertRaisesRegex(ValueError, "each arm"):
                prepare.prepare(SOURCE, "0" * 64, file)


if __name__ == "__main__":
    unittest.main()
