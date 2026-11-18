#!/usr/bin/env python3
"""Contract checks for the hourly relation-yield telemetry parser."""
import unittest

from hourly_relation_finding import compare, percent_change, verified_metrics


def fixture():
    return {
        "schema": "koblitz_relation_yield_bridge.v1",
        "status": "complete",
        "production_defaults_used": True,
        "frozen_policy": {"n": 23, "a": 0, "m": 2},
        "factor_base": {"factor_base_blake3": "fixed-base"},
        "arms": {
            "natural": {"count": 256, "hits": 32, "verified_point_witnesses": 32},
            "planted_sat": {"count": 64, "hits": 64, "verified_point_witnesses": 64},
            "proven_unsat": {"count": 64, "hits": 0, "verified_point_witnesses": 0},
        },
    }


class TelemetryTests(unittest.TestCase):
    def test_verified_metric_definitions(self):
        measured = verified_metrics(fixture(), 8.0)
        self.assertEqual(measured["natural_relations"], 32)
        self.assertEqual(measured["natural_trials"], 256)
        self.assertEqual(measured["relations_per_sec"], 4.0)
        self.assertEqual(measured["success_rate"], 0.125)
        self.assertEqual(measured["planted_verified"], 64)
        self.assertEqual(measured["unsat_hits"], 0)

    def test_reject_smoke_results_and_wrong_curve(self):
        data = fixture()
        data["production_defaults_used"] = False
        with self.assertRaises(ValueError):
            verified_metrics(data, 1.0)
        data = fixture()
        data["frozen_policy"]["n"] = 131
        with self.assertRaises(ValueError):
            verified_metrics(data, 1.0)

    def test_reject_invalid_control_or_witness(self):
        for arm, field, value in [
            ("natural", "verified_point_witnesses", 31),
            ("planted_sat", "hits", 63),
            ("proven_unsat", "hits", 1),
        ]:
            data = fixture()
            data["arms"][arm][field] = value
            with self.subTest(arm=arm), self.assertRaises(ValueError):
                verified_metrics(data, 1.0)

    def test_threshold_and_identity(self):
        m = verified_metrics(fixture(), 8.0)
        baseline = {
            "curve_n": 23, "curve_a": 0, "factor_base_hash": "fixed-base",
            "relations_per_sec": 4.0, "success_rate": 0.125,
        }
        self.assertEqual(compare(m, baseline), [])
        m["relations_per_sec"] = 4.8
        self.assertEqual(compare(m, baseline), [])
        m["relations_per_sec"] = 4.81
        self.assertEqual(len(compare(m, baseline)), 1)
        m["factor_base_hash"] = "other"
        self.assertIn("identity", compare(m, baseline)[0])

    def test_zero_baseline(self):
        self.assertEqual(percent_change(0, 0), 0.0)
        self.assertEqual(percent_change(1, 0), float("inf"))


if __name__ == "__main__":
    unittest.main()
