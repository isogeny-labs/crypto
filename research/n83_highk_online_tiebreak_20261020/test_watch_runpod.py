"""The provider termination mutation returns Void and must omit subfields."""

import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from research.n83_highk_online_tiebreak_20261020 import watch_runpod


class WatchRunpodTest(unittest.TestCase):
    def test_done_without_archive_retires_pod(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp) / "result"
            with (mock.patch.object(watch_runpod.runpod_runner, "read_allocation",
                                    return_value={"pod_id": "abc123",
                                                  "stop_after_utc": "2030-01-01T00:00:00Z"}),
                  mock.patch.object(watch_runpod, "pod_status",
                                    return_value={"id": "abc123"}),
                  mock.patch.object(watch_runpod, "remote",
                                    return_value="DONE screen=1"),
                  mock.patch.object(watch_runpod, "spool", return_value=False),
                  mock.patch.object(watch_runpod, "retire") as retire):
                code = watch_runpod.watch(Path("allocation"), Path("trigger"),
                                          "127.0.0.1", 22, root, "secret")
            self.assertEqual(code, 2)
            retire.assert_called_once_with("abc123", "secret", root)
            self.assertEqual(json.loads((root / "archive_failure.json").read_text())
                             ["archive_ready"], False)

    def test_exited_pod_terminates_without_void_selection(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            status = {"id": "abc123", "desiredStatus": "EXITED"}
            with mock.patch.object(watch_runpod, "pod_status",
                                   side_effect=(status, status, None)):
                with mock.patch.object(watch_runpod, "graphql",
                                       return_value={"data": {"podTerminate": None}}) as call:
                    watch_runpod.retire("abc123", "secret", root)
            query = call.call_args.args[0]
            self.assertIn('podTerminate(input: { podId: "abc123" }) }', query)
            receipt = json.loads((root / "provider_post_terminate.json").read_text())
            self.assertIs(receipt["pod_present"], False)
            self.assertEqual(receipt["pod_id"], "abc123")


if __name__ == "__main__":
    unittest.main()
