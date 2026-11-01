"""Provider identity and fail-closed collection controls."""

import copy
import hashlib
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from collect import (assess, collect, fetch_artifact, HEAD, RUN_ID, ATTEMPT,
                     PROTOCOL, WORKFLOW)


def fixture():
    run = {"id": RUN_ID, "run_attempt": ATTEMPT, "head_sha": HEAD,
           "path": WORKFLOW, "event": "push",
           "head_branch": "codex/n83-aws-handoff-20261025",
           "status": "completed", "conclusion": "success"}
    jobs = {"total_count": 8, "jobs": [
        {"name": f"solve-setting-stratum ({row['id']})",
         "run_id": RUN_ID, "head_sha": HEAD,
         "status": "completed", "conclusion": "success"}
        for row in PROTOCOL["strata"]]}
    artifacts = {"total_count": 8, "artifacts": [
        {"id": index + 1,
         "name": f"n83-solver-setting-{row['id']}-{RUN_ID}-{ATTEMPT}",
         "expired": False,
         "workflow_run": {"id": RUN_ID, "head_sha": HEAD}}
        for index, row in enumerate(PROTOCOL["strata"])]}
    return run, jobs, artifacts


class ProviderCollectionTests(unittest.TestCase):
    def test_ready_requires_all_eight_exact_artifacts(self):
        run, jobs, artifacts = fixture()
        result = assess(run, jobs, artifacts)
        self.assertEqual(result["status"], "READY_TO_AUDIT")
        self.assertEqual(len(result["artifact_ids"]), 8)
        artifacts["artifacts"][0]["workflow_run"]["head_sha"] = "0" * 40
        with self.assertRaisesRegex(ValueError, "eight exact unexpired"):
            assess(run, jobs, artifacts)

    def test_inflight_waits_and_failed_job_stops(self):
        run, jobs, artifacts = fixture()
        run["status"] = "in_progress"
        jobs["jobs"][0]["status"] = "in_progress"
        jobs["jobs"][0]["conclusion"] = None
        artifacts["artifacts"] = []
        artifacts["total_count"] = 0
        self.assertEqual(assess(run, jobs, artifacts)["status"],
                         "WAITING_FOR_PROVIDER_RUN")
        jobs["jobs"][0]["status"] = "completed"
        jobs["jobs"][0]["conclusion"] = "failure"
        self.assertEqual(assess(run, jobs, artifacts)["status"],
                         "FAILED_PROVIDER_JOB")

    def test_completed_jobs_wait_for_artifacts_without_collecting(self):
        run, jobs, artifacts = fixture()
        artifacts["artifacts"] = []
        artifacts["total_count"] = 0
        state = assess(run, jobs, artifacts)
        self.assertEqual(state["status"], "WAITING_FOR_EIGHT_ARTIFACTS")
        with tempfile.TemporaryDirectory() as temporary:
            destination = Path(temporary) / "not-created"
            with self.assertRaisesRegex(ValueError, "terminal"):
                collect(destination, (b"", b"", b""), state)
            self.assertFalse(destination.exists())

    def test_wrong_run_and_wrong_artifact_name_rejected(self):
        run, jobs, artifacts = fixture()
        bad = copy.deepcopy(run)
        bad["head_sha"] = "0" * 40
        with self.assertRaisesRegex(ValueError, "exact frozen provider run"):
            assess(bad, jobs, artifacts)
        artifacts["artifacts"][0]["name"] += "-other"
        with self.assertRaisesRegex(ValueError, "eight exact unexpired"):
            assess(run, jobs, artifacts)

    def test_download_requires_provider_digest_and_size(self):
        raw = b"frozen ZIP bytes"
        digest = hashlib.sha256(raw).hexdigest()
        metadata = {"id": 123, "expired": False,
                    "size_in_bytes": len(raw), "digest": f"sha256:{digest}"}
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "artifact"
            with patch("collect.api", side_effect=[json.dumps(metadata).encode(), raw]):
                _, _, observed = fetch_artifact(123, path, digest)
            self.assertEqual(observed, digest)
            metadata["digest"] = "sha256:" + "0" * 64
            with patch("collect.api", side_effect=[json.dumps(metadata).encode(), raw]):
                with self.assertRaisesRegex(ValueError, "provider archive"):
                    fetch_artifact(123, path, digest)


if __name__ == "__main__":
    unittest.main()
