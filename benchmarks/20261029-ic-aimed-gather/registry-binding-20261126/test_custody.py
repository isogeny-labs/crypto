#!/usr/bin/env python3
"""Controls for the limited catalog-provenance exception, without curve arithmetic."""
import copy
import unittest
from refresh_custody import TARGET, target_comparison


class Custody(unittest.TestCase):
    def setUp(self):
        self.before = {"curves": [{"slug": TARGET, "field": {"p": "123"},
                                   "sources": ["before"], "representations": [
                                       {"curve": {"cofactor": 1}, "sources": ["before"]}]}]}
        self.after = copy.deepcopy(self.before)
        self.after["curves"][0]["sources"] = ["after"]

    def test_source_delta_is_recorded(self):
        complete, provenance = target_comparison(self.before, self.after)
        self.assertFalse(complete)
        self.assertEqual(provenance[0]["sources"], ["before"])
        self.assertEqual(provenance[1]["sources"], ["after"])

    def test_unchanged_complete_target(self):
        self.assertTrue(target_comparison(self.before, self.before)[0])

    def test_parameter_change_rejected(self):
        self.after["curves"][0]["field"]["p"] = "124"
        with self.assertRaisesRegex(ValueError, "beyond allowed"):
            target_comparison(self.before, self.after)

    def test_boolean_type_drift_rejected(self):
        self.after["curves"][0]["representations"][0]["curve"]["cofactor"] = True
        with self.assertRaisesRegex(ValueError, "beyond allowed"):
            target_comparison(self.before, self.after)

    def test_duplicate_target_rejected(self):
        self.after["curves"].append(copy.deepcopy(self.after["curves"][0]))
        with self.assertRaisesRegex(ValueError, "not unique"):
            target_comparison(self.before, self.after)

    def test_non_string_provenance_rejected(self):
        self.after["curves"][0]["sources"] = [1]
        with self.assertRaisesRegex(ValueError, "string lists"):
            target_comparison(self.before, self.after)


if __name__ == "__main__":
    unittest.main()
