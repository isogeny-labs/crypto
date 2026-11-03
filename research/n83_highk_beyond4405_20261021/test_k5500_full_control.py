"""Process lifecycle regressions; these tests never allocate the K5500 table."""

from pathlib import Path
import signal
import subprocess
import unittest
from unittest.mock import Mock, patch

from research.n83_highk_beyond4405_20261021 import run_k5500_full_control as runner


class SupervisorTests(unittest.TestCase):
    def child(self):
        child = Mock(pid=12345)
        child.poll.return_value = None
        child.wait.return_value = -signal.SIGTERM
        return child

    def test_stalled_rss_sample_is_bounded(self):
        with patch.object(runner.subprocess, "run", side_effect=
                          subprocess.TimeoutExpired("ps", 5)) as sample:
            self.assertIsNone(runner.rss_bytes(12345))
        self.assertEqual(sample.call_args.kwargs["timeout"], 5)

    def test_missing_sample_retires_child_and_records_failure(self):
        child = self.child()
        with patch.object(runner, "rss_bytes", return_value=None), \
             patch.object(runner.time, "monotonic", return_value=10), \
             patch.object(runner.shutil, "disk_usage", return_value=
                          Mock(free=runner.MIN_FREE_BYTES)), \
             patch.object(runner, "terminate_group") as stop:
            code, reason, peak = runner.supervise(child, Path("."), 0)
        self.assertEqual((code, reason, peak),
                         (-signal.SIGTERM, "RSS_SAMPLE_FAILED", 0))
        stop.assert_called_once_with(12345, signal.SIGTERM)
        child.wait.assert_called_once_with(timeout=30)

    def test_monitor_exception_and_interrupt_both_reap_child(self):
        for error in (OSError("monitor failed"), KeyboardInterrupt()):
            with self.subTest(error=type(error).__name__):
                child = self.child()
                with patch.object(runner, "rss_bytes", side_effect=error), \
                     patch.object(runner, "terminate_group") as stop:
                    with self.assertRaises(type(error)):
                        runner.supervise(child, Path("."), 0)
                stop.assert_called_once_with(12345, signal.SIGTERM)
                child.wait.assert_called_once_with(timeout=30)

    def test_term_resistant_child_is_killed_and_reaped(self):
        child = self.child()
        child.wait.side_effect = [subprocess.TimeoutExpired("child", 30), -9]
        with patch.object(runner, "rss_bytes", side_effect=OSError("monitor")), \
             patch.object(runner, "terminate_group") as stop:
            with self.assertRaises(OSError):
                runner.supervise(child, Path("."), 0)
        self.assertEqual([call.args for call in stop.call_args_list],
                         [(12345, signal.SIGTERM), (12345, signal.SIGKILL)])
        self.assertEqual(child.wait.call_count, 2)


if __name__ == "__main__":
    unittest.main()
