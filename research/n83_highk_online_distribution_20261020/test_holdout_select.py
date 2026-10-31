"""Fresh one-target holdout gates for a screened N83 candidate."""

import unittest

from research.n83_highk_online_distribution_20261020 import holdout_select
from research.n83_highk_online_distribution_20261020 import select as screen_selector


def fixture():
    candidate = screen_selector.IDS[3]
    screen = {"schema": "n83.online-distribution-screen-selection/v1",
              "admission_status": "PASS", "status": "SELECTED",
              "selected_candidate_id": candidate}
    holdout = {
        "schema": "n83.online-distribution-holdout-validated/v1",
        "source_sha256": holdout_select.SOURCE_SHA256,
        "targets_sha256": holdout_select.TARGETS_SHA256,
        "candidate_id": candidate,
        "evidence_status": "PASS_INDEPENDENT_REPLAY",
        "single_cloud_instance": True, "same_candidate_base": True,
        "host_strata": [{"point_indices": list(range(8)),
                         "tier_status": "PASS_TIER1",
                         "aa_status": "PASS_20_PERCENT_PER_PAIR"}],
        "points": [{"point_index": index,
                    "ic": {"candidate_id": candidate,
                           "status": "PASS_INDEPENDENT_REPLAY",
                           "resource_status": "PASS",
                           "one_target_online_ms": 5.0 + index / 10,
                           "cold_process_ms": 1_000_000.0},
                    "rho": {"status": "PASS_INDEPENDENT_REPLAY",
                            "resource_status": "PASS",
                            "online_ms": 3_000.0 + index},
                    "calibration": {"status": "PASS_20_PERCENT"}}
                   for index in range(8)]}
    return screen, holdout


class HoldoutSelectorTest(unittest.TestCase):
    def test_fresh_paired_speedup(self):
        screen, holdout = fixture()
        result = holdout_select.select(screen, holdout)
        self.assertEqual(result["status"], "PASS_ALL_EIGHT_SAME_POINT")
        self.assertEqual(result["verified_speedup_candidate_id"],
                         screen["selected_candidate_id"])
        self.assertGreater(result["lower_95_paired_difference_ms"], 0)

    def test_screen_and_replay_gates_fail_closed(self):
        screen, holdout = fixture()
        screen["status"] = "BOUNDED_UNKNOWN"
        self.assertEqual(holdout_select.select(screen, holdout)["admission_status"],
                         "SCREEN_NOT_SELECTED")
        screen, holdout = fixture()
        holdout["points"][4]["ic"]["resource_status"] = "OOM"
        self.assertIsNone(holdout_select.select(screen, holdout)[
            "verified_speedup_candidate_id"])

    def test_pair_without_speedup_is_not_promoted(self):
        screen, holdout = fixture()
        holdout["points"][2]["rho"]["online_ms"] = 1.0
        result = holdout_select.select(screen, holdout)
        self.assertEqual(result["admission_status"], "PASS")
        self.assertEqual(result["status"], "BOUNDED_UNKNOWN")


if __name__ == "__main__":
    unittest.main()
