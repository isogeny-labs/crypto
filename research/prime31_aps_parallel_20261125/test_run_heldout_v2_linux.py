#!/usr/bin/env python3
"""Exercise Linux child accounting without invoking a V2 solver target."""

import os
import pathlib
import signal
import sys
import tempfile
import unittest

from run_heldout_v2 import invoke


@unittest.skipUnless(sys.platform == "linux", "Linux wait4/affinity control")
class LinuxChildAccounting(unittest.TestCase):
    def run_dummy(self, body, timeout):
        with tempfile.TemporaryDirectory() as directory:
            binary = pathlib.Path(directory) / "dummy"
            binary.write_text("#!/bin/sh\n" + body + "\n")
            binary.chmod(0o755)
            cpus = [min(os.sched_getaffinity(0))]
            return invoke(binary, cpus, 512 * 1024**2, timeout,
                          [578200132, 373493797],
                          {"aps_source_seed_hex": "0123456789abcdef"},
                          "queued_aps")

    def test_completed_child_has_exact_receipts(self):
        result = self.run_dummy("cat", 3)
        self.assertEqual(result["exit_code"], 0)
        self.assertFalse(result["timed_out"])
        self.assertEqual(result["native_report"], result["input"])
        self.assertGreater(result["external_cold_wall_ns"], 0)
        self.assertGreaterEqual(result["child_cpu_ns"], 0)
        self.assertGreater(result["peak_rss_bytes"], 0)

    def test_watchdog_keeps_wait4_cpu_and_rss(self):
        result = self.run_dummy("exec sleep 5", 0.2)
        self.assertEqual(result["exit_code"], -signal.SIGKILL)
        self.assertTrue(result["timed_out"])
        self.assertGreater(result["external_cold_wall_ns"], 0)
        self.assertGreaterEqual(result["child_cpu_ns"], 0)
        self.assertGreater(result["peak_rss_bytes"], 0)


if __name__ == "__main__":
    unittest.main()
