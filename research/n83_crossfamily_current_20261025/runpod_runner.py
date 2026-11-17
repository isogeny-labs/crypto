#!/usr/bin/env python3
"""Run the sealed current-source high-K stratum on an identified RunPod Pod."""

import argparse
import json
import os
import platform
import shutil
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if sys.path and Path(sys.path[0]).resolve() == HERE:
    sys.path.pop(0)
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from research.n83_crossfamily_current_20261025 import run as runner
from research.n83_crossfamily_current_20261025 import source_bundle


def read_provider(path, expected_stratum):
    receipt = json.loads(path.read_text())
    if (receipt.get('schema') != 'n83.crossfamily-current-runpod-allocation/v1' or
            receipt.get('stratum') != expected_stratum or
            not isinstance(receipt.get('pod_id'), str) or
            not receipt['pod_id'].isalnum() or
            not isinstance(receipt.get('machine_id'), str) or
            not receipt['machine_id'] or
            not isinstance(receipt.get('source_archive_sha256'), str) or
            len(receipt['source_archive_sha256']) != 64 or
            not isinstance(receipt.get('source_manifest_sha256'), str) or
            len(receipt['source_manifest_sha256']) != 64 or
            receipt.get('desired_status') != 'RUNNING' or
            not isinstance(receipt.get('cost_per_hr'), (int, float)) or
            receipt['cost_per_hr'] <= 0):
        raise ValueError('RunPod allocation receipt is incomplete')
    return receipt


def bind_provider(receipt, head_sha):
    original_host_receipt = runner.parent.host_receipt
    original_write_json = runner.parent.write_json
    adapter_sha256 = runner.parent.digest(Path(__file__))

    def host_receipt(group):
        host = original_host_receipt(group)
        host.update({
            'provider': 'runpod',
            'instance_id': f"rp-{receipt['pod_id']}-{receipt['machine_id']}",
            'run_url': f"https://www.runpod.io/console/pods/{receipt['pod_id']}",
            'git_sha': head_sha,
            'source_archive_sha256': receipt['source_archive_sha256'],
            'source_manifest_sha256': receipt['source_manifest_sha256'],
            'runner_name': receipt['pod_id'],
            'runner_os': platform.system(),
            'runner_arch': platform.machine(),
            'image_version': receipt.get('image_name'),
            'runpod_pod_id': receipt['pod_id'],
            'runpod_machine_id': receipt['machine_id'],
            'runpod_allocation_sha256': runner.parent.digest(receipt['_path']),
            'runpod_adapter_sha256': adapter_sha256,
        })
        return host

    def write_json(path, value):
        if Path(path).name == 'source.json':
            value = {**value, 'runpod_adapter_sha256': adapter_sha256,
                     'source_archive_sha256': receipt['source_archive_sha256'],
                     'source_manifest_sha256': receipt['source_manifest_sha256'],
                     'runpod_allocation_sha256':
                     runner.parent.digest(receipt['_path'])}
        return original_write_json(path, value)

    runner.parent.host_receipt = host_receipt
    runner.parent.write_json = write_json


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--stratum', type=int, choices=range(2), required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--rho-binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--provider-receipt', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    args = parser.parse_args()
    receipt_path = args.provider_receipt.resolve()
    receipt = read_provider(receipt_path, args.stratum)
    receipt['_path'] = receipt_path
    if os.environ.get('RUNPOD_POD_ID') not in (None, receipt['pod_id']):
        raise ValueError('Pod environment differs from allocation receipt')
    bundle_path = args.source_bundle.resolve()
    bundle = source_bundle.verify(bundle_path / 'source.tar.gz',
                                  bundle_path / 'manifest.json', ROOT)
    head_sha = bundle['source_head_sha']
    if head_sha != receipt.get('source_head_sha'):
        raise ValueError('Pod source differs from frozen allocation head')
    if (bundle['source_archive_sha256'] != receipt['source_archive_sha256'] or
            source_bundle.digest(bundle_path / 'manifest.json') !=
            receipt['source_manifest_sha256']):
        raise ValueError('Pod source bundle differs from allocation')
    runner.fixture()
    runner.choose()
    bind_provider(receipt, head_sha)
    output = args.output.resolve()
    try:
        return runner.run(args.binary.resolve(), args.rho_binary.resolve(),
                          output, args.stratum)
    finally:
        if output.is_dir():
            shutil.copy2(receipt_path, output / 'runpod_allocation.json')


if __name__ == '__main__':
    raise SystemExit(main())
