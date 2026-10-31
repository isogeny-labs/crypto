"""IAM-invoked wake requests and fail-closed 30-minute idle scale-down."""
import os
import time


def read(db, table, key):
    return db.get_item(TableName=table, Key={"Id": {"S": key}},
                       ConsistentRead=True).get("Item", {})


def number(item, name, default=0):
    return int(item.get(name, {}).get("N", str(default)))


def run(db, autoscaling, event, owner, now, table, group, enabled, idle_seconds=1800):
    action = event.get("action", "tick")
    if action not in {"wake", "tick", "status", "release"}:
        raise ValueError("Unsupported lifecycle action")
    if action == "status":
        return {"state": "enabled" if enabled else "staged",
                "control": read(db, table, "control")}
    if not enabled:
        return {"state": "staged", "reason": "Managed worker lifecycle is not enabled"}
    if action == 'release':
        return {'state': 'idle-timer-managed', 'idle_seconds': idle_seconds}
    key = {"Id": {"S": "lifecycle-lock"}}
    try:
        db.put_item(TableName=table, Item={**key, "Owner": {"S": owner},
                    "Expires": {"N": str(now + 90)}},
                    ConditionExpression="attribute_not_exists(Id) OR Expires < :now",
                    ExpressionAttributeValues={":now": {"N": str(now)}})
    except db.exceptions.ConditionalCheckFailedException:
        return {"state": "busy", "retry": True}
    try:
        state = read(db, table, "control") or {"Id": {"S": "control"}}
        if action == "wake":
            # Persist demand first: the next tick retries scaling if the API fails.
            state["WakeUntil"] = {"N": str(now + idle_seconds)}
            state.pop("IdleSince", None)
            state.pop("StoppingAt", None)
            db.put_item(TableName=table, Item=state)
            autoscaling.set_desired_capacity(AutoScalingGroupName=group,
                                              DesiredCapacity=1, HonorCooldown=False)
            return {"state": "starting", "wake_until": now + idle_seconds}
        groups = autoscaling.describe_auto_scaling_groups(
            AutoScalingGroupNames=[group])["AutoScalingGroups"]
        if len(groups) != 1:
            raise RuntimeError("Expected exactly one worker group")
        current = groups[0]
        desired = current["DesiredCapacity"]
        wake_until = number(state, "WakeUntil")
        if desired == 0:
            if wake_until > now:
                autoscaling.set_desired_capacity(AutoScalingGroupName=group,
                                                  DesiredCapacity=1, HonorCooldown=False)
                return {"state": "starting"}
            return {"state": "asleep"}
        workers = [i for i in current["Instances"]
                   if not i["LifecycleState"].startswith(("Terminating", "Detached"))]
        reports = [read(db, table, "activity#" + i["InstanceId"]) for i in workers]
        safe_idle = bool(workers) and all(i["LifecycleState"] == "InService" for i in workers)
        for report in reports:
            safe_idle = safe_idle and (
                0 <= now - number(report, "Observed", -10000) <= 90
                and report.get("Busy", {}).get("BOOL") is False
                and report.get("Unknown", {}).get("BOOL") is False
                and number(report, "Pending", -1) == 0)
        if not safe_idle:
            state.pop("IdleSince", None)
            db.put_item(TableName=table, Item=state)
            return {"state": "working-or-unconfirmed"}
        since = max(number(state, "IdleSince", now),
                    max(number(report, 'LastBusy', now) for report in reports))
        state["IdleSince"] = {"N": str(since)}
        db.put_item(TableName=table, Item=state)
        remaining = max(since + idle_seconds, wake_until) - now
        if remaining > 0:
            return {"state": "idle", "remaining_seconds": remaining}
        # Wake uses the same lease. A concurrent prompt either renews before this
        # check or scales up after it; demand is never overwritten by a stale tick.
        state['StoppingAt'] = {'N': str(now)}
        db.put_item(TableName=table, Item=state)
        autoscaling.set_desired_capacity(AutoScalingGroupName=group,
                                          DesiredCapacity=0, HonorCooldown=False)
        state.pop("IdleSince", None)
        db.put_item(TableName=table, Item=state)
        return {"state": "scaled-to-zero"}
    finally:
        db.delete_item(TableName=table, Key=key, ConditionExpression="#owner = :owner",
                       ExpressionAttributeNames={"#owner": "Owner"},
                       ExpressionAttributeValues={":owner": {"S": owner}})


def handler(event, context):
    import boto3
    if (os.environ.get('PROTECTED_SOURCE') and event.get('action') == 'wake'
            and event.get('managed_work') is not True):
        status = handler({'action':'status'}, context)
        if status.get('protected_source_active'):
            return {'state':'ready','endpoint_instance_id':status['endpoint_instance_id'],'protected_source_active':True}
    if event.get('action') == 'ssh-key':
        status = handler({'action':'status'}, context)
        if status.get('state') != 'ready':
            return {'state':'waiting-for-endpoint'}
        target = status['endpoint_instance_id']
        ssm = boto3.client('ssm')
        command = ssm.send_command(InstanceIds=[target], DocumentName='AWS-RunShellScript',
            Parameters={'commands':['cat /etc/ssh/ssh_host_ed25519_key.pub']},
            Comment='Read public SSH host key for fixed-IP authentication')['Command']['CommandId']
        for _ in range(10):
            try:
                response = ssm.get_command_invocation(CommandId=command, InstanceId=target)
                if response['Status'] == 'Success':
                    key = response['StandardOutputContent'].strip().split()
                    if len(key) < 2 or key[0] != 'ssh-ed25519':
                        raise RuntimeError('Unexpected public SSH host key')
                    return {'state':'ready','instance_id':target,'host_key':' '.join(key[:2])}
                if response['Status'] in {'Failed','Cancelled','TimedOut'}:
                    raise RuntimeError('Could not read the public SSH host key')
            except ssm.exceptions.InvocationDoesNotExist:
                pass
            time.sleep(.5)
        return {'state':'waiting-for-host-key'}
    result = run(boto3.client("dynamodb"), boto3.client("autoscaling"), event,
               context.aws_request_id, int(time.time()), os.environ["TABLE"],
               os.environ["GROUP"], os.environ.get("ENABLED") == "true",
               int(os.environ.get("IDLE_SECONDS", "1800")))
    if event.get('action') == 'status' and result['state'] == 'enabled':
        group = boto3.client('autoscaling').describe_auto_scaling_groups(
            AutoScalingGroupNames=[os.environ['GROUP']])['AutoScalingGroups'][0]
        ids = [i['InstanceId'] for i in group['Instances'] if i['LifecycleState'] == 'InService']
        result['desired'] = group['DesiredCapacity']
        ec2 = boto3.client('ec2')
        address = ec2.describe_addresses(AllocationIds=[os.environ['ALLOCATION']])['Addresses'][0]
        target = address.get('InstanceId')
        instances = [i for r in ec2.describe_instances(InstanceIds=ids)['Reservations'] for i in r['Instances']] if ids else []
        ready = [i for i in instances if i.get('State', {}).get('Name') == 'running' and
                 {t['Key']: t['Value'] for t in i.get('Tags', [])}.get('SpotReady') == 'true']
        result['compute_ready'] = bool(ready)
        result['ready_instance_ids'] = [i['InstanceId'] for i in ready]
        result['cutover_enabled'] = os.environ.get('CUTOVER') == 'true'
        result['ssh_endpoint_ready'] = target in result['ready_instance_ids']
        source = os.environ.get('PROTECTED_SOURCE')
        if source and target == source:
            original = [i for r in ec2.describe_instances(InstanceIds=[source])['Reservations'] for i in r['Instances']]
            if len(original) == 1 and original[0]['State']['Name'] == 'running':
                result['ssh_endpoint_ready'] = True
                result['endpoint_instance_id'] = source
                result['protected_source_active'] = True
        elif result['ssh_endpoint_ready']:
            result['endpoint_instance_id'] = target
        if result['ssh_endpoint_ready']:
            result['state'] = 'ready'
    return result
