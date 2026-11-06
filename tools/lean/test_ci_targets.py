"""Regression controls for the Actions proof matrix transport."""

import importlib.util
import json
from pathlib import Path
import subprocess
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "ci_targets", Path(__file__).with_name("ci-targets.py")
)
ci = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ci)


class MatrixTests(unittest.TestCase):
    def test_public_paths_never_enter_job_output(self):
        targets = ["research/public-but-masked/Proof.lean", "project with spaces"]
        output = ci.address(targets)
        self.assertEqual(json.loads(output), [0, 1])
        self.assertFalse(any(path in output for path in targets))

    def test_every_target_is_selected_exactly_once_in_discovery_order(self):
        targets = ["z/Proof.lean", "a/project", "a/Other.lean"]
        self.assertEqual([ci.address(targets, i) for i in json.loads(ci.address(targets))], targets)

    def test_no_targets_produces_empty_matrix(self):
        self.assertEqual(ci.address([]), "[]")

    def test_invalid_indexes_fail_instead_of_selecting_another_target(self):
        for index in (-1, 1, 100):
            with self.subTest(index=index), self.assertRaises(ValueError):
                ci.address(["one.lean"], index)

    def test_duplicate_discovery_fails(self):
        with self.assertRaises(ValueError):
            ci.address(["one.lean", "one.lean"])

    def test_failed_discovery_is_not_an_empty_success(self):
        with patch.object(ci.subprocess, "check_output", side_effect=subprocess.CalledProcessError(3, "discover")):
            with self.assertRaises(subprocess.CalledProcessError):
                ci.discover()


if __name__ == "__main__":
    unittest.main()
