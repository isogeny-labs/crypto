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
