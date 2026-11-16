"""Check the fixed31 Pod's independent deletion path without provider calls."""

from datetime import datetime, timezone
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock

from research.n83_highk_beyond4405_20261021 import pod_deadline_guard as subject


ALLOCATION = {
    "pod_id": "abc123", "created_utc": "2030-01-01T00:00:00Z",
    "stop_after_utc": "2030-01-01T10:00:00Z",
}
READY = {
    "schema": "n83.fixed31-watchdog-ready/v1", "pod_id": "abc123",
    "allocation_sha256": "a" * 64,
    "recorded_utc": "2030-01-01T00:59:00Z",
    "retire_at_utc": "2030-01-01T09:55:00Z",
}


class PodDeadlineGuardTests(unittest.TestCase):
    def test_armed_guard_retries_deletion_of_bound_pod_at_deadline(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            allocation = root / "allocation.json"
            ready = root / "ready.json"
            receipt = root / "guard.json"
            allocation.write_text("{}")
            ready.write_text(json.dumps(READY))
            current = [datetime(2030, 1, 1, 1, tzinfo=timezone.utc)]
            deleted = []

            def pause(_):
                current[0] = datetime(2030, 1, 1, 10, tzinfo=timezone.utc)

            def delete(pod_id):
                deleted.append(pod_id)
                if len(deleted) == 1:
                    raise ConnectionError("transient provider failure")

            with (mock.patch.object(subject.runpod_runner, "read_allocation",
                                    return_value=ALLOCATION),
                  mock.patch.object(subject.run, "digest", return_value="a" * 64)):
                subject.guard(allocation, ready, receipt,
                              clock=lambda: current[0], pause=pause,
                              delete=delete)
            self.assertEqual(deleted, ["abc123", "abc123"])
            self.assertEqual(json.loads(receipt.read_text())["status"],
                             "DELETION_REQUESTED")

    def test_unbound_readiness_cannot_arm_or_delete(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            allocation = root / "allocation.json"
            ready = root / "ready.json"
            receipt = root / "guard.json"
            allocation.write_text("{}")
            ready.write_text(json.dumps({**READY, "pod_id": "another"}))
            deleted = []
            with (mock.patch.object(subject.runpod_runner, "read_allocation",
                                    return_value=ALLOCATION),
                  mock.patch.object(subject.run, "digest", return_value="a" * 64)):
                with self.assertRaises(ValueError):
                    subject.guard(allocation, ready, receipt,
                                  clock=lambda: datetime(2030, 1, 1, 1,
                                                         tzinfo=timezone.utc),
                                  delete=deleted.append)
            self.assertFalse(receipt.exists())
            self.assertEqual(deleted, [])


if __name__ == "__main__":
    unittest.main()
