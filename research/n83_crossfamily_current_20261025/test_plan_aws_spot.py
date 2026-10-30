"""Check the read-only two-instance Spot budget and host gates."""

from datetime import datetime, timezone
import unittest

from research.n83_crossfamily_current_20261025 import plan_aws_spot as plan


CLOCK = datetime(2026, 10, 26, 18, 30, tzinfo=timezone.utc)
BUNDLE = {"source_archive_sha256": "a" * 64,
          "source_manifest_sha256": "b" * 64}


class FakeAws:
    def __init__(self, quota=64, score_d=9):
        self.quota = quota
        self.score_d = score_d

    def __call__(self, service, operation, *args):
        if (service, operation) == ("sts", "get-caller-identity"):
            return {"Account": plan.ACCOUNT_ID}
        if (service, operation) == ("service-quotas", "get-service-quota"):
            return {"Quota": {"Value": self.quota}}
        if operation == "describe-instance-types":
            return {"InstanceTypes": [{
                "InstanceType": plan.INSTANCE_TYPE,
                "VCpuInfo": {"DefaultVCpus": 16},
                "MemoryInfo": {"SizeInMiB": 32768},
                "ProcessorInfo": {"SupportedArchitectures": ["arm64"]}}]}
        if operation == "describe-subnets":
            return {"Subnets": [{
                "SubnetId": fixed["subnet_id"],
                "VpcId": plan.VPC_ID,
                "AvailabilityZone": fixed["zone"],
                "State": "available", "AvailableIpAddressCount": 100,
                "MapPublicIpOnLaunch": True} for fixed in plan.ZONES]}
        if operation == "get-spot-placement-scores":
            return {"SpotPlacementScores": [
                {"AvailabilityZoneId": "use1-az6", "Score": self.score_d}]}
        if operation == "describe-availability-zones":
            return {"AvailabilityZones": [
                {"ZoneName": "us-east-1d", "ZoneId": "use1-az6"},
                {"ZoneName": "us-east-1a", "ZoneId": "use1-az1"}]}
        if (service, operation) == ("ssm", "get-parameter"):
            return {"Parameter": {"Value": "ami-fixture"}}
        if operation == "describe-images":
            return {"Images": [{"ImageId": "ami-fixture",
                                "Architecture": "arm64", "State": "available"}]}
        if operation == "describe-spot-price-history":
            return {"SpotPriceHistory": [
                {"InstanceType": plan.INSTANCE_TYPE,
                 "AvailabilityZone": "us-east-1d", "SpotPrice": "0.2475",
                 "Timestamp": "2026-10-26T18:00:00+00:00"},
                {"InstanceType": plan.INSTANCE_TYPE,
                 "AvailabilityZone": "us-east-1a", "SpotPrice": "0.3657",
                 "Timestamp": "2026-10-26T17:00:00+00:00"}]}
        raise AssertionError((service, operation, args))


class AwsPlanTests(unittest.TestCase):
    def test_two_instances_fit_the_bid_cap(self):
        result = plan.provider_plan(BUNDLE, "c" * 40, CLOCK, FakeAws())
        self.assertEqual(result["projected_at_quotes_usd"], "3.10")
        self.assertEqual(result["projected_at_bids_usd"], "3.37")
        self.assertEqual({zone["zone"] for zone in result["zones"]},
                         {"us-east-1d"})
        self.assertEqual(result["source_archive_sha256"], "a" * 64)

    def test_quota_and_placement_fail_closed(self):
        with self.assertRaisesRegex(ValueError, "quota"):
            plan.provider_plan(BUNDLE, "c" * 40, CLOCK, FakeAws(quota=0))
        with self.assertRaisesRegex(ValueError, "placement score"):
            plan.provider_plan(BUNDLE, "c" * 40, CLOCK, FakeAws(score_d=1))


if __name__ == "__main__":
    unittest.main()
