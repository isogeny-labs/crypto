"""Frozen ordinal-prefix and source-disposition checks for the D18 suffix."""

import json
import unittest

import exact_d18_frontier_v5 as v5
import paired_frontier_v4 as v4


class ExactD18FrontierV5Test(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.design = v5.build()
        cls.saved = json.loads(v5.OUTPUT.read_text())

    def test_frozen_file_and_append_only_prefix(self):
        self.assertEqual(v5.v3.canonical_bytes(self.design), v5.OUTPUT.read_bytes())
        self.assertEqual(self.design["v4_tuple_count_preserved"], 26939278080)
        self.assertEqual(self.design["total_tuple_count"], 26939329920)
        for ordinal in (0, 518918399, 518918400, 26939278079):
            with self.subTest(ordinal=ordinal):
                self.assertEqual(
                    v5.case(self.saved, ordinal),
                    v4.case(json.loads(v4.OUTPUT.read_text()),
                            json.loads(v4.PARENT.read_text()), ordinal),
                )

    def test_exact_suffix_and_null_selection(self):
        self.assertEqual(self.design["added_base"]["point_count"], 2760082)
        self.assertEqual(self.design["disposition_counts"]["m5_source_gate_available_screening_pending"], 1)
        self.assertEqual(self.design["disposition_counts"]["bounded_m4_screening_censored_full_pipeline_unmeasured"], 1)
        for index in (0, 12345, 51839):
            row = v5.case(self.saved, 26939278080 + index)
            self.assertEqual(row["solver_index"], index)
            self.assertEqual(row["base"], v5.BASE)
            self.assertIsNone(row["candidate_id"])
            self.assertIsNone(row["selected_best_total_runtime"])
            self.assertFalse(row["executed_complete_pipeline"])

    def test_bounds_fail_closed(self):
        for ordinal in (-1, 26939329920):
            with self.assertRaises(ValueError):
                v5.case(self.saved, ordinal)


if __name__ == "__main__":
    unittest.main()
