"""Replay layout admission against the frozen producer's Bloom allocation."""

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("wide3_verify", HERE / "verify.py")
verify = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(verify)


class ReachedArithmeticReplay(Exception):
    """Stop after layout admission without allocating the production tables."""


class BloomAllocationTest(unittest.TestCase):
    def check_layout(self, profile, bloom_bytes):
        k = 3425
        arm = f"root4k{k}{'std' if profile == 'standard' else profile}_a"
        summary = {
            "n": 83, "a": 1, "targets": 0, "targets_solved": 0,
            "targets_failed": 0, "orbit_columns": k, "rank": k,
            "root_table_layout": "indexed83_4byte_exact",
            "root_table_slot_bytes": 4, "root_table_indexed4_enabled": True,
            "root_table_indexed5_enabled": False,
            "root_state_layout": "root_only83_11byte_states_reconstructed_second_root",
            "root_state_slot_bytes": 11,
            "regular_states": 83 * k * (k - 1) // 2 + 42 * k,
            "symmetric_s3_states_enabled": True,
            "indexed5_load75_enabled": False, "indexed5_fixed30_enabled": False,
            "indexed4_fixed31_enabled": True, "root_only11_enabled": True,
            "sparse_canonical_enabled": True, "root_bloom_enabled": True,
            "root_bloom_profile": profile, "root_bloom_hashes": 3,
            "root_bloom_bits_per_request": 16 if profile == "wide3" else 8,
            "root_bloom_bytes": bloom_bytes, "base_hash": "test-only",
            "root_table_entries": 1,
            "root_table_request_entries": 1 << 30,
            "root_table_allocated_bytes": 4 * (1 << 31),
        }
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp)
            folder = output / "runs" / arm
            folder.mkdir(parents=True)
            for name, value in (("usage.json", {"status": "PASS_PRODUCER"}),
                                ("stdout.jsonl", summary),
                                ("base_dump.jsonl", {})):
                (folder / name).write_text(json.dumps(value))
            (folder / "targets_result.jsonl").write_text("")
            with patch.object(verify.group, "replay_base",
                              side_effect=ReachedArithmeticReplay):
                verify.verify_arm(output, arm, k, "root4", profile, [], [])

    def test_standard_one_gib_reaches_arithmetic_replay(self):
        with self.assertRaises(ReachedArithmeticReplay):
            self.check_layout("standard", (1 << 30) * 8 // 8)

    def test_wide3_two_gib_reaches_arithmetic_replay(self):
        # The frozen Rust producer requests 2^30 keys and 16 bits per key.
        with self.assertRaises(ReachedArithmeticReplay):
            self.check_layout("wide3", (1 << 30) * 16 // 8)

    def test_wide3_one_gib_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "source layout"):
            self.check_layout("wide3", 1 << 30)

    def test_standard_two_gib_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "source layout"):
            self.check_layout("standard", 1 << 31)


if __name__ == "__main__":
    unittest.main()
