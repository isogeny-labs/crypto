#!/usr/bin/env python3
"""Mock Pod creation to check the preflight and immediate deletion gates."""

from decimal import Decimal
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

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

    def test_missing_price_deleted(self):
        self.run_case({**POD, "costPerHr": None}, False)

    def test_malformed_price_deleted(self):
        self.run_case({**POD, "costPerHr": "unavailable"}, False)

    def test_post_create_failures_clean_up_identified_pod(self):
        for failure in ("inventory", "detail", "create_receipt",
                        "detail_receipt", "allocation_receipt", "guard"):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as folder:
                calls = []

                def request(_key, method, path, _body=None):
                    calls.append((method, path))
                    if method == "POST":
                        return 201, POD
                    if path == "/pods":
                        if calls.count(("GET", "/pods")) == 1:
                            return 200, []
                        if failure == "inventory":
                            raise OSError("inventory unavailable")
                        return 200, [POD]
                    if failure == "detail":
                        raise OSError("detail unavailable")
                    return 200, POD

                original_save = control.save_new

                def save(path, data):
                    if path.name == {"create_receipt": "create.json",
                                     "detail_receipt": "post_create.json",
                                     "allocation_receipt": "allocation.json"}.get(failure):
                        raise OSError("receipt disk unavailable")
                    original_save(path, data)

                with patch.object(control, "request", request), \
                        patch.object(control, "save_new", save), \
                        patch.object(control, "delete_until_absent") as delete, \
                        patch.object(control.runpod_guard, "run",
                                     side_effect=OSError("guard unavailable")):
                    with self.assertRaises((OSError, RuntimeError)):
                        control.create_and_guard("unused", Path(folder) / "allocation",
                                                 "n83-maxk-control", "ssh-ed25519 PUBLIC",
                                                 Decimal("5"), 43200)
                    delete.assert_called_once_with(
                        "unused", "abc123", "n83-maxk-control",
                        Path(folder) / "allocation" / "guard.jsonl")
                self.assertEqual(calls.count(("POST", "/pods")), 1)

    def test_delete_retries_network_and_receipt_failures_until_absent(self):
        with patch.object(control.runpod_guard, "observe", side_effect=[
                ("POD_PRESENT", {}), ("POD_PRESENT", {}), ("POD_ABSENT", {})]), \
                patch.object(control.runpod_guard, "request",
                             side_effect=[OSError("transient network failure"), (204, None)]) as request, \
                patch.object(control.runpod_guard, "write_receipt",
                             side_effect=OSError("disk unavailable")), \
                patch.object(control.time, "sleep"):
            control.delete_until_absent("unused", "abc123", "n83-maxk-control",
                                        Path("unused-receipt"))
            self.assertEqual(request.call_count, 2)

    def test_ambiguous_create_waits_for_inventory_visibility_without_reposting(self):
        responses = [(200, []), OSError("response lost"), (200, []),
                     (200, [POD]), (200, POD)]
        with tempfile.TemporaryDirectory() as folder, \
                patch.object(control, "request", side_effect=responses) as request, \
                patch.object(control.time, "sleep") as sleep, \
                patch.object(control.runpod_guard, "run", return_value=0), \
                patch.object(control, "delete_until_absent") as delete:
            self.assertEqual(control.create_and_guard(
                "unused", Path(folder) / "allocation", "n83-maxk-control",
                "ssh-ed25519 PUBLIC", Decimal("5"), 43200), 0)
            self.assertEqual(sum(call.args[1] == "POST" for call in request.call_args_list), 1)
            sleep.assert_called_once_with(10)
            delete.assert_not_called()

    def test_unsuccessful_guard_exit_still_cleans_up(self):
        with tempfile.TemporaryDirectory() as folder, \
                patch.object(control, "request", side_effect=[
                    (200, []), (201, POD), (200, [POD]), (200, POD)]), \
                patch.object(control.runpod_guard, "run", return_value=2), \
                patch.object(control, "delete_until_absent") as delete:
            with self.assertRaisesRegex(RuntimeError, "without verified Pod absence"):
                control.create_and_guard(
                    "unused", Path(folder) / "allocation", "n83-maxk-control",
                    "ssh-ed25519 PUBLIC", Decimal("5"), 43200)
            delete.assert_called_once()

    def test_ambiguous_multiple_new_pods_are_cleaned_up(self):
        other = {**POD, "id": "abc456"}
        with tempfile.TemporaryDirectory() as folder, \
                patch.object(control, "request", side_effect=[
                    (200, []), (201, POD), (200, [POD, other])]), \
                patch.object(control.runpod_guard, "run") as guard, \
                patch.object(control, "delete_until_absent") as delete:
            with self.assertRaisesRegex(RuntimeError, "identity unresolved"):
                control.create_and_guard(
                    "unused", Path(folder) / "allocation", "n83-maxk-control",
                    "ssh-ed25519 PUBLIC", Decimal("5"), 43200)
            self.assertEqual([call.args[1] for call in delete.call_args_list],
                             ["abc123", "abc456"])
            guard.assert_not_called()


if __name__ == "__main__":
    unittest.main()
