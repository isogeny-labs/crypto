#!/usr/bin/env python3
"""Fail closed if a non-benchmark Pod thread can run on the measured core."""

import argparse
import json
import os
from pathlib import Path
import signal
import time


MEASURED = {5, 29}
BENCHMARK_CPU = {5}


def scan(runner_pid):
    violations = []
    for name in os.listdir('/proc'):
        if not name.isdigit():
            continue
        pid = int(name)
        process = Path('/proc') / name
        try:
            ppid = int((process / 'status').read_text().split('PPid:')[1].split()[0])
            tids = [int(value) for value in os.listdir(process / 'task') if value.isdigit()]
            command = (process / 'cmdline').read_bytes().replace(b'\0', b' ').decode(
                errors='replace').strip()[:160]
        except FileNotFoundError:
            continue
        except PermissionError as error:
            violations.append({'pid': pid, 'error': str(error)})
            continue
        for tid in tids:
            try:
                affinity = set(os.sched_getaffinity(tid))
            except (FileNotFoundError, ProcessLookupError):
                continue
            except PermissionError as error:
                violations.append({'pid': pid, 'tid': tid, 'error': str(error)})
                continue
            if not affinity.intersection(MEASURED):
                continue
            benchmark_child = (
                runner_pid is not None and ppid == runner_pid and
                affinity == BENCHMARK_CPU and
                ('rho_worker_batch_benchmark' in command or 'run_tier1_worker_batch.py' in command)
            )
            if not benchmark_child:
                violations.append({
                    'pid': pid, 'tid': tid, 'ppid': ppid,
                    'affinity': sorted(affinity),
                    'program': command.split()[0].rsplit('/', 1)[-1] if command else '',
                })
    return violations


def runner_live(pid):
    try:
        status = Path(f'/proc/{pid}/status').read_text()
    except FileNotFoundError:
        return False
    return not status.split('State:')[1].split()[0].startswith('Z')


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--runner-pid-file', type=Path, required=True)
    parser.add_argument('--ready-file', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--interval-s', type=float, default=0.25)
    parser.add_argument('--quiet-period-s', type=float, default=5.0)
    parser.add_argument('--startup-timeout-s', type=float, default=90.0)
    args = parser.parse_args()
    if not 0 < args.interval_s <= 1 or args.quiet_period_s < 1:
        parser.error('interval must be in (0, 1] and quiet period at least 1 second')
    if args.startup_timeout_s <= args.quiet_period_s:
        parser.error('startup timeout must exceed the quiet period')
    if args.ready_file.exists():
        parser.error('ready file already exists')
    started = time.time()
    started_monotonic = time.monotonic()
    polls = 0
    runner_pid = None
    violation = None
    armed = False
    clear_since = None
    startup_issue_count = 0
    startup_issue_samples = []
    while True:
        if runner_pid is None and args.runner_pid_file.exists():
            runner_pid = int(args.runner_pid_file.read_text().strip())
        polls += 1
        issues = scan(runner_pid)
        if not armed:
            if issues:
                clear_since = None
                startup_issue_count += len(issues)
                if len(startup_issue_samples) < 20:
                    startup_issue_samples.extend(issues[:20 - len(startup_issue_samples)])
            elif runner_pid is not None:
                if clear_since is None:
                    clear_since = time.monotonic()
                if time.monotonic() - clear_since >= args.quiet_period_s:
                    args.ready_file.write_text(json.dumps({
                        'schema': 'rho-worker-affinity-ready-v1',
                        'observer_pid': os.getpid(),
                        'runner_pid': runner_pid,
                        'armed_at_unix': time.time(),
                        'quiet_period_s': args.quiet_period_s,
                        'startup_issue_count': startup_issue_count,
                    }, indent=2, sort_keys=True) + '\n')
                    armed = True
            if runner_pid is not None and not runner_live(runner_pid):
                violation = {'reason': 'runner exited before observer armed'}
                break
            if time.monotonic() - started_monotonic > args.startup_timeout_s:
                violation = {'reason': 'startup quiet-window timeout'}
                break
        else:
            if issues:
                violation = {'observed_at_unix': time.time(), 'issues': issues}
                break
            if not runner_live(runner_pid):
                break
        if violation:
            break
        time.sleep(args.interval_s)
    if violation and runner_pid is not None and runner_live(runner_pid):
        try:
            os.kill(runner_pid, signal.SIGINT)
        except ProcessLookupError:
            pass
    receipt = {
        'schema': 'rho-worker-pod-affinity-observer-v2',
        'started_at_unix': started,
        'ended_at_unix': time.time(),
        'runner_pid': runner_pid,
        'polls': polls,
        'interval_s': args.interval_s,
        'quiet_period_s': args.quiet_period_s,
        'startup_timeout_s': args.startup_timeout_s,
        'startup_issue_count': startup_issue_count,
        'startup_issue_samples': startup_issue_samples,
        'armed': armed,
        'ready_file': str(args.ready_file),
        'measured_cpus': sorted(MEASURED),
        'status': 'violation' if violation else 'passed' if armed else 'startup_failed',
        'violation': violation,
    }
    args.output.write_text(json.dumps(receipt, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'status': receipt['status'], 'polls': polls}))
    if violation:
        raise SystemExit(2)


if __name__ == '__main__':
    main()
