"""Cost deadline and provider deletion controls for the fixed31 Pod."""

import io
import json
import tempfile
import unittest
from datetime import datetime, timezone
from pathlib import Path
from unittest import mock

from research.n83_highk_beyond4405_20261021 import watch_runpod as watch
from research.n83_highk_beyond4405_20261021 import verify_watchdog_ready


ALLOCATION = {"pod_id": "abc123", "machine_id": "machine-a",
              "cost_per_hr": "0.20", "created_utc": "2030-01-01T00:00:00Z",
              "stop_after_utc": "2030-01-01T10:00:00Z"}
POD = {"id": "abc123", "machineId": "machine-a", "costPerHr": 0.2,
       "createdAt": "2030-01-01T00:00:00Z", "desiredStatus": "RUNNING"}


class WatchRunpodTest(unittest.TestCase):
    def test_graphql_uses_bearer_header_without_sdk(self):
        response = io.BytesIO(b'{"data":{"myself":{"pods":[]}}}')
        with mock.patch.object(watch.url_request, "urlopen",
                               return_value=response) as urlopen:
            result = watch.graphql("query { myself { pods { id } } }",
                                   "dummy-secret")
        self.assertEqual(result["data"]["myself"]["pods"], [])
        request = urlopen.call_args.args[0]
        self.assertEqual(request.get_header("Authorization"),
                         "Bearer dummy-secret")
        self.assertEqual(urlopen.call_args.kwargs["timeout"], 30)

    def test_pod_launch_requires_fresh_bound_readiness(self):
        at = datetime(2030, 1, 1, 1, tzinfo=timezone.utc)
        ready = {"schema": "n83.fixed31-watchdog-ready/v1",
                 "pod_id": "abc123", "allocation_sha256": "a" * 64,
                 "recorded_utc": "2030-01-01T00:59:00Z",
                 "retire_at_utc": "2030-01-01T09:55:00Z"}
        verify_watchdog_ready.verify(ALLOCATION, ready, "a" * 64, at)
        with self.assertRaises(ValueError):
            verify_watchdog_ready.verify(ALLOCATION,
                                          {**ready, "pod_id": "other"},
                                          "a" * 64, at)
        with self.assertRaises(ValueError):
            verify_watchdog_ready.verify(ALLOCATION, ready, "b" * 64, at)

    def test_done_transfers_verified_archive_and_retires(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "result"
            with (mock.patch.object(watch.runpod_runner, "read_allocation",
                                    return_value=ALLOCATION),
                  mock.patch.object(watch.provider, "digest", return_value="a" * 64),
                  mock.patch.object(watch, "now",
                                    return_value=datetime(2030, 1, 1, 1,
                                                          tzinfo=timezone.utc)),
                  mock.patch.object(watch, "pod_status", return_value=POD),
                  mock.patch.object(watch, "remote_status", return_value="DONE screen=0"),
                  mock.patch.object(watch, "spool", return_value=True),
                  mock.patch.object(watch, "archive_ready", return_value=True),
                  mock.patch.object(watch, "terminate") as terminate):
                code = watch.watch(Path("allocation"), "127.0.0.1", 22,
                                  output, "secret")
            self.assertEqual(code, 0)
            terminate.assert_called_once_with("abc123", "secret", output)
            transfer = json.loads((output / "watchdog_transfer.json").read_text())
            self.assertEqual(transfer["reason"], "PRODUCER_DONE")
            self.assertTrue(transfer["archive_ready"])
            self.assertTrue((output / "watchdog_ready.json").is_file())

    def test_failed_transfer_retires_and_preserves_failure(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "result"
            with (mock.patch.object(watch.runpod_runner, "read_allocation",
                                    return_value=ALLOCATION),
                  mock.patch.object(watch.provider, "digest", return_value="a" * 64),
                  mock.patch.object(watch, "now",
                                    return_value=datetime(2030, 1, 1, 1,
                                                          tzinfo=timezone.utc)),
                  mock.patch.object(watch, "pod_status", return_value=POD),
                  mock.patch.object(watch, "remote_status", return_value="DONE screen=2"),
                  mock.patch.object(watch, "spool", return_value=False),
                  mock.patch.object(watch, "terminate") as terminate):
                code = watch.watch(Path("allocation"), "127.0.0.1", 22,
                                  output, "secret")
            self.assertEqual(code, 2)
            terminate.assert_called_once_with("abc123", "secret", output)
            self.assertFalse(json.loads((output / "watchdog_transfer.json")
                                         .read_text())["archive_ready"])

    def test_late_start_still_terminates(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp) / "result"
            with (mock.patch.object(watch.runpod_runner, "read_allocation",
                                    return_value=ALLOCATION),
                  mock.patch.object(watch.provider, "digest", return_value="a" * 64),
                  mock.patch.object(watch, "now",
                                    return_value=datetime(2030, 1, 1, 9, 56,
                                                          tzinfo=timezone.utc)),
                  mock.patch.object(watch, "terminate") as terminate):
                code = watch.watch(Path("allocation"), "127.0.0.1", 22,
                                  output, "secret")
            self.assertEqual(code, 2)
            terminate.assert_called_once_with("abc123", "secret", output)
            self.assertEqual(json.loads((output / "watchdog_transfer.json")
                                        .read_text())["reason"],
                             "LATE_WATCHDOG_START")

    def test_terminate_uses_void_mutation_and_requires_absence(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            with (mock.patch.object(watch, "pod_status",
                                    side_effect=(POD, None)),
                  mock.patch.object(watch, "graphql",
                                    return_value={"data": {"podTerminate": None}})
                  as graphql):
                watch.terminate("abc123", "secret", output)
            self.assertIn('podTerminate(input: { podId: "abc123" }) }',
                          graphql.call_args.args[0])
            self.assertFalse(json.loads((output / "provider_post_terminate.json")
                                        .read_text())["pod_present"])


if __name__ == "__main__":
    unittest.main()
