#!/usr/bin/env python3
"""Bind root4 host receipts to two concurrent RunPod allocations."""

import argparse
import hashlib
import json
from datetime import datetime
from decimal import Decimal
from pathlib import Path

HERE = Path(__file__).resolve().parent


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def timestamp(value):
    result = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if result.tzinfo is None:
        raise ValueError('provider timestamp lacks a timezone')
    return result


def verify(snapshot_path, strata, head_sha):
    snapshot = json.loads(snapshot_path.read_text())
    prior_path = HERE / 'prior_attempts.json'
    prior = json.loads(prior_path.read_text())
    if (snapshot.get('schema') != 'n83.highk-runpod-snapshot/v1' or
            snapshot.get('source_head_sha') != head_sha or
            snapshot.get('prior_attempts_sha256') != digest(prior_path) or
            snapshot.get('allocation_time_cap_seconds') !=
            prior['new_allocation_time_cap_seconds'] or
            len(snapshot.get('pods', [])) != 2):
        raise ValueError('RunPod provider snapshot differs')
    recorded = timestamp(snapshot['recorded_utc'])
    pods = {pod['stratum']: pod for pod in snapshot['pods']}
    if set(pods) != {0, 1}:
        raise ValueError('RunPod snapshot lacks both strata')
    if (len({pod['pod_id'] for pod in pods.values()}) != 2 or
            len({pod['machine_id'] for pod in pods.values()}) != 2 or
            any(pod['desired_status'] != 'RUNNING' or
                not 0 < pod['cost_per_hr'] <= 0.24 or
                pod.get('cloud_type') != 'COMMUNITY' or
                pod.get('compute_type') != 'GPU' or
                pod.get('gpu_type_id') not in
                ('NVIDIA RTX A2000', 'Tesla V100-PCIE-16GB') or
                pod.get('gpu_count') != 1 or
                pod.get('vcpu_count', 0) < 4 or
                pod.get('memory_in_gb', 0) < 32 or
                not 0 < (timestamp(pod['stop_after_utc']) -
                         timestamp(pod['created_utc'])).total_seconds() <=
                prior['new_allocation_time_cap_seconds']
                for pod in pods.values())):
        raise ValueError('two distinct active RunPod machines required')
    new_cap = Decimal(prior['new_allocation_hard_cap_usd'])
    projected = sum((Decimal(str(pod['cost_per_hr'])) for pod in pods.values()),
                    Decimal(0)) * Decimal(prior['new_allocation_time_cap_seconds']) / Decimal(3600)
    if (projected > new_cap or
            projected + Decimal(prior['reserved_prior_spend_upper_usd']) >
            Decimal(prior['approved_combined_cap_usd'])):
        raise ValueError('RunPod allocation exceeds approved combined cap')
    intervals = []
    for number in range(2):
        root = strata[number]
        host = json.loads((root / 'host.json').read_text())
        source = json.loads((root / 'source.json').read_text())
        allocation_path = root / 'runpod_allocation.json'
        allocation = json.loads(allocation_path.read_text())
        after = json.loads((root / 'host_telemetry_after.json').read_text())
        pod = pods[number]
        if (allocation.get('schema') != 'n83.highk-runpod-allocation/v1' or
                allocation.get('stratum') != number or
                any(allocation.get(key) != pod.get(key) for key in
                    ('pod_id', 'machine_id', 'desired_status', 'cost_per_hr',
                     'image_name', 'vcpu_count', 'memory_in_gb',
                     'cloud_type', 'compute_type', 'gpu_type_id', 'gpu_count',
                     'created_utc', 'stop_after_utc')) or
                allocation.get('source_head_sha') != head_sha or
                allocation.get('snapshot_sha256') != digest(snapshot_path) or
                host.get('provider') != 'runpod' or
                host.get('git_sha') != head_sha or
                host.get('runpod_pod_id') != pod['pod_id'] or
                host.get('runpod_machine_id') != pod['machine_id'] or
                host.get('runpod_allocation_sha256') != digest(allocation_path) or
                host.get('instance_id') !=
                f"rp-{pod['pod_id']}-{pod['machine_id']}-highk-fresh-stratum{number}" or
                source.get('runpod_allocation_sha256') != digest(allocation_path) or
                source.get('runpod_adapter_sha256') !=
                host.get('runpod_adapter_sha256') or
                source.get('runpod_adapter_sha256') !=
                digest(HERE / 'runpod_runner.py')):
            raise ValueError(f'stratum {number}: RunPod provider binding differs')
        started, stopped = timestamp(host['recorded_utc']), timestamp(
            after['recorded_utc'])
        if started > stopped or recorded > started:
            raise ValueError(f'stratum {number}: provider time differs')
        intervals.append((started, stopped))
    if max(interval[0] for interval in intervals) >= min(
            interval[1] for interval in intervals):
        raise ValueError('RunPod benchmark intervals did not overlap')
    return {
        'schema': 'n83.highk-runpod-provider-proof/v1',
        'status': 'PASS_TWO_DISTINCT_CONCURRENT_RUNPOD_MACHINES',
        'snapshot_sha256': digest(snapshot_path),
        'source_head_sha': head_sha,
        'prior_attempts_sha256': digest(prior_path),
        'projected_new_spend_usd': str(projected),
        'pods': [{'stratum': number, 'pod_id': pods[number]['pod_id'],
                  'machine_id': pods[number]['machine_id'],
                  'allocation_sha256': digest(strata[number] /
                                              'runpod_allocation.json')}
                 for number in range(2)],
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--stratum0', type=Path, required=True)
    parser.add_argument('--stratum1', type=Path, required=True)
    parser.add_argument('--head-sha', required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.snapshot, [args.stratum0, args.stratum1],
                            args.head_sha), sort_keys=True))


if __name__ == '__main__':
    main()
