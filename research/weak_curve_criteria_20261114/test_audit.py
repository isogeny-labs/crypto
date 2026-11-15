import hashlib
import json
import math
from pathlib import Path
import tempfile
import unittest

from audit import bounded_embedding_degree, field_bits, load_and_audit


class CriteriaAuditTest(unittest.TestCase):
    def test_bounded_embedding_is_least_positive_witness(self):
        self.assertEqual(bounded_embedding_degree(5, 13, 4), 4)
        self.assertIsNone(bounded_embedding_degree(5, 13, 3))
        self.assertEqual(bounded_embedding_degree(7, 3, 20), 1)
        with self.assertRaises(ValueError):
            bounded_embedding_degree(6, 3, 20)

    def test_bounded_embedding_matches_independent_power_grid(self):
        for q in range(2, 21):
            for r in range(2, 51):
                if math.gcd(q, r) == 1:
                    expected = next((k for k in range(1, 13) if pow(q, k, r) == 1), None)
                    self.assertEqual(bounded_embedding_degree(q, r, 12), expected)

    def test_field_bit_boundary_for_binary_extensions(self):
        self.assertEqual(field_bits(2**191), 191)
        self.assertEqual(field_bits(2**192), 192)
        self.assertEqual(field_bits(2**252), 252)
        self.assertEqual(field_bits(5), 3)

    def fixture(self, folder: Path, degree=2, model_hash=None):
        model_json = '{"field":"fp-5","p":"5"}'
        registry = {
            "curves": [
                {
                    "slug": "sample",
                    "icv1": "ICV1:sample",
                    "model_json": model_json,
                    "order": "26",
                    "representations": [
                        {
                            "ec1": "EC1:sample",
                            "field": {"characteristic": 5, "degree": 1},
                            "curve": {"subgroup_order": "13", "cofactor": 2},
                        }
                    ],
                }
            ]
        }
        registry_path = folder / "registry.json"
        registry_path.write_text(json.dumps(registry))
        covers = {
            "registry_sha256": hashlib.sha256(registry_path.read_bytes()).hexdigest(),
            "curves": [
                {
                    "slug": "sample",
                    "model_sha256": model_hash or hashlib.sha256(model_json.encode()).hexdigest(),
                    "status": "verified_over_declared_field",
                    "exists": True,
                    "certificate": {"genus": 2, "degree": degree, "construction": "fixture"},
                }
            ],
        }
        cover_path = folder / "covers.json"
        cover_path.write_text(json.dumps(covers))
        transfers = {
            "curves": [
                {
                    "slug": "sample",
                    "icv1": "ICV1:sample",
                    "routes": [
                        {
                            "kind": "ghs_weil_descent",
                            "evidence": "exact_invariant",
                            "transport": "size_admits",
                            "destination": {"genus": "4"},
                        }
                    ],
                }
            ]
        }
        transfer_path = folder / "transfers.json"
        transfer_path.write_text(json.dumps(transfers))
        return registry_path, cover_path, transfer_path

    def test_certificate_and_route_are_distinct(self):
        with tempfile.TemporaryDirectory() as folder:
            summary, records = load_and_audit(*self.fixture(Path(folder)), pairing_bound=4)
            self.assertEqual(records[0]["cover"]["subgroup_gate"], "coprime_degree")
            self.assertEqual(records[0]["pairing"]["first_divisibility_degree"], 4)
            self.assertEqual(summary["enumerated_routes_genus_2_8"][0]["evidence"], "exact_invariant")
            self.assertEqual(summary["certified_same_field_models_by_genus"], {"2": 1})

    def test_noncoprime_degree_is_unresolved(self):
        with tempfile.TemporaryDirectory() as folder:
            _, records = load_and_audit(*self.fixture(Path(folder), degree=13))
            self.assertEqual(records[0]["cover"]["subgroup_gate"], "unresolved")

    def test_model_binding_rejects_tampering(self):
        with tempfile.TemporaryDirectory() as folder:
            with self.assertRaisesRegex(ValueError, "model hash mismatch"):
                load_and_audit(*self.fixture(Path(folder), model_hash="0" * 64))

    def test_registry_binding_rejects_tampering(self):
        with tempfile.TemporaryDirectory() as folder:
            paths = self.fixture(Path(folder))
            with paths[0].open("a") as out:
                out.write(" ")
            with self.assertRaisesRegex(ValueError, "different registry"):
                load_and_audit(*paths)

    def test_selected_order_must_match_ambient_order(self):
        with tempfile.TemporaryDirectory() as folder:
            paths = self.fixture(Path(folder))
            registry = json.loads(paths[0].read_text())
            registry["curves"][0]["order"] = "25"
            paths[0].write_text(json.dumps(registry))
            covers = json.loads(paths[1].read_text())
            covers["registry_sha256"] = hashlib.sha256(paths[0].read_bytes()).hexdigest()
            paths[1].write_text(json.dumps(covers))
            with self.assertRaisesRegex(ValueError, "selected subgroup/order mismatch"):
                load_and_audit(*paths)

    def test_selected_field_must_match_curve_model(self):
        with tempfile.TemporaryDirectory() as folder:
            paths = self.fixture(Path(folder))
            registry = json.loads(paths[0].read_text())
            registry["curves"][0]["representations"][0]["field"]["characteristic"] = 7
            paths[0].write_text(json.dumps(registry))
            covers = json.loads(paths[1].read_text())
            covers["registry_sha256"] = hashlib.sha256(paths[0].read_bytes()).hexdigest()
            paths[1].write_text(json.dumps(covers))
            with self.assertRaisesRegex(ValueError, "selected subgroup/field mismatch"):
                load_and_audit(*paths)


if __name__ == "__main__":
    unittest.main()
