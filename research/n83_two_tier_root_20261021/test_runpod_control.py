#!/usr/bin/env python3
"""Mock Pod creation to check the preflight and immediate deletion gates."""

from decimal import Decimal
import json
from pathlib import Path
import tempfile
import unittest

from research.n83_two_tier_root_20261021 import runpod_control as control


POD = {"id": "abc123", "name": "n83-maxk-control",
       "desiredStatus": "RUNNING", "machineId": "machine1",
       "machine": {"gpuTypeId": control.GPU, "secureCloud": False},
       "gpu": {"id": control.GPU, "count": 1},
       "costPerHr": 0.22, "memoryInGb": 125, "vcpuCount": 16,
       "image": control.IMAGE}


class CreateControl(unittest.TestCase):
    def run_case(self, pod, expect_eligible, ambiguous=False):
        calls = []

        def request(_key, method, path, _body=None):
            calls.append((method, path))
            if (method, path) == ("GET", "/pods"):
                return (200, [] if calls.count(("GET", "/pods")) == 1
                        else [pod])
            if (method, path) == ("POST", "/pods"):
                if ambiguous:
                    raise OSError("simulated response loss after provider creation")
                return 201, pod
            if (method, path) == ("GET", "/pods/abc123?includeMachine=true"):
                return 200, pod
            raise AssertionError("unexpected API request")

        original_request = control.request
        original_guard = control.runpod_guard.run
        original_delete = control.delete_until_absent
        try:
            control.request = request
            control.runpod_guard.run = lambda *_: 0
            deleted = []
            control.delete_until_absent = lambda *args: deleted.append(args)
            with tempfile.TemporaryDirectory() as folder:
                path = Path(folder) / "allocation"
                if expect_eligible:
                    result = control.create_and_guard("unused", path,
                                                      "n83-maxk-control",
                                                      "ssh-ed25519 PUBLIC", Decimal("5"), 43200)
                    self.assertEqual(result, 0)
                    allocation = json.loads((path / "allocation.json").read_text())
                    self.assertEqual(allocation["cost_per_hr_usd"], "0.22")
                    self.assertEqual(deleted, [])
                else:
                    with self.assertRaisesRegex(RuntimeError, "was deleted"):
                        control.create_and_guard("unused", path,
                                                 "n83-maxk-control",
                                                 "ssh-ed25519 PUBLIC", Decimal("5"), 43200)
                    self.assertEqual(len(deleted), 1)
                    self.assertFalse((path / "allocation.json").exists())
                self.assertEqual(calls.count(("POST", "/pods")), 1)
        finally:
            control.request = original_request
            control.runpod_guard.run = original_guard
            control.delete_until_absent = original_delete

    def test_eligible_pod_starts_guard(self):
        self.run_case(POD, True)

    def test_overpriced_pod_deleted(self):
        self.run_case({**POD, "costPerHr": 0.5}, False)

    def test_ambiguous_create_uses_inventory_without_second_post(self):
        self.run_case(POD, True, ambiguous=True)


if __name__ == "__main__":
    unittest.main()
