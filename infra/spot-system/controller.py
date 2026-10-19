"""Scheduled Elastic IP reconciliation. AWS Lambda supplies boto3."""
import logging
import os
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
    if source_state(ec2, source, confirmed) != 'terminated':
        return
    states = {source: 'terminated'}
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
        if record.get('OriginTerminated', {}).get('BOOL') is not True:
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


def reconcile(ec2, autoscaling, group, allocation, enabled, source='', ownership=None):
    if not enabled:
        return {"state": "staged"}
    owners=ownership if isinstance(ownership,list) else [ownership or {}]
    if source and any(o and o.get('Origin',{}).get('S',source)!=source for o in owners):
        raise ValueError('Legacy ownership origin mismatch')
    ownership={'OriginTerminated':{'BOOL':any(o.get('OriginTerminated',{}).get('BOOL') is True for o in owners)},
               'Complete':{'BOOL':all(o.get('Complete',{}).get('BOOL') is True for o in owners)}}
    if source:
        if source_state(ec2, source, ownership) != 'terminated':
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
        ec2 = boto3.client('ec2')
        if records:
            retain_confirmed_termination(db, ec2, os.environ['LOCK_TABLE'], os.environ['PROTECTED_SOURCE'], records)
        ownership = [records[name] for name in owner_keys] if records else []
        autoscaling = boto3.client('autoscaling')
        if os.environ.get('PROTECTED_SOURCE'):
            release_legacy_cpu_restriction(autoscaling, os.environ['GROUP'], all(o.get('Complete',{}).get('BOOL') is True for o in ownership))
        return reconcile(ec2, autoscaling,
                         os.environ["GROUP"], os.environ["ALLOCATION"], True,
                         os.environ.get('PROTECTED_SOURCE',''), ownership)
    finally:
        db.delete_item(TableName=os.environ["LOCK_TABLE"], Key=key,
                       ConditionExpression="#owner = :owner",
                       ExpressionAttributeNames={"#owner": "Owner"},
                       ExpressionAttributeValues={":owner": {"S": context.aws_request_id}})
