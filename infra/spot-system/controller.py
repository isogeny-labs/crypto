"""Scheduled Elastic IP reconciliation. AWS Lambda supplies boto3."""
import logging
import os
import time

log = logging.getLogger(__name__)
log.setLevel(logging.INFO)


def reconcile(ec2, autoscaling, group, allocation, enabled):
    if not enabled:
        return {"state": "staged"}
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
        return reconcile(boto3.client("ec2"), boto3.client("autoscaling"),
                         os.environ["GROUP"], os.environ["ALLOCATION"], True)
    finally:
        db.delete_item(TableName=os.environ["LOCK_TABLE"], Key=key,
                       ConditionExpression="#owner = :owner",
                       ExpressionAttributeNames={"#owner": "Owner"},
                       ExpressionAttributeValues={":owner": {"S": context.aws_request_id}})
