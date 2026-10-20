"""Exercise the budget and prior-Pod gates before any provider mutation."""

import datetime as dt
import json
import tempfile
import unittest
from decimal import Decimal
from pathlib import Path
from unittest import mock

from research.n83_highk_online_tiebreak_20261020 import allocate_runpod
from research.n83_highk_online_tiebreak_20261020.test_check_trigger import TriggerTest


class AllocationPreflightTest(unittest.TestCase):
    def setUp(self):
        TriggerTest.setUp(self)
        root = Path(self.temp.name)
        self.output = root / "new_allocation"
        self.termination = root / "prior_termination.json"
        self.termination.write_text(json.dumps({
            "recorded_utc": "2026-10-20T04:31:24.216912+00:00", "pod_present": False,
            "pod_id": "yajulte7twpr9p"}))
        TriggerTest.run_check(self)

    def test_worst_case_budget_and_termination_gate(self):
        at = dt.datetime(2026, 10, 20, 4, 57, tzinfo=dt.timezone.utc)
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            trigger = allocate_runpod.preflight(
                self.aggregate, self.archive, self.termination, self.output)
        self.assertEqual(trigger["status"], "PASS_ADMITTED_BOUNDED_UNKNOWN")
        self.assertLessEqual(allocate_runpod.prior_upper(json.loads(self.termination.read_text())) +
                             allocate_runpod.PRICE_CAP * Decimal(4),
                             Decimal("5.00"))
        self.termination.write_text(json.dumps({
            "recorded_utc": "2026-10-20T04:31:24.216912+00:00", "pod_present": True,
            "pod_id": "yajulte7twpr9p"}))
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            with self.assertRaises(ValueError):
                allocate_runpod.preflight(
                    self.aggregate, self.archive, self.termination, self.output)

    def test_failed_allocation_reserve_covers_conservative_interval(self):
        at = dt.datetime(2026, 10, 20, 4, 57, tzinfo=dt.timezone.utc)
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            allocate_runpod.preflight(
                self.aggregate, self.archive, self.termination, self.output)
        self.assertGreater(allocate_runpod.FAILED_ALLOCATION_RESERVE_USD,
                           Decimal("0.025"))

    def test_partial_provider_response_retires_allocated_pod(self):
        at = dt.datetime(2026, 10, 20, 4, 57, tzinfo=dt.timezone.utc)
        raw = {"id": "partial123", "createdAt": None, "costPerHr": 0.18}
        with (mock.patch.object(allocate_runpod, "utc_now", return_value=at),
              mock.patch.object(allocate_runpod, "provider_pods", side_effect=[[]] + [[]] * 6),
              mock.patch.object(allocate_runpod, "graphql", return_value={
                  "data": {"podFindAndDeployOnDemand": raw}}),
              mock.patch.object(allocate_runpod.time, "sleep"),
              mock.patch("research.n83_highk_online_tiebreak_20261020.watch_runpod.retire") as retire):
            with self.assertRaisesRegex(ValueError, "no admissible"):
                allocate_runpod.allocate(self.aggregate, self.archive,
                                         self.termination, self.output, "key")
        retire.assert_called_once_with("partial123", "key", self.output)
        errors = json.loads((self.output / "allocation_errors.json").read_text())
        self.assertEqual(errors[0]["type"], "ValueError")
        self.assertTrue((self.output / "provider_api_raw.json").exists())

    def test_mutation_error_discovers_and_retires_pod(self):
        at = dt.datetime(2026, 10, 20, 4, 57, tzinfo=dt.timezone.utc)
        with (mock.patch.object(allocate_runpod, "utc_now", return_value=at),
              mock.patch.object(allocate_runpod, "provider_pods",
                                side_effect=[[], [{"id": "late123", "name":
                                                   allocate_runpod.POD_NAME}]]),
              mock.patch.object(allocate_runpod, "graphql",
                                side_effect=ConnectionError("reply lost")),
              mock.patch("research.n83_highk_online_tiebreak_20261020.watch_runpod.retire") as retire):
            with self.assertRaisesRegex(ValueError, "no admissible"):
                allocate_runpod.allocate(self.aggregate, self.archive,
                                         self.termination, self.output, "key")
        retire.assert_called_once_with("late123", "key", self.output)

    def test_uncertain_mutation_retries_discovery_and_fails_closed(self):
        at = dt.datetime(2026, 10, 20, 4, 57, tzinfo=dt.timezone.utc)
        source_evidence = allocate_runpod.EVIDENCE
        test_evidence = Path(self.temp.name) / "evidence"
        test_evidence.mkdir()
        for name in ("failed_allocation_provider_raw.json",
                     "failed_allocation_termination.json",
                     "failed_v2_allocation_provider.json",
                     "failed_v2_allocation_termination.json"):
            (test_evidence / name).write_bytes((source_evidence / name).read_bytes())
        with mock.patch.object(allocate_runpod, "EVIDENCE", test_evidence):
            with (mock.patch.object(allocate_runpod, "utc_now", return_value=at),
                  mock.patch.object(allocate_runpod, "provider_pods",
                                    side_effect=[[]] + [ConnectionError("readback unavailable")] * 6) as pods,
                  mock.patch.object(allocate_runpod, "graphql",
                                    side_effect=ConnectionError("create reply lost")),
                  mock.patch.object(allocate_runpod.time, "sleep"),
                  mock.patch("research.n83_highk_online_tiebreak_20261020.watch_runpod.retire") as retire):
                with self.assertRaisesRegex(ValueError, "no admissible"):
                    allocate_runpod.allocate(self.aggregate, self.archive,
                                             self.termination, self.output, "key")
            retire.assert_not_called()
            self.assertEqual(pods.call_count, 1 +
                             allocate_runpod.UNCERTAIN_ALLOCATION_READBACK_ATTEMPTS)
            unresolved = json.loads((self.output / "unresolved_allocation.json").read_text())
            self.assertEqual(unresolved["status"], "UNRESOLVED_PROVIDER_MUTATION")
            self.assertTrue(unresolved["retry_allocation_prohibited"])
            self.assertEqual(len(unresolved["discovery_attempts"]),
                             allocate_runpod.UNCERTAIN_ALLOCATION_READBACK_ATTEMPTS)
            with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
                with self.assertRaisesRegex(ValueError, "prior allocation outcome is unresolved"):
                    allocate_runpod.preflight(self.aggregate, self.archive,
                                               self.termination, self.temp.name + "/different-output")


if __name__ == "__main__":
    unittest.main()
