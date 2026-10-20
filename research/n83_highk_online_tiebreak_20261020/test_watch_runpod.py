"""The provider termination mutation returns Void and must omit subfields."""

import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from research.n83_highk_online_tiebreak_20261020 import watch_runpod


class WatchRunpodTest(unittest.TestCase):
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
