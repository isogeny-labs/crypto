#!/usr/bin/env python3
"""Independent Pod-side expiry path, using RunPod's scoped Pod credentials."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import time


def guard(allocation_path, plan_path, receipt, clock=None, pause=time.sleep,
          remove=None):
    clock = clock or (lambda: datetime.now(timezone.utc))
    remove = remove or (lambda pod: subprocess.run(
        ['runpodctl', 'pod', 'remove', pod], capture_output=True,
        timeout=30, check=True))
    allocation = json.loads(allocation_path.read_text())
    plan = json.loads(plan_path.read_text())
    index = allocation.get('stratum')
    names = plan.get('names')
    if (plan.get('schema') != 'n83.crossfamily-current-allocation-plan/v1' or
            allocation.get('schema') != 'n83.crossfamily-current-runpod-allocation/v1' or
            allocation.get('source_head_sha') != plan.get('source_head_sha') or
            allocation.get('source_archive_sha256') != plan.get('source_archive_sha256') or
            allocation.get('source_manifest_sha256') != plan.get('source_manifest_sha256') or
            not isinstance(allocation.get('pod_id'), str) or
            not allocation['pod_id'].isalnum() or
            index not in (0, 1) or not isinstance(names, list) or len(names) != 2 or
            names[index] != f"n83-crossfamily-{plan['source_head_sha'][:12]}-s{index}"):
        raise ValueError('Pod deadline identity differs')
    deadline = datetime.fromisoformat(plan['terminate_at_utc'].replace('Z', '+00:00'))
    if deadline.tzinfo is None:
        raise ValueError('Pod deadline is timezone naive')
    receipt.write_text(json.dumps({'status': 'ARMED',
                                   'terminate_at_utc': plan['terminate_at_utc']}) + '\n')
    while clock() < deadline:
        pause(min(60, max(0.1, (deadline - clock()).total_seconds())))
    while True:
        try:
            remove(allocation['pod_id'])
            receipt.write_text(json.dumps({'status': 'TERMINATION_REQUESTED',
                                           'recorded_utc': clock().isoformat()}) + '\n')
            return
        except Exception as error:
            receipt.write_text(json.dumps({'status': 'RETRYING_TERMINATION',
                                           'error_type': type(error).__name__,
                                           'recorded_utc': clock().isoformat()}) + '\n')
            pause(30)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--allocation', type=Path, required=True)
    parser.add_argument('--plan', type=Path, required=True)
    parser.add_argument('--receipt', type=Path, required=True)
    args = parser.parse_args()
    guard(args.allocation, args.plan, args.receipt)


if __name__ == '__main__':
    main()
