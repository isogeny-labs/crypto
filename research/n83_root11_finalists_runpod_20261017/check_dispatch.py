#!/usr/bin/env python3
"""Check the frozen RunPod dispatch before allocation or replay."""

import hashlib
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE = ROOT / 'research/n83_root11_finalists_tier1_20261017'


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def check():
    value = json.loads((HERE / 'dispatch.json').read_text())
    if (value.get('schema') != 'n83.root11-finalists-runpod-dispatch/v1' or
            value.get('status') != 'SEALED_PENDING_EXPLICIT_ALLOCATION_APPROVAL' or
            value.get('provider') != 'runpod' or
            value.get('provider_spec') != {
                'cloud': 'SECURE', 'cpu_flavor': 'cpu3m', 'vcpu_count': 4,
                'memory_gb': 32, 'image': 'runpod/base:1.0.2-ubuntu2404',
                'ephemeral_disk_gb': 30, 'persistent_volume_gb': 0,
                'ssh': '22/tcp', 'pod_count': 4} or
            value.get('max_hourly_usd_per_pod') != '0.22' or
            value.get('max_allocation_minutes_from_first_creation') != 300 or
            value.get('max_total_spend_usd_including_storage') != '6.00' or
            value.get('targets_sha256') != digest(BASE / 'targets.jsonl') or
            value.get('answers_sha256') != digest(BASE / 'answers.jsonl') or
            value.get('producer_sha256') != digest(BASE / 'run.py') or
            value.get('stratum_verifier_sha256') != digest(BASE / 'verify.py') or
            value.get('selector_sha256') != digest(BASE / 'select.py')):
        raise ValueError('sealed RunPod dispatch differs')
    for relative, expected in value['source_files_sha256'].items():
        if digest(ROOT / relative) != expected:
            raise ValueError(f'frozen adapter source differs: {relative}')
    return {'dispatch_sha256': digest(HERE / 'dispatch.json'),
            'source_file_count': len(value['source_files_sha256']),
            'candidate_ids': value['candidate_ids']}


if __name__ == '__main__':
    print(json.dumps(check(), sort_keys=True))
