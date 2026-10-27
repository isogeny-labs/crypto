"""Independent controls for the sealed confirmation schedule and selector."""

import json
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from unittest import mock

from research.n83_nearby_grid_confirm16_20261024 import aggregate, protocol, verify
from research.n83_nearby_grid_tier1_20261024 import protocol as screen
from research.n83_nearby_grid_tier1_20261024 import run as screen_run


class ConfirmationTests(unittest.TestCase):
    def test_archived_launch_paths_replay_and_tampering_is_rejected(self):
        selected = [1107, 1110, 1115]
        events = protocol.events(0, selected)
        with TemporaryDirectory() as temp:
            original = Path(temp) / "runner" / "n83-confirm-results-s0"
            extracted = Path(temp) / "download" / original.name

            def folder(root, event):
                return (root / "points" / f"p{event['point_index']:02d}" /
                        "runs" / event["name"])

            def save_launch(event, command, flags, cap):
                local = folder(extracted, event)
                local.mkdir(parents=True, exist_ok=True)
                (local / "launch.json").write_text(json.dumps({
                    "command": command, "environment_flags": flags,
                    "cap_seconds": cap,
                    "rss_cap_bytes": protocol.RSS_CAP_BYTES}))
                return local

            first = events[0]
            first_original = folder(original, first)
            first_target = first_original.parent.parent / "target.jsonl"
            first_local = save_launch(first, [
                str(original / "executed_bins/koblitz_orbit_dlp_fast"),
                f"construct:83:1:{first['k']}", str(first_target), "29",
                str(first_original / "targets_result.jsonl")],
                screen_run.ic_flags(first_original, first["k"]),
                protocol.IC_CAP_SECONDS)
            recorded = verify.recorded_output_root(extracted, 0, selected)
            self.assertEqual(recorded, original)
            self.assertTrue(verify.launch_gate(first_local, first, recorded))

            rho = next(event for event in events if event["kind"] == "rho")
            rho_original = folder(original, rho)
            rho_target = rho_original.parent.parent / "target.jsonl"
            config = (Path(temp) / "checkout" /
                      screen.RHO_CONFIG.relative_to(screen.ROOT))
            rho_command = [
                str(original / "executed_bins/n83_rho_optimization_eval"),
                str(rho_target), str(rho["seed"]), str(config),
                str(rho_original / "receipt.json")]
            rho_local = save_launch(rho, rho_command, {},
                                    protocol.RHO_CAP_SECONDS)
            self.assertTrue(verify.launch_gate(rho_local, rho, recorded))

            rho_command[3] = str(Path(temp) / "wrong-rho-config.json")
            save_launch(rho, rho_command, {}, protocol.RHO_CAP_SECONDS)
            self.assertFalse(verify.launch_gate(rho_local, rho, recorded))
            rho_command[3] = str(config)
            rho_command[1] = str(rho_target.with_name("wrong-target.jsonl"))
            save_launch(rho, rho_command, {}, protocol.RHO_CAP_SECONDS)
            self.assertFalse(verify.launch_gate(rho_local, rho, recorded))

    def test_every_public_point_and_workload_replays(self):
        manifest, points = protocol.fixture()
        self.assertEqual(manifest["point_count"], 16)
        self.assertEqual(len(points), 16)
        self.assertEqual(len({item[1] for item in points}), 16)
        self.assertEqual(len({item[3] for item in points}), 16)

    def test_each_point_has_one_pair_per_candidate_and_two_controls(self):
        selected = [1102, 1104, 1107, 1110, 1114, 1119, 1120]
        schedules = [protocol.events(i, selected) for i in range(2)]
        self.assertEqual(len(schedules[0]), 128)
        self.assertEqual(len(schedules[1]), 128)
        self.assertFalse({row["point_index"] for row in schedules[0]} &
                         {row["point_index"] for row in schedules[1]})
        seeds = set()
        for point_index in range(16):
            rows = [row for schedule in schedules for row in schedule
                    if row["point_index"] == point_index]
            self.assertEqual([row["kind"] for row in (rows[0], rows[-1])],
                             ["aa", "aa"])
            for k in selected:
                pair = [row for row in rows if row["k"] == k and
                        row["kind"] in ("ic", "rho")]
                self.assertEqual({row["kind"] for row in pair}, {"ic", "rho"})
                seed = next(row["seed"] for row in pair if row["kind"] == "rho")
                self.assertNotIn(seed, seeds)
                seeds.add(seed)
        self.assertEqual(len(seeds), 16 * len(selected))

    def test_exact_test_and_selection_require_consistent_paired_gain(self):
        self.assertEqual(aggregate.exact_sign_flip_p([1.0] * 16),
                         2 / (1 << 16))
        self.assertEqual(aggregate.exact_sign_flip_p([0.0] * 16), 1.0)
        selected = [1104, 1107, 1120]
        values = {1104: {i: 1.0 for i in range(16)},
                  1107: {i: 2.0 for i in range(16)},
                  1120: {i: 3.0 for i in range(16)}}
        result = aggregate.select_metric(selected, values, "online", 0.05 / 3)
        self.assertEqual(result["selected_k"], 1104)
        values[1107] = {i: (0.9 if i % 2 else 1.1) for i in range(16)}
        result = aggregate.select_metric(selected, values, "online", 0.05 / 3)
        self.assertIsNone(result["selected_k"])

    def test_incomplete_stratum_keeps_aggregate_bounded(self):
        bounded = {"status": "BOUNDED_UNKNOWN", "stratum": 0,
                   "selected_ks": None, "host_instance_id": None,
                   "rows": []}
        with mock.patch.object(aggregate.verify, "verify", return_value=bounded):
            result = aggregate.aggregate(["s0", "s1"], "screen")
        self.assertEqual(result["status"], "BOUNDED_UNKNOWN")
        self.assertIsNone(result["online"])


if __name__ == "__main__":
    unittest.main()
