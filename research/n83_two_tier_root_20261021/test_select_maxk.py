#!/usr/bin/env python3
"""Check decisive, tied, and incomplete max-K selection inputs."""

import unittest

from research.n83_two_tier_root_20261021 import select_maxk as selector


IDS = {
    "k4300": "IC1N83Ckb1a1fb713800PDP4rootRCguidedLAgaussTDdirectISO0h" + "1" * 12,
    "k4405": "IC1N83Ckb1a1fb731230PDP4rootRCguidedLAgaussTDdirectISO0h" + "2" * 12,
    "k5086": "IC1N83Ckb1a1fb844276PDP4rootRCguidedLAgaussTDdirectISO0h" + "3" * 12,
}


def matrix():
    return {
        "schema": "n83.maxk-screen-validated/v1",
        "source_sha256": selector.SOURCE_SHA256,
        "targets_sha256": selector.TARGETS_SHA256,
        "evidence_status": "PASS_INDEPENDENT_REPLAY",
        "tier_status": "PASS_TIER1",
        "stage_gate_status": "PASS_EXACT_BASE_AND_INDEX",
        "single_pod": True,
        "candidate_ids": IDS,
        "candidate_manifest_sha256": {label: "a" * 64 for label in IDS},
        "block_candidate_orders": [list(selector.LABELS),
                                   list(reversed(selector.LABELS))],
        "points": [
            {"point_index": index, "block": index // 96,
             "resource_status": "PASS", "aa_status": "PASS",
             "arms": {
                 label: {"candidate_id": IDS[label],
                         "status": "PASS_INDEPENDENT_REPLAY",
                         "group_verified": True,
                         "resource_status": "PASS", "online_ms": timing}
                 for label, timing in zip(selector.LABELS, (4.0, 3.9, 3.0))}}
            for index in range(selector.POINT_COUNT)
        ],
    }


class SelectionControl(unittest.TestCase):
    def test_clear_winner(self):
        result = selector.select(matrix())
        self.assertEqual(result["selected_candidate_id"], IDS["k5086"])
        self.assertEqual(result["status"], "SELECTED")
        self.assertEqual(len(result["comparisons"]), 3)

    def test_failed_replay_blocks_selection(self):
        data = matrix()
        data["points"][0]["arms"]["k5086"]["group_verified"] = False
        result = selector.select(data)
        self.assertEqual(result["status"], "BOUNDED_UNKNOWN")
        self.assertIsNone(result["selected_candidate_id"])

    def test_near_tie_remains_unknown(self):
        data = matrix()
        for index, point in enumerate(data["points"]):
            point["arms"]["k5086"]["online_ms"] = 2.0 if index % 2 else 6.0
        result = selector.select(data)
        self.assertEqual(result["status"], "BOUNDED_UNKNOWN")
        self.assertEqual(selector.exact_two_sided_sign_p(96, 96), 1.0)

    def test_missing_point_and_bad_order_block_selection(self):
        data = matrix()
        data["points"].pop()
        self.assertEqual(selector.select(data)["status"], "BOUNDED_UNKNOWN")
        data = matrix()
        data["block_candidate_orders"][1] = list(selector.LABELS)
        self.assertEqual(selector.select(data)["status"], "BOUNDED_UNKNOWN")


if __name__ == "__main__":
    unittest.main()
