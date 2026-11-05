#!/usr/bin/env python3
"""Independent local EC2 termination guard for a bounded N83 allocation."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import time

def terminate(ec2, ids):
    if not ids:
        return []
    response = ec2.terminate_instances(InstanceIds=sorted(ids))
    return response.get('TerminatingInstances', [])


def verified_live_ids(ec2, receipt):
    ids = set(receipt.get('instance_ids', []))
    response = ec2.describe_instances(Filters=[
        {'Name': 'tag:Project', 'Values': ['N83HighKCurrent']},
        {'Name': 'tag:RunId', 'Values': [receipt['run_id']]}])
    observed = [instance for reservation in response['Reservations']
                for instance in reservation['Instances']]
    if not ids.issubset({item['InstanceId'] for item in observed}):
        raise ValueError('guard EC2 instance set differs from receipt')
    live = []
    for instance in observed:
        tags = {tag['Key']: tag['Value'] for tag in instance.get('Tags', [])}
        if (tags.get('Project') != 'N83HighKCurrent' or
                tags.get('RunId') != receipt['run_id'] or
                tags.get('SourceHead') != receipt['source_head_sha']):
            raise ValueError('guard EC2 tags differ from N83 allocation')
        if instance['State']['Name'] not in ('terminated', 'shutting-down'):
            live.append(instance['InstanceId'])
    return live


def guard(receipt, output, clock=lambda: datetime.now(timezone.utc),
          sleep=time.sleep, ec2=None):
    if ec2 is None:
        import boto3
        ec2 = boto3.client('ec2', region_name='us-east-1')
    initial = json.loads(receipt.read_text())
    deadline = datetime.fromisoformat(initial['deadline_utc'])
    if deadline.tzinfo is None:
        raise ValueError('deadline must have a timezone')
    while clock() < deadline:
        sleep(min(30, max(0.1, (deadline - clock()).total_seconds())))
    attempts = []
    for retry in range(5):
        value = json.loads(receipt.read_text())
        ids = value.get('instance_ids', [])
        if value.get('account_id') != '590183823895' or not all(
                isinstance(item, str) and item.startswith('i-') for item in ids):
            raise ValueError('guard receipt account or instances differ')
        try:
            live = verified_live_ids(ec2, value)
            if live:
                attempts.append({'ids': live, 'response': terminate(ec2, live),
                                 'at_utc': clock().isoformat()})
            else:
                attempts.append({'ids': [], 'response': [],
                                 'at_utc': clock().isoformat()})
            break
        except Exception as error:
            attempts.append({'ids': ids, 'error': str(error),
                             'at_utc': clock().isoformat()})
        sleep(10 * (retry + 1))
    result = {'schema': 'n83.crossfamily-aws-local-guard/v1',
              'deadline_utc': deadline.isoformat(),
              'status': 'TERMINATE_REQUESTED' if attempts and
              'response' in attempts[-1] else 'TERMINATE_FAILED',
              'attempts': attempts}
    output.write_text(json.dumps(result, sort_keys=True, indent=2) + '\n')
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--receipt', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = guard(args.receipt, args.output)
    return 0 if result['status'] == 'TERMINATE_REQUESTED' else 2


if __name__ == '__main__':
    raise SystemExit(main())
