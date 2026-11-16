"""Append-only and exact-partner checks for the D18 mixed suffix."""

import json
import unittest

import d18_mixed_frontier_v6 as v6
import exact_d18_frontier_v5 as v5


class D18MixedFrontierV6Test(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.design = v6.build()
        cls.saved = json.loads(v6.OUTPUT.read_text())

    def test_saved_bytes_counts_and_v5_prefix(self):
        self.assertEqual(v6.v3.canonical_bytes(self.design), v6.OUTPUT.read_bytes())
        self.assertEqual(self.design["ordered_mixed_pair_count"], 1321)
        self.assertEqual(self.design["added_tuple_count"], 205441920)
        self.assertEqual(self.design["total_tuple_count"], 27144771840)
        parent = json.loads(v5.OUTPUT.read_text())
        for ordinal in (0, 518918400, 26939278080, 26939329919):
            with self.subTest(ordinal=ordinal):
                self.assertEqual(v6.case(self.saved, ordinal), v5.case(parent, ordinal))

    def test_exact_uniform_partner_in_both_orders(self):
        base = json.loads(v6.v3.PARENT.read_text())
        slots = v6.primary_indices(base)
        self.assertIn(1133, slots)
        for orientation in (0, 1):
            pair_index = 2 * slots.index(1133) + orientation
            ordinal = (self.design["v5_tuple_count_preserved"]
                       + pair_index * 3 * 51840 + 51840 + 0)
            row = v6.case(self.saved, ordinal)
            self.assertEqual(row["pair"]["orientation"],
                             "D18_first" if orientation == 0 else "D18_second")
            self.assertEqual(row["pair"]["left" if orientation else "right"]
                             ["parent_base_index"], 1133)
            self.assertEqual(row["residual_summands"], 1)
            self.assertIsNone(row["candidate_id"])
            self.assertIsNone(row["selected_best_total_runtime"])
            self.assertFalse(row["executed_complete_pipeline"])

    def test_suffix_end_and_bounds(self):
        row = v6.case(self.saved, self.design["total_tuple_count"] - 1)
        self.assertEqual(row["pair_index"], 1320)
        self.assertEqual(row["pair"]["orientation"], "D18_self_redundant")
        self.assertEqual(row["disposition"], "D18_self_pair_redundant")
        self.assertEqual(row["solver_index"], 51839)
        self.assertEqual(row["residual_index"], 2)
        for ordinal in (-1, self.design["total_tuple_count"]):
            with self.assertRaises(ValueError):
                v6.case(self.saved, ordinal)


if __name__ == "__main__":
    unittest.main()
