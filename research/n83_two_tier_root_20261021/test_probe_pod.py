#!/usr/bin/env python3
"""Check the Tier 1 CPU and memory admission threshold."""

import unittest

from research.n83_two_tier_root_20261021 import probe_pod


class ResourceGate(unittest.TestCase):
    def test_four_cpus_and_forty_eight_gib_pass(self):
        snapshot = {"cpu_max": "400000 100000",
                    "memory_max": str(48 * probe_pod.GIB)}
        result = probe_pod.gate(snapshot, 64 * probe_pod.GIB,
                                [0, 1, 2, 3])
        self.assertEqual(result["status"], "PASS_CAPACITY_PREFLIGHT")
        self.assertEqual(result["frozen_online_affinity"], [0, 1, 2, 3])

    def test_cpu_and_memory_deficits_fail(self):
        snapshot = {"cpu_max": "300000 100000",
                    "memory_max": str(48 * probe_pod.GIB)}
        self.assertEqual(probe_pod.gate(snapshot, 64 * probe_pod.GIB,
                                        [0, 1, 2, 3])["status"],
                         "REJECTED_HOST_CAPACITY")
        snapshot["cpu_max"] = "400000 100000"
        snapshot["memory_max"] = str(47 * probe_pod.GIB)
        self.assertEqual(probe_pod.gate(snapshot, 64 * probe_pod.GIB,
                                        [0, 1, 2, 3])["status"],
                         "REJECTED_HOST_CAPACITY")


if __name__ == "__main__":
    unittest.main()
