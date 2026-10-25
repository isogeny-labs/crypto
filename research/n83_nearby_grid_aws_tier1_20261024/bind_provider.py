#!/usr/bin/env python3
"""Bind an AWS API instance receipt to a completed N83 Spot artifact."""

import argparse
import hashlib
import json
from pathlib import Path
import shutil
import signal
import subprocess

from research.n83_nearby_grid_aws_tier1_20261024 import host as cloud


def digest(path):
    sha = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            sha.update(block)
    return sha.hexdigest()


def archive_commit(path):
    decoder = shutil.which("zstd") or "/opt/homebrew/bin/zstd"
    subprocess.run([decoder, "-t", str(path)], check=True,
                   stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    process = subprocess.Popen([decoder, "-dc", str(path)],
                               stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    try:
        commit = subprocess.check_output(["git", "get-tar-commit-id"],
                                         stdin=process.stdout, text=True).strip()
    finally:
        process.stdout.close()
    stderr = process.communicate()[1].decode(errors="replace")
    if (process.returncode not in (0, -signal.SIGPIPE) and
            not (process.returncode == 70 and "Broken pipe" in stderr)) or \
            len(commit) != 40:
        raise ValueError("source archive commit could not be replayed")
    return commit


def bind(output, profile, bundle_path):
    host = json.loads((output / "host.json").read_text())
    bundle = json.loads((output / "bundle_receipt.json").read_text())
    source = json.loads((output / "source.json").read_text())
    bundle_sha = digest(bundle_path)
    commit = archive_commit(bundle_path)
    if (bundle_sha != bundle.get("sha256") or
            bundle_sha != source.get("bundle_sha256") or
            commit != bundle.get("commit") or
            commit != source.get("git_head") or
            bundle.get("s3_uri") != source.get("bundle_s3_uri")):
        raise ValueError("source archive differs from executed receipt")
    instance_id = host["instance_id"]
    command = ["aws", "--profile", profile, "ec2", "describe-instances",
               "--region", cloud.REGION, "--instance-ids", instance_id,
               "--output", "json"]
    raw = subprocess.check_output(command, text=True)
    response = json.loads(raw)
    instances = [instance for reservation in response["Reservations"]
                 for instance in reservation["Instances"]]
    if len(instances) != 1:
        raise ValueError("AWS provider returned other than one instance")
    instance = instances[0]
    identity = host["instance_identity"]["document"]
    if (instance["InstanceId"] != instance_id or
            instance["InstanceType"] != cloud.INSTANCE_TYPE or
            instance["Placement"]["AvailabilityZone"] != cloud.ZONE or
            instance.get("InstanceLifecycle") != "spot" or
            instance["ImageId"] != identity["imageId"] or
            host["account_id"] != identity["accountId"] or
            not instance.get("SpotInstanceRequestId")):
        raise ValueError("AWS provider instance differs from host receipt")
    receipt = {"schema": "n83.nearby-grid-aws-provider/v1",
               "instance_id": instance_id,
               "instance_type": instance["InstanceType"],
               "availability_zone": instance["Placement"]["AvailabilityZone"],
               "instance_lifecycle": instance["InstanceLifecycle"],
               "image_id": instance["ImageId"],
               "spot_request_id": instance["SpotInstanceRequestId"],
               "launch_time": str(instance["LaunchTime"]),
               "state": instance["State"]["Name"],
               "bundle_sha256": bundle_sha,
               "bundle_commit": commit,
               "bundle_s3_uri": bundle["s3_uri"],
               "bundle_local_verification_status": "PASS",
               "provider_response": response}
    path = output / "provider_receipt.json"
    blob = json.dumps(receipt, sort_keys=True, indent=2) + "\n"
    if path.exists() and path.read_text() != blob:
        raise ValueError("sealed provider receipt differs")
    path.write_text(blob)
    return {key: receipt[key] for key in
            ("instance_id", "instance_type", "availability_zone",
             "instance_lifecycle", "state")}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    parser.add_argument("--bundle-path", type=Path, required=True)
    parser.add_argument("--profile", default="ecc-challenges-new")
    args = parser.parse_args()
    print(json.dumps(bind(args.output, args.profile, args.bundle_path),
                     sort_keys=True))


if __name__ == "__main__":
    main()
