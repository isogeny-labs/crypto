#!/usr/bin/env python3
"""File-custody canaries against a completed actual replay, not arithmetic tests."""
import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest
from check_evidence import check, HERE

ATTEMPT = Path(os.environ["P192_REPLAY_ATTEMPT"]).resolve()


class Evidence(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="custody-control-", dir=HERE / "evidence")
        self.addCleanup(self.temp.cleanup)
        self.copy = Path(self.temp.name) / "replay"
        shutil.copytree(ATTEMPT, self.copy)

    def test_original_complete_replay(self):
        self.assertTrue(check(self.copy, True)["complete_chain"])

    def test_changed_actual_quartic_is_rejected(self):
        path = self.copy / "construction/replay/nist_cover_three_tests_20261116/evidence/p192-d7-kernel3-plane-quartic.json"
        data = json.loads(path.read_bytes())
        data["terms"][0]["coefficient"] = "0" if data["terms"][0]["coefficient"] != "0" else "1"
        path.write_text(json.dumps(data))
        with self.assertRaisesRegex(ValueError, "regenerated quartic differs"):
            check(self.copy, True)

    def test_missing_final_artifact_is_rejected(self):
        (self.copy / "global/result.json").unlink()
        with self.assertRaisesRegex(ValueError, "complete chain not established"):
            check(self.copy, True)

    def test_timeout_cannot_be_counted_as_canary_rejection(self):
        (self.copy / "calls/0009-native/exit-code.txt").write_text("124\n")
        with self.assertRaisesRegex(ValueError, "complete chain not established"):
            check(self.copy, True)

    def test_changed_execution_pin_is_rejected(self):
        (self.copy / "calls/0002-native/executable.sha256").write_text("0" * 64 + "  altered\n")
        with self.assertRaisesRegex(ValueError, "binary bytes changed"):
            check(self.copy, True)

    def test_changed_frozen_input_is_rejected(self):
        path = self.copy / "source/nist_cover_three_tests_20261116/inputs/p192-d7-prym.json"
        path.write_bytes(path.read_bytes() + b" ")
        with self.assertRaisesRegex(ValueError, "source/input bytes changed"):
            check(self.copy, True)


if __name__ == "__main__":
    unittest.main()
