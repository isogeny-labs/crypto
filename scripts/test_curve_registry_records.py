"""Complete frozen EC1 records must remain registered under their exact UID."""
import copy
import fnmatch
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import build_curve_registry as builder


class CurveRecordTests(unittest.TestCase):
    def record(self):
        source = builder.REPO / "research/ic_single_target_20260930/curve_ids.json"
        records = json.loads(source.read_text())["curves"]
        return copy.deepcopy(next(r for r in records if r.get("n") == 73))

    def harvest(self, root):
        def tracked(*patterns):
            return [p for p in root.iterdir()
                    if any(fnmatch.fnmatch(p.name, pattern) for pattern in patterns)]

        reg = builder.Registry()
        with patch.object(builder, "REPO", root), patch.object(builder, "tracked", tracked):
            builder.harvest_curve_records(reg)
            curves = list(reg.curves.values())
            for curve in curves:
                key = builder.ec1.digest(json.loads(curve["model_json"]))
                # Registry representation keys are the canonical model hashes.
                self.assertIn(key, reg.reps)
                curve["representations"] = list(reg.reps[key].values())
                for uid, rep in reg.reps[key].items():
                    rep["curve_uid"] = uid
            fatal = []
            builder.check_thread_ids(curves, fatal)
        return reg, fatal

    def test_complete_curve_id_record_retains_historical_representation(self):
        record = self.record()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "curve_ids.json").write_text(json.dumps({"curves": [record]}))
            reg, fatal = self.harvest(root)
        self.assertFalse(fatal)
        self.assertTrue(any(record["curve_uid"] in reps for reps in reg.reps.values()))

    def test_prime_r_alias_preserves_frozen_representation(self):
        source = builder.REPO / "research/conductor_gap_cm_orbits_20261015/prepared/curve_ids.json"
        record = json.loads(source.read_text())[0]
        explicit = dict(record, subgroup_order=record["r"])
        self.assertEqual(builder.subgroup_of(record, record["order"]),
                         builder.subgroup_of(explicit, record["order"]))
        self.assertEqual(builder.subgroup_of(record, record["order"]), (record["r"], 1))
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            fixture = root / "curve_ids.json"
            fixture.write_text(json.dumps([record]))
            registry = builder.Registry()
            problems, fatal = [], []
            with patch.object(builder, "REPO", root), patch.object(builder, "tracked", return_value=[fixture]):
                builder.harvest_json(registry, problems, fatal)
        self.assertFalse(problems)
        self.assertFalse(fatal)
        self.assertEqual(sum(map(len, registry.reps.values())), 1)
        expected = builder.ec1.curve_identity(*builder.prime_record(
            record["p"], record["a"], record["b"], record["order"], record["r"], 1,
            builder.generator_of(record)), "x")["curve_uid"]
        self.assertTrue(any(expected in reps for reps in registry.reps.values()))

    def test_invalid_subgroup_alias_does_not_enter_registry(self):
        self.assertIsNone(builder.subgroup_of({"r": 0}, 13))
        self.assertIsNone(builder.subgroup_of({"r": 5}, 13))
        with self.assertRaisesRegex(ValueError, "conflicting"):
            builder.subgroup_of({"r": 13, "subgroup_order": 7}, 13)

    def test_declared_uid_mismatch_fails_closed(self):
        record = self.record()
        record["curve_uid"] = "urn:ec-record:1:sha256:" + "0" * 64
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "curve_ids.json").write_text(json.dumps({"curves": [record]}))
            with self.assertRaisesRegex(ValueError, "UID does not match"):
                self.harvest(root)


if __name__ == "__main__":
    unittest.main()
