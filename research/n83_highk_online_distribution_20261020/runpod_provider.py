#!/usr/bin/env python3
"""Bind both N83 distribution phases to one recorded RunPod allocation."""
import argparse
import hashlib
import json
from datetime import datetime
from decimal import Decimal
from pathlib import Path

HERE = Path(__file__).resolve().parent
GPU_TYPES = (
    'NVIDIA RTX A2000', 'Tesla V100-PCIE-16GB',
    'NVIDIA GeForce RTX 3070', 'NVIDIA GeForce RTX 3080',
    'NVIDIA GeForce RTX 3080 Ti', 'NVIDIA GeForce RTX 4070 Ti',
    'NVIDIA RTX 4000 Ada Generation',
    'NVIDIA RTX 4000 SFF Ada Generation',
    'NVIDIA RTX A4000', 'NVIDIA RTX A4500', 'NVIDIA RTX A5000',
)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def timestamp(value):
    result = datetime.fromisoformat(value.replace('Z', '+00:00'))
    if result.tzinfo is None:
        raise ValueError('provider timestamp lacks timezone')
    return result


def verify(snapshot_path, results, head_sha):
    snapshot = json.loads(snapshot_path.read_text())
    prior_path = HERE / 'prior_attempts.json'
    prior = json.loads(prior_path.read_text())
    if (snapshot.get('schema') != 'n83.online-distribution-runpod-snapshot/v1' or
            snapshot.get('source_head_sha') != head_sha or
            snapshot.get('prior_attempts_sha256') != digest(prior_path) or
            snapshot.get('allocation_time_cap_seconds') !=
            prior['new_allocation_time_cap_seconds'] or
            len(snapshot.get('pods', [])) != 1):
        raise ValueError('provider snapshot differs')
    pod = snapshot['pods'][0]
    if (pod.get('stratum') != 0 or pod.get('desired_status') != 'RUNNING' or
            not isinstance(pod.get('pod_id'), str) or
            not isinstance(pod.get('machine_id'), str) or
            not pod['pod_id'] or not pod['machine_id'] or
            not 0 < Decimal(str(pod.get('cost_per_hr', 0))) <=
            Decimal(prior['new_per_pod_price_cap_usd']) or
            pod.get('cloud_type') != 'COMMUNITY' or
            pod.get('compute_type') != 'GPU' or
            pod.get('gpu_type_id') not in GPU_TYPES or
            pod.get('gpu_count') != 1 or
            not isinstance(pod.get('cpu_affinity_ids'), list) or
            len(pod['cpu_affinity_ids']) != 4 or
            pod['cpu_affinity_ids'] != sorted(set(pod['cpu_affinity_ids'])) or
            any(not isinstance(cpu, int) or cpu < 0 for cpu in
                pod['cpu_affinity_ids']) or
            pod.get('vcpu_count', 0) < 4 or
            pod.get('memory_in_gb', 0) < 32 or
            not 0 < (timestamp(pod['stop_after_utc']) -
                     timestamp(pod['created_utc'])).total_seconds() <=
            prior['new_allocation_time_cap_seconds']):
        raise ValueError('RunPod machine outside frozen envelope')
    projected = Decimal(str(pod['cost_per_hr'])) * Decimal(
        prior['new_allocation_time_cap_seconds']) / Decimal(3600)
    if (projected > Decimal(prior['new_allocation_hard_cap_usd']) or
            projected + Decimal(prior['reserved_prior_spend_upper_usd']) >
            Decimal(prior['approved_combined_cap_usd'])):
        raise ValueError('RunPod allocation exceeds approved cap')
    allocation_path = results / 'runpod_allocation.json'
    allocation = json.loads(allocation_path.read_text())
    host = json.loads((results / 'host.json').read_text())
    source = json.loads((results / 'source.json').read_text())
    screen_after = json.loads((results / 'screen/host_telemetry_after.json').read_text())
    holdout_after_path = results / 'holdout/host_telemetry_after.json'
    holdout_after = (json.loads(holdout_after_path.read_text()) if
                     holdout_after_path.exists() else None)
    if (allocation.get('schema') != 'n83.online-distribution-runpod-allocation/v1' or
            allocation.get('stratum') != 0 or
            any(allocation.get(key) != pod.get(key) for key in
                ('pod_id', 'machine_id', 'desired_status', 'cost_per_hr',
                 'image_name', 'vcpu_count', 'memory_in_gb', 'cloud_type',
                 'compute_type', 'gpu_type_id', 'gpu_count', 'created_utc',
                 'stop_after_utc', 'cpu_affinity_ids')) or
            allocation.get('source_head_sha') != head_sha or
            allocation.get('snapshot_sha256') != digest(snapshot_path) or
            host.get('provider') != 'runpod' or
            host.get('git_sha') != head_sha or
            host.get('runpod_pod_id') != pod['pod_id'] or
            host.get('runpod_machine_id') != pod['machine_id'] or
            host.get('runpod_frozen_cpu_affinity') != pod['cpu_affinity_ids'] or
            host.get('visible_affinity') != pod['cpu_affinity_ids'] or
            host.get('runpod_allocation_sha256') != digest(allocation_path) or
            host.get('instance_id') != f"rp-{pod['pod_id']}-{pod['machine_id']}" or
            source.get('runpod_allocation_sha256') != digest(allocation_path) or
            source.get('runpod_adapter_sha256') !=
            host.get('runpod_adapter_sha256') or
            source.get('runpod_adapter_sha256') !=
            digest(HERE / 'runpod_runner.py')):
        raise ValueError('RunPod provider binding differs')
    recorded = timestamp(snapshot['recorded_utc'])
    started = timestamp(host['recorded_utc'])
    screened = timestamp(screen_after['recorded_utc'])
    stopped = (timestamp(holdout_after['recorded_utc']) if holdout_after
               else screened)
    if not recorded <= started <= screened <= stopped <= timestamp(pod['stop_after_utc']):
        raise ValueError('provider and benchmark times differ')
    return {'schema': 'n83.online-distribution-runpod-provider-proof/v1',
            'status': 'PASS_SINGLE_RECORDED_RUNPOD_ALLOCATION',
            'snapshot_sha256': digest(snapshot_path),
            'source_head_sha': head_sha,
            'prior_attempts_sha256': digest(prior_path),
            'projected_new_spend_usd': str(projected),
            'pod_id': pod['pod_id'], 'machine_id': pod['machine_id'],
            'cpu_affinity_ids': pod['cpu_affinity_ids'],
            'allocation_sha256': digest(allocation_path)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--snapshot', type=Path, required=True)
    parser.add_argument('--results', type=Path, required=True)
    parser.add_argument('--head-sha', required=True)
    args = parser.parse_args()
    print(json.dumps(verify(args.snapshot, args.results, args.head_sha), sort_keys=True))


if __name__ == '__main__':
    main()
