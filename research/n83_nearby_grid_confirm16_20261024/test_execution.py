"""Independent controls for the sealed confirmation schedule and selector."""

import unittest
from unittest import mock

from research.n83_nearby_grid_confirm16_20261024 import aggregate, protocol


class ConfirmationTests(unittest.TestCase):
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
