#!/usr/bin/env python3
"""Release the frozen benchmark runner only after its affinity observer arms."""

import argparse
import json
import os
from pathlib import Path
import sys
import time


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--runner-pid-file', type=Path, required=True)
    parser.add_argument('--ready-file', type=Path, required=True)
    parser.add_argument('--observer-pid-file', type=Path, required=True)
    parser.add_argument('--release-receipt', type=Path, required=True)
    parser.add_argument('--manifest', type=Path, required=True)
    parser.add_argument('--queue-root', type=Path, required=True)
    parser.add_argument('--runner', type=Path, required=True)
    parser.add_argument('--timeout-s', type=float, default=100.0)
    args = parser.parse_args()
    if args.runner_pid_file.exists() or args.ready_file.exists():
        parser.error('pid or ready file already exists')
    os.sched_setaffinity(0, {2})
    pid = os.getpid()
    args.runner_pid_file.write_text(str(pid) + '\n')
    deadline = time.monotonic() + args.timeout_s
    while not args.ready_file.exists():
        if time.monotonic() >= deadline:
            raise SystemExit('affinity observer did not arm before timeout')
        try:
            observer_pid = int(args.observer_pid_file.read_text().strip())
            status = Path(f'/proc/{observer_pid}/status').read_text()
            if status.split('State:')[1].split()[0].startswith('Z'):
                raise SystemExit('affinity observer exited before arming')
        except FileNotFoundError:
            raise SystemExit('affinity observer disappeared before arming')
        time.sleep(0.05)
    ready = json.loads(args.ready_file.read_text())
    observer_pid = int(args.observer_pid_file.read_text().strip())
    if ready['runner_pid'] != pid or ready['observer_pid'] != observer_pid:
        raise SystemExit('affinity ready receipt identifies a different process')
    args.release_receipt.write_text(json.dumps({
        'schema': 'rho-worker-affinity-barrier-release-v1',
        'released_at_unix': time.time(),
        'runner_pid': pid,
        'observer_pid': observer_pid,
        'ready_receipt': str(args.ready_file),
        'manifest': str(args.manifest),
    }, indent=2, sort_keys=True) + '\n')
    os.execv(sys.executable, [
        sys.executable, str(args.runner), '--manifest', str(args.manifest),
        '--queue-root', str(args.queue_root),
    ])


if __name__ == '__main__':
    main()
