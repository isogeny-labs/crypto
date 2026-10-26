#!/usr/bin/env python3
"""Read-only, source-bound AWS Spot plan for two N83 Tier 1 instances."""

import argparse
from datetime import datetime, timedelta, timezone
from decimal import Decimal, ROUND_UP
import json
from pathlib import Path
import subprocess

from research.n83_crossfamily_current_20261025 import allocate_runpod
from research.n83_crossfamily_current_20261025 import source_bundle


ACCOUNT_ID = "590183823895"
REGION = "us-east-1"
VPC_ID = "vpc-0da45a1bf46c94fa9"
INSTANCE_TYPE = "g5g.4xlarge"
VCPU_PER_INSTANCE = 16
MEMORY_MIB_PER_INSTANCE = 32768
SPOT_QUOTA_CODE = "L-3819A6DF"
CAP_SECONDS = 6 * 3600
ROOT_DISK_GB = 50
GP3_USD_PER_GB_MONTH = Decimal("0.08")
MIN_BILLING_MONTH_HOURS = Decimal(28 * 24)
S3_ALLOWANCE_USD = Decimal("0.05")
TOTAL_CAP_USD = Decimal("4.20")
AMI_PARAMETER = (
    "/aws/service/canonical/ubuntu/server/24.04/stable/current/"
    "arm64/hvm/ebs-gp3/ami-id"
)
ZONES = (
    {"stratum": 0, "zone": "us-east-1d", "subnet_id":
     "subnet-0561480728d2b1ffd", "max_hourly_usd": Decimal("0.27")},
    {"stratum": 1, "zone": "us-east-1d", "subnet_id":
     "subnet-0561480728d2b1ffd", "max_hourly_usd": Decimal("0.27")},
)


def aws(*args):
    result = subprocess.check_output(
        ["aws", *args, "--region", REGION, "--output", "json"], text=True)
    return json.loads(result)


def cents(value):
    return value.quantize(Decimal("0.01"), rounding=ROUND_UP)


def projected(prices):
    hours = Decimal(CAP_SECONDS) / Decimal(3600)
    compute = sum(prices, Decimal(0)) * hours
    disk = (Decimal(len(prices) * ROOT_DISK_GB) * GP3_USD_PER_GB_MONTH *
            hours / MIN_BILLING_MONTH_HOURS)
    return cents(compute) + cents(disk) + S3_ALLOWANCE_USD


def latest_quote(history, zone, clock):
    matching = [row for row in history["SpotPriceHistory"]
                if row.get("InstanceType") == INSTANCE_TYPE and
                row.get("AvailabilityZone") == zone]
    if not matching:
        raise ValueError(f"no Spot quote for {zone}")
    row = max(matching, key=lambda item: item["Timestamp"])
    quoted = datetime.fromisoformat(row["Timestamp"].replace("Z", "+00:00"))
    if quoted.tzinfo is None or quoted > clock or clock - quoted > timedelta(hours=12):
        raise ValueError(f"stale Spot quote for {zone}")
    price = Decimal(row["SpotPrice"])
    if price <= 0:
        raise ValueError(f"invalid Spot quote for {zone}")
    return price, row["Timestamp"]


def provider_plan(bundle, head, clock, api=aws):
    account = api("sts", "get-caller-identity")["Account"]
    if account != ACCOUNT_ID:
        raise ValueError("AWS account differs from frozen N83 allocation")
    quota = api("service-quotas", "get-service-quota", "--service-code", "ec2",
                "--quota-code", SPOT_QUOTA_CODE)["Quota"]
    if Decimal(str(quota["Value"])) < 2 * VCPU_PER_INSTANCE:
        raise ValueError("G and VT Spot vCPU quota is too small")
    spec = api("ec2", "describe-instance-types", "--instance-types",
               INSTANCE_TYPE)["InstanceTypes"][0]
    if (spec["InstanceType"] != INSTANCE_TYPE or
            spec["VCpuInfo"]["DefaultVCpus"] < VCPU_PER_INSTANCE or
            spec["MemoryInfo"]["SizeInMiB"] < MEMORY_MIB_PER_INSTANCE or
            "arm64" not in spec["ProcessorInfo"]["SupportedArchitectures"]):
        raise ValueError("AWS instance resources or architecture differ")
    subnets = {row["SubnetId"]: row for row in api(
        "ec2", "describe-subnets", "--subnet-ids",
        *(zone["subnet_id"] for zone in ZONES))["Subnets"]}
    scores = {row["AvailabilityZoneId"]: row["Score"] for row in api(
        "ec2", "get-spot-placement-scores", "--instance-types",
        INSTANCE_TYPE, "--target-capacity", "2",
        "--target-capacity-unit-type", "units",
        "--single-availability-zone", "--region-names", REGION)[
            "SpotPlacementScores"]}
    zone_ids = {row["ZoneName"]: row["ZoneId"] for row in api(
        "ec2", "describe-availability-zones")["AvailabilityZones"]}
    image_id = api("ssm", "get-parameter", "--name", AMI_PARAMETER)[
        "Parameter"]["Value"]
    image = api("ec2", "describe-images", "--image-ids", image_id)["Images"][0]
    if (image["ImageId"] != image_id or image["Architecture"] != "arm64" or
            image["State"] != "available"):
        raise ValueError("Ubuntu ARM64 image differs")
    history = api("ec2", "describe-spot-price-history", "--instance-types",
                  INSTANCE_TYPE, "--product-descriptions", "Linux/UNIX",
                  "--start-time", (clock - timedelta(hours=12)).isoformat(),
                  "--max-results", "100")
    zones = []
    for fixed in ZONES:
        subnet = subnets.get(fixed["subnet_id"])
        if (subnet is None or subnet["VpcId"] != VPC_ID or
                subnet["AvailabilityZone"] != fixed["zone"] or
                subnet["State"] != "available" or
                subnet["AvailableIpAddressCount"] < 2 or
                not subnet["MapPublicIpOnLaunch"]):
            raise ValueError(f"approved subnet differs in {fixed['zone']}")
        score = scores.get(zone_ids.get(fixed["zone"]))
        if score is None or score < 3:
            raise ValueError(f"Spot placement score is too low in {fixed['zone']}")
        price, quote_time = latest_quote(history, fixed["zone"], clock)
        if price > fixed["max_hourly_usd"]:
            raise ValueError(f"Spot quote exceeds bid in {fixed['zone']}")
        zones.append({"stratum": fixed["stratum"], "zone": fixed["zone"],
                      "subnet_id": fixed["subnet_id"], "zone_id": zone_ids[fixed["zone"]],
                      "spot_placement_score": score,
                      "quote_usd_per_hour": str(price), "quote_time": quote_time,
                      "max_hourly_usd": str(fixed["max_hourly_usd"])})
    bid_total = projected([zone["max_hourly_usd"] for zone in ZONES])
    if bid_total > TOTAL_CAP_USD:
        raise ValueError("six-hour two-host bid envelope exceeds cap")
    return {"schema": "n83.crossfamily-aws-spot-plan/v1",
            "status": "READY_FOR_SEPARATE_ALLOCATION_APPROVAL",
            "tier": "cloud_instance_tier1",
            "placement_model": "two_distinct_instances_same_availability_zone",
            "recorded_utc": clock.isoformat(), "account_id": account,
            "region": REGION, "instance_type": INSTANCE_TYPE,
            "architecture": "arm64", "image_id": image_id,
            "source_head_sha": head,
            "source_archive_sha256": bundle["source_archive_sha256"],
            "source_manifest_sha256": bundle["source_manifest_sha256"],
            "spot_quota_vcpus": quota["Value"], "cap_seconds": CAP_SECONDS,
            "root_disk_gb_per_host": ROOT_DISK_GB,
            "gp3_usd_per_gb_month": str(GP3_USD_PER_GB_MONTH),
            "s3_allowance_usd": str(S3_ALLOWANCE_USD),
            "projected_at_quotes_usd": str(projected(
                [Decimal(zone["quote_usd_per_hour"]) for zone in zones])),
            "projected_at_bids_usd": str(bid_total),
            "total_cap_usd": str(TOTAL_CAP_USD), "zones": zones}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-bundle", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    head = allocate_runpod.frozen_head()
    bundle_path = args.source_bundle.resolve()
    bundle = source_bundle.verify(bundle_path / "source.tar.gz",
                                  bundle_path / "manifest.json")
    if bundle["source_head_sha"] != head:
        raise ValueError("source bundle differs from exact checkout")
    bound = {**bundle, "source_manifest_sha256": source_bundle.digest(
        bundle_path / "manifest.json")}
    plan = provider_plan(bound, head, datetime.now(timezone.utc))
    output = args.output.resolve()
    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("x") as stream:
        json.dump(plan, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({key: plan[key] for key in
                      ("status", "source_head_sha", "projected_at_quotes_usd",
                       "projected_at_bids_usd")}, sort_keys=True))


if __name__ == "__main__":
    main()
