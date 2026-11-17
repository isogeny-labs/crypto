#!/usr/bin/env python3
"""Run the sealed finalist stratum with an auditable RunPod host receipt."""

import argparse
import importlib.util
import os
import platform
import re
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
BASE = ROOT / 'research/n83_root11_finalists_tier1_20261017'

spec = importlib.util.spec_from_file_location('n83_runpod_finalist_runner', BASE / 'run.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def pod_host_receipt(pod_id, source_commit):
    original = runner.parent.host_receipt

    def receipt(group):
        host = original(group)
        host.update({
            'provider': 'runpod',
            'instance_id': f'rp-{pod_id}',
            'run_url': f'https://console.runpod.io/pods/{pod_id}',
            'git_sha': source_commit,
            'runner_name': pod_id,
            'runner_os': 'Linux',
            'runner_arch': platform.machine(),
            'image_version': os.environ.get('N83_RUNPOD_IMAGE'),
            'runpod_pod_id': pod_id,
        })
        return host

    return receipt


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--stratum', type=int, choices=range(4), required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--rho-binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    pod_id = os.environ.get('N83_RUNPOD_POD_ID', '')
    if not re.fullmatch(r'[a-z0-9]{10,32}', pod_id):
        raise ValueError('N83_RUNPOD_POD_ID is missing or malformed')
    if os.environ.get('RUNPOD_POD_ID') not in (None, pod_id):
        raise ValueError('RunPod environment and dispatch pod IDs differ')
    source_commit = subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    if not re.fullmatch(r'[0-9a-f]{40}', source_commit):
        raise ValueError('checkout has no full Git commit')
    runner.parent.host_receipt = pod_host_receipt(pod_id, source_commit)
    return runner.run(args.binary.resolve(), args.rho_binary.resolve(),
                      args.output.resolve(), args.stratum)


if __name__ == '__main__':
    raise SystemExit(main())
