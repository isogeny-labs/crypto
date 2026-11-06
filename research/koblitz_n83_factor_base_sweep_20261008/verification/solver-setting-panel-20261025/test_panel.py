"""Focused controls for the frozen N83 solver-setting comparison."""

import copy
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

from audit_artifact import interpret, members
from run import frozen, flags


HERE = Path(__file__).resolve().parent


class SolverSettingControls(unittest.TestCase):
    def test_exact_eight_formula_and_setting_grid(self):
        protocol = json.loads((HERE / "protocol.json").read_bytes())
        frozen(protocol)
        self.assertEqual(len(protocol["strata"]), 8)
        self.assertEqual(protocol["configurations"]["base"],
                         protocol["configurations"]["base_repeat"])
        self.assertIn("48", flags(protocol, protocol["configurations"]["matrix48"]))

    def test_changed_setting_or_cap_fails_before_search(self):
        protocol = json.loads((HERE / "protocol.json").read_bytes())
        changed = copy.deepcopy(protocol)
        changed["configurations"]["vsids"]["branchstr"] = "vmtf+vsids"
        with self.assertRaisesRegex(ValueError, "paired branch-and-matrix"):
            frozen(changed)
        changed = copy.deepcopy(protocol)
        changed["solver_time_cap_seconds"] = 601
        with self.assertRaisesRegex(ValueError, "frozen solver limits"):
            frozen(changed)

    def test_status_requires_one_consistent_solver_verdict(self):
        receipt = {"exit_code": 15, "watchdog": False, "oom_kill_delta": 0}
        stdout = b"s INDETERMINATE\nc conflicts : 123\n"
        self.assertEqual(interpret(stdout, b"", receipt), ("INDETERMINATE", 123))
        self.assertEqual(interpret(stdout + b"s SATISFIABLE\n", b"", receipt)[0],
                         "MALFORMED")
        self.assertEqual(interpret(stdout, b"error", receipt)[0], "MALFORMED")
        self.assertEqual(interpret(stdout, b"", dict(receipt, exit_code=10))[0],
                         "MALFORMED")

    def test_zip_digest_and_safe_member_names(self):
        with tempfile.TemporaryDirectory() as temporary:
            archive = Path(temporary) / "artifact.zip"
            with zipfile.ZipFile(archive, "w") as bundle:
                bundle.writestr("safe.json", b"{}")
            import hashlib
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            self.assertEqual(members(archive, digest), {"safe.json": b"{}"})
            with self.assertRaisesRegex(ValueError, "exact provider ZIP"):
                members(archive, "0" * 64)
            with zipfile.ZipFile(archive, "w") as bundle:
                bundle.writestr("../escape", b"x")
            digest = hashlib.sha256(archive.read_bytes()).hexdigest()
            with self.assertRaisesRegex(ValueError, "safe unique ZIP"):
                members(archive, digest)


if __name__ == "__main__":
    unittest.main()
