import unittest

from research.n83_nearby_grid_confirm16_20261024 import select
from research.n83_nearby_grid_tier1_20261024 import protocol as screen


class SelectionTests(unittest.TestCase):
    def admitted(self):
        values = {str(k): 1.0 for k in screen.CANDIDATES}
        return {
            "status": "PASS_TIER1_GRID_SCREEN", "source_verified": True,
            "host_verified": True, "noise_gate_passed": True,
            "point_index": 0,
            "median_paired_online_ratio_to_baseline": values.copy(),
            "median_paired_cold_ratio_to_baseline": values.copy(),
        }

    def test_tie_break_and_union(self):
        row = self.admitted()
        row["median_paired_online_ratio_to_baseline"].update(
            {"1104": 0.7, "1105": 0.8, "1106": 0.9})
        row["median_paired_cold_ratio_to_baseline"].update(
            {"1118": 0.6, "1119": 0.7, "1120": 0.8})
        result = select.choose(row)
        self.assertEqual(result["selected_ks"],
                         [1104, 1105, 1106, 1107, 1118, 1119, 1120])

    def test_failed_tier_gate_cannot_select(self):
        row = self.admitted()
        row["noise_gate_passed"] = False
        self.assertIsNone(select.choose(row)["selected_ks"])

    def test_missing_candidate_ratio_fails_closed(self):
        row = self.admitted()
        row["median_paired_cold_ratio_to_baseline"].pop("1112")
        with self.assertRaisesRegex(ValueError, "exact candidate coverage"):
            select.choose(row)


if __name__ == "__main__":
    unittest.main()
