#!/usr/bin/env python3
"""Build a staged CloudFormation deployment using the inspected Ohio network."""
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent


def ref(name):
    return {"Ref": name}


def sub(value):
    return {"Fn::Sub": value}


def build():
    bootstrap = """#!/bin/bash
set -euo pipefail
# AL2023 supplies /usr/bin/aws in the AMI; there is no awscli2 RPM.
command -v aws
dnf install -y amazon-efs-utils python3 postgresql17-server zstd libnet libnl3 protobuf-c libaio git gcc make openssl-devel pkgconf-pkg-config
mkdir -p /srv/experiments
echo '${FileSystem}:/ /srv/experiments efs _netdev,tls,iam,noresvport 0 0' >> /etc/fstab
mount /srv/experiments
chown ec2-user:ec2-user /srv/experiments
# Fail closed if the operator has not finished durable migration and the job manifest.
test -f '${WorkRoot}/jobs.json'
aws s3 cp 's3://${RuntimeBucket}/runtime/${RuntimeDigest}.tar.gz' /tmp/spot-runtime.tar.gz --region '${AWS::Region}'
echo '${RuntimeDigest}  /tmp/spot-runtime.tar.gz' | sha256sum -c -
mkdir -p /opt/spot
tar -xzf /tmp/spot-runtime.tar.gz -C /opt/spot
mkdir -p /opt/spot-criu
for module in criu_local.py criu_export.py legacy_recovery.py; do
  install -m 700 "/opt/spot/$module" "/opt/spot-criu/$module"
done
install -m 700 /opt/spot/bin/criu-4.2.1 /opt/spot-criu/criu
install -m 644 /opt/spot/spot-legacy-backup.service /opt/spot/spot-legacy-backup.timer /opt/spot/spot-diem-backup.service /opt/spot/spot-diem-backup.timer /etc/systemd/system/
# Both producers use the same reviewed modules on replacements.
mkdir -p /opt/spot-criu-diem
for module in criu_local.py criu_export.py legacy_recovery.py; do
  ln -sf "/opt/spot-criu/$module" "/opt/spot-criu-diem/$module"
done
# Reusing existing private host keys requires explicit operator authorization.
# The default keeps each replacement's own keys; verify fingerprints through SSM.
if [ '${PreserveHostKeys}' = 'true' ]; then
  test -f /srv/experiments/ssh-host-keys/ssh_host_ed25519_key
  cp -a /srv/experiments/ssh-host-keys/ssh_host_* /etc/ssh/
  restorecon -R /etc/ssh
  systemctl restart sshd
fi
mkdir -p '${NodeProject}'
chown ec2-user:ec2-user '${NodeProject}'
# Preserve durable checkouts. Populate a new empty workspace without fetching
# historical research blobs; no checkout is overwritten during replacement.
if [ ! -d '${NodeProject}/.git' ]; then
  test -z "$(ls -A '${NodeProject}')"
  runuser -u ec2-user -- git clone --filter=blob:none --depth 1 --sparse --no-checkout https://github.com/aburan28/crypto.git '${NodeProject}'
  runuser -u ec2-user -- git -C '${NodeProject}' fetch --depth 1 origin fa8948a7f90b05c1fb127a908ac083ef5be69208
  runuser -u ec2-user -- git -C '${NodeProject}' sparse-checkout set src examples benches .cargo .agents scripts docs assets problems tests
  runuser -u ec2-user -- git -C '${NodeProject}' checkout --detach fa8948a7f90b05c1fb127a908ac083ef5be69208
fi
if [ ! -x /home/ec2-user/.cargo/bin/cargo ]; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o /tmp/spot-rustup.sh
  runuser -u ec2-user -- sh /tmp/spot-rustup.sh -y --profile minimal --default-toolchain 1.99.1
fi
runuser -u ec2-user -- python3 /opt/spot/runner.py --root '${WorkRoot}' --validate
# Prove durable checkpoint selection/restore before publishing readiness.
# The private temporary workspace is separate from registered workloads.
runuser -u ec2-user -- env SPOT_SELFTEST_RUNTIME=/opt/spot SPOT_SELFTEST_PARENT='${WorkRoot}' /opt/spot/bin/checkpoint-selftest --exact supervisor_restores_a_discovered_generation_after_abrupt_worker_death --nocapture > /var/log/spot-checkpoint-selftest.log 2>&1
runuser -u ec2-user -- env SPOT_NATIVE_TEST_PARENT='${WorkRoot}' /opt/spot/bin/solver-checkpoint-selftest --exact cryptanalysis::koblitz_groebner::checkpoint::tests::real_solver_recovers_after_sigkill_and_rejects_corruption --nocapture > /var/log/spot-native-checkpoint-selftest.log 2>&1
# Future jobs reference this immutable runtime path, not the live legacy binary.
mkdir -p '${WorkRoot}/bin/${RuntimeDigest}'
install -m 755 /opt/spot/bin/factor-base-shape-ladder '${WorkRoot}/bin/${RuntimeDigest}/factor_base_shape_ladder'
python3 /opt/spot/install_services.py --root '${WorkRoot}' --project '${NodeProject}' --table '${ControllerLock}' --region '${AWS::Region}' --legacy-origin '${ProtectedSource}'
node_ready=0
for attempt in $(seq 150); do
  if curl -fsS --max-time 2 http://127.0.0.1:9180/health >/dev/null && curl -fsS --max-time 2 http://127.0.0.1:8080/ >/dev/null; then
    node_ready=1
    break
  fi
  sleep 2
done
test "$node_ready" -eq 1
systemctl is-active --quiet sshd
systemctl is-active --quiet spot-experiments
systemctl is-active --quiet spot-activity
systemctl is-active --quiet spot-checkpoint-discovery.timer
TOKEN=$(curl -fsS --max-time 3 -X PUT -H 'X-aws-ec2-metadata-token-ttl-seconds: 60' http://169.254.169.254/latest/api/token)
INSTANCE=$(curl -fsS --max-time 3 -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/instance-id)
aws ec2 create-tags --region '${AWS::Region}' --resources "$INSTANCE" --tags Key=SpotReady,Value=true
"""
    resources = {}
    def resource(name, kind, properties, **extra):
        resources[name] = {"Type": kind, "Properties": properties, **extra}
    resource('RuntimeBucket', 'AWS::S3::Bucket', {
        'VersioningConfiguration': {'Status': 'Enabled'},
        'BucketEncryption': {'ServerSideEncryptionConfiguration': [{'ServerSideEncryptionByDefault': {'SSEAlgorithm': 'AES256'}}]},
        'PublicAccessBlockConfiguration': {'BlockPublicAcls': True, 'BlockPublicPolicy': True,
            'IgnorePublicAcls': True, 'RestrictPublicBuckets': True}},
        DeletionPolicy='Retain', UpdateReplacePolicy='Retain')
    resource('RuntimeBucketPolicy', 'AWS::S3::BucketPolicy', {'Bucket': ref('RuntimeBucket'),
        'PolicyDocument': {'Version': '2012-10-17', 'Statement': [{'Effect': 'Deny', 'Principal': '*',
            'Action': 's3:*', 'Resource': [sub('${RuntimeBucket.Arn}'), sub('${RuntimeBucket.Arn}/*')],
            'Condition': {'Bool': {'aws:SecureTransport': 'false'}}}]}})
    resource("FileSystem", "AWS::EFS::FileSystem", {
        "Encrypted": True, "PerformanceMode": "generalPurpose", "ThroughputMode": "elastic",
        "BackupPolicy": {"Status": "ENABLED"},
        "FileSystemTags": [{"Key": "Name", "Value": sub("${AWS::StackName}-experiments")}],
        "FileSystemPolicy": {"Version": "2012-10-17", "Statement": [{
            "Effect": "Deny", "Principal": "*", "Action": "elasticfilesystem:Client*",
            "Resource": "*", "Condition": {"Bool": {"aws:SecureTransport": "false"}}}, {
            "Effect": "Allow", "Principal": {"AWS": [
                {"Fn::GetAtt": ["WorkerRole", "Arn"]},
                sub("arn:${AWS::Partition}:iam::${AWS::AccountId}:role/InstanceRole")]},
            "Action": ["elasticfilesystem:ClientMount", "elasticfilesystem:ClientWrite",
                       "elasticfilesystem:ClientRootAccess"],
            "Resource": "*", "Condition": {"Bool": {"aws:SecureTransport": "true",
                "elasticfilesystem:AccessedViaMountTarget": "true"}}}]},
    }, DeletionPolicy="Retain", UpdateReplacePolicy="Retain")
    resource("EfsSecurityGroup", "AWS::EC2::SecurityGroup", {
        "GroupDescription": "NFS only from experiment hosts", "VpcId": ref("VpcId"),
        "SecurityGroupIngress": [{"IpProtocol": "tcp", "FromPort": 2049, "ToPort": 2049,
                                  "SourceSecurityGroupId": ref("HostSecurityGroup")}],
    })
    for i in range(3):
        resource(f"Mount{i}", "AWS::EFS::MountTarget", {
            "FileSystemId": ref("FileSystem"),
            "SubnetId": {"Fn::Select": [i, ref("Subnets")]},
            "SecurityGroups": [ref("EfsSecurityGroup")]})
    resource("WorkerRole", "AWS::IAM::Role", {
        "AssumeRolePolicyDocument": {"Version": "2012-10-17", "Statement": [{
            "Effect": "Allow", "Principal": {"Service": "ec2.amazonaws.com"}, "Action": "sts:AssumeRole"}]},
        "ManagedPolicyArns": [sub("arn:${AWS::Partition}:iam::aws:policy/AmazonSSMManagedInstanceCore")],
        "Policies": [{"PolicyName": "OwnReadiness", "PolicyDocument": {
            "Version": "2012-10-17", "Statement": [
                {"Effect": "Allow", "Action": "s3:GetObject", "Resource": [sub("${RuntimeBucket.Arn}/runtime/*"),sub("${RuntimeBucket.Arn}/checkpoints/criu/*")]},
                {"Effect":"Allow","Action":["s3:PutObject","s3:AbortMultipartUpload"],"Resource":sub("${RuntimeBucket.Arn}/checkpoints/criu/*")},
                {"Effect":"Allow","Action":"ec2:DescribeInstances","Resource":"*"},
                {"Effect": "Allow", "Action": "autoscaling:DescribeAutoScalingInstances", "Resource": "*"},
                {"Effect": "Allow", "Action": "dynamodb:PutItem",
                 "Resource": {"Fn::GetAtt": ["ControllerLock", "Arn"]},
                 "Condition": {"ForAllValues:StringLike": {"dynamodb:LeadingKeys": ["activity#*", "lifecycle-lock", "legacy-owner*"]}}},
                {"Effect":"Allow","Action":"dynamodb:UpdateItem","Resource":{"Fn::GetAtt":["ControllerLock","Arn"]},
                 "Condition":{"ForAllValues:StringLike":{"dynamodb:LeadingKeys":["legacy-owner*"]}}},
                {"Effect": "Allow", "Action": "dynamodb:GetItem",
                 "Resource": {"Fn::GetAtt": ["ControllerLock", "Arn"]},
                 "Condition": {"ForAllValues:StringLike": {"dynamodb:LeadingKeys": ["control", "activity#*", "legacy-owner*"]}}},
                {"Effect": "Allow", "Action": "dynamodb:DeleteItem",
                 "Resource": {"Fn::GetAtt": ["ControllerLock", "Arn"]},
                 "Condition": {"ForAllValues:StringEquals": {"dynamodb:LeadingKeys": ["lifecycle-lock"]}}},
                {"Effect": "Allow", "Action": "ec2:CreateTags",
                "Resource": sub("arn:${AWS::Partition}:ec2:${AWS::Region}:${AWS::AccountId}:instance/*"),
                "Condition": {"StringEquals": {"ec2:ResourceTag/SpotSystem": ref("AWS::StackName")},
                              "ForAllValues:StringEquals": {"aws:TagKeys": ["SpotReady", "LegacyReady"]}}}]}}]})
    resource("WorkerProfile", "AWS::IAM::InstanceProfile", {"Roles": [ref("WorkerRole")]})
    resource("Template", "AWS::EC2::LaunchTemplate", {"LaunchTemplateData": {
        "ImageId": ref("ImageId"), "KeyName": "rootkey",
        "IamInstanceProfile": {"Name": ref("WorkerProfile")},
        "NetworkInterfaces": [{"DeviceIndex": 0, "AssociatePublicIpAddress": True,
                               "Groups": [ref("HostSecurityGroup")]}],
        "MetadataOptions": {"HttpTokens": "required", "HttpEndpoint": "enabled"},
        "BlockDeviceMappings": [{"DeviceName": "/dev/xvda", "Ebs": {
            "VolumeSize": 100, "VolumeType": "gp3", "Encrypted": True, "DeleteOnTermination": True}}],
        "UserData": {"Fn::Base64": sub(bootstrap)},
    }})
    resource("Group", "AWS::AutoScaling::AutoScalingGroup", {
        # The wake/idle controller owns desired capacity. Including a zero
        # here reapplies it on stack updates and terminates active workers.
        # Creation defaults to MinSize; later updates preserve current demand.
        "MinSize": "0", "MaxSize": "2",
        "CapacityRebalance": True, "VPCZoneIdentifier": ref("Subnets"),
        "HealthCheckType": "EC2", "HealthCheckGracePeriod": 600,
        "MixedInstancesPolicy": {"InstancesDistribution": {
            "OnDemandBaseCapacity": 0, "OnDemandPercentageAboveBaseCapacity": 0,
            "SpotAllocationStrategy": "price-capacity-optimized"},
            "LaunchTemplate": {"LaunchTemplateSpecification": {"LaunchTemplateId": ref("Template"),
                "Version": {"Fn::GetAtt": ["Template", "LatestVersionNumber"]}}, "Overrides": [{
                    "InstanceRequirements": {"VCpuCount": {"Min": 32, "Max": 32},
                        "MemoryMiB": {"Min": 131072, "Max": 131072},
                        "CpuManufacturers": ["intel", "amd"], "BurstablePerformance": "excluded",
                        "AcceleratorCount": {"Max": 0}, "LocalStorage": "excluded",
                        "AllowedInstanceTypes": {"Fn::If":["LegacyProtection",["m8i.*"],["m6i.*", "m6a.*", "m7i.*", "m7a.*", "m8i.*", "m8a.*"]]},
                        "MaxSpotPriceAsPercentageOfOptimalOnDemandPrice": ref("PriceProtectionPercent")}}]}},
        "Tags": [{"Key": "SpotSystem", "Value": ref("AWS::StackName"), "PropagateAtLaunch": True}],
    }, DependsOn=[f"Mount{i}" for i in range(3)])
    resource("ControllerRole", "AWS::IAM::Role", {
        "AssumeRolePolicyDocument": {"Version": "2012-10-17", "Statement": [{
            "Effect": "Allow", "Principal": {"Service": "lambda.amazonaws.com"}, "Action": "sts:AssumeRole"}]},
        "ManagedPolicyArns": [sub("arn:${AWS::Partition}:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole")],
        "Policies": [{"PolicyName": "ReconcileAddress", "PolicyDocument": {"Version": "2012-10-17", "Statement": [
            {"Effect": "Allow", "Action": ["dynamodb:GetItem", "dynamodb:PutItem", "dynamodb:DeleteItem"],
                "Resource": {"Fn::GetAtt": ["ControllerLock", "Arn"]}},
            {"Effect":"Allow","Action":"dynamodb:UpdateItem","Resource":{"Fn::GetAtt":["ControllerLock","Arn"]},
             "Condition":{"ForAllValues:StringLike":{"dynamodb:LeadingKeys":["legacy-owner*"]}}},
            {"Effect":"Allow","Action":["autoscaling:SetDesiredCapacity","autoscaling:UpdateAutoScalingGroup"],
             "Resource":sub("arn:${AWS::Partition}:autoscaling:${AWS::Region}:${AWS::AccountId}:autoScalingGroup:*:autoScalingGroupName/${Group}")},
            {"Effect": "Allow", "Action": ["ec2:DescribeInstances", "ec2:DescribeInstanceStatus",
                "ec2:DescribeAddresses", "autoscaling:DescribeAutoScalingGroups"], "Resource": "*"},
            {"Effect": "Allow", "Action": "ec2:AssociateAddress",
                "Resource": sub("arn:${AWS::Partition}:ec2:${AWS::Region}:${AWS::AccountId}:elastic-ip/${AllocationId}")},
            {"Effect": "Allow", "Action": "ec2:AssociateAddress",
                "Resource": sub("arn:${AWS::Partition}:ec2:${AWS::Region}:${AWS::AccountId}:instance/*"),
                "Condition": {"StringEquals": {"ec2:ResourceTag/SpotSystem": ref("AWS::StackName")}}},
        ]}}]})
    resource("ControllerLock", "AWS::DynamoDB::Table", {
        "BillingMode": "PAY_PER_REQUEST", "AttributeDefinitions": [{"AttributeName": "Id", "AttributeType": "S"}],
        "KeySchema": [{"AttributeName": "Id", "KeyType": "HASH"}],
        "TimeToLiveSpecification": {"AttributeName": "Expires", "Enabled": True},
        "SSESpecification": {"SSEEnabled": True}})
    resource("Controller", "AWS::Lambda::Function", {
        "Runtime": "python3.12", "Handler": "index.handler", "Timeout": 30,
        "Role": {"Fn::GetAtt": ["ControllerRole", "Arn"]},
        "Environment": {"Variables": {"GROUP": ref("Group"), "ALLOCATION": ref("AllocationId"),
                                      "ENABLED": ref("Cutover"), "LOCK_TABLE": ref("ControllerLock"), "PROTECTED_SOURCE":ref("ProtectedSource"),
                                      "LEGACY_OWNER_KEYS":"legacy-owner,legacy-owner-diem"}},
        "Code": {"ZipFile": (HERE / "controller.py").read_text()}})
    resource("ReconcileTimer", "AWS::Events::Rule", {"ScheduleExpression": "rate(1 minute)",
        "State": "ENABLED", "Targets": [{"Id": "controller", "Arn": {"Fn::GetAtt": ["Controller", "Arn"]}}]})
    resource("InvokePermission", "AWS::Lambda::Permission", {
        "Action": "lambda:InvokeFunction", "FunctionName": ref("Controller"),
        "Principal": "events.amazonaws.com", "SourceArn": {"Fn::GetAtt": ["ReconcileTimer", "Arn"]}})
    resource('LifecycleRole', 'AWS::IAM::Role', {
        'AssumeRolePolicyDocument': {'Version': '2012-10-17', 'Statement': [{'Effect': 'Allow',
            'Principal': {'Service': 'lambda.amazonaws.com'}, 'Action': 'sts:AssumeRole'}]},
        'ManagedPolicyArns': [sub('arn:${AWS::Partition}:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole')],
        'Policies': [{'PolicyName': 'WakeAndIdle', 'PolicyDocument': {'Version': '2012-10-17', 'Statement': [
            {'Effect': 'Allow', 'Action': ['dynamodb:GetItem', 'dynamodb:PutItem', 'dynamodb:DeleteItem'],
                'Resource': {'Fn::GetAtt': ['ControllerLock', 'Arn']}},
            {'Effect': 'Allow', 'Action': ['autoscaling:DescribeAutoScalingGroups', 'ec2:DescribeInstances', 'ec2:DescribeAddresses'], 'Resource': '*'},
            {'Effect': 'Allow', 'Action': 'autoscaling:SetDesiredCapacity',
                'Resource': sub('arn:${AWS::Partition}:autoscaling:${AWS::Region}:${AWS::AccountId}:autoScalingGroup:*:autoScalingGroupName/${Group}')},
            {'Effect':'Allow','Action':'ssm:SendCommand','Resource':sub('arn:${AWS::Partition}:ssm:${AWS::Region}::document/AWS-RunShellScript')},
            {'Effect':'Allow','Action':'ssm:SendCommand','Resource':sub('arn:${AWS::Partition}:ec2:${AWS::Region}:${AWS::AccountId}:instance/*'),
             'Condition':{'StringEquals':{'ssm:resourceTag/SpotSystem':ref('AWS::StackName')}}},
            {'Effect':'Allow','Action':'ssm:GetCommandInvocation','Resource':'*'}
        ]}}]})
    resource('Lifecycle', 'AWS::Lambda::Function', {'Runtime': 'python3.12', 'Handler': 'index.handler', 'Timeout': 30,
        'Role': {'Fn::GetAtt': ['LifecycleRole', 'Arn']},
        'Code': {'ZipFile': (HERE/'lifecycle.py').read_text()},
        'Environment': {'Variables': {'GROUP': ref('Group'), 'TABLE': ref('ControllerLock'),
            'ALLOCATION': ref('AllocationId'), 'IDLE_SECONDS': ref('IdleSeconds'),
            'CUTOVER': ref('Cutover'),
            'PROTECTED_SOURCE': ref('ProtectedSource'),
            'ENABLED': {'Fn::If': ['LifecycleActive', 'true', 'false']}}}})
    resource('IdleTimer', 'AWS::Events::Rule', {'ScheduleExpression': 'rate(1 minute)', 'State': 'ENABLED',
        'Targets': [{'Id': 'lifecycle', 'Arn': {'Fn::GetAtt': ['Lifecycle', 'Arn']}}]})
    resource('IdleInvokePermission', 'AWS::Lambda::Permission', {'Action': 'lambda:InvokeFunction',
        'FunctionName': ref('Lifecycle'), 'Principal': 'events.amazonaws.com',
        'SourceArn': {'Fn::GetAtt': ['IdleTimer', 'Arn']}})
    resource("Errors", "AWS::CloudWatch::Alarm", {"AlarmDescription": "Spot IP controller failures",
        "Namespace": "AWS/Lambda", "MetricName": "Errors", "Statistic": "Sum", "Period": 300,
        "EvaluationPeriods": 1, "Threshold": 1, "ComparisonOperator": "GreaterThanOrEqualToThreshold",
        "TreatMissingData": "notBreaching", "Dimensions": [{"Name": "FunctionName", "Value": ref("Controller")}],
        "AlarmActions": {"Fn::If": ["HasAlarmTopic", [ref("AlarmTopicArn")], ref("AWS::NoValue")]}})
    return {"AWSTemplateFormatVersion": "2010-09-09", "Description": "Staged Spot-only experiment host with durable EFS and movable existing EIP",
        "Parameters": {
            "VpcId": {"Type": "AWS::EC2::VPC::Id", "Default": "vpc-09e1badf3d477f243"},
            "Subnets": {"Type": "List<AWS::EC2::Subnet::Id>", "Default": "subnet-0761aef3e12b3bdba,subnet-0f7c5dbe095545329,subnet-09342db00f6592d1f"},
            "HostSecurityGroup": {"Type": "AWS::EC2::SecurityGroup::Id", "Default": "sg-06e59b5aec8715bf3"},
            "ImageId": {"Type": "AWS::EC2::Image::Id", "Default": "ami-0d31a22b09ceec4db",
                "Description": "AL2023 x86_64 AMI; bootstrap installs the supervisor. Install workload dependencies before cutover."},
            "AllocationId": {"Type": "String", "Default": "eipalloc-00c2a8a867901bf7d"},
            'ProtectedSource': {'Type':'String','Default':'i-01f85a6c905181498','AllowedPattern':'(i-[a-f0-9]+|)'},
            'LifecycleEnabled': {'Type': 'String', 'Default': 'false', 'AllowedValues': ['false', 'true']},
            'IdleSeconds': {'Type': 'Number', 'Default': 1800, 'MinValue': 300, 'MaxValue': 86400},
            'RuntimeDigest': {'Type': 'String', 'Default': (HERE/'runtime.tar.sha256').read_text().strip()
                if (HERE/'runtime.tar.sha256').exists() else 'unpublished', 'AllowedPattern': '([a-f0-9]{64}|unpublished)'},
            'NodeProject': {'Type': 'String', 'Default': '/srv/experiments/work',
                'AllowedPattern': '/srv/experiments/[A-Za-z0-9_-]+(/[A-Za-z0-9_-]+)*'},
            'WorkRoot': {'Type': 'String', 'Default': '/srv/experiments',
                'AllowedPattern': '/srv/experiments(/[A-Za-z0-9_-]+)*',
                'Description': 'Existing durable workspace with jobs.json; isolates managed workers from legacy node/work state.'},
            "Cutover": {"Type": "String", "Default": "false", "AllowedValues": ["false", "true"]},
            "PreserveHostKeys": {"Type": "String", "Default": "false", "AllowedValues": ["false", "true"],
                "Description": "Enable only after approval to store the original SSH private host keys in EFS."},
            "PriceProtectionPercent": {"Type": "Number", "Default": 60, "MinValue": 1, "MaxValue": 100,
                "Description": "Spot price limit as percent of optimal On-Demand price; limits may reduce capacity."},
            "AlarmTopicArn": {"Type": "String", "Default": ""}},
        "Conditions": {"HasAlarmTopic": {"Fn::Not": [{"Fn::Equals": [ref("AlarmTopicArn"), ""]}]},
            'LegacyProtection': {'Fn::Not':[{'Fn::Equals':[ref('ProtectedSource'),'']}]},
            'LifecycleActive': {'Fn::Equals': [ref('LifecycleEnabled'), 'true']}
        },
        "Resources": resources,
        "Outputs": {"FileSystemId": {"Value": ref("FileSystem")}, "GroupName": {"Value": ref("Group")},
                    "ControllerName": {"Value": ref("Controller")}, 'RuntimeBucketName': {'Value': ref('RuntimeBucket')},
                    'LifecycleFunctionArn': {'Value': {'Fn::GetAtt': ['Lifecycle', 'Arn']}}}}


if __name__ == "__main__":
    print(json.dumps(build(), indent=2))
