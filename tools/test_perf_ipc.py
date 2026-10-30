#!/usr/bin/env python3
"""Exercise counter availability and exit statuses without a hardware PMU."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).with_name('perf_ipc.sh')
FAKE_PERF = r'''
import os, pathlib, subprocess, sys
args = sys.argv[1:]
if args == ['--version']:
    print('perf version controlled-fixture')
    sys.exit(0)
mode = os.environ['COUNTER_TEST_MODE']
probe = '-o' not in args
if probe and mode == 'permission':
    print('No permission to enable cycles:u event', file=sys.stderr)
    sys.exit(255)
if probe and mode in ('unsupported', 'probe_not_counted', 'software'):
    marker = '<not supported>' if mode != 'probe_not_counted' else '<not counted>'
    print(marker + ',,cycles:u,0,100.00,', file=sys.stderr)
    sys.exit(0)
if probe:
    print('100,,cycles:u,100,100.00,\n200,,instructions:u,100,100.00,', file=sys.stderr)
    sys.exit(0)
software = args[args.index('-e') + 1].startswith('task-clock')
count = '<not counted>' if mode == 'run_not_counted' else ('0' if mode == 'zero_cycles' else '100')
pct = '50.00' if mode in ('multiplexed', 'failed_multiplexed') else '100.00'
rows = ['0.5,msec,task-clock,100,100.00,']
if not software:
    rows += [f'{count},,cycles:u,100,{pct},']
    if mode != 'missing_instructions':
        rows += [f'200,,instructions:u,100,{pct},', '20,,branches:u,100,100.00,', '2,,branch-misses:u,100,100.00,']
pathlib.Path(args[args.index('-o') + 1]).write_text('\n'.join(rows) + '\n')
if mode.startswith('failed'):
    sys.exit(7)
sys.exit(subprocess.call(args[args.index('--') + 1:]))
'''


class CounterReporting(unittest.TestCase):
    def run_mode(self, mode, *options):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            perf = root / 'perf'
            perf.write_text(f'#!{sys.executable}\n' + FAKE_PERF)
            perf.chmod(0o755)
            # Host metadata is orthogonal to the counter parser; make it portable.
            for name, body in {
                'nproc': 'echo 4',
                'ls': 'case "$*" in /sys/bus/event_source/devices) echo software;; *) exec /bin/ls "$@";; esac',
            }.items():
                command = root / name
                command.write_text('#!/bin/sh\n' + body + '\n')
                command.chmod(0o755)
            env = dict(os.environ, PERF=str(perf), COUNTER_TEST_MODE=mode,
                       PATH=str(root) + os.pathsep + os.environ['PATH'])
            proc = subprocess.run(['bash', str(SCRIPT), '--repeat', '1', *options,
                                   '--', '/usr/bin/true'], env=env,
                                  text=True, capture_output=True, check=False)
            self.assertNotEqual(proc.stdout.strip(), '', proc.stderr)
            records = [json.loads(line) for line in proc.stdout.splitlines()]
            return proc, records

    def test_measured_counts(self):
        proc, rows = self.run_mode('counted', '--steps', '10')
        self.assertEqual(proc.returncode, 0, proc.stderr)
        summary = rows[-1]
        self.assertEqual(summary['ipc'], {'median': 2, 'min': 2, 'max': 2, 'n': 1})
        self.assertEqual(summary['per_step']['cycles']['median'], 10)

    def test_unavailable_probe(self):
        for mode in ('unsupported', 'probe_not_counted', 'permission'):
            with self.subTest(mode=mode):
                proc, rows = self.run_mode(mode)
                self.assertEqual(proc.returncode, 3, proc.stderr)
                self.assertEqual([r['kind'] for r in rows], ['unsupported'])
                self.assertTrue(rows[0]['reason'])

    def test_unusable_counts_after_successful_probe(self):
        for mode in ('run_not_counted', 'zero_cycles', 'missing_instructions'):
            with self.subTest(mode=mode):
                proc, rows = self.run_mode(mode)
                self.assertEqual(proc.returncode, 3, proc.stderr)
                self.assertEqual([r['kind'] for r in rows], ['run', 'summary', 'unsupported'])
                self.assertIsNone(rows[0]['ipc'])
                self.assertEqual(rows[1]['ipc']['n'], 0)
                self.assertTrue(rows[2]['reason'])

    def test_command_failures_remain_failures(self):
        for mode in ('failed', 'failed_multiplexed'):
            with self.subTest(mode=mode):
                proc, rows = self.run_mode(mode)
                self.assertEqual(proc.returncode, 4, proc.stderr)
                self.assertEqual(rows[-1]['failed_runs'], 1)

    def test_multiplexing_is_not_success(self):
        proc, rows = self.run_mode('multiplexed')
        self.assertEqual(proc.returncode, 5, proc.stderr)
        self.assertEqual(rows[-1]['multiplexed_runs'], 1)

    def test_software_selftest_does_not_claim_ipc(self):
        proc, rows = self.run_mode('software', '--selftest')
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(rows[-1]['task_clock_ms']['n'], 1)
        self.assertEqual(rows[-1]['ipc']['n'], 0)


if __name__ == '__main__':
    unittest.main()
