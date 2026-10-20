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
            "recorded_utc": "2026-10-20T04:00:00Z", "pod_present": False,
            "pod_id": "yajulte7twpr9p"}))
        TriggerTest.run_check(self)

    def test_worst_case_budget_and_termination_gate(self):
        at = dt.datetime(2026, 10, 20, 4, 15, tzinfo=dt.timezone.utc)
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            trigger = allocate_runpod.preflight(
                self.aggregate, self.archive, self.termination, self.output)
        self.assertEqual(trigger["status"], "PASS_ADMITTED_BOUNDED_UNKNOWN")
        self.assertLessEqual(allocate_runpod.prior_upper(at) +
                             allocate_runpod.PRICE_CAP * Decimal(4),
                             Decimal("5.00"))
        self.termination.write_text(json.dumps({
            "recorded_utc": "2026-10-20T04:00:00Z", "pod_present": True,
            "pod_id": "yajulte7twpr9p"}))
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            with self.assertRaises(ValueError):
                allocate_runpod.preflight(
                    self.aggregate, self.archive, self.termination, self.output)

    def test_budget_closes_after_window(self):
        at = dt.datetime(2026, 10, 20, 6, 0, tzinfo=dt.timezone.utc)
        with mock.patch.object(allocate_runpod, "utc_now", return_value=at):
            with self.assertRaisesRegex(ValueError, "exceeds"):
                allocate_runpod.preflight(
                    self.aggregate, self.archive, self.termination, self.output)


if __name__ == "__main__":
    unittest.main()
