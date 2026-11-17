#!/usr/bin/env python3
"""EC2 spot launcher for the rented IC-leads jobs: the AWS twin of
research/dreg_closure_m4l6_20261018/pod/launch.py. Plumbing only.

The instance fetches the same pod/boot.sh from BRANCH and runs it with the same
JOB, so a job behaves as on RunPod. Results are served on :8000 (port opened to
FETCH_CIDR, default this host's egress /32 is NOT knowable here, so 0.0.0.0/0;
the results are research logs, no secrets). The instance shuts itself down
(InstanceInitiatedShutdownBehavior=terminate) after the job plus SERVE_SECONDS,
or at the lifetime cap, whichever comes first.

Credentials: boto3's default chain (AWS_ACCESS_KEY_ID + AWS_SECRET_ACCESS_KEY,
or a profile). Nothing secret goes into user-data.

  aws_spot.py prices --types r7i.8xlarge,r6i.8xlarge,r7a.8xlarge
  aws_spot.py run --job a3 --name dreg-a3 --branch BR --type r7i.8xlarge --max-seconds 36000 [--env K=V] [--max-price 0.9]
  aws_spot.py ls | url ID | terminate ID | fetch ID DEST --path closure/runs/
"""
import argparse, json, os, subprocess, sys, time
import boto3

REGION = os.environ.get("AWS_REGION") or os.environ.get("AWS_DEFAULT_REGION") or "us-west-2"
TAG = "ic-leads-20261018"
AMI_PARAM = "/aws/service/canonical/ubuntu/server/24.04/stable/current/amd64/hvm/ebs-gp3/ami-id"

USER_DATA = r'''#!/bin/bash
# EC2 user-data: mirror of launch.py START. No secret in here.
set -u
export JOB='{job}' BRANCH='{branch}' BOOT='{boot}' POD_MAX_SECONDS='{max_seconds}' SERVE_SECONDS='{serve}' {env}
shutdown -h +$(( {max_seconds} / 60 )) "lifetime cap"           # hard cap; terminates (shutdown behaviour)
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null 2>&1; apt-get install -y -qq curl ca-certificates >/dev/null 2>&1
mkdir -p /workspace; export HOME=/root
curl -sSL "https://raw.githubusercontent.com/aburan28/crypto/$BRANCH/$BOOT" -o /root/boot.sh
# boot.sh ends with `sleep SERVE_SECONDS; kill -TERM 1`; on EC2 PID 1 is systemd, so power off after it instead.
sed -i 's/^kill -TERM 1$/true/' /root/boot.sh
bash /root/boot.sh "$JOB" >>/root/boot.log 2>&1 </dev/null
shutdown -h now "job done"
'''

def ec2():
    return boto3.client("ec2", region_name=REGION)

def ami():
    return boto3.client("ssm", region_name=REGION).get_parameter(Name=AMI_PARAM)["Parameter"]["Value"]

def security_group(c, cidr):
    vpc = c.describe_vpcs(Filters=[{"Name": "isDefault", "Values": ["true"]}])["Vpcs"][0]["VpcId"]
    name = f"{TAG}-results"
    sgs = c.describe_security_groups(Filters=[{"Name": "group-name", "Values": [name]}, {"Name": "vpc-id", "Values": [vpc]}])["SecurityGroups"]
    if sgs:
        return sgs[0]["GroupId"]
    sg = c.create_security_group(GroupName=name, Description="results on :8000 for ic-leads jobs", VpcId=vpc)["GroupId"]
    c.authorize_security_group_ingress(GroupId=sg, IpPermissions=[{"IpProtocol": "tcp", "FromPort": 8000, "ToPort": 8000, "IpRanges": [{"CidrIp": cidr}]}])
    return sg

def run(a):
    env = " ".join(f"{k}='{v}'" for k, v in (kv.split("=", 1) for kv in a.env))
    ud = USER_DATA.format(job=a.job, branch=a.branch, boot=a.boot, max_seconds=a.max_seconds, serve=a.serve, env=env)
    req = dict(
        ImageId="(ssm:" + AMI_PARAM + ")" if a.dry_run else ami(), InstanceType=a.type, MinCount=1, MaxCount=1, UserData=ud,
        InstanceInitiatedShutdownBehavior="terminate",
        BlockDeviceMappings=[{"DeviceName": "/dev/sda1", "Ebs": {"VolumeSize": a.disk, "VolumeType": "gp3", "DeleteOnTermination": True}}],
        InstanceMarketOptions={"MarketType": "spot", "SpotOptions": {"SpotInstanceType": "one-time", "InstanceInterruptionBehavior": "terminate",
                                                                     **({"MaxPrice": str(a.max_price)} if a.max_price else {})}},
        TagSpecifications=[{"ResourceType": "instance", "Tags": [{"Key": "Name", "Value": a.name}, {"Key": "project", "Value": TAG}, {"Key": "job", "Value": a.job}]}],
    )
    if a.dry_run:
        print(json.dumps({k: v for k, v in req.items() if k != "UserData"}, indent=1)); print(ud); return
    c = ec2()
    req["SecurityGroupIds"] = [security_group(c, a.cidr)]
    i = c.run_instances(**req)["Instances"][0]
    iid = i["InstanceId"]
    c.get_waiter("instance_running").wait(InstanceIds=[iid])
    d = c.describe_instances(InstanceIds=[iid])["Reservations"][0]["Instances"][0]
    print(json.dumps({"id": iid, "type": a.type, "az": d["Placement"]["AvailabilityZone"], "ip": d.get("PublicIpAddress"), "spot": d.get("SpotInstanceRequestId")}))
    print("results:", f"http://{d.get('PublicIpAddress')}:8000/")

def prices(a):
    c = ec2()
    for t in a.types.split(","):
        h = c.describe_spot_price_history(InstanceTypes=[t], ProductDescriptions=["Linux/UNIX"], MaxResults=20)["SpotPriceHistory"]
        best = {}
        for p in h:
            az = p["AvailabilityZone"]; best.setdefault(az, float(p["SpotPrice"]))
        print(json.dumps({"type": t, "region": REGION, "spot_by_az": best}))

def url(iid):
    d = ec2().describe_instances(InstanceIds=[iid])["Reservations"][0]["Instances"][0]
    return f"http://{d.get('PublicIpAddress')}:8000/"

def fetch(base, path, dest):
    import html, re, urllib.parse
    listing = subprocess.run(["curl", "-sSfL", "-m", "120", base + path], capture_output=True, text=True).stdout
    for href in re.findall(r'<a href="([^"]+)">', listing):
        name = urllib.parse.unquote(html.unescape(href))
        if name.startswith(("/", "..", "?")):
            continue
        if name.endswith("/"):
            fetch(base, path + href, os.path.join(dest, name))
        else:
            os.makedirs(dest, exist_ok=True)
            subprocess.run(["curl", "-sSfL", "-m", "600", base + path + href, "-o", os.path.join(dest, name)], check=False)
            print(os.path.join(dest, name))

def main():
    p = argparse.ArgumentParser(); s = p.add_subparsers(dest="cmd", required=True)
    r = s.add_parser("run")
    for k in ("job", "name", "branch", "type"): r.add_argument("--" + k, required=True)
    r.add_argument("--max-seconds", type=int, default=36000); r.add_argument("--serve", type=int, default=1800)
    r.add_argument("--disk", type=int, default=100); r.add_argument("--max-price", type=float)
    r.add_argument("--env", action="append", default=[]); r.add_argument("--cidr", default="0.0.0.0/0")
    r.add_argument("--boot", default="research/dreg_closure_m4l6_20261018/pod/boot.sh"); r.add_argument("--dry-run", action="store_true")
    pr = s.add_parser("prices"); pr.add_argument("--types", default="r7i.8xlarge,r6i.8xlarge,r7a.8xlarge,r7i.24xlarge,r6i.32xlarge,x2iedn.16xlarge")
    s.add_parser("ls")
    for k in ("url", "terminate"): s.add_parser(k).add_argument("id")
    f = s.add_parser("fetch"); f.add_argument("id"); f.add_argument("dest"); f.add_argument("--path", default="")
    a = p.parse_args()
    if a.cmd == "run": run(a)
    elif a.cmd == "prices": prices(a)
    elif a.cmd == "url": print(url(a.id))
    elif a.cmd == "terminate": print(ec2().terminate_instances(InstanceIds=[a.id])["TerminatingInstances"])
    elif a.cmd == "fetch": fetch(url(a.id), a.path, a.dest)
    elif a.cmd == "ls":
        for res in ec2().describe_instances(Filters=[{"Name": "tag:project", "Values": [TAG]}])["Reservations"]:
            for i in res["Instances"]:
                n = {t["Key"]: t["Value"] for t in i.get("Tags", [])}
                print(i["InstanceId"], n.get("Name"), i["InstanceType"], i["State"]["Name"], i.get("PublicIpAddress"))

if __name__ == "__main__": main()
