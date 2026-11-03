#!/usr/bin/env python3

import json
import subprocess
import sys
import unittest
from pathlib import Path


HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import audit


class ProgrammeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = json.loads((HERE / "obligations.json").read_text())

    def test_graph_validates(self):
        self.assertEqual(audit.validate(self.data), [])
        self.assertEqual(len(self.data["obligations"]), 6)
        self.assertGreaterEqual(len(self.data["routes"]), 7)

    def test_fixed_arity_bridge(self):
        gate = audit.fixed_arity_gate(
            {"kind": "fixed_arity", "m": 3, "setup_exponent_a": 1.49, "query_exponent_e": 0.49}
        )
        self.assertTrue(gate["passes"])
        boundary = audit.fixed_arity_gate(
            {"kind": "fixed_arity", "m": 3, "setup_exponent_a": 1.5, "query_exponent_e": 0.5}
        )
        self.assertFalse(boundary["passes"])

    def test_fail_closed(self):
        report = audit.score(self.data)
        self.assertFalse(any(route["ready"] for route in report["routes"]))
        pair = next(route for route in report["routes"] if route["id"] == "R-GENERIC-PAIR")
        self.assertEqual(pair["cost_gate"]["total_p_exponent"], 1.0)
        self.assertFalse(pair["cost_gate"]["passes"])

    def test_evidence_recomputes(self):
        result = subprocess.run(
            [sys.executable, str(HERE / "extract_existing.py"), "--check", str(HERE / "evidence-summary.json")],
            check=False,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)


if __name__ == "__main__":
    unittest.main()
