#!/usr/bin/env python3
"""Show fresh Ohio Spot pool quotes for the deployed size envelope."""
import argparse
from datetime import datetime, timezone
from decimal import Decimal
import json
import subprocess


def aws(region, *args):
    return json.loads(subprocess.check_output(
        ["aws", "--region", region, *args, "--output", "json"], text=True))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--region", default="us-east-2")
    args = parser.parse_args()
    requirements = {"VCpuCount": {"Min": 32, "Max": 32},
                    "MemoryMiB": {"Min": 131072, "Max": 131072},
                    "CpuManufacturers": ["intel", "amd"], "LocalStorage": "excluded",
                    "BurstablePerformance": "excluded", "AcceleratorCount": {"Max": 0},
                    "AllowedInstanceTypes": ["m6i.*", "m6a.*", "m7i.*", "m7a.*", "m8i.*", "m8a.*"]}
    types = aws(args.region, "ec2", "get-instance-types-from-instance-requirements",
                "--architecture-types", "x86_64", "--virtualization-types", "hvm",
                "--instance-requirements", json.dumps(requirements))["InstanceTypes"]
    now = datetime.now(timezone.utc).isoformat()
    prices = aws(args.region, "ec2", "describe-spot-price-history", "--instance-types",
                 *[t["InstanceType"] for t in types], "--product-descriptions", "Linux/UNIX",
                 "--start-time", now)["SpotPriceHistory"]
    latest = {}
    for offer in prices:
        key = (offer["InstanceType"], offer["AvailabilityZone"])
        if key not in latest or latest[key]["Timestamp"] < offer["Timestamp"]:
            latest[key] = offer
    print(f"Observed {now}; current quotes do not guarantee capacity or future prices.")
    print(f"{'Instance':18} {'AZ':14} {'USD/hour':>10} {'USD/24h*':>10}")
    for offer in sorted(latest.values(), key=lambda o: Decimal(o["SpotPrice"])):
        price = Decimal(offer["SpotPrice"])
        print(f"{offer['InstanceType']:18} {offer['AvailabilityZone']:14} {price:10.4f} {price * 24:10.2f}")
    print("*Illustrative constant-price compute cost; excludes disks, EFS, backups, IPv4 and transfer.")


if __name__ == "__main__":
    main()
