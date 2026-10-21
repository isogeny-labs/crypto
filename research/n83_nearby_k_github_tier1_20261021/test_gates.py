"""Regression checks for the hosted experiment's fail-closed gates."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from research.n83_nearby_k_github_tier1_20261021 import protocol, run, verify


class GateTests(unittest.TestCase):
    def test_schedule_is_paired_and_alternates_challengers(self):
        events = protocol.events()
        self.assertEqual(len(events), 30)
        for index in range(5):
            block = events[6 * index:6 * index + 6]
            self.assertEqual([row["kind"] for row in block],
                             ["rho", "ic", "ic", "ic", "ic", "rho"])
            self.assertEqual([row["k"] for row in block if row["kind"] == "ic"],
                             [1107, 1104, 1120, 1107] if index % 2 == 0
                             else [1107, 1120, 1104, 1107])

    def test_host_gate_rejects_small_memory_and_changed_revision(self):
        host = {"provider": "github_actions", "git_sha": "a" * 40,
                "visible_affinity": [0, 1], "mem_total_kib": 8 * 1024**2,
                "disk_free_bytes": 8 * 1024**3,
                "cgroup_before": {"memory.max": str(6 * 1024**3),
                                  "cpu.stat": "nr_throttled 0\n",
                                  "memory.events": "oom 0\n"}}
        self.assertTrue(run.host_gate(host, require_current_run=False))
        host["cgroup_before"]["memory.max"] = str(4 * 1024**3)
        self.assertFalse(run.host_gate(host, require_current_run=False))
        host["cgroup_before"]["memory.max"] = str(6 * 1024**3)
        with patch.dict("os.environ", {"GITHUB_SHA": "b" * 40}):
            self.assertFalse(run.host_gate(host))

    def test_rho_replay_rejects_forged_scalar(self):
        point_line, point, scalar = run.fixture()
        with tempfile.TemporaryDirectory() as temporary:
            folder = Path(temporary)
            target = folder / "target.jsonl"
            target.write_text(point_line)
            event = {"seed": protocol.RHO_SEED_BASE}
            receipt = {"status": "PASS_VERIFIED_PUBLIC_TARGET",
                       "public_target_q": json.loads(point_line),
                       "recovered_scalar": str(scalar),
                       "walk_seed": event["seed"], "target_count": 1,
                       "known_answer_input": False,
                       "validation_sidecar_opened": False,
                       "group_verified": True,
                       "generic_scalar_replay_verified": True,
                       "public_input_sha256": verify.parent.digest(target),
                       "compiled_source_sha256": {
                           "examples/n83_rho_optimization_eval.rs":
                           protocol.RHO_SOURCE_SHA256},
                       "online_ms": 1.0}
            (folder / "receipt.json").write_text(json.dumps(receipt))
            self.assertEqual(verify.replay_rho(folder, event, point, scalar,
                                               target)["status"],
                             "PASS_INDEPENDENT_REPLAY")
            receipt["recovered_scalar"] = str(scalar + 1)
            (folder / "receipt.json").write_text(json.dumps(receipt))
            with self.assertRaises(ValueError):
                verify.replay_rho(folder, event, point, scalar, target)


if __name__ == "__main__":
    unittest.main()
