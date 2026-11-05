#!/usr/bin/env python3
"""Save versioned AWS raw archives and a fresh EC2 provider snapshot."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import shutil
import tarfile

from research.n83_crossfamily_current_20261025 import aggregate_runpod
from research.n83_crossfamily_current_20261025 import allocate_aws


def api_instance(instance, account):
    return {
        'account_id': account,
        'instance_id': instance['InstanceId'],
        'instance_type': instance['InstanceType'],
        'instance_lifecycle': instance.get('InstanceLifecycle'),
        'image_id': instance['ImageId'],
        'availability_zone': instance['Placement']['AvailabilityZone'],
        'subnet_id': instance['SubnetId'],
        'state': instance['State']['Name'],
        'launch_time': instance['LaunchTime'].isoformat(),
        'tags': {tag['Key']: tag['Value'] for tag in instance.get('Tags', [])},
    }


def safe_extract(archive, destination):
    destination.mkdir(parents=True, exist_ok=False)
    seen = set()
    with tarfile.open(archive, 'r:gz') as stream:
        members = stream.getmembers()
        for member in members:
            path = Path(member.name)
            if (path.is_absolute() or '..' in path.parts or not path.parts or
                    path.parts[0] != 'results' or
                    not (member.isfile() or member.isdir()) or
                    member.issym() or member.islnk() or path in seen):
                raise ValueError('unsafe or duplicate AWS archive member')
            seen.add(path)
        for member in members:
            target = destination / member.name
            if member.isdir():
                target.mkdir(parents=True, exist_ok=True)
                continue
            target.parent.mkdir(parents=True, exist_ok=True)
            source = stream.extractfile(member)
            if source is None:
                raise ValueError('AWS archive member cannot be read')
            with target.open('xb') as output:
                shutil.copyfileobj(source, output)
    return destination / 'results'


def collect(receipt_path, output):
    import boto3
    from botocore.exceptions import ClientError
    controller = json.loads(receipt_path.read_text())
    if (controller.get('schema') != 'n83.crossfamily-aws-controller/v1' or
            controller.get('account_id') != '590183823895' or
            len(controller.get('instance_ids', [])) != 2):
        raise ValueError('complete two-instance controller receipt required')
    output.mkdir(parents=True, exist_ok=False)
    ec2 = boto3.client('ec2', region_name=controller['region'])
    s3 = boto3.client('s3', region_name=controller['region'])
    launch_snapshot = receipt_path.parent / 'provider_snapshot_at_launch.json'
    if launch_snapshot.exists():
        snapshot = json.loads(launch_snapshot.read_text())
        if (snapshot.get('schema') !=
                'n83.crossfamily-aws-provider-snapshot/v1' or
                snapshot.get('controller', {}).get('run_id') !=
                controller['run_id'] or
                set(item['instance_id'] for item in
                    snapshot.get('instances', [])) !=
                set(controller['instance_ids'])):
            raise ValueError('launch EC2 snapshot differs from controller')
    else:
        response = ec2.describe_instances(InstanceIds=controller['instance_ids'])
        instances = [api_instance(instance, controller['account_id'])
                     for reservation in response['Reservations']
                     for instance in reservation['Instances']]
        snapshot = {'schema': 'n83.crossfamily-aws-provider-snapshot/v1',
                    'captured_utc': datetime.now(timezone.utc).isoformat(),
                    'snapshot_source': 'ec2_describe_instances_at_collection',
                    'controller': controller, 'instances': instances}
    (output / 'provider_snapshot.json').write_text(json.dumps(
        snapshot, sort_keys=True, indent=2) + '\n')
    archives = []
    status_objects = []
    for stratum in range(2):
        status_key = controller['status_objects'][str(stratum)]
        try:
            status_head = s3.head_object(Bucket=allocate_aws.BUCKET,
                                         Key=status_key)
        except ClientError as error:
            if error.response['Error']['Code'] not in ('404', 'NoSuchKey'):
                raise
        else:
            status_version = status_head['VersionId']
            status_path = output / f'stratum{stratum}.status.tar.gz'
            status_body = s3.get_object(Bucket=allocate_aws.BUCKET,
                                        Key=status_key,
                                        VersionId=status_version)['Body']
            with status_path.open('xb') as stream:
                shutil.copyfileobj(status_body, stream)
            status_objects.append({'stratum': stratum, 's3_key': status_key,
                                   's3_version_id': status_version,
                                   'sha256': aggregate_runpod.sha256(status_path)})
        key = controller['result_objects'][str(stratum)]
        try:
            head = s3.head_object(Bucket=allocate_aws.BUCKET, Key=key)
        except ClientError as error:
            if error.response['Error']['Code'] in ('404', 'NoSuchKey'):
                archives.append({'stratum': stratum, 'status': 'PENDING'})
                continue
            raise
        path = output / f'stratum{stratum}.raw_results.tar.gz'
        version = head['VersionId']
        if not version:
            raise ValueError('AWS raw result object lacks a version')
        result = s3.get_object(Bucket=allocate_aws.BUCKET, Key=key,
                               VersionId=version)
        with path.open('xb') as stream:
            shutil.copyfileobj(result['Body'], stream)
        extracted = safe_extract(path, output / f'stratum{stratum}')
        archive_proof = aggregate_runpod.bind_archive(path, extracted)
        archives.append({'stratum': stratum, 'status': 'RAW_ARCHIVE_BOUND',
                         's3_key': key, 's3_version_id': version,
                         'archive_path': str(path),
                         'results_path': str(extracted), **archive_proof})
    report = {'schema': 'n83.crossfamily-aws-collection/v1',
              'status': 'COMPLETE_RAW_ARCHIVES' if all(
                  item['status'] == 'RAW_ARCHIVE_BOUND' for item in archives)
              else 'BOUNDED_UNKNOWN',
              'provider_snapshot_path': str(output / 'provider_snapshot.json'),
              'archives': archives, 'status_objects': status_objects}
    (output / 'collection.json').write_text(json.dumps(
        report, sort_keys=True, indent=2) + '\n')
    return report


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--controller-receipt', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    report = collect(args.controller_receipt.resolve(), args.output.resolve())
    print(json.dumps(report, sort_keys=True))
    return 0 if report['status'] == 'COMPLETE_RAW_ARCHIVES' else 2


if __name__ == '__main__':
    raise SystemExit(main())
