#!/usr/bin/env python3
"""Exercise the full-run admission contract without scheduling a runner."""

from datetime import datetime, timedelta, timezone
import unittest

from research.n83_k5500_highmem_full_20261021 import preflight


NOW = datetime(2026, 10, 21, 12, tzinfo=timezone.utc)
CAP_SHA = "b" * 64
HEAD_SHA = "a" * 40


def pair():
    approval = {
        "schema": "n83.k5500-highmem-full-approval/v1",
        "status": "APPROVED",
        "runner_label": "crypto-eks-highmem",
        "source_sha256": preflight.SOURCE_SHA256,
        "runner_script_sha256": preflight.RUNNER_SHA256,
        "capacity_head_sha": HEAD_SHA,
        "capacity_run_id": 1234,
        "capacity_artifact_id": 5678,
        "capacity_receipt_sha256": CAP_SHA,
        "quote_source": "example quote receipt",
        "quoted_usd_per_hour": "0.40",
        "billing_reserve_usd": "0.50",
        "approved_upper_usd": "2.50",
        "max_allocation_seconds": 18000,
        "approved_by": "owner",
        "approved_utc": (NOW - timedelta(minutes=1)).isoformat(),
        "expires_utc": (NOW + timedelta(hours=1)).isoformat(),
    }
    capacity = {
        "schema": "n83.k5500-existing-highmem-capacity/v1",
        "status": "PASS_CAPACITY_PREFLIGHT",
        "checks": {name: True for name in preflight.CAPACITY_CHECKS},
        "experiment_launched": False,
        "github_run_id": "1234",
        "github_sha": HEAD_SHA,
        "source_sha256": preflight.SOURCE_SHA256,
        "runner_script_sha256": preflight.RUNNER_SHA256,
    }
    return approval, capacity


class AdmissionTest(unittest.TestCase):
    def test_matching_receipts_admit(self):
        approval, capacity = pair()
        self.assertEqual(preflight.validate(approval, capacity, CAP_SHA, NOW)
                         ["approved_upper_usd"], "2.50")

    def test_missing_or_false_capacity_check_rejects(self):
        for action in (lambda checks: checks.clear(),
                       lambda checks: checks.update({"disk_free_at_least_8_gib": False}),
                       lambda checks: checks.update({"extra": False})):
            with self.subTest(action=action):
                approval, capacity = pair()
                action(capacity["checks"])
                with self.assertRaises(ValueError):
                    preflight.validate(approval, capacity, CAP_SHA, NOW)

    def test_approval_budget_and_provenance_reject(self):
        for field, value in (("status", "AWAITING_OWNER_APPROVAL"),
                             ("approved_upper_usd", "2.49"),
                             ("capacity_receipt_sha256", "c" * 64),
                             ("capacity_head_sha", "not-a-head"),
                             ("max_allocation_seconds", 14400),
                             ("source_sha256", "d" * 64),
                             ("expires_utc", (NOW - timedelta(seconds=1)).isoformat())):
            with self.subTest(field=field):
                approval, capacity = pair()
                approval[field] = value
                with self.assertRaises(ValueError):
                    preflight.validate(approval, capacity, CAP_SHA, NOW)

    def test_wrong_capacity_source_and_launched_reject(self):
        for field, value in (("github_run_id", "9999"),
                             ("source_sha256", "e" * 64),
                             ("experiment_launched", True)):
            with self.subTest(field=field):
                approval, capacity = pair()
                capacity[field] = value
                with self.assertRaises(ValueError):
                    preflight.validate(approval, capacity, CAP_SHA, NOW)


if __name__ == "__main__":
    unittest.main()
