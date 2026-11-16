import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("n83_pattern_runner", HERE / "run_panel.py")
RUNNER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNNER)


class FrozenPanelTests(unittest.TestCase):
    def setUp(self):
        self.protocol = json.loads((HERE / "protocol.json").read_text())

    def test_all_sixteen_formula_addresses_and_four_host_strata_are_frozen(self):
        RUNNER.frozen_protocol(self.protocol)
        self.assertEqual(len({row["sha256"] for row in
                              self.protocol["formulas"].values()}), 16)
        self.assertEqual(sum(len(row["arms"]) for row in
                             self.protocol["strata"]), 24)
        for stratum in self.protocol["strata"]:
            self.assertEqual(len({arm["formula"] for arm in
                                  stratum["arms"]}), 4)

    def test_changed_arm_order_or_cap_fails_before_search(self):
        changed = copy.deepcopy(self.protocol)
        changed["strata"][0]["arms"][2]["formula"] = \
            changed["strata"][0]["arms"][3]["formula"]
        with self.assertRaisesRegex(ValueError, "frozen six-arm order"):
            RUNNER.frozen_protocol(changed)
        changed = copy.deepcopy(self.protocol)
        changed["solver_time_cap_seconds"] = 600
        with self.assertRaisesRegex(ValueError, "frozen solver bounds"):
            RUNNER.frozen_protocol(changed)

    def test_export_gate_rejects_changed_trial_and_formula_bytes(self):
        spec = copy.deepcopy(self.protocol["formulas"]["hash_4_2_t1_m1"])
        with tempfile.TemporaryDirectory() as directory:
            xcnf = Path(directory) / "instance.xcnf"
            xcnf.write_bytes(b"p cnf 1 0\n")
            spec["bytes"] = xcnf.stat().st_size
            spec["sha256"] = RUNNER.file_sha(xcnf)
            metadata = {
                "schema": "n83.cross-stream-pattern-xcnf/v2",
                "study": "koblitz_n83_factor_base_sweep_20261008",
                "curve_slug": self.protocol["curve"],
                "base_object_blake3": self.protocol["base_object_blake3"],
                "pool_object_blake3": self.protocol["pool_objects_blake3"]["hash"],
                "trial": 1, "source_id": 2, "identity_mask": 1,
                "order_first_pair": True, "base_summands": 4,
                "residual_summands": 2, "coef_a": spec["coef_a"],
                "coef_b": spec["coef_b"], "variables": spec["variables"],
                "original_clauses": spec["original_clauses"],
                "xor_rows": spec["xor_rows"], "xcnf_bytes": spec["bytes"],
                "selected_best_total_runtime": None,
            }
            RUNNER.check_export_metadata(metadata, spec, self.protocol, xcnf)
            changed = dict(metadata, trial=2)
            with self.assertRaisesRegex(ValueError, "exact fresh-trial"):
                RUNNER.check_export_metadata(changed, spec, self.protocol, xcnf)
            xcnf.write_bytes(b"p cnf 1 1\n1 0\n")
            with self.assertRaisesRegex(ValueError, "exact fresh-trial"):
                RUNNER.check_export_metadata(metadata, spec, self.protocol, xcnf)


if __name__ == "__main__":
    unittest.main()
