#!/usr/bin/env python3
"""Run a frozen N83 high-K stratum on its allocated EC2 Spot instance."""

import argparse
import json
import platform
import shutil
import urllib.request
from pathlib import Path

from research.n83_crossfamily_current_20261025 import run as runner
from research.n83_crossfamily_current_20261025 import source_bundle

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
SCHEMA = 'n83.crossfamily-current-aws-allocation/v1'


def imds_identity():
    token_request = urllib.request.Request(
        'http://169.254.169.254/latest/api/token', method='PUT',
        headers={'X-aws-ec2-metadata-token-ttl-seconds': '21600'})
    with urllib.request.urlopen(token_request, timeout=3) as response:
        token = response.read().decode()

    def get(path):
        request = urllib.request.Request(
            'http://169.254.169.254/latest/' + path,
            headers={'X-aws-ec2-metadata-token': token})
        with urllib.request.urlopen(request, timeout=3) as response:
            return response.read().decode()

    return {'document': json.loads(get('dynamic/instance-identity/document')),
            'pkcs7': get('dynamic/instance-identity/pkcs7'),
            'instance_life_cycle': get('meta-data/instance-life-cycle').strip()}


def read_allocation(path, stratum):
    receipt = json.loads(path.read_text())
    if (receipt.get('schema') != SCHEMA or
            receipt.get('stratum') != stratum or
            receipt.get('account_id') != '590183823895' or
            receipt.get('region') != 'us-west-2' or
            receipt.get('instance_type') != 'g5g.4xlarge' or
            receipt.get('market') != 'spot' or
            receipt.get('state') != 'running' or
            not isinstance(receipt.get('instance_id'), str) or
            not receipt['instance_id'].startswith('i-') or
            not isinstance(receipt.get('run_id'), str) or
            not receipt['run_id'] or
            not isinstance(receipt.get('availability_zone'), str) or
            not isinstance(receipt.get('image_id'), str) or
            not isinstance(receipt.get('source_head_sha'), str) or
            not isinstance(receipt.get('source_archive_sha256'), str) or
            len(receipt['source_archive_sha256']) != 64 or
            not isinstance(receipt.get('source_manifest_sha256'), str) or
            len(receipt['source_manifest_sha256']) != 64):
        raise ValueError('AWS allocation receipt is incomplete')
    return receipt


def check_instance(allocation, identity):
    document = identity['document']
    if (identity['instance_life_cycle'] != 'spot' or
            not identity['pkcs7'].strip() or
            document.get('accountId') != allocation['account_id'] or
            document.get('instanceId') != allocation['instance_id'] or
            document.get('instanceType') != allocation['instance_type'] or
            document.get('availabilityZone') != allocation['availability_zone'] or
            document.get('region') != allocation['region'] or
            document.get('imageId') != allocation['image_id']):
        raise ValueError('EC2 instance identity differs from allocation')


def bind_provider(allocation, path, head, identity):
    original_host = runner.parent.host_receipt
    original_write = runner.parent.write_json
    allocation_sha = runner.parent.digest(path)
    adapter_sha = runner.parent.digest(Path(__file__))

    def host_receipt(group):
        host = original_host(group)
        host.update({
            'provider': 'aws_ec2_spot',
            'instance_id': allocation['instance_id'],
            'instance_type': allocation['instance_type'],
            'availability_zone': allocation['availability_zone'],
            'region': allocation['region'],
            'account_id': allocation['account_id'],
            'image_id': allocation['image_id'],
            'instance_identity': identity,
            'git_sha': head,
            'source_archive_sha256': allocation['source_archive_sha256'],
            'source_manifest_sha256': allocation['source_manifest_sha256'],
            'aws_allocation_sha256': allocation_sha,
            'aws_adapter_sha256': adapter_sha,
            'runner_os': platform.system(),
            'runner_arch': platform.machine(),
        })
        return host

    def write_json(output, value):
        if Path(output).name == 'source.json':
            value = {**value, 'aws_allocation_sha256': allocation_sha,
                     'aws_adapter_sha256': adapter_sha,
                     'source_archive_sha256': allocation['source_archive_sha256'],
                     'source_manifest_sha256': allocation['source_manifest_sha256']}
        return original_write(output, value)

    runner.parent.host_receipt = host_receipt
    runner.parent.write_json = write_json


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--stratum', type=int, choices=range(2), required=True)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--rho-binary', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--allocation', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    args = parser.parse_args()
    allocation_path = args.allocation.resolve()
    allocation = read_allocation(allocation_path, args.stratum)
    identity = imds_identity()
    check_instance(allocation, identity)
    bundle_path = args.source_bundle.resolve()
    bundle = source_bundle.verify(bundle_path / 'source.tar.gz',
                                  bundle_path / 'manifest.json', ROOT)
    head = bundle['source_head_sha']
    if (head != allocation['source_head_sha'] or
            bundle['source_archive_sha256'] !=
            allocation['source_archive_sha256'] or
            source_bundle.digest(bundle_path / 'manifest.json') !=
            allocation['source_manifest_sha256']):
        raise ValueError('EC2 source bundle differs from allocation')
    runner.fixture()
    runner.choose()
    bind_provider(allocation, allocation_path, head, identity)
    output = args.output.resolve()
    try:
        return runner.run(args.binary.resolve(), args.rho_binary.resolve(),
                          output, args.stratum)
    finally:
        if output.is_dir():
            shutil.copy2(allocation_path, output / 'aws_allocation.json')


if __name__ == '__main__':
    raise SystemExit(main())
