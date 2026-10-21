"""Check the native v2 counter gate used by the conditional Pod runner."""

import unittest

from research.n83_highk_online_distribution_20261020 import verify_common


class CgroupV2Test(unittest.TestCase):
    def snapshot(self, throttled=0, oom=0):
        return {
            "cgroup_interface": "v2", "cpu.max": "680000 100000\n",
            "cpu.stat": ("usage_usec 1000\nnr_periods 10\n"
                         f"nr_throttled {throttled}\n"),
            "memory.current": str(100 * 1024**2),
            "memory.peak": str(200 * 1024**2),
            "memory.max": str(30 * 1000**3),
            "memory.events": f"low 0\nhigh 0\nmax 0\noom {oom}\noom_kill 0\n",
        }

    def test_valid_v2_receipts_admit_zero_resource_deltas(self):
        before, after = self.snapshot(), self.snapshot()
        self.assertTrue(verify_common.snapshot_consistent(before))
        usage = {"status": "PASS_PRODUCER", "peak_rss_bytes": 1000,
                 "cgroup_before": before, "cgroup_after": after}
        self.assertEqual(verify_common.resources(usage)["status"], "PASS")

    def test_throttling_oom_and_missing_quota_fail(self):
        before = self.snapshot()
        for after in (self.snapshot(throttled=1), self.snapshot(oom=1)):
            usage = {"status": "PASS_PRODUCER", "peak_rss_bytes": 1000,
                     "cgroup_before": before, "cgroup_after": after}
            self.assertEqual(verify_common.resources(usage)["status"],
                             "FAILED_OR_INCOMPLETE")
        bad = self.snapshot()
        bad["cpu.max"] = "max 100000\n"
        self.assertFalse(verify_common.snapshot_consistent(bad))


if __name__ == "__main__":
    unittest.main()
