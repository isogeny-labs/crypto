"""Contract tests for the full campaign's draft and launch preflight."""

import copy
import importlib.util
import unittest
from pathlib import Path

SCRIPT = Path(__file__).resolve().parents[1] / "tools/ml_dataset_full_protocol_check.py"
SPEC = importlib.util.spec_from_file_location("full_protocol_check", SCRIPT)
checker = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checker)


def draft():
    parameters = {
        "families": sorted(checker.FAMILIES), "n": sorted(checker.N),
        "m": sorted(checker.M), "solvers": sorted(checker.SOLVERS),
        "curve_seeds": 16, "generators": 8, "keys": 64,
        "twists": True, "population": "natural",
    }
    reference = copy.deepcopy(parameters)
    reference["solvers"] = ["bsgs"]
    return {
        "schema": "experiment.protocol/v1", "repository": "crypto",
        "arms": [
            {"id": "dataset", "parameters": parameters},
            {"id": "reference-bsgs", "parameters": reference},
        ],
        "sample_size": {"instances": 16515072, "replicates": 1},
        "claims": [{"id": name, "class": cls} for name, cls in (
            ("C-integrity", "exact_computation"), ("C-coverage", "exact_computation"),
            ("C-falls", "out_of_scope"), ("C-model", "out_of_scope"),
            ("C-speed", "out_of_scope"),
        )],
        "controls": {name: {"present": True} for name in ("aa", "positive", "negative")},
        "metrics": {"primary": [{"id": name, "kind": "exact"}
                                for name in ("verified_claims", "coverage")]},
        "measurement": {"clock": "counted"},
        "execution": {"measurement_class": "throughput", "checkpoint": {
            "after_restore": {"wall_time": "invalid", "counted_units": "valid"}}},
        "randomization": {"instance_sampling": {"independent_of_arm": True}},
        "freeze": {"status": "draft"},
    }


class FullProtocolCheckTest(unittest.TestCase):
    def test_full_draft_axes(self):
        self.assertEqual(checker.validate(draft()), [])

    def test_missing_twist_or_method_rejected(self):
        p = draft()
        p["arms"][0]["parameters"]["twists"] = False
        p["arms"][0]["parameters"]["solvers"].remove("double-large-prime")
        errors = checker.validate(p)
        self.assertTrue(any("twists" in x for x in errors))
        self.assertTrue(any("solvers" in x for x in errors))

    def test_duplicate_axis_and_malformed_json_type_rejected_without_crash(self):
        p = draft()
        p["arms"][0]["parameters"]["n"][0] = p["arms"][0]["parameters"]["n"][1]
        self.assertTrue(any("dataset.n" in x for x in checker.validate(p)))
        p["arms"][0]["parameters"]["n"][0] = {}
        self.assertTrue(any("dataset.n" in x for x in checker.validate(p)))

    def test_claim_and_restore_semantics_rejected(self):
        p = draft()
        p["claims"][0]["class"] = "comparative"
        p["execution"]["checkpoint"]["after_restore"]["wall_time"] = "valid"
        errors = checker.validate(p)
        self.assertTrue(any("C-integrity" in x for x in errors))
        self.assertTrue(any("wall_time" in x for x in errors))

    def test_admission_requires_source_analysis_image_grant_budget(self):
        errors = checker.validate(draft(), admit=True)
        for field in ("instrument.commit", "analysis_plan.script.sha256", "wave_admission.image",
                      "wave_admission.worker_grant", "wave_admission.budget", "freeze.status"):
            self.assertTrue(any(field in x for x in errors), field)


if __name__ == "__main__":
    unittest.main()
