#!/usr/bin/env python3
"""Check that separate runner processes contend on one queue-root lock."""

import argparse
import fcntl
from hashlib import sha256
import json
from pathlib import Path
import subprocess
import sys
import tempfile


RUNNER = Path(__file__).with_name('run_tier1_worker_batch.py')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='rho-worker-lock-check-') as temp:
        root = Path(temp)
        manifest = root / 'manifest.json'
        manifest.write_text('{}\n')
        command = [sys.executable, str(RUNNER), '--manifest', str(manifest),
                   '--queue-root', str(root)]
        with (root / '.rho-worker-panel.lock').open('a+') as held:
            fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
            blocked = subprocess.run(command, capture_output=True, text=True)
        released = subprocess.run(command, capture_output=True, text=True)
        if blocked.returncode == 0 or 'BlockingIOError' not in blocked.stderr:
            raise AssertionError('second runner did not reject occupied queue lock')
        if (released.returncode == 0 or
                'expected frozen Tier 1 manifest' not in released.stderr):
            raise AssertionError('runner did not enter manifest validation after release')
        if (root / 'results').exists():
            raise AssertionError('lock check created a measurement job')
    result = {
        'schema': 'rho-worker-shared-lock-process-check-v1',
        'data_kind': 'local_lock_behavior_only',
        'runner_sha256': sha256(RUNNER.read_bytes()).hexdigest(),
        'same_queue_root_concurrent_invocation': 'rejected_by_flock',
        'after_release': 'entered_manifest_validation',
        'measurement_job_created': False,
        'performance_result': None,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'status': 'pass', 'output': str(args.output)}))


if __name__ == '__main__':
    main()
