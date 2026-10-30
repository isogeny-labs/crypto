#!/usr/bin/env bash
# Reproducer for the observed authorization check. --dry-run creates no instance.
# A permitted AWS dry run also exits nonzero, with error code DryRunOperation.
set -euo pipefail
aws ec2 run-instances \
  --region us-east-1 \
  --dry-run \
  --launch-template 'LaunchTemplateId=lt-013690ecd37cbddf1,Version=$Default' \
  --image-id ami-0ee616dc75412341a \
  --instance-type m7i.4xlarge \
  --subnet-id subnet-052a1486b71ae8df1 \
  --security-group-ids sg-0c77e42f1ff01e296 \
  --block-device-mappings 'DeviceName=/dev/xvda,Ebs={VolumeSize=100,VolumeType=gp3,Encrypted=true,DeleteOnTermination=true}' \
  --tag-specifications \
    'ResourceType=instance,Tags=[{Key=Name,Value=ephemeral-fd-e12b-od-preflight},{Key=ephemeral:managed,Value=true},{Key=ephemeral:job,Value=fd-e12b-od-preflight}]' \
    'ResourceType=volume,Tags=[{Key=ephemeral:managed,Value=true},{Key=ephemeral:job,Value=fd-e12b-od-preflight}]' \
  --query 'Instances[0].InstanceId' \
  --output text
