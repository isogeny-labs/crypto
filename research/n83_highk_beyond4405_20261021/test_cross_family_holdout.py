"""Exercise the frozen cross-family schedule and its admission boundary."""

import unittest
from pathlib import Path
from unittest.mock import patch

from research.n83_highk_beyond4405_20261021 import holdout, verify_holdout


HIGHK_ID = "selected-highk-control"


def rows(highk_times, k1107_times):
    result = []
    for index, (highk_ms, k1107_ms) in enumerate(zip(highk_times, k1107_times)):
        for role, candidate_id, online_ms in (
            ("highk", HIGHK_ID, highk_ms),
            ("k1107", holdout.K1107_CANDIDATE_ID, k1107_ms),
        ):
            result.append({
                "point_index": index,
                "role": role,
                "candidate_id": candidate_id,
                "workload_id": f"sealed-point-{index}",
                "status": "PASS_VERIFIED_ONE_TARGET_TIER1",
                "ic_online_ms": online_ms,
            })
    return result


class CrossFamilyHoldoutTests(unittest.TestCase):
    def test_k1107_runtime_matches_source_bound_manifest(self):
        folder = Path("/tmp/n83_k1107_manifest_gate")
        source = holdout.run.ROOT / "examples/koblitz_orbit_dlp_fast.rs"
        digest = holdout.run.digest
        # Exercise the frozen source contract even after the repository's
        # worker advances. The production gate still hashes the actual file.
        def frozen_digest(path):
            if path == source:
                return "58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8"
            return digest(path)
        with patch.object(holdout.run, "digest", side_effect=frozen_digest):
            env = holdout.ic_environment(folder, holdout.K1107_ARM)
        self.assertEqual(env["KIC_DUMP_BASE"], str(folder / "base_dump.jsonl"))
        self.assertEqual(env["KIC_ROOT_ONLY11"], "1")
        self.assertEqual(env["KIC_INDEXED5_LOAD75"], "1")

    def test_k1107_rejects_changed_source(self):
        source = holdout.run.ROOT / "examples/koblitz_orbit_dlp_fast.rs"
        digest = holdout.run.digest
        with patch.object(holdout.run, "digest", side_effect=lambda path:
                          "0" * 64 if path == source else digest(path)):
            with self.assertRaisesRegex(ValueError, "candidate differs"):
                holdout.ic_environment(Path("/tmp/n83_k1107_changed_source"),
                                       holdout.K1107_ARM)

    def test_schedule_pairs_same_points_and_balances_order(self):
        schedule = holdout.events(HIGHK_ID)
        self.assertEqual(len(schedule), 256)
        self.assertEqual(len({event["name"] for event in schedule}), 256)
        for index in range(16):
            point = schedule[index * 16:(index + 1) * 16]
            self.assertEqual({event["point_index"] for event in point}, {index})
            first_arm = "highk" if index % 2 == 0 else "k1107"
            self.assertEqual(point[0]["arm"], first_arm)
            self.assertEqual(point[8]["arm"],
                             "k1107" if first_arm == "highk" else "highk")
            for arm in ("highk", "k1107"):
                events = [event for event in point if event["arm"] == arm]
                self.assertEqual(len(events), 8)
                self.assertEqual([event["kind"] for event in events[:3]],
                                 ["calibration"] * 3)
                self.assertEqual([event["kind"] for event in events[-3:]],
                                 ["calibration"] * 3)
                self.assertEqual(set(event["kind"] for event in events[3:5]),
                                 {"ic", "rho"})
                expected_id = (HIGHK_ID if arm == "highk" else
                               holdout.K1107_CANDIDATE_ID)
                self.assertEqual({event["candidate_id"] for event in events},
                                 {expected_id})

    def test_clear_pairwise_advantage_selects(self):
        data = rows([100.0] * 16, [200.0] * 16)
        selected = verify_holdout.cross_family_select(data, HIGHK_ID)
        self.assertEqual(selected["status"], "SELECTED")
        self.assertEqual(selected["selected_candidate_id"], HIGHK_ID)
        self.assertEqual(selected["two_sided_exact_sign_p"], "0.000030517578125")

    def test_mixed_or_incomplete_evidence_stays_unknown(self):
        mixed = rows([100.0] * 9 + [200.0] * 7,
                     [200.0] * 9 + [100.0] * 7)
        selected = verify_holdout.cross_family_select(mixed, HIGHK_ID)
        self.assertEqual(selected["status"], "BOUNDED_UNKNOWN")
        self.assertIsNone(selected["selected_candidate_id"])
        mixed[1]["status"] = "PRODUCER_TIMEOUT"
        incomplete = verify_holdout.cross_family_select(mixed, HIGHK_ID)
        self.assertEqual(incomplete["verified_pair_count"], 15)
        self.assertEqual(incomplete["status"], "BOUNDED_UNKNOWN")

    def test_different_workload_or_nonfinite_interval_is_rejected(self):
        data = rows([100.0] * 16, [200.0] * 16)
        data[1]["workload_id"] = "different-point"
        self.assertEqual(verify_holdout.cross_family_select(data, HIGHK_ID)
                         ["status"], "BOUNDED_UNKNOWN")
        data[1]["workload_id"] = data[0]["workload_id"]
        data[0]["ic_online_ms"] = float("nan")
        self.assertEqual(verify_holdout.cross_family_select(data, HIGHK_ID)
                         ["status"], "BOUNDED_UNKNOWN")

    def test_extreme_finite_intervals_do_not_overflow_log_ratio(self):
        data = rows([1e-300] * 16, [1e300] * 16)
        selected = verify_holdout.cross_family_select(data, HIGHK_ID)
        self.assertEqual(selected["status"], "SELECTED")
        self.assertEqual(selected["selected_candidate_id"], HIGHK_ID)


if __name__ == "__main__":
    unittest.main()
