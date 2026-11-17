"""Fail-closed Linux solver stream classification and frozen input identity."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock

import run_panel
import audit_artifact


HERE = Path(__file__).resolve().parent


class LinuxNaturalPanelTests(unittest.TestCase):
    def test_frozen_protocol_binds_four_complete_inputs_and_binary_artifact(self):
        protocol = json.loads((HERE / "protocol.json").read_bytes())
        manifest = json.loads(run_panel.INPUT_MANIFEST.read_bytes())
        self.assertEqual(run_panel.file_sha(run_panel.INPUT_MANIFEST),
                         protocol["input_manifest_sha256"])
        self.assertEqual({row["name"]: row["sha256"] for row in manifest["entries"]
                          if row["kind"] == "xcnf"}, protocol["inputs"])
        self.assertEqual(len(manifest["entries"]), 11)
        self.assertEqual(len(protocol["arms"]), 6)
        self.assertEqual(protocol["preflight_artifact_sha256"],
                         "00713618c7c09188a9f75fef6ce6323af376428cc48d537491394c91429d1237")
        self.assertIsNone(protocol["selected_best_total_runtime"])

    def test_real_status_requires_one_status_one_counter_and_matching_exit(self):
        body = b"c conflicts                : 2000\ns INDETERMINATE\n"
        self.assertEqual(run_panel.classify(body, b"", 15, False, False),
                         ("INDETERMINATE", 2000))
        self.assertEqual(run_panel.classify(body, b"", 10, False, False)[0],
                         "MALFORMED")
        self.assertEqual(run_panel.classify(body + b"s SATISFIABLE\n", b"", 15,
                                            False, False)[0], "MALFORMED")
        self.assertEqual(run_panel.classify(b"s SATISFIABLE\n", b"", 10,
                                            False, False)[0], "MALFORMED")

    def test_watchdog_and_oom_are_censored_even_with_sat_text(self):
        body = b"c conflicts : 1\ns SATISFIABLE\nv 1 0\n"
        self.assertEqual(run_panel.classify(body, b"", 10, True, False)[0],
                         "WATCHDOG")
        self.assertEqual(run_panel.classify(body, b"", 10, False, True)[0],
                         "OOM")
        self.assertEqual(run_panel.classify(body, b"warning", 10, False, False)[0],
                         "MALFORMED")

    def test_process_receipt_replays_raw_status_and_refuses_overwrite(self):
        with tempfile.TemporaryDirectory(prefix="n83-linux-panel-test-") as tmp:
            root = Path(tmp)
            solver = root / "solver"
            solver.write_text("#!/bin/sh\nprintf 'c conflicts : 2\\ns INDETERMINATE\\n'\nexit 15\n")
            solver.chmod(0o700)
            inputs = root / "inputs"
            inputs.mkdir()
            (inputs / "hash_5_1.xcnf").write_bytes(b"p cnf 1 0\n")
            out = root / "out"
            out.mkdir()
            protocol = {"solver_flags": [], "process_watchdog_seconds": 5,
                        "memory_address_space_limit_bytes": 8 * 1024**3,
                        "inputs": {"hash_5_1": "test-input-sha"},
                        "solver_sha256": "test-solver-sha"}
            arm = {"id": "A", "input": "hash_5_1"}
            # macOS does not implement finite RLIMIT_AS; exercise the process
            # receipt path while the hosted Linux arm enforces the real limit.
            with mock.patch.object(run_panel.resource, "setrlimit"):
                receipt = run_panel.run_arm(protocol, arm, inputs, solver, out)
            self.assertEqual((receipt["status"], receipt["exit_code"],
                              receipt["conflicts"]), ("INDETERMINATE", 15, 2))
            raw = (out / "A/solver.stdout").read_bytes()
            self.assertEqual(receipt["stdout_sha256"], run_panel.sha(raw))
            self.assertEqual(json.loads((out / "A/process.json").read_bytes()),
                             receipt)
            with self.assertRaises(FileExistsError):
                with mock.patch.object(run_panel.resource, "setrlimit"):
                    run_panel.run_arm(protocol, arm, inputs, solver, out)

    def test_independent_audit_rejects_forged_extra_status(self):
        receipt = {"watchdog": False, "oom_kill_delta": 0, "exit_code": 10}
        body = b"c conflicts : 11\ns SATISFIABLE\nv 1 0\n"
        self.assertEqual(audit_artifact.interpret(body, b"", receipt),
                         ("SATISFIABLE", 11))
        forged = body + b"s INDETERMINATE\n"
        self.assertEqual(audit_artifact.interpret(forged, b"", receipt)[0],
                         "MALFORMED")
        with self.assertRaises(ValueError):
            audit_artifact.exact_prefix({"one/host.json", "two/host.json"},
                                        "host.json")


if __name__ == "__main__":
    unittest.main()
