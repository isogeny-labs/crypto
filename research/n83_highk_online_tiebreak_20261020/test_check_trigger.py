"""Fail-closed trigger checks for the conditional tie-break."""

import copy
import json
import tempfile
import unittest
from pathlib import Path

from research.n83_highk_online_tiebreak_20261020 import check_trigger, select


class TriggerTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        self.archive = root / "raw_results.tar.gz"
        self.archive.write_bytes(b"test archive bytes")
        self.aggregate = root / "aggregate.json"
        self.record = {
            "schema": "n83.online-distribution-local-aggregate/v1",
            "provider": {
                "schema": "n83.online-distribution-runpod-provider-proof/v1",
                "status": "PASS_SINGLE_RECORDED_RUNPOD_ALLOCATION",
            },
            "archive": {"archive_sha256": check_trigger.digest(self.archive),
                        "file_count": 10},
            "screen_selection": {
                "schema": "n83.online-distribution-screen-selection/v1",
                "admission_status": "PASS", "status": "BOUNDED_UNKNOWN",
                "selected_candidate_id": None,
                "candidate_ids": list(select.IDS), "point_count": 192,
                "screen_mode": "secondary_shared_setup_per_point_intervals",
            },
            "holdout": {
                "schema": "n83.online-distribution-not-dispatched/v1",
                "reason": "SCREEN_BOUNDED_UNKNOWN",
            },
        }

    def run_check(self):
        self.aggregate.write_text(json.dumps(self.record))
        return check_trigger.check(self.aggregate, self.archive)

    def test_admitted_null_selection_binds_both_file_hashes(self):
        receipt = self.run_check()
        self.assertEqual(receipt["status"], "PASS_ADMITTED_BOUNDED_UNKNOWN")
        self.assertEqual(receipt["trigger_archive_sha256"],
                         check_trigger.digest(self.archive))
        self.assertEqual(receipt["trigger_aggregate_sha256"],
                         check_trigger.digest(self.aggregate))

    def test_selected_noisy_or_changed_archive_cannot_trigger(self):
        original = copy.deepcopy(self.record)
        for key, value in (("status", "SELECTED"),
                           ("admission_status", "NOISY_OR_INCOMPLETE"),
                           ("selected_candidate_id", select.IDS[0])):
            self.record = copy.deepcopy(original)
            self.record["screen_selection"][key] = value
            with self.assertRaises(ValueError):
                self.run_check()
        self.record = original
        self.archive.write_bytes(b"changed archive")
        with self.assertRaises(ValueError):
            self.run_check()


if __name__ == "__main__":
    unittest.main()
