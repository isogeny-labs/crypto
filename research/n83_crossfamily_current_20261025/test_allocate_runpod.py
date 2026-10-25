"""Check budgeted allocation, provider identity and cleanup gates offline."""

import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from research.n83_crossfamily_current_20261025 import allocate_runpod as subject


HEAD = "a" * 40
BUNDLE = {"source_head_sha": HEAD,
          "source_archive_sha256": "b" * 64,
          "source_manifest_sha256": "c" * 64}


class FakeProvider:
    def __init__(self, duplicate_machine=False, lose_second_response=False):
        self.live = []
        self.terminated = []
        self.duplicate_machine = duplicate_machine
        self.lose_second_response = lose_second_response

    def pods(self, _key):
        return list(self.live)

    def api(self, query, _key):
        if "terminateAfter:" not in query or "stopAfter:" in query:
            raise ValueError("provider auto-termination deadline is missing")
        stratum = len(self.live)
        name = f"n83-crossfamily-{HEAD[:12]}-s{stratum}"
        if name not in query:
            raise ValueError("unexpected Pod name")
        pod = {
            "id": f"pod{stratum}", "name": name,
            "desiredStatus": "RUNNING", "costPerHr": 0.27,
            "createdAt": "2026-10-25T02:00:00Z",
            "machineId": "machine0" if self.duplicate_machine else f"machine{stratum}",
            "memoryInGb": 27, "vcpuCount": 4,
            "imageName": subject.IMAGE, "gpuCount": 1,
            "containerDiskInGb": 30, "volumeInGb": 0,
        }
        self.live.append(pod)
        if stratum == 1 and self.lose_second_response:
            raise ConnectionError("response lost after provider mutation")
        return {"podFindAndDeployOnDemand": pod}

    def terminate(self, pod_id, _key):
        self.terminated.append(pod_id)
        self.live = [pod for pod in self.live if pod["id"] != pod_id]
        return True


class AllocateTests(unittest.TestCase):
    def test_pod_fixture_replays_without_historical_git_inventory(self):
        with patch.object(subject.run.generator, "produce",
                          side_effect=AssertionError("historical Git scan on Pod")):
            hashes, points = subject.run.fixture()
        self.assertEqual(len(points), 16)
        self.assertEqual(hashes["examples/koblitz_orbit_dlp_fast.rs"],
                         subject.run.SOURCE_SHA256)

    def test_two_distinct_pods_are_bound_below_cap(self):
        provider = FakeProvider()
        with tempfile.TemporaryDirectory() as temp, patch.object(
                subject, "frozen_head", return_value=HEAD):
            output = Path(temp) / "allocation"
            result = subject.allocate(output, "fake", BUNDLE, provider.api,
                                      provider.pods, provider.terminate)
            self.assertEqual(result["status"], "PASS_TWO_BUDGETED_PODS")
            self.assertEqual(provider.terminated, [])
            snapshot_path = output / "provider_snapshot.json"
            snapshot = json.loads(snapshot_path.read_text())
            self.assertEqual(len({p["machine_id"] for p in snapshot["pods"]}), 2)
            self.assertEqual(snapshot['projected_compute_and_container_disk_usd'],
                             '3.30')
            for stratum in range(2):
                allocation = json.loads((output / f"allocation_s{stratum}.json").read_text())
                self.assertEqual(allocation["snapshot_sha256"],
                                 subject.sha(snapshot_path.read_bytes()))
                self.assertEqual(allocation["source_head_sha"], HEAD)
                self.assertEqual(allocation["source_archive_sha256"],
                                 BUNDLE["source_archive_sha256"])

    def test_same_machine_is_retired(self):
        provider = FakeProvider(duplicate_machine=True)
        with tempfile.TemporaryDirectory() as temp, patch.object(
                subject, "frozen_head", return_value=HEAD):
            output = Path(temp) / "allocation"
            with self.assertRaisesRegex(ValueError, "distinct"):
                subject.allocate(output, "fake", BUNDLE, provider.api,
                                 provider.pods, provider.terminate)
            self.assertEqual(set(provider.terminated), {"pod0", "pod1"})
            self.assertEqual(provider.live, [])
            failure = json.loads((output / "allocation_failure.json").read_text())
            self.assertEqual(failure["status"], "FAILED_ALLOCATIONS_TERMINATED")

    def test_lost_mutation_response_retains_uncertainty(self):
        provider = FakeProvider(lose_second_response=True)
        with tempfile.TemporaryDirectory() as temp, patch.object(
                subject, "frozen_head", return_value=HEAD):
            output = Path(temp) / "allocation"
            with self.assertRaisesRegex(ConnectionError, "response lost"):
                subject.allocate(output, "fake", BUNDLE, provider.api,
                                 provider.pods, provider.terminate)
            self.assertEqual(set(provider.terminated), {"pod0", "pod1"})
            failure = json.loads((output / "allocation_failure.json").read_text())
            self.assertEqual(failure["status"], "UNRESOLVED_PROVIDER_MUTATION")


if __name__ == "__main__":
    unittest.main()
