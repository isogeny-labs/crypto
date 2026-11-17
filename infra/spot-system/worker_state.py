"""Worker-side participation in the same lifecycle lock as wake/scale-down."""
from contextlib import contextmanager
import json
import os
import subprocess
import time
import uuid


def call(operation, **kwargs):
    args = ['aws', '--region', os.environ['AWS_REGION'], '--cli-connect-timeout', '3',
            '--cli-read-timeout', '10', 'dynamodb', operation,
            '--table-name', os.environ['SPOT_ACTIVITY_TABLE']]
    for k, v in kwargs.items():
        if isinstance(v, bool):
            if v:
                args += ['--' + k.replace('_', '-')]
            continue
        args += ['--' + k.replace('_', '-'), json.dumps(v) if not isinstance(v, str) else v]
    result = subprocess.run(args + ['--output', 'json'], capture_output=True, text=True, timeout=20)
    if result.returncode:
        if 'ConditionalCheckFailedException' in result.stderr:
            raise BlockingIOError('Lifecycle lease held')
        raise RuntimeError('Worker lifecycle API failed')
    return json.loads(result.stdout or '{}')


@contextmanager
def lease():
    owner = uuid.uuid4().hex
    now = int(time.time())
    key = {'Id': {'S': 'lifecycle-lock'}}
    call('put-item', item={**key, 'Owner': {'S': owner}, 'Expires': {'N': str(now + 90)}},
         condition_expression='attribute_not_exists(Id) OR Expires < :now',
         expression_attribute_values={':now': {'N': str(now)}})
    try:
        yield
    finally:
        call('delete-item', key=key, condition_expression='#owner = :owner',
             expression_attribute_names={'#owner': 'Owner'},
             expression_attribute_values={':owner': {'S': owner}})


def admit(root):
    from activity import instance_id
    with lease():
        state = call('get-item', key={'Id': {'S': 'control'}}, consistent_read=True).get('Item', {})
        if 'StoppingAt' in state:
            raise RuntimeError('Worker is draining; a prompt must wake the group before admission')
        current = instance_id()
        membership = json.loads(subprocess.check_output(['aws', '--region', os.environ['AWS_REGION'],
            'autoscaling', 'describe-auto-scaling-instances', '--instance-ids', current, '--output', 'json'],
            text=True, timeout=15))['AutoScalingInstances']
        if len(membership) != 1 or membership[0]['LifecycleState'] != 'InService':
            raise RuntimeError('Worker is not in service; do not admit work on a terminating instance')
        now = int(time.time())
        call('put-item', item={'Id': {'S': 'activity#' + current},
            'Observed': {'N': str(now)}, 'LastBusy': {'N': str(now)}, 'Expires': {'N': str(now + 600)},
            'Busy': {'BOOL': True}, 'Unknown': {'BOOL': False}, 'Pending': {'N': '1'}})
