"""Exercise the preallocation rate and lifetime ceiling without creating a Pod."""

from datetime import datetime, timedelta, timezone
import json
from pathlib import Path
import tempfile
import unittest

from research.n83_highk_beyond4405_20261021 import run, runpod_runner


class RunPodAllocationCeilingTests(unittest.TestCase):
    def receipt(self, rate: str, duration: timedelta) -> dict:
        dispatch_path = runpod_runner.HERE / "dispatch.json"
        dispatch = json.loads(dispatch_path.read_text())
        created = datetime(2030, 1, 1, tzinfo=timezone.utc)
        return {
            "schema": "n83.fixed31-runpod-allocation/v1",
            "approval_status": "APPROVED_NEW_CAP",
            "desired_status": "RUNNING",
            "source_sha256": dispatch["source_sha256"],
            "dispatch_sha256": run.digest(dispatch_path),
            "pod_id": "pod123",
            "machine_id": "machine123",
            "cpu_affinity_ids": [0, 1, 2, 3],
            "vcpu_count": 16,
            "memory_in_gb": 125,
            "source_head_sha": "0" * 40,
            "snapshot_sha256": "0" * 64,
            "cost_per_hr": rate,
            "created_utc": created.isoformat(),
            "stop_after_utc": (created + duration).isoformat(),
        }

    def read(self, receipt: dict) -> dict:
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "allocation.json"
            path.write_text(json.dumps(receipt) + "\n")
            return runpod_runner.read_allocation(path)

    def test_catalog_rate_at_ten_hour_cap_is_admitted(self):
        receipt = self.receipt("0.22", timedelta(hours=10))
        self.assertEqual(self.read(receipt), receipt)

    def test_rate_and_lifetime_overruns_are_rejected(self):
        for rate, duration in (
            ("0.220001", timedelta(hours=10)),
            ("0.22", timedelta(hours=10, seconds=1)),
        ):
            with self.subTest(rate=rate, duration=duration):
                with self.assertRaisesRegex(ValueError, "budget envelope"):
                    self.read(self.receipt(rate, duration))


if __name__ == "__main__":
    unittest.main()
