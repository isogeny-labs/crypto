import copy
import json
import tempfile
import unittest
from pathlib import Path

from research.n83_load80_online_pair_20261020 import check_reported_completion as check


class CompletionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.protocol = json.loads((check.HERE / "protocol.json").read_text())
        self.write("source.json", {"protocol": self.protocol,
                                  "protocol_sha256": check.PROTOCOL_SHA256})
        self.summary = {"schema": "n83.load80-online-pair-summary/v1",
                        "status": "COMPLETE_REPORTED_OUTPUT",
                        "expected_events": len(self.protocol["events"]),
                        "events": []}
        for event in self.protocol["events"]:
            self.summary["events"].append({
                "event": event, "process": "PASS_PROCESS", "resource": "PASS",
                "reported_verification": "PASS_REPORTED_OUTPUT"})
            self.write(f"runs/{event['name']}/usage.json", {
                "name": event["name"], "status": "PASS_PROCESS",
                "exit_code": 0, "resource_status": "PASS"})
            self.write(f"runs/{event['name']}/reported_verification.json", {
                "event": event, "status": "PASS_REPORTED_OUTPUT"})
        self.write("summary.json", self.summary)

    def write(self, name, value):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value))

    def reject(self):
        self.assertEqual(check.check(self.root)["status"], "BOUNDED_UNKNOWN")

    def test_complete_is_only_reported_completion(self):
        result = check.check(self.root)
        self.assertEqual(result["status"], "PASS_REPORTED_COMPLETION")
        self.assertFalse(result["independent_arithmetic_replay"])

    def test_final_failed_event_is_not_complete(self):
        self.summary["events"][-1]["reported_verification"] = "RHO_OUTPUT_MISMATCH"
        self.write("summary.json", self.summary)
        self.reject()

    def test_failed_process_is_not_complete(self):
        self.summary["events"][0]["process"] = "TIMEOUT_120S"
        self.write("summary.json", self.summary)
        self.reject()

    def test_incomplete_schedule(self):
        self.summary["events"].pop()
        self.write("summary.json", self.summary)
        self.reject()

    def test_duplicate_event(self):
        self.summary["events"][-1] = copy.deepcopy(self.summary["events"][0])
        self.write("summary.json", self.summary)
        self.reject()

    def test_missing_receipt(self):
        name = self.protocol["events"][-1]["name"]
        (self.root / "runs" / name / "reported_verification.json").unlink()
        self.reject()

    def test_disagreeing_receipt(self):
        name = self.protocol["events"][-1]["name"]
        self.write(f"runs/{name}/usage.json", {
            "name": name, "status": "PASS_PROCESS",
            "exit_code": 0, "resource_status": "FAILED"})
        self.reject()

    def test_producer_stop(self):
        self.write("producer_stop.json", {"status": "GLOBAL_OR_PROVIDER_CAP_BEFORE_EVENT"})
        self.reject()

    def test_source_protocol_mismatch(self):
        self.write("source.json", {"protocol_sha256": check.PROTOCOL_SHA256,
                                  "protocol": {}})
        self.reject()


if __name__ == "__main__":
    unittest.main()
