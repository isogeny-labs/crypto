"""No-launch controls for the frozen N83 5+1 hosted split."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
from unittest import TestCase, main

import blake3


HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "k600-gray-hash-paired-protocol-20261014"))
sys.path.insert(0, str(HERE.parents[1]))
import receipt_audit as pinned  # noqa: E402
import audit  # noqa: E402
import protocol  # noqa: E402
import paired_frontier_v4 as grid  # noqa: E402


class SplitControls(TestCase):
    def setUp(self) -> None:
        self.plan = json.loads(protocol.OUTPUT.read_text())
        self.arm = self.plan["arms"][0]
        self.config = {
            "schema": "n83.cross-stream-pilot-partial-config/v1",
            "study": "koblitz_n83_factor_base_sweep_20261008",
            "base_object_blake3": self.arm["base_object_blake3"],
            "pool_object_blake3": self.arm["pool_object_blake3"],
            "pilot_manifest_blake3": blake3.blake3(
                pinned.preflight.PINNED_SHA256["manifest"][0].read_bytes()).hexdigest(),
            "source_blake3": blake3.blake3(
                pinned.preflight.PINNED_SHA256["worker_source"][0].read_bytes()).hexdigest(),
            "public_corpus_path": "/corpus.json",
            "public_corpus_canonical_json_blake3": pinned.PRIOR.CORPUS_BLAKE3,
            "budget_seconds": 1800, "conflict_budget": 20000,
            "max_models": 32, "base_summands": 5, "residual_summands": 1,
            "order_first_pair": True, "max_variables": 150000,
            "max_domain_clauses": 12000000,
            "memory_cgroup_limit_bytes": 4294967296,
            "memory_cgroup_swap_limit_bytes": 0,
        }
        shape = self.arm["source_capacity"]
        self.result = {
            "schema": "n83.cross-stream-pilot-partial-result/v1",
            "study": "koblitz_n83_factor_base_sweep_20261008",
            "base_object_blake3": self.arm["base_object_blake3"],
            "pool_object_blake3": self.arm["pool_object_blake3"],
            "base_policy": "public_x_sequential", "base_seed": 2026100802,
            "pool_policy": "public_x_gray_prefix", "pool_seed": 2026100801,
            "public_corpus_canonical_json_blake3": pinned.PRIOR.CORPUS_BLAKE3,
            "base_signed_orbits": 600, "pool_signed_orbits": 600,
            "shared_signed_orbits": self.arm["shared_signed_orbits"],
            "residual_signed_orbits": self.arm["residual_signed_orbits"],
            "trial": 0, "a": self.plan["first_trial_a"],
            "b": self.plan["first_trial_b"],
            "graph_rows": None, "matrix_rank": None,
            "total_index_calculus_runtime_ms": None,
            "selected_best_total_runtime": None,
            "base_import_elapsed_ms_L0": 100, "worker_wall_ms_L0": 1000,
            "memory_cgroup_peak_bytes": 1000000,
            "search": {
                "domain_clauses": shape["domain_clauses"],
                "peak_variables": shape["total_variables"],
                "peak_xors": shape["xor_rows"],
                "peak_clauses": shape["domain_clauses"],
                "conflicts": 20000, "models": 0, "invalid_models": 0,
                "group_rejected_models": 0, "solver_calls": 1,
                "patterns_attempted": 1, "exhausted": True, "refuted": False,
            },
            "status": "UNKNOWN_search_cap", "candidate": None,
        }

    def replay(self, base_points=None, residual=None) -> str:
        return audit.search(self.config, self.result, self.arm,
                            [(1, 2)] if base_points is None else base_points,
                            {(3, 4)} if residual is None else residual,
                            pinned, self.plan)

    def test_frozen_5_plus_1_protocol_and_runner_hash(self) -> None:
        self.assertEqual(protocol.make(), self.plan)
        self.assertEqual(self.plan["schema"], "n83.hosted-sixsum-split-protocol/v1")
        self.assertEqual((self.plan["base_summands"], self.plan["residual_summands"]),
                         (5, 1))
        self.assertEqual(self.plan["pinned_runner_sha256"],
                         protocol.LEGACY_RUNNER_SHA256)
        for arm in self.plan["arms"]:
            self.assertEqual(arm["worker_search_argv"][-3:-1], ["5", "1"])
            self.assertTrue(arm["source_capacity"]["current_worker_cap_pass"])

    def test_v4_grid_addresses_bind_the_split_and_base_streams(self) -> None:
        self.assertEqual(hashlib.sha256(grid.OUTPUT.read_bytes()).hexdigest(),
                         self.plan["grid_v4_design_sha256"])
        design = json.loads(grid.OUTPUT.read_text())
        parent = json.loads(grid.PARENT.read_text())
        for arm in self.plan["arms"]:
            case = grid.case(design, parent, arm["grid_v4_ordinal"])
            self.assertEqual((case["base_summands"], case["residual_summands"]),
                             (5, 1))
            self.assertEqual(case["pair"]["curve_a"], 0)
            self.assertEqual(case["pair"]["base"]["policy"],
                             "public_x_sequential")
            self.assertEqual(case["pair"]["base"]["seed"], 2026100802)
            self.assertEqual(case["pair"]["base"]["orbits"], 600)
            self.assertEqual(case["pair"]["pool"]["policy"],
                             "public_x_gray_prefix" if arm["role"] == "gray_pool"
                             else "public_x_hash")
            self.assertEqual(case["pair"]["pool"]["seed"], 2026100801)
            self.assertEqual(case["pair"]["pool"]["orbits"], 600)
            self.assertEqual(case["solver"], {
                "summands": 6, "split_bits": 0,
                "symmetry": "ordered_first_pair", "backend": "sat_cdcl",
                "enumeration": "binary", "max_large_primes": 1,
                "threads": 1, "linear_algebra": "modular_biguint",
            })
            self.assertEqual(case["disposition"],
                             "pilot_exact_import_gate_available_unexecuted")

    def test_bounded_unknown_is_censored(self) -> None:
        self.assertEqual(self.replay(), "UNKNOWN_search_cap")

    def test_four_plus_two_config_is_rejected(self) -> None:
        self.config["base_summands"] = 4
        self.config["residual_summands"] = 2
        with self.assertRaisesRegex(ValueError, "base_summands"):
            self.replay()

    def test_candidate_requires_five_base_indices(self) -> None:
        self.result["status"] = "PASS_one_bounded_partial"
        self.result["candidate"] = {
            "source_id": 1, "a": self.plan["first_trial_a"],
            "b": self.plan["first_trial_b"],
            "base_point_indices": [0, 0, 0, 0],
            "residual_points": [self.plan["public_target"]],
        }
        with self.assertRaisesRegex(ValueError, "outside exact domains"):
            self.replay()

    def test_five_plus_one_group_equation_and_tamper(self) -> None:
        prior = pinned.PRIOR
        generator = prior.PRIMARY_GENERATOR
        target = tuple(map(int, self.plan["public_target"]))
        a, b = map(int, (self.plan["first_trial_a"], self.plan["first_trial_b"]))
        residual_point = prior.point_add(
            prior.point_mul(generator, (a - 5) % prior.PRIMARY_ORDER),
            prior.point_mul(target, b))
        self.assertIsNotNone(residual_point)
        self.result["status"] = "PASS_one_bounded_partial"
        self.result["search"]["models"] = 1
        self.result["search"]["exhausted"] = False
        self.result["candidate"] = {
            "source_id": 1, "a": self.plan["first_trial_a"],
            "b": self.plan["first_trial_b"],
            "base_point_indices": [0] * 5,
            "residual_points": [[str(word) for word in residual_point]],
        }
        altered = prior.point_add(residual_point, generator)
        self.assertEqual(self.replay([generator], {residual_point, altered}),
                         "PASS_one_bounded_partial")
        self.result["candidate"]["residual_points"] = [
            [str(word) for word in altered]]
        with self.assertRaisesRegex(ValueError, "group equation fails"):
            self.replay([generator], {residual_point, altered})


if __name__ == "__main__":
    main()
