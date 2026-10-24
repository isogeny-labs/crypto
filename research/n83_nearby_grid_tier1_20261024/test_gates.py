"""Regression checks for the hosted experiment's fail-closed gates."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from research.n83_nearby_grid_tier1_20261024 import protocol, run, verify


class GateTests(unittest.TestCase):
    def test_rho_noise_controls_repeat_work_with_independent_rounds(self):
        events = protocol.events()
        seeds = []
        size = protocol.BLOCK_SIZE + 4
        for index in range(len(events) // size):
            block = events[size * index:size * (index + 1)]
            before, after = block[0], block[-1]
            self.assertEqual(before["kind"], "rho")
            self.assertEqual(after["kind"], "rho")
            self.assertEqual(before["seed"], after["seed"])
            seeds.append(before["seed"])
        self.assertEqual(len(set(seeds)), len(events) // size)

    def test_independent_point_is_outside_fixed31_and_prior_grid(self):
        _line, point, _scalar = run.fixture()
        reserved = {tuple(int(x) for x in json.loads(line)) for line in
                    (protocol.ROOT /
                     "research/n83_highk_beyond4405_20261021/holdout_targets.jsonl"
                     ).read_text().splitlines()}
        self.assertEqual(len(reserved), 16)
        self.assertNotIn(point, reserved)
        prior = tuple(int(x) for x in json.loads((protocol.ROOT /
            "research/n83_nearby_k_github_tier1_20261021/target.jsonl"
            ).read_text()))
        self.assertNotEqual(point, prior)
        audit = json.loads(protocol.EXCLUSION_AUDIT.read_text())
        self.assertEqual(audit["status"], "PASS_ZERO_OVERLAP")
        self.assertEqual(audit["overlap_count"], 0)

    def test_schedule_covers_all_19_bases_in_opposite_orders(self):
        events = protocol.events()
        size = protocol.BLOCK_SIZE + 4
        blocks_per_round = (len(protocol.CANDIDATES) - 1) // protocol.BLOCK_SIZE
        self.assertEqual(len(events), size * blocks_per_round * protocol.ROUNDS)
        orders = []
        for index in range(protocol.ROUNDS):
            order = []
            for block_index in range(blocks_per_round):
                offset = size * (index * blocks_per_round + block_index)
                block = events[offset:offset + size]
                self.assertEqual(block[0]["kind"], "rho")
                self.assertEqual(block[-1]["kind"], "rho")
                self.assertTrue(all(row["kind"] == "ic" for row in block[1:-1]))
                ks = [row["k"] for row in block[1:-1]]
                self.assertEqual(ks[0], protocol.BASELINE_K)
                self.assertEqual(ks[-1], protocol.BASELINE_K)
                self.assertEqual(len(ks), size - 2)
                order.extend(ks[1:-1])
            self.assertEqual(set(order),
                             set(protocol.CANDIDATES) - {protocol.BASELINE_K})
            self.assertEqual(len(order), len(protocol.CANDIDATES) - 1)
            orders.append(order)
        self.assertEqual(orders[0], list(reversed(orders[1])))

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

    def test_block_selector_uses_paired_cold_ratios_and_fails_on_noise(self):
        schedule = protocol.events()
        children = []
        for event in schedule:
            if event["kind"] == "rho":
                online, cold = 1000.0, 1000.0
            elif event["k"] == protocol.BASELINE_K:
                online, cold = 100.0, 100000.0
            else:
                online = 90.0 + (event["k"] - 1102) / 10
                cold = 90000.0 + 100 * (event["k"] - 1102)
            children.append({"event": event, "status": "PASS_INDEPENDENT_REPLAY",
                             "resources": {"status": "PASS"},
                             "online_ms": online, "cold_process_ms": cold})
        blocks, noise, _online, _cold, online_ratios, cold_ratios, status = \
            verify.summarize_blocks(children, schedule, True)
        self.assertEqual(status, "PASS_TIER1_GRID_SCREEN")
        self.assertTrue(noise)
        self.assertEqual(len(blocks), 12)
        self.assertEqual(min(cold_ratios, key=cold_ratios.get), "1102")
        self.assertEqual(min(online_ratios, key=online_ratios.get), "1102")
        target = next(row for row in children if row["event"]["name"] ==
                      "r00b00_baseline_after")
        target["cold_process_ms"] = 160000.0
        blocks, noise, _online, _cold, online_ratios, cold_ratios, status = \
            verify.summarize_blocks(children, schedule, True)
        self.assertEqual(blocks[0]["status"], "NOISY_BLOCK")
        self.assertFalse(noise)
        self.assertEqual(status, "BOUNDED_UNKNOWN")
        self.assertIsNone(cold_ratios)


if __name__ == "__main__":
    unittest.main()
