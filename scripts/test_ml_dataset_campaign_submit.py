"""Offline tests for the pre-submission protocol and byte-binding gate."""

import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "tools"))
import ml_dataset_campaign_submit as submit
import ml_dataset_full_protocol_check as checker

SPEC = importlib.util.spec_from_file_location(
    "checker_tests", ROOT / "tests/test_ml_dataset_full_protocol_check.py")
checker_tests = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(checker_tests)


def write(path, value):
    data = (json.dumps(value, separators=(",", ":"), sort_keys=True) + "\n").encode()
    path.write_bytes(data)
    return hashlib.sha256(data).hexdigest()


def admitted_fixture(root):
    p = checker_tests.draft()
    params = p["arms"][0]["parameters"]
    params.update(factor_base_size=8, degree_bound=3, work_limit=100000)
    p["arms"][1]["parameters"].update(factor_base_size=8, degree_bound=3, work_limit=100000)
    for arm in p["arms"]:
        arm["instrument"] = {"commit": "a" * 40, "binary_sha256": "b" * 64,
                             "threads": 1, "build": [["cargo", "build", "--release"]]}
    p["freeze"] = {"status": "frozen", "filed_before_affected_data": True,
                   "protocol_sha256": "e" * 64, "commit": "a" * 40,
                   "frozen_at": "2026-11-01T00:00:00Z"}
    p["analysis_plan"] = {"script": {"path": "analysis.py", "sha256": "f" * 64},
                          "frozen_before_data": True}
    p["sample_size"]["preflight_runtime"] = {"measured_on": "source a, host h, 1 cell measured"}
    for name in ("positive", "negative"):
        p["controls"][name]["fixed_case_ids"] = [name + "-01"]
    plan = {"schema": "crypto.ml-dataset.plan/v1", "protocol_sha256": "e" * 64,
            "families": params["families"], "field_sizes": params["n"],
            "arities": params["m"], "solvers": params["solvers"],
            **{key: params[key] for key in ("curve_seeds", "generators", "keys", "twists",
                                               "factor_base_size", "degree_bound", "work_limit")}}
    plan_sha = write(root / "plan.json", plan)
    grant_sha = write(root / "grant.json", {
        "schema": "crypto.ml-dataset.worker-access-admission/v1",
        "status": "applied_and_verified", "region": "us-east-2",
        "role_arn": "arn:aws:iam::123456789012:role/software-test",
        "policy_sha256": "2" * 64})
    budget_sha = write(root / "budget.json", {
        "schema": "crypto.ml-dataset.campaign-budget-admission/v1",
        "status": "approved", "region": "us-east-2", "wave_id": "wave-001", "cap_usd": 1})
    resources = dict(money_micro_usd=1000, output_bytes=100, put_attempts=2,
                     read_attempts=2, download_bytes=100, local_bytes=100)
    target = dict(cluster_identity="arn:aws:eks:us-east-2:123456789012:cluster/software-test",
                  namespace="isolab", job_name="dataset-test-01")
    reservation = dict(schema="crypto.ml-dataset.budget-launch-reservation/v1",
                       campaign_id="software-test", policy_sha256="d" * 64,
                       request=dict(schema="crypto.ml-dataset.budget-launch-request/v1",
                           launch_id="wave-001", target=target, worker_journal_path="/work/budget.sqlite",
                           binding=dict(source_commit="a" * 40, binary_sha256="b" * 64,
                                        plan_sha256=plan_sha, protocol_sha256="e" * 64,
                                        shard_index=0, shard_count=1), expires_unix_seconds=2000000000,
                           resource_envelope_sha256="f" * 64, resources=resources,
                           finalization=resources, credential_role_arn=None))
    write(root / "reservation.json", reservation)
    reservation_sha = submit.launch.reservation_digest(reservation)
    job = {"apiVersion": "batch/v1", "kind": "Job",
           "metadata": {"name": target["job_name"], "namespace": "isolab",
                        "annotations": {"dataset.isolabs.dev/reservation-sha256": reservation_sha}},
           "spec": {"backoffLimit": 0, "parallelism": 1, "completions": 1,
                    "activeDeadlineSeconds": 3600,
                    "template": {"spec": {"restartPolicy": "Never", "automountServiceAccountToken": False,
                                          "containers": [{"image": "repo@sha256:" + "1" * 64,
                                                          "command": ["/usr/local/bin/crypto", "dataset", "start"]}]}}}}
    job_sha = write(root / "job.json", job)
    expiry = "2099-01-01T00:00:00Z"
    p["execution"].update(timeout_seconds=3600, admission={
        "plan_sha256": plan_sha, "job_sha256": job_sha,
        "image": "repo@sha256:" + "1" * 64, "artifact_prefix": "s3://bucket/test/",
        "region": "us-east-2", "wave_id": "wave-001",
        "worker_grant": {"uri": "s3://bucket/grant", "sha256": grant_sha, "expires_at": expiry},
        "budget": {"uri": "s3://bucket/budget", "sha256": budget_sha,
                   "expires_at": expiry, "region": "us-east-2", "cap_usd": 1}})
    write(root / "protocol.json", p)
    paths = ["--protocol", str(root / "protocol.json"), "--plan", str(root / "plan.json"),
             "--worker-grant", str(root / "grant.json"), "--budget", str(root / "budget.json"),
             "--reservation", str(root / "reservation.json"), "--job", str(root / "job.json")]
    return paths


class CampaignSubmitTests(unittest.TestCase):
    def test_current_shape_draft_rejected_before_other_inputs_or_cloud_calls(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            draft = checker_tests.draft()
            write(root / "draft.json", draft)
            marker = root / "kubectl-called"
            fake = root / "kubectl"
            fake.write_text("#!/bin/sh\ntouch '" + str(marker) + "'\n")
            fake.chmod(0o755)
            command = [sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                       "--dry-run", "--protocol", str(root / "draft.json"), "--kubectl", str(fake)]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("full protocol admission refused", result.stderr)
            self.assertFalse(marker.exists())

    def test_admitted_dry_run_is_offline_and_byte_bound(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            paths = admitted_fixture(root)
            self.assertEqual(checker.validate(json.loads((root / "protocol.json").read_text()), admit=True), [])
            result = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                     "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(json.loads(result.stdout)["cloud_requests"], 0)
            (root / "budget.json").write_text('{"cap_usd":2,"region":"us-east-2"}\n')
            tampered = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                       "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(tampered.returncode, 2)
            self.assertIn("budget bytes differ", tampered.stderr)

    def test_submit_uses_exact_job_and_one_intent(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            paths = admitted_fixture(root)
            marker = root / "called"
            fake = root / "kubectl"
            fake.write_text("#!/bin/sh\nprintf '%s\\n' \"$*\" >> '" + str(marker) + "'\n")
            fake.chmod(0o755)
            (root / "kubeconfig").write_text("software-fixture\n")
            intent = root / "intent.json"
            command = [sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                       "--submit", *paths, "--intent", str(intent),
                       "--kubeconfig", str(root / "kubeconfig"), "--context", "software-test",
                       "--kubectl", str(fake)]
            first = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(first.returncode, 0, first.stderr)
            self.assertEqual(len(marker.read_text().splitlines()), 1)
            self.assertTrue(intent.exists())
            self.assertTrue((root / "intent.json.result.json").exists())
            second = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(second.returncode, 2)
            self.assertEqual(len(marker.read_text().splitlines()), 1)


if __name__ == "__main__":
    unittest.main()
