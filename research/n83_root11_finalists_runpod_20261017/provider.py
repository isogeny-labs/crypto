#!/usr/bin/env python3
"""Capture and verify RunPod allocation identities without retaining API keys."""

import argparse
import getpass
import hashlib
import json
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

SCHEMA = 'n83.root11-finalists-runpod-snapshot/v1'
POD_QUERY = '''query N83PodIdentity {
  myself { pods {
    id name desiredStatus costPerHr gpuCount memoryInGb vcpuCount
    imageName machineId lastStatusChange podType
    runtime { ports { ip isIpPublic privatePort publicPort type } }
  } }
}'''
SAFE_FIELDS = ('id', 'name', 'desiredStatus', 'costPerHr', 'gpuCount',
               'memoryInGb', 'vcpuCount', 'imageName', 'machineId',
               'lastStatusChange', 'podType', 'runtime')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def instant(value):
    result = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if result.tzinfo is None:
        raise ValueError('timestamp has no timezone')
    return result


def capture(pod_ids, key):
    if len(pod_ids) != 4 or len(set(pod_ids)) != 4:
        raise ValueError('four distinct Pod IDs required')
    request = urllib.request.Request(
        'https://api.runpod.io/graphql',
        data=json.dumps({'query': POD_QUERY}).encode(),
        headers={'Authorization': f'Bearer {key}',
                 'Content-Type': 'application/json'},
        method='POST')
    with urllib.request.urlopen(request, timeout=30) as response:
        data = json.load(response)
    if data.get('errors'):
        raise ValueError('RunPod identity query returned errors')
    pods = {pod['id']: pod for pod in data['data']['myself']['pods']}
    if not set(pod_ids) <= set(pods):
        raise ValueError('one or more allocated Pods are absent from RunPod')
    return {'schema': SCHEMA,
            'observed_utc': datetime.now(timezone.utc).isoformat(),
            'pod_ids': list(pod_ids),
            'pods': [{field: pods[pod_id].get(field) for field in SAFE_FIELDS}
                     for pod_id in pod_ids]}


def verify(start_path, end_path, strata, head_sha):
    start, end = (json.loads(path.read_text())
                  for path in (start_path, end_path))
    if start.get('schema') != SCHEMA or end.get('schema') != SCHEMA:
        raise ValueError('RunPod snapshot schema differs')
    ids = start.get('pod_ids')
    if (not isinstance(ids, list) or len(ids) != 4 or len(set(ids)) != 4 or
            end.get('pod_ids') != ids or len(strata) != 4 or
            len(start.get('pods', [])) != 4 or len(end.get('pods', [])) != 4):
        raise ValueError('RunPod Pod coverage differs')
    before, after = instant(start['observed_utc']), instant(end['observed_utc'])
    if before >= after:
        raise ValueError('RunPod snapshots are out of order')
    starts, finishes, rows = [], [], []
    for number, (pod_id, item, old, new) in enumerate(
            zip(ids, strata, start['pods'], end['pods'])):
        host = item['host']
        if (item['stratum'] != number or
                old.get('id') != pod_id or new.get('id') != pod_id or
                old.get('desiredStatus') != 'RUNNING' or
                new.get('desiredStatus') != 'RUNNING' or
                old.get('machineId') is None or
                old.get('machineId') != new.get('machineId') or
                old.get('gpuCount') != 0 or new.get('gpuCount') != 0 or
                old.get('vcpuCount') != 4 or new.get('vcpuCount') != 4 or
                old.get('memoryInGb') != 32 or new.get('memoryInGb') != 32 or
                old.get('imageName') != 'runpod/base:1.0.2-ubuntu2404' or
                new.get('imageName') != old.get('imageName') or
                old.get('costPerHr') is None or
                float(old['costPerHr']) > 0.22 or
                new.get('costPerHr') != old.get('costPerHr') or
                host.get('provider') != 'runpod' or
                host.get('runpod_pod_id') != pod_id or
                host.get('parent_instance_id') != f'rp-{pod_id}' or
                host.get('instance_id') != f'rp-{pod_id}-finalists-stratum{number}' or
                host.get('runner_name') != pod_id or
                host.get('run_url') != f'https://console.runpod.io/pods/{pod_id}' or
                host.get('git_sha') != head_sha):
            raise ValueError(f'stratum {number}: RunPod identity or size differs')
        recorded = instant(host['recorded_utc'])
        results = Path(item['results'])
        launches = list(results.glob('points/p*/runs/*/launch.json'))
        usages = list(results.glob('points/p*/runs/*/usage.json'))
        if not launches or len(launches) != len(usages):
            raise ValueError(f'stratum {number}: missing event time receipts')
        first = min(instant(json.loads(path.read_text())['started_utc'])
                    for path in launches)
        last = max(instant(json.loads(path.read_text())['finished_utc'])
                   for path in usages)
        if not before <= recorded <= first < last <= after:
            raise ValueError(f'stratum {number}: event window outside Pod snapshots')
        starts.append(first)
        finishes.append(last)
        rows.append({'stratum': number, 'pod_id': pod_id,
                     'machine_id': old['machineId'],
                     'cost_per_hour': old['costPerHr'],
                     'vcpu_count': old['vcpuCount'],
                     'memory_gb': old['memoryInGb'],
                     'first_event_utc': first.isoformat(),
                     'last_event_utc': last.isoformat(),
                     'host_receipt_sha256': digest(results / 'host.json')})
    overlap_start, overlap_end = max(starts), min(finishes)
    if overlap_start >= overlap_end:
        raise ValueError('four Pod producer intervals did not overlap')
    return {'schema': 'n83.root11-finalists-runpod-identity/v1',
            'status': 'PASS_FOUR_DISTINCT_CONCURRENT_PODS',
            'start_snapshot_sha256': digest(start_path),
            'end_snapshot_sha256': digest(end_path),
            'overlap_start': overlap_start.isoformat(),
            'overlap_end': overlap_end.isoformat(), 'pods': rows}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('pod_ids', nargs=4)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError('snapshot path already exists')
    key = getpass.getpass('RunPod API key: ')
    value = capture(args.pod_ids, key)
    args.output.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')
    print(json.dumps({'snapshot_sha256': digest(args.output),
                      'pod_ids': value['pod_ids']}, sort_keys=True))


if __name__ == '__main__':
    main()
