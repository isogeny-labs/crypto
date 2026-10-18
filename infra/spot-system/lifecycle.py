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
        return {"state": "staged", "reason": "Workload migration/cutover is not enabled"}
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
        if target in ids:
            instances = ec2.describe_instances(InstanceIds=[target])['Reservations'][0]['Instances']
            if {t['Key']: t['Value'] for t in instances[0].get('Tags', [])}.get('SpotReady') == 'true':
                result['state'] = 'ready'
    return result
