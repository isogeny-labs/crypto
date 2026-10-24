#!/usr/bin/env python3
"""Stage a pinned source archive and launch one budget-capped N83 Spot host."""

import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import shlex
import subprocess
import tempfile

from research.n83_nearby_grid_aws_tier1_20261024 import bind_provider
from research.n83_nearby_grid_aws_tier1_20261024 import host as cloud

PROFILE = "ecc-challenges-new"
BUCKET = "ephemeral-runs-104186811093-us-east-1"
SUBNET = "subnet-05d4e28f2c7a467ec"
SECURITY_GROUP = "sg-0c77e42f1ff01e296"
INSTANCE_PROFILE = "ephemeral-runner"
MAX_HOURLY_USD = "1.70"
MAX_MINUTES = 160
AMI_PARAMETER = "/aws/service/canonical/ubuntu/server/24.04/stable/current/arm64/hvm/ebs-gp3/ami-id"
HERE = Path(__file__).resolve().parent


def aws(*args):
    return json.loads(subprocess.check_output(
        ["aws", "--profile", PROFILE, *args, "--output", "json"], text=True))


def plan(bundle_path, commit):
    if not re.fullmatch(r"[0-9a-f]{40}", commit):
        raise ValueError("expected commit must be a full lowercase git SHA")
    bundle_sha = bind_provider.digest(bundle_path)
    if bind_provider.archive_commit(bundle_path) != commit:
        raise ValueError("source archive commit differs")
    quote = aws("ec2", "describe-spot-price-history", "--region", cloud.REGION,
                "--instance-types", cloud.INSTANCE_TYPE,
                "--product-descriptions", "Linux/UNIX",
                "--availability-zone", cloud.ZONE, "--max-items", "1")
    price = float(quote["SpotPriceHistory"][0]["SpotPrice"])
    if price > float(MAX_HOURLY_USD):
        raise ValueError("current Spot quote exceeds the frozen hourly bid")
    existing = aws("ec2", "describe-instances", "--region", cloud.REGION,
                   "--filters", "Name=tag:Project,Values=n83-nearby-grid-aws-tier1",
                   "Name=instance-state-name,Values=pending,running,stopping")
    if any(reservation["Instances"] for reservation in existing["Reservations"]):
        raise ValueError("another N83 grid Spot allocation is still live")
    parameter = aws("ssm", "get-parameter", "--region", cloud.REGION,
                    "--name", AMI_PARAMETER)
    ami = parameter["Parameter"]["Value"]
    image = aws("ec2", "describe-images", "--region", cloud.REGION,
                "--image-ids", ami)["Images"][0]
    if image["Architecture"] != "arm64" or image["State"] != "available":
        raise ValueError("Ubuntu ARM64 image differs")
    source_uri = f"s3://{BUCKET}/jobs/n83-nearby-grid-aws-tier1/{commit}/source.tar.zst"
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    results_uri = f"s3://{BUCKET}/jobs/n83-nearby-grid-aws-tier1/{commit}/runs/{stamp}"
    return {"commit": commit, "bundle_path": str(bundle_path),
            "bundle_sha256": bundle_sha, "bundle_s3_uri": source_uri,
            "results_s3_uri": results_uri, "quote_usd_per_hour": price,
            "quote_time": quote["SpotPriceHistory"][0]["Timestamp"],
            "max_hourly_usd": MAX_HOURLY_USD, "max_minutes": MAX_MINUTES,
            "ami": ami, "root_device_name": image["RootDeviceName"]}


def stage_bundle(value):
    key = value["bundle_s3_uri"].split(f"s3://{BUCKET}/", 1)[1]
    lookup = subprocess.run(
        ["aws", "--profile", PROFILE, "s3api", "head-object",
         "--bucket", BUCKET, "--key", key, "--region", cloud.REGION,
         "--output", "json"], capture_output=True, text=True)
    if lookup.returncode == 0:
        metadata = json.loads(lookup.stdout)["Metadata"]
        if (metadata.get("sha256") != value["bundle_sha256"] or
                metadata.get("commit") != value["commit"]):
            raise ValueError("existing S3 bundle identity differs")
        return
    create = subprocess.run(
        ["aws", "--profile", PROFILE, "s3api", "put-object",
         "--bucket", BUCKET, "--key", key, "--body", value["bundle_path"],
         "--metadata", f"sha256={value['bundle_sha256']},commit={value['commit']}",
         "--if-none-match", "*", "--region", cloud.REGION,
         "--output", "json"], capture_output=True, text=True)
    if create.returncode != 0:
        raise ValueError("immutable S3 source bundle publish failed: " +
                         create.stderr.strip()[:300])


def user_data(value):
    variables = {
        "N83_COMMIT": value["commit"],
        "N83_BUNDLE_SHA256": value["bundle_sha256"],
        "N83_BUNDLE_S3_URI": value["bundle_s3_uri"],
        "N83_RESULTS_S3_URI": value["results_s3_uri"],
    }
    lines = ["#!/bin/bash", *(f"export {key}={shlex.quote(data)}"
                              for key, data in variables.items())]
    body = (HERE / "bootstrap.sh").read_text().splitlines()
    if not body[0].startswith("#!"):
        raise ValueError("bootstrap script has no shebang")
    return "\n".join([*lines, *body[1:]]) + "\n"


def launch(value):
    stage_bundle(value)
    market = {"MarketType": "spot", "SpotOptions": {
        "MaxPrice": MAX_HOURLY_USD, "SpotInstanceType": "one-time",
        "InstanceInterruptionBehavior": "terminate"}}
    devices = [{"DeviceName": value["root_device_name"], "Ebs": {
        "VolumeSize": 80, "VolumeType": "gp3", "Encrypted": True,
        "DeleteOnTermination": True}}]
    tags = [{"Key": "Project", "Value": "n83-nearby-grid-aws-tier1"},
            {"Key": "Commit", "Value": value["commit"]},
            {"Key": "Name", "Value": "n83-nearby-grid-aws-tier1"}]
    tag_spec = [{"ResourceType": "instance", "Tags": tags},
                {"ResourceType": "volume", "Tags": tags}]
    script = user_data(value)
    with tempfile.NamedTemporaryFile(mode="w", suffix=".sh") as temporary:
        temporary.write(script)
        temporary.flush()
        response = aws(
            "ec2", "run-instances", "--region", cloud.REGION,
            "--image-id", value["ami"], "--instance-type", cloud.INSTANCE_TYPE,
            "--count", "1", "--subnet-id", SUBNET,
            "--security-group-ids", SECURITY_GROUP,
            "--iam-instance-profile", f"Name={INSTANCE_PROFILE}",
            "--instance-market-options", json.dumps(market),
            "--metadata-options", "HttpTokens=required,HttpEndpoint=enabled",
            "--instance-initiated-shutdown-behavior", "terminate",
            "--block-device-mappings", json.dumps(devices),
            "--tag-specifications", json.dumps(tag_spec),
            "--user-data", f"file://{temporary.name}")
    instances = response["Instances"]
    if len(instances) != 1:
        raise ValueError("AWS launch did not return exactly one instance")
    return {**value, "instance_id": instances[0]["InstanceId"],
            "instance_state": instances[0]["State"]["Name"],
            "user_data_sha256": hashlib.sha256(script.encode()).hexdigest(),
            "bootstrap_sha256": bind_provider.digest(HERE / "bootstrap.sh"),
            "launcher_sha256": bind_provider.digest(Path(__file__)),
            "launch_requested_utc": datetime.now(timezone.utc).isoformat()}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--bundle-path", type=Path, required=True)
    parser.add_argument("--commit", required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    if args.receipt.exists():
        raise ValueError("launch receipt already exists")
    value = plan(args.bundle_path, args.commit)
    if args.execute:
        value = launch(value)
        args.receipt.write_text(json.dumps(value, sort_keys=True, indent=2) + "\n")
    print(json.dumps(value, sort_keys=True))


if __name__ == "__main__":
    main()
