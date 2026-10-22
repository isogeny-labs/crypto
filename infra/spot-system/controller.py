"""Scheduled Elastic IP reconciliation. AWS Lambda supplies boto3."""
import logging
import os
import re
import time

log = logging.getLogger(__name__)
log.setLevel(logging.INFO)


def release_legacy_cpu_restriction(autoscaling, group, complete=True):
    current = autoscaling.describe_auto_scaling_groups(AutoScalingGroupNames=[group])['AutoScalingGroups'][0]
    policy = current.get('MixedInstancesPolicy', {})
    overrides = policy.get('LaunchTemplate', {}).get('Overrides', [])
    desired=['m6i.*','m6a.*','m7i.*','m7a.*','m8i.*','m8a.*'] if complete else ['m8i.*']
    if len(overrides)==1 and overrides[0].get('InstanceRequirements') and overrides[0]['InstanceRequirements'].get('AllowedInstanceTypes')!=desired:
        requirements = overrides[0]['InstanceRequirements']
        requirements['AllowedInstanceTypes'] = desired
        requirements['CpuManufacturers'] = ['intel','amd']
        specification = policy['LaunchTemplate']['LaunchTemplateSpecification']
        if specification.get('LaunchTemplateId'):
            specification.pop('LaunchTemplateName', None)
        autoscaling.update_auto_scaling_group(AutoScalingGroupName=group, MixedInstancesPolicy=policy)


def source_state(ec2, source, ownership):
    if ownership.get('OriginTerminated', {}).get('BOOL') is True:
        return 'terminated'
    if (ownership.get('OriginReleased', {}).get('BOOL') is True
            and re.fullmatch(r'[0-9a-f]{64}', ownership.get('ReleaseReceipt', {}).get('S', ''))):
        return 'released'
    instances = [i for r in ec2.describe_instances(InstanceIds=[source])['Reservations'] for i in r['Instances']]
    if len(instances) != 1:
        raise RuntimeError('Protected source state is unknown')
    return instances[0]['State']['Name']


def retain_confirmed_termination(db, ec2, table, source, records):
    """Keep irreversible death evidence through long Spot capacity gaps.

    Missing/expired heartbeats and missing EC2 records are not death evidence.
    Conditional writes cannot attach an old owner's death to its successor.
    """
    confirmed = {'OriginTerminated': {'BOOL': any(
        record.get('OriginTerminated', {}).get('BOOL') is True for record in records.values())}}
    released = [r for r in records.values() if origin_released(r, source)]
    if released and all(r.get('Complete', {}).get('BOOL') is True or
                        origin_released(r, source) for r in records.values()):
        confirmed.update(OriginReleased={'BOOL':True}, ReleaseReceipt=released[0]['ReleaseReceipt'])
    original_state = source_state(ec2, source, confirmed)
    if original_state not in {'terminated', 'released'}:
        return
    states = {source: original_state}
    for key, record in records.items():
        if not record:
            continue
        if record.get('Origin', {}).get('S') != source:
            raise ValueError('Legacy ownership origin mismatch')
        owner = record['Owner']['S']
        dead = record.get('OwnerTerminated', {}).get('BOOL') is True
        if not dead:
            if owner not in states:
                states[owner] = source_state(ec2, owner, {})
            dead = states[owner] == 'terminated'
        changes = {}
        if original_state == 'terminated' and record.get('OriginTerminated', {}).get('BOOL') is not True:
            changes['OriginTerminated'] = {'BOOL': True}
        if dead and record.get('OwnerTerminated', {}).get('BOOL') is not True:
            changes['OwnerTerminated'] = {'BOOL': True}
        if changes:
            values = {':'+name:value for name,value in changes.items()}
            db.update_item(TableName=table, Key={'Id': {'S': key}},
                UpdateExpression='SET '+', '.join(name+' = :'+name for name in changes),
                ConditionExpression='#owner = :owner AND #origin = :origin',
                ExpressionAttributeNames={'#owner':'Owner', '#origin':'Origin'},
                ExpressionAttributeValues={**values, ':owner':{'S':owner}, ':origin':{'S':source}})
            record.update(changes)


def origin_released(record, source):
    return (record.get('Origin', {}).get('S') == source
            and record.get('OriginReleased', {}).get('BOOL') is True
            and re.fullmatch(r'[0-9a-f]{64}', record.get('ReleaseReceipt', {}).get('S', '')) is not None)


def reconcile(ec2, autoscaling, group, allocation, enabled, source='', ownership=None):
    if not enabled:
        return {"state": "staged"}
    owners=ownership if isinstance(ownership,list) else [ownership or {}]
    if source and any(o and o.get('Origin',{}).get('S',source)!=source for o in owners):
        raise ValueError('Legacy ownership origin mismatch')
    ownership={'OriginTerminated':{'BOOL':any(o.get('OriginTerminated',{}).get('BOOL') is True for o in owners)},
               'Complete':{'BOOL':all(o.get('Complete',{}).get('BOOL') is True for o in owners)}}
    released = [o for o in owners if origin_released(o, source)]
    if released and all(o.get('Complete',{}).get('BOOL') is True or
                        origin_released(o, source) for o in owners):
        ownership.update(OriginReleased={'BOOL':True}, ReleaseReceipt=released[0]['ReleaseReceipt'])
    if source:
        if source_state(ec2, source, ownership) not in {'terminated', 'released'}:
            return {'state':'protected-source-active','instance':source}
        if ownership.get('Complete', {}).get('BOOL') is not True:
            # Demand persists through Spot capacity gaps; root recovery uses the
            # same irreversible-death ownership fence before running an image.
            autoscaling.set_desired_capacity(AutoScalingGroupName=group,
                                            DesiredCapacity=1, HonorCooldown=False)
    groups = autoscaling.describe_auto_scaling_groups(
        AutoScalingGroupNames=[group])["AutoScalingGroups"]
    if len(groups) != 1:
        raise RuntimeError("Expected exactly one Auto Scaling group")
    ids = [i["InstanceId"] for i in groups[0]["Instances"]
           if i["LifecycleState"] == "InService" and i["HealthStatus"] == "Healthy"]
    if not ids:
        return {"state": "waiting-for-capacity"}
    instances = [i for r in ec2.describe_instances(InstanceIds=ids)["Reservations"]
                 for i in r["Instances"]]
    statuses = ec2.describe_instance_status(InstanceIds=ids)["InstanceStatuses"]
    healthy = {s["InstanceId"] for s in statuses
               if s["SystemStatus"]["Status"] == "ok"
               and s["InstanceStatus"]["Status"] == "ok"}
    ready = [i for i in instances if i["InstanceId"] in healthy
             and i["State"]["Name"] == "running"
             and {t["Key"]: t["Value"] for t in i.get("Tags", [])}.get("SpotReady") == "true"]
    if source and ownership.get('Complete', {}).get('BOOL') is not True:
        ready = [i for i in ready if all(o.get('Complete',{}).get('BOOL') is True or
                 (i['InstanceId']==o.get('Owner',{}).get('S') and o.get('Ready',{}).get('BOOL') is True) for o in owners)
                 and {t['Key']:t['Value'] for t in i.get('Tags', [])}.get('LegacyReady') == 'true']
    if not ready:
        return {"state": "waiting-for-bootstrap"}
    newest = max(ready, key=lambda i: (i["LaunchTime"], i["InstanceId"]))
    address = ec2.describe_addresses(AllocationIds=[allocation])["Addresses"][0]
    target = newest["InstanceId"]
    if address.get("InstanceId") == target:
        return {"state": "attached", "instance": target}
    ec2.associate_address(AllocationId=allocation, InstanceId=target, AllowReassociation=True)
    log.info("Moved address %s to ready instance %s", allocation, target)
    return {"state": "moved", "instance": target}


def handler(event, context):
    import boto3
    if os.environ.get("ENABLED") != "true":
        return {"state": "staged"}
    db = boto3.client("dynamodb")
    now = int(time.time())
    key = {"Id": {"S": "ip-controller"}}
    try:
        db.put_item(TableName=os.environ["LOCK_TABLE"],
                    Item={**key, "Owner": {"S": context.aws_request_id},
                          "Expires": {"N": str(now + 90)}},
                    ConditionExpression="attribute_not_exists(Id) OR Expires < :now",
                    ExpressionAttributeValues={":now": {"N": str(now)}})
    except db.exceptions.ConditionalCheckFailedException:
        return {"state": "another-controller-active"}
    try:
        owner_keys = os.environ.get('LEGACY_OWNER_KEYS','legacy-owner').split(',')
        records = {name:db.get_item(TableName=os.environ['LOCK_TABLE'],Key={'Id':{'S':name}},
            ConsistentRead=True).get('Item',{}) for name in [*owner_keys,'legacy-owner-host']} if os.environ.get('PROTECTED_SOURCE') else {}
        # Codex is optional. Once enrolled, its ownership must also retain
        # confirmed successor death across long capacity gaps and gate cutover.
        if records:
            codex = db.get_item(TableName=os.environ['LOCK_TABLE'],
                Key={'Id':{'S':'legacy-owner-codex'}}, ConsistentRead=True).get('Item',{})
            if codex:
                records['legacy-owner-codex'] = codex
        ec2 = boto3.client('ec2')
        if records:
            retain_confirmed_termination(db, ec2, os.environ['LOCK_TABLE'], os.environ['PROTECTED_SOURCE'], records)
        ownership = [records[name] for name in owner_keys] if records else []
        autoscaling = boto3.client('autoscaling')
        if os.environ.get('PROTECTED_SOURCE'):
            legacy_complete = all(o.get('Complete',{}).get('BOOL') is True for o in ownership)
            try:
                release_legacy_cpu_restriction(autoscaling, os.environ['GROUP'], legacy_complete)
            except Exception:
                # Widening the pool after recovery is optional. A denied launch
                # template update must not prevent a ready owner's IP cutover.
                # Keep failures to enforce an unfinished workload's CPU fence
                # fatal, and leave all ownership/readiness checks below intact.
                if not legacy_complete:
                    raise
                log.exception('CPU pool expansion deferred; retaining current instance restrictions')
        if 'legacy-owner-codex' in records:
            ownership.append(records['legacy-owner-codex'])
        return reconcile(ec2, autoscaling,
                         os.environ["GROUP"], os.environ["ALLOCATION"], True,
                         os.environ.get('PROTECTED_SOURCE',''), ownership)
    finally:
        db.delete_item(TableName=os.environ["LOCK_TABLE"], Key=key,
                       ConditionExpression="#owner = :owner",
                       ExpressionAttributeNames={"#owner": "Owner"},
                       ExpressionAttributeValues={":owner": {"S": context.aws_request_id}})
