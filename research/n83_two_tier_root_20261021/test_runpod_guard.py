#!/usr/bin/env python3
"""Exercise the spending guard's deadline, rate, and provider-absence gates."""

from datetime import datetime, timedelta, timezone
import json
from pathlib import Path
import tempfile
import unittest

from research.n83_two_tier_root_20261021 import runpod_guard as guard


def allocation(hours_ago=12):
    return {
        "schema": "n83.maxk-runpod-allocation/v1",
        "pod_id": "abc123",
        "pod_name": "n83-maxk-control",
        "created_utc": (datetime.now(timezone.utc) -
                        timedelta(hours=hours_ago)).isoformat(),
        "cost_per_hr_usd": "0.22",
        "approved_cap_usd": "5",
        "max_lifetime_seconds": 43200,
    }


class GuardControl(unittest.TestCase):
    def test_deadline_deletes_and_verifies_absence(self):
        state = {"present": True, "deletes": 0}

        def request(_key, method, path):
            if method == "DELETE":
                state["deletes"] += 1
                state["present"] = False
                return 204, None
            if path == "/pods":
                return 200, []
            if state["present"]:
                return 200, {"id": "abc123", "name": "n83-maxk-control",
                             "costPerHr": 0.22}
            return 404, None

        original_request, original_sleep = guard.request, guard.time.sleep
        try:
            guard.request = request
            guard.time.sleep = lambda _: None
            with tempfile.TemporaryDirectory() as folder:
                receipt = Path(folder) / "guard.jsonl"
                self.assertEqual(guard.run(allocation(), receipt, "unused", 5), 0)
                rows = [json.loads(line) for line in receipt.read_text().splitlines()]
                self.assertEqual(state["deletes"], 1)
                self.assertEqual(rows[-1]["status"], "VERIFIED_POD_ABSENT")
        finally:
            guard.request, guard.time.sleep = original_request, original_sleep

    def test_rate_and_spending_bound(self):
        record = allocation(0)
        self.assertEqual(guard.gate_allocation(record)[0], "abc123")
        record["cost_per_hr_usd"] = "0.36"
        with self.assertRaises(ValueError):
            guard.gate_allocation(record)
        record["cost_per_hr_usd"] = "0.35"
        record["approved_cap_usd"] = "4"
        with self.assertRaises(ValueError):
            guard.gate_allocation(record)


if __name__ == "__main__":
    unittest.main()
