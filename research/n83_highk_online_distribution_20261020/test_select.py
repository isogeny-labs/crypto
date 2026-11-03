"""Selector gates and exact sign probabilities for the frozen 192-point screen."""

import unittest

from research.n83_highk_online_distribution_20261020 import select


def admitted_fixture():
    ids = list(select.IDS)
    return {
        "schema": "n83.online-distribution-screen-validated/v1",
        "source_sha256": select.SOURCE_SHA256,
        "targets_sha256": select.TARGETS_SHA256,
        "candidate_ids": ids,
        "screen_mode": "secondary_shared_setup_per_point_intervals",
        "evidence_status": "PASS_INDEPENDENT_REPLAY",
        "single_cloud_instance": True,
        "same_candidate_bases": True,
        "host_strata": [{"point_indices": list(range(select.POINT_COUNT)),
                         "tier_status": "PASS_TIER1",
                         "aa_status": "PASS_20_PERCENT_PER_CHILD"}],
        "block_point_indices": [list(block) for block in select.BLOCKS],
        "block_candidate_orders": [ids, list(reversed(ids))],
        "points": [
            {"point_index": point,
             "rho": {"status": "PASS_INDEPENDENT_REPLAY",
                     "resource_status": "PASS", "online_ms": 1000.0},
             "arms": [
                 {"candidate_id": candidate,
                  "status": "PASS_INDEPENDENT_REPLAY",
                  "resource_status": "PASS",
                  "one_target_online_ms": float((index + 1) * 2 +
                                                 (point % 3) * 0.01)}
                 for index, candidate in enumerate(ids)]}
            for point in range(select.POINT_COUNT)]}


class ScreenSelectorTest(unittest.TestCase):
    def test_exact_sign_probability_and_unique_winner(self):
        self.assertEqual(select.exact_two_sided_sign_p(8, 0), 0.0078125)
        self.assertEqual(select.exact_two_sided_sign_p(4, 4), 1.0)
        selected = select.select(admitted_fixture())
        self.assertEqual(selected["admission_status"], "PASS")
        self.assertEqual(selected["status"], "SELECTED")
        self.assertEqual(selected["selected_candidate_id"], select.IDS[0])
        self.assertEqual(len(selected["comparisons"]), 10)

    def test_missing_or_noisy_pair_is_censored(self):
        fixture = admitted_fixture()
        del fixture["points"][-1]
        self.assertEqual(select.select(fixture)["selected_candidate_id"], None)
        fixture = admitted_fixture()
        fixture["points"][31]["arms"][2]["resource_status"] = "THROTTLED"
        self.assertEqual(select.select(fixture)["selected_candidate_id"], None)
        fixture = admitted_fixture()
        fixture["host_strata"][0]["aa_status"] = "NOISY"
        self.assertEqual(select.select(fixture)["selected_candidate_id"], None)

    def test_pairwise_tie_stays_unselected(self):
        fixture = admitted_fixture()
        for point in fixture["points"]:
            point["arms"][1]["one_target_online_ms"] = point["arms"][0]["one_target_online_ms"]
        selected = select.select(fixture)
        self.assertEqual(selected["status"], "BOUNDED_UNKNOWN")
        self.assertIsNone(selected["selected_candidate_id"])


if __name__ == "__main__":
    unittest.main()
