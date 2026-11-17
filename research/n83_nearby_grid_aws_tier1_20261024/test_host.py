"""Fail-closed controls for unbounded cgroups and Spot resource noise."""

import json
from pathlib import Path
import tempfile
import unittest

from research.n83_nearby_grid_aws_tier1_20261024 import host, verify


class HostGateTests(unittest.TestCase):
    def test_unbounded_quota_chain(self):
        self.assertTrue(host.unbounded([
            {"path": "/sys/fs/cgroup/a", "cpu_max": "max 100000\n"},
            {"path": "/sys/fs/cgroup", "cpu_max": None}]))
        self.assertFalse(host.unbounded([
            {"path": "/sys/fs/cgroup/a", "cpu_max": "200000 100000\n"}]))
        self.assertFalse(host.unbounded([]))

    def test_missing_throttle_counter_only_with_unbounded_chain(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            chain = [{"path": "/sys/fs/cgroup", "cpu_max": None}]
            host_receipt = {"cgroup_cpu_quota_chain_before": chain,
                            "clock_ticks_per_second": 100,
                            "visible_affinity": list(range(8))}
            cgroup = {"cpu.max": None, "memory.max": "max\n",
                      "cpu.stat": "usage_usec 100\n",
                      "memory.events": "oom 0\noom_kill 0\n"}
            usage = {"status": "PASS_PRODUCER", "peak_rss_bytes": 1024,
                     "wall_seconds": 100.0,
                     "cgroup_before": cgroup, "cgroup_after": cgroup}
            telemetry = {
                "quota_chain_before": chain, "quota_chain_after": chain,
                "before": {"steal_jiffies": 0,
                           "frequency_khz_by_visible_cpu": {"0": "2400000"}},
                "after": {"steal_jiffies": 1,
                          "frequency_khz_by_visible_cpu": {"0": "2400000"}},
            }
            (folder / "usage.json").write_text(json.dumps(usage))
            (folder / "host_telemetry.json").write_text(json.dumps(telemetry))
            self.assertEqual(verify.resource_gate(folder, host_receipt)["status"],
                             "PASS")
            telemetry["quota_chain_after"] = [
                {"path": "/sys/fs/cgroup", "cpu_max": "800000 100000\n"}]
            (folder / "host_telemetry.json").write_text(json.dumps(telemetry))
            self.assertEqual(verify.resource_gate(folder, host_receipt)["status"],
                             "NOISY_OR_INCOMPLETE")
            telemetry["quota_chain_after"] = chain
            telemetry["after"]["steal_jiffies"] = 500
            (folder / "host_telemetry.json").write_text(json.dumps(telemetry))
            self.assertEqual(verify.resource_gate(folder, host_receipt)["status"],
                             "NOISY_OR_INCOMPLETE")

    def test_provider_binding_rejects_changed_instance(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            instance = {"InstanceId": "i-0123456789abcdef0",
                        "InstanceType": host.INSTANCE_TYPE,
                        "Placement": {"AvailabilityZone": host.ZONE},
                        "InstanceLifecycle": "spot",
                        "ImageId": "ami-0123456789abcdef0",
                        "SpotInstanceRequestId": "sir-123",
                        "LaunchTime": "2026-10-24T22:00:00+00:00",
                        "State": {"Name": "terminated"}}
            receipt = {"schema": "n83.nearby-grid-aws-provider/v1",
                       "instance_id": instance["InstanceId"],
                       "instance_type": instance["InstanceType"],
                       "availability_zone": host.ZONE,
                       "instance_lifecycle": "spot",
                       "image_id": instance["ImageId"],
                       "spot_request_id": "sir-123",
                       "launch_time": instance["LaunchTime"],
                       "state": "terminated",
                       "bundle_local_verification_status": "PASS",
                       "bundle_sha256": "a" * 64,
                       "bundle_commit": "b" * 40,
                       "bundle_s3_uri": "s3://example/jobs/n83/source.tar.zst",
                       "provider_response": {"Reservations": [{"Instances": [instance]}]}}
            host_receipt = {"instance_id": instance["InstanceId"],
                            "instance_identity": {"document": {
                                "instanceId": instance["InstanceId"],
                                "imageId": instance["ImageId"]}}}
            (output / "provider_receipt.json").write_text(json.dumps(receipt))
            (output / "source.json").write_text(json.dumps({
                "bundle_sha256": receipt["bundle_sha256"],
                "bundle_s3_uri": receipt["bundle_s3_uri"],
                "git_head": receipt["bundle_commit"]}))
            (output / "bundle_receipt.json").write_text(json.dumps({
                "sha256": receipt["bundle_sha256"],
                "s3_uri": receipt["bundle_s3_uri"],
                "commit": receipt["bundle_commit"]}))
            self.assertTrue(verify.provider_gate(output, host_receipt))
            receipt["instance_id"] = "i-00000000000000000"
            (output / "provider_receipt.json").write_text(json.dumps(receipt))
            self.assertFalse(verify.provider_gate(output, host_receipt))


if __name__ == "__main__":
    unittest.main()
