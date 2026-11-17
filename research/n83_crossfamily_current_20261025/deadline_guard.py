#!/usr/bin/env python3
"""Detached, independent termination path for budgeted RunPod allocations."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import time

from research.n83_crossfamily_current_20261025 import allocate_runpod


def write(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')


def matching_pods(plan, pods):
    names = set(plan['names'])
    started = datetime.fromisoformat(plan['started_utc'].replace('Z', '+00:00'))
    if len(names) != 2 or started.tzinfo is None:
        raise ValueError('invalid deadline guard allocation identity')
    result = []
    for pod in pods:
        if pod.get('name') in names:
            created = datetime.fromisoformat(pod['createdAt'].replace('Z', '+00:00'))
            if created.tzinfo is None or created < started:
                raise ValueError('named Pod predates allocation plan')
            result.append(pod)
    return result


def guard(output, key, pods=allocate_runpod.provider_pods,
          terminate=allocate_runpod.terminate, clock=None, pause=time.sleep):
    clock = clock or (lambda: datetime.now(timezone.utc))
    plan = json.loads((output / 'allocation_plan.json').read_text())
    if plan.get('schema') != 'n83.crossfamily-current-allocation-plan/v1':
        raise ValueError('allocation plan schema differs')
    deadline = datetime.fromisoformat(plan['terminate_at_utc'].replace('Z', '+00:00'))
    if deadline.tzinfo is None or len(plan.get('names', [])) != 2:
        raise ValueError('invalid deadline guard plan')
    matching_pods(plan, pods(key))  # Verify provider access before the first mutation.
    write(output / 'deadline_guard.ready', {'status': 'ARMED',
                                            'terminate_at_utc': plan['terminate_at_utc']})
    while clock() < deadline:
        pause(min(60, max(0.1, (deadline - clock()).total_seconds())))
    while True:
        try:
            found = matching_pods(plan, pods(key))
            if not found:
                write(output / 'deadline_guard.result.json', {
                    'status': 'MATCHING_PODS_ABSENT',
                    'recorded_utc': clock().isoformat()})
                return
            for pod in found:
                try:
                    terminate(pod['id'], key)
                except Exception:
                    pass
        except Exception as error:
            write(output / 'deadline_guard.error.json', {
                'error_type': type(error).__name__,
                'recorded_utc': clock().isoformat()})
        pause(15)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--allocation-dir', type=Path, required=True)
    args = parser.parse_args()
    guard(args.allocation_dir, allocate_runpod.configured_key())


if __name__ == '__main__':
    main()
