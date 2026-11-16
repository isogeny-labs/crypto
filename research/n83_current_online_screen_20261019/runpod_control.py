#!/usr/bin/env python3
"""Allocate one bounded RunPod CPU Pod without persisting the API key."""

import argparse
from datetime import datetime, timezone
import getpass
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

import runpod

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
STATE = Path('/private/tmp/n83_screen_runpod_state.json')
LOG = Path('/private/tmp/n83_screen_runpod_watchdog.log')
BRANCH = 'codex/n83-current-online-screen-runpod-20261019'
CAP_SECONDS = 165 * 60
MAX_HOURLY_USD = 0.40


def now():
    return datetime.now(timezone.utc).isoformat()


def save(path, value):
    path.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')


def source_head():
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'],
                                   cwd=ROOT, text=True).strip()
    dirty = subprocess.check_output(['git', 'status', '--porcelain',
                                     '--untracked-files=all'], cwd=ROOT,
                                    text=True)
    remote = subprocess.check_output(['git', 'ls-remote', 'origin',
                                      f'refs/heads/{BRANCH}'], cwd=ROOT,
                                     text=True).strip()
    if dirty or not remote or remote.split()[0] != head:
        raise ValueError('RunPod source must be clean and pushed')
    dispatch = json.loads((HERE / 'runpod_dispatch.json').read_text())
    if (dispatch['schema'] != 'n83.current-online-screen-runpod-dispatch/v1'
            or dispatch['branch'] != BRANCH):
        raise ValueError('RunPod dispatch differs')
    for relative, expected in dispatch['sha256'].items():
        if hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() != expected:
            raise ValueError(f'sealed input changed: {relative}')
    return head


def terminate_id(pod_id):
    runpod.terminate_pod(pod_id)
    print(f'terminated {pod_id}', flush=True)


def watchdog():
    key = sys.stdin.readline().strip()
    runpod.api_key = key
    try:
        while True:
            state = json.loads(STATE.read_text())
            if state.get('terminated_utc') or state.get('watchdog_terminated_utc'):
                return
            if time.time() >= state['deadline_epoch']:
                try:
                    terminate_id(state['pod_id'])
                    state['watchdog_terminated_utc'] = now()
                except Exception as error:
                    state['watchdog_error'] = f'{type(error).__name__}: {error}'
                save(STATE, state)
                return
            time.sleep(min(30, max(1, state['deadline_epoch'] - time.time())))
    finally:
        runpod.api_key = None
        key = None


def allocate(output):
    head = source_head()
    if STATE.exists() and not json.loads(STATE.read_text()).get('terminated_utc'):
        raise ValueError('prior screen Pod state requires inspection')
    if output.exists():
        raise FileExistsError(output)
    key = getpass.getpass('RunPod API key: ')
    runpod.api_key = key
    pod_id = None
    try:
        existing = runpod.get_pods()
        if any((pod.get('name') or '').startswith('n83-screen-rp-') and
               pod.get('desiredStatus') == 'RUNNING' for pod in existing):
            raise ValueError('an N83 screen Pod is already running')
        pod = runpod.create_pod(
            name='n83-screen-rp-20261019', image_name='ubuntu:22.04',
            gpu_type_id=None, cloud_type='SECURE', start_ssh=True,
            support_public_ip=False, gpu_count=1, volume_in_gb=0,
            container_disk_in_gb=30, min_vcpu_count=4,
            min_memory_in_gb=20, docker_args='sleep infinity',
            ports='22/tcp', instance_id='cpu3g-8-32')
        pod_id = pod.get('id')
        if not pod_id:
            raise ValueError('RunPod did not return Pod ID')
        deadline = time.monotonic() + 180
        while True:
            match = next((p for p in runpod.get_pods()
                          if p.get('id') == pod_id), None)
            if match and match.get('desiredStatus') == 'RUNNING' and \
                    match.get('machineId') and match.get('costPerHr'):
                break
            if time.monotonic() > deadline:
                raise TimeoutError('Pod did not expose machine/cost in 180s')
            time.sleep(8)
        price = float(match['costPerHr'])
        if price > MAX_HOURLY_USD:
            raise ValueError('observed Pod price exceeds frozen cap')
        output.mkdir(parents=True)
        allocation = {
            'schema': 'n83.current-online-screen-runpod-allocation/v1',
            'recorded_utc': now(), 'source_head_sha': head,
            'pod_id': pod_id, 'machine_id': match['machineId'],
            'desired_status': match['desiredStatus'],
            'cost_per_hr': price, 'image_name': match.get('imageName'),
            'vcpu_count': match.get('vcpuCount'),
            'memory_in_gb': match.get('memoryInGb')}
        save(output / 'allocation.json', allocation)
        state = {'schema': 'n83.current-online-screen-runpod-control/v1',
                 'created_utc': now(), 'pod_id': pod_id,
                 'machine_id': match['machineId'], 'hourly_usd': price,
                 'deadline_epoch': time.time() + CAP_SECONDS,
                 'source_head_sha': head, 'output_dir': str(output)}
        save(STATE, state)
        with LOG.open('a') as stream:
            child = subprocess.Popen([sys.executable, __file__, '--watchdog'],
                                     stdin=subprocess.PIPE, stdout=stream,
                                     stderr=stream, start_new_session=True,
                                     text=True)
            child.stdin.write(key + '\n')
            child.stdin.close()
        state['watchdog_pid'] = child.pid
        save(STATE, state)
        print(json.dumps({'pod_id': pod_id, 'machine_id': match['machineId'],
                          'hourly_usd': price, 'deadline_epoch':
                          state['deadline_epoch'], 'watchdog_pid': child.pid},
                         sort_keys=True), flush=True)
    except Exception:
        if pod_id:
            try:
                terminate_id(pod_id)
            except Exception as error:
                print(f'cleanup failed: {type(error).__name__}: {error}',
                      flush=True)
        raise
    finally:
        runpod.api_key = None
        key = None


def terminate():
    state = json.loads(STATE.read_text())
    if state.get('terminated_utc') or state.get('watchdog_terminated_utc'):
        print('Pod already stopped', flush=True)
        return
    key = getpass.getpass('RunPod API key: ')
    runpod.api_key = key
    try:
        terminate_id(state['pod_id'])
        state['terminated_utc'] = now()
        save(STATE, state)
    finally:
        runpod.api_key = None
        key = None


def main():
    parser = argparse.ArgumentParser()
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument('--allocate', action='store_true')
    action.add_argument('--terminate', action='store_true')
    action.add_argument('--watchdog', action='store_true')
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    if args.watchdog:
        return watchdog()
    if args.terminate:
        return terminate()
    if args.output is None:
        parser.error('--output is required for allocation')
    return allocate(args.output.resolve())


if __name__ == '__main__':
    main()
