"""The Linux solver qualification retains failures and accepts exact SAT status."""

import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest import mock


SOURCE = Path(__file__).with_name("smoke.py")
SPEC = importlib.util.spec_from_file_location("n83_linux_xor_smoke", SOURCE)
SMOKE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SMOKE)


class SmokeTests(unittest.TestCase):
    def run_case(self, output: bytes, exit_code: int) -> tuple[dict, Path]:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        solver = root / "cryptominisat5"
        solver.write_bytes(b"pinned test solver")
        responses = [
            subprocess.CompletedProcess([], 0, "cryptominisat=5.11\n", ""),
            subprocess.CompletedProcess([], 0, b"version", b""),
            subprocess.CompletedProcess([], exit_code, output, b""),
        ]
        with mock.patch.object(SMOKE.shutil, "which", return_value=str(solver)), \
             mock.patch.object(SMOKE.subprocess, "run", side_effect=responses):
            if exit_code == 10 and output == b"s SATISFIABLE\nv 1 -2 0\n":
                receipt = SMOKE.smoke(root / "out")
            else:
                with self.assertRaisesRegex(RuntimeError, "flag smoke failed"):
                    SMOKE.smoke(root / "out")
                receipt = json.loads((root / "out/smoke.json").read_text())
        return receipt, root

    def test_exact_sat_status_and_binary_digest(self):
        receipt, root = self.run_case(b"s SATISFIABLE\nv 1 -2 0\n", 10)
        self.assertTrue(receipt["smoke_sat_line"])
        self.assertEqual(receipt["smoke_exit_code"], 10)
        self.assertEqual((root / "out/smoke.stdout").read_bytes(),
                         b"s SATISFIABLE\nv 1 -2 0\n")

    def test_comment_forgery_and_bad_exit_fail_closed(self):
        forged, _ = self.run_case(b"c s SATISFIABLE\n", 10)
        self.assertFalse(forged["smoke_sat_line"])
        wrong_exit, _ = self.run_case(b"s SATISFIABLE\n", 15)
        self.assertTrue(wrong_exit["smoke_sat_line"])
        self.assertEqual(wrong_exit["smoke_exit_code"], 15)


if __name__ == "__main__":
    unittest.main()
