#!/usr/bin/env python3
"""Allocate two bounded, distinct-zone AWS Spot hosts for the sealed panel."""

import argparse
from datetime import datetime, timedelta, timezone
import json
from pathlib import Path
import shlex
import subprocess
import sys
import time
import uuid

from research.n83_crossfamily_current_20261025 import allocate_runpod
from research.n83_crossfamily_current_20261025 import plan_aws_spot
from research.n83_crossfamily_current_20261025 import source_bundle

HERE = Path(__file__).resolve().parent
BUCKET = 'aburan28-n83-tier1-20261016-590183823895'
SECURITY_GROUP = 'sg-030ffadf6a62d2ce2'
INSTANCE_PROFILE = 'n83-online-screen-ssm-20261019'
PROJECT_TAG = 'N83HighKCurrent'


def utcnow():
    return datetime.now(timezone.utc)


def json_write(path, value):
    temporary = path.with_suffix(path.suffix + '.tmp')
    temporary.write_text(json.dumps(value, sort_keys=True, indent=2) + '\n')
    temporary.replace(path)


def validate_plan(path, bundle_path):
    saved = json.loads(path.read_text())
    bundle = source_bundle.verify(bundle_path / 'source.tar.gz',
                                  bundle_path / 'manifest.json')
    head = allocate_runpod.frozen_head()
    if bundle['source_head_sha'] != head:
        raise ValueError('source bundle differs from clean frozen checkout')
    bound = {**bundle, 'source_manifest_sha256': source_bundle.digest(
        bundle_path / 'manifest.json')}
    fresh = plan_aws_spot.provider_plan(bound, head, utcnow())
    fixed = ('schema', 'account_id', 'region', 'instance_type', 'architecture',
             'image_id', 'source_head_sha', 'source_archive_sha256',
             'source_manifest_sha256', 'cap_seconds', 'root_disk_gb_per_host',
             'projected_at_bids_usd', 'total_cap_usd')
    if any(saved.get(key) != fresh[key] for key in fixed):
        raise ValueError('approved read-only Spot plan differs from current source')
    for old, new in zip(saved['zones'], fresh['zones']):
        for key in ('stratum', 'zone', 'zone_id', 'subnet_id', 'max_hourly_usd'):
            if old.get(key) != new[key]:
                raise ValueError('Spot placement differs from frozen zones')
    return fresh


def preflight_aws(ec2, iam, s3, plan):
    group = ec2.describe_security_groups(GroupIds=[SECURITY_GROUP])['SecurityGroups'][0]
    if group['VpcId'] != plan_aws_spot.VPC_ID or group.get('IpPermissions'):
        raise ValueError('N83 security group differs or permits inbound traffic')
    profile = iam.get_instance_profile(InstanceProfileName=INSTANCE_PROFILE)[
        'InstanceProfile']
    if profile['Arn'].split(':')[4] != plan['account_id']:
        raise ValueError('EC2 instance profile belongs to another account')
    s3.head_bucket(Bucket=BUCKET)
    existing = ec2.describe_instances(Filters=[
        {'Name': 'tag:Project', 'Values': [PROJECT_TAG]},
        {'Name': 'instance-state-name', 'Values':
         ['pending', 'running', 'stopping']}])
    if any(reservation['Instances'] for reservation in existing['Reservations']):
        raise ValueError('another high-K Spot allocation is still live')


def stage(s3, bundle_path, prefix, plan):
    keys = {}
    for filename, digest in (
            ('source.tar.gz', plan['source_archive_sha256']),
            ('manifest.json', plan['source_manifest_sha256'])):
        key = f'{prefix}/{filename}'
        with (bundle_path / filename).open('rb') as stream:
            response = s3.put_object(Bucket=BUCKET, Key=key, Body=stream,
                                     Metadata={'sha256': digest,
                                               'source-head': plan['source_head_sha']})
        if not response.get('VersionId'):
            raise ValueError('versioned S3 custody was not returned')
        keys[filename] = {'key': key, 'version_id': response['VersionId'],
                          'sha256': digest}
    return keys


def user_data(plan, zone, source_urls, result_url, status_url, run_id):
    allocation = {
        'schema': 'n83.crossfamily-current-aws-allocation/v1',
        'stratum': zone['stratum'], 'account_id': plan['account_id'],
        'region': plan['region'], 'instance_type': plan['instance_type'],
        'availability_zone': zone['zone'], 'image_id': plan['image_id'],
        'market': 'spot', 'source_head_sha': plan['source_head_sha'],
        'source_archive_sha256': plan['source_archive_sha256'],
        'source_manifest_sha256': plan['source_manifest_sha256'],
        'run_id': run_id,
    }
    values = {'N83_STRATUM': str(zone['stratum']),
              'N83_CAP_SECONDS': str(plan['cap_seconds']),
              'N83_SOURCE_GET_URL': source_urls['source.tar.gz'],
              'N83_MANIFEST_GET_URL': source_urls['manifest.json'],
              'N83_RESULTS_PUT_URL': result_url,
              'N83_STATUS_PUT_URL': status_url,
              'N83_ALLOCATION_JSON': json.dumps(allocation, sort_keys=True,
                                                separators=(',', ':'))}
    bootstrap = (HERE / 'aws_bootstrap.sh').read_text().splitlines()
    if not bootstrap or not bootstrap[0].startswith('#!'):
        raise ValueError('AWS bootstrap is missing its shebang')
    lines = ['#!/bin/bash', *(f'export {key}={shlex.quote(value)}'
                             for key, value in values.items()), *bootstrap[1:]]
    script = '\n'.join(lines) + '\n'
    if len(script.encode()) > 16384:
        raise ValueError('EC2 user data exceeds 16 KiB')
    return script


def launch(ec2, plan, zone, script, run_id):
    tags = [{'Key': 'Project', 'Value': PROJECT_TAG},
            {'Key': 'RunId', 'Value': run_id},
            {'Key': 'SourceHead', 'Value': plan['source_head_sha']},
            {'Key': 'Stratum', 'Value': str(zone['stratum'])}]
    response = ec2.run_instances(
        ImageId=plan['image_id'], InstanceType=plan['instance_type'],
        MinCount=1, MaxCount=1, ClientToken=f'n83-{run_id}-{zone["stratum"]}',
        SubnetId=zone['subnet_id'], SecurityGroupIds=[SECURITY_GROUP],
        IamInstanceProfile={'Name': INSTANCE_PROFILE},
        InstanceMarketOptions={'MarketType': 'spot', 'SpotOptions': {
            'MaxPrice': zone['max_hourly_usd'], 'SpotInstanceType': 'one-time',
            'InstanceInterruptionBehavior': 'terminate'}},
        MetadataOptions={'HttpTokens': 'required', 'HttpEndpoint': 'enabled'},
        InstanceInitiatedShutdownBehavior='terminate',
        BlockDeviceMappings=[{'DeviceName': '/dev/sda1', 'Ebs': {
            'VolumeSize': plan['root_disk_gb_per_host'], 'VolumeType': 'gp3',
            'Encrypted': True, 'DeleteOnTermination': True}}],
        TagSpecifications=[{'ResourceType': resource, 'Tags': tags}
                           for resource in ('instance', 'volume')],
        UserData=script)
    instances = response.get('Instances', [])
    if len(instances) != 1:
        raise ValueError('EC2 did not return exactly one instance')
    return instances[0]['InstanceId']


def start_guard(receipt, output):
    log = (output / 'local_guard.log').open('ab')
    process = subprocess.Popen(
        [sys.executable, '-m',
         'research.n83_crossfamily_current_20261025.aws_deadline_guard',
         '--receipt', str(receipt), '--output',
         str(output / 'local_guard.json')],
        cwd=HERE.parents[1], stdin=subprocess.DEVNULL, stdout=log,
        stderr=subprocess.STDOUT, start_new_session=True)
    log.close()
    return process.pid


def provider_snapshot(ec2, receipt):
    expected = set(receipt['instance_ids'])
    for attempt in range(8):
        try:
            response = ec2.describe_instances(InstanceIds=sorted(expected))
        except Exception as error:
            if getattr(error, 'response', {}).get('Error', {}).get(
                    'Code') != 'InvalidInstanceID.NotFound':
                raise
            time.sleep(min(2 ** attempt, 15))
            continue
        instances = [instance for reservation in response['Reservations']
                     for instance in reservation['Instances']]
        if ({item['InstanceId'] for item in instances} == expected and
                all({tag['Key']: tag['Value'] for tag in item.get('Tags', [])}.get(
                    'RunId') == receipt['run_id'] for item in instances)):
            break
        time.sleep(min(2 ** attempt, 15))
    else:
        raise ValueError('EC2 did not return both allocated instances')
    frozen = []
    for item in instances:
        frozen.append({
            'account_id': receipt['account_id'],
            'instance_id': item['InstanceId'],
            'instance_type': item['InstanceType'],
            'instance_lifecycle': item.get('InstanceLifecycle'),
            'image_id': item['ImageId'],
            'availability_zone': item['Placement']['AvailabilityZone'],
            'subnet_id': item['SubnetId'],
            'state': item['State']['Name'],
            'launch_time': item['LaunchTime'].isoformat(),
            'tags': {tag['Key']: tag['Value'] for tag in item.get('Tags', [])},
        })
    return {'schema': 'n83.crossfamily-aws-provider-snapshot/v1',
            'captured_utc': utcnow().isoformat(),
            'snapshot_source': 'ec2_describe_instances_at_launch',
            'controller': receipt, 'instances': frozen}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--plan', type=Path, required=True)
    parser.add_argument('--source-bundle', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--execute', action='store_true')
    args = parser.parse_args()
    plan = validate_plan(args.plan.resolve(), args.source_bundle.resolve())
    if not args.execute:
        print(json.dumps({'status': 'READY_FOR_ALLOCATION_APPROVAL',
                          'source_head_sha': plan['source_head_sha'],
                          'projected_at_bids_usd':
                          plan['projected_at_bids_usd']}, sort_keys=True))
        return 0
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    import boto3
    ec2 = boto3.client('ec2', region_name=plan['region'])
    iam = boto3.client('iam')
    s3 = boto3.client('s3', region_name=plan['region'])
    preflight_aws(ec2, iam, s3, plan)
    run_id = utcnow().strftime('%Y%m%dT%H%M%SZ') + uuid.uuid4().hex[:8]
    prefix = f'n83-highk-current/{plan["source_head_sha"]}/{run_id}'
    deadline = utcnow() + timedelta(seconds=plan['cap_seconds'])
    receipt_path = output / 'allocation.json'
    receipt = {'schema': 'n83.crossfamily-aws-controller/v1',
               'account_id': plan['account_id'], 'region': plan['region'],
               'source_head_sha': plan['source_head_sha'], 'run_id': run_id,
               'deadline_utc': deadline.isoformat(), 'instance_ids': [],
               'strata': [], 'source_objects': {}, 'result_objects': {},
               'status_objects': {}}
    json_write(receipt_path, receipt)
    keys = stage(s3, args.source_bundle.resolve(), prefix, plan)
    receipt['source_objects'] = keys
    json_write(receipt_path, receipt)
    source_urls = {name: s3.generate_presigned_url(
        'get_object', Params={'Bucket': BUCKET, 'Key': item['key'],
                              'VersionId': item['version_id']},
        ExpiresIn=plan['cap_seconds']) for name, item in keys.items()}
    receipt['local_guard_pid'] = start_guard(receipt_path, output)
    json_write(receipt_path, receipt)
    try:
        for zone in plan['zones']:
            stratum = zone['stratum']
            key = f'{prefix}/stratum{stratum}/raw_results.tar.gz'
            result_url = s3.generate_presigned_url(
                'put_object', Params={'Bucket': BUCKET, 'Key': key},
                ExpiresIn=plan['cap_seconds'] + 900)
            status_key = f'{prefix}/stratum{stratum}/status.tar.gz'
            status_url = s3.generate_presigned_url(
                'put_object', Params={'Bucket': BUCKET, 'Key': status_key},
                ExpiresIn=plan['cap_seconds'] + 900)
            script = user_data(plan, zone, source_urls, result_url,
                               status_url, run_id)
            instance_id = launch(ec2, plan, zone, script, run_id)
            receipt['instance_ids'].append(instance_id)
            receipt['strata'].append({'stratum': stratum,
                                      'instance_id': instance_id,
                                      'zone': zone['zone'],
                                      'subnet_id': zone['subnet_id'],
                                      'max_hourly_usd': zone['max_hourly_usd']})
            receipt['result_objects'][str(stratum)] = key
            receipt['status_objects'][str(stratum)] = status_key
            json_write(receipt_path, receipt)
        json_write(output / 'provider_snapshot_at_launch.json',
                   provider_snapshot(ec2, receipt))
    except BaseException:
        discovered = ec2.describe_instances(Filters=[
            {'Name': 'tag:Project', 'Values': [PROJECT_TAG]},
            {'Name': 'tag:RunId', 'Values': [run_id]},
            {'Name': 'instance-state-name', 'Values':
             ['pending', 'running', 'stopping', 'stopped']}])
        ids = sorted({instance['InstanceId'] for reservation in
                      discovered['Reservations'] for instance in
                      reservation['Instances']})
        if ids:
            ec2.terminate_instances(InstanceIds=ids)
        raise
    print(json.dumps({'status': 'ALLOCATED_GUARDED', 'run_id': run_id,
                      'instance_ids': receipt['instance_ids'],
                      'deadline_utc': receipt['deadline_utc'],
                      'receipt': str(receipt_path)}, sort_keys=True))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
