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
    p["analysis_plan"] = {"script": {"path": "analysis.py", "sha256": "f" * 64},
                          "frozen_before_data": True}
    p["sample_size"]["preflight_runtime"] = {"measured_on": "source a, host h, 1 cell measured"}
    for name in ("positive", "negative"):
        p["controls"][name]["fixed_case_ids"] = [name + "-01"]
    p["execution"]["timeout_seconds"] = 3600
    body = {key: value for key, value in p.items() if key != "freeze"}
    body_sha = hashlib.sha256(json.dumps(body, sort_keys=True, separators=(",", ":"),
                                         ensure_ascii=False).encode()).hexdigest()
    p["freeze"] = {"status": "frozen", "filed_before_affected_data": True,
                   "protocol_sha256": body_sha, "commit": "a" * 40,
                   "frozen_at": "2026-11-01T00:00:00Z"}
    protocol_file_sha = write(root / "protocol.json", p)
    plan = {"schema": "crypto.ml-dataset.plan/v1", "protocol_sha256": protocol_file_sha,
            "families": params["families"], "field_sizes": params["n"],
            "arities": params["m"], "solvers": params["solvers"],
            "seed": 20261027, "split_rule": "curve-twist-component-sha256-v1",
            **{key: params[key] for key in ("curve_seeds", "generators", "keys", "twists",
                                               "factor_base_size", "degree_bound", "work_limit")}}
    plan_file_sha = write(root / "plan.json", plan)
    plan_binding_sha = submit.native_plan_binding_sha256(plan)
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
                                        plan_sha256=plan_binding_sha, protocol_sha256=protocol_file_sha,
                                        shard_index=0, shard_count=1), expires_unix_seconds=2000000000,
                           resource_envelope_sha256="f" * 64, resources=resources,
                           finalization=resources, credential_role_arn=None))
    write(root / "reservation.json", reservation)
    reservation_sha = submit.launch.reservation_digest(reservation)
    job = {"apiVersion": "batch/v1", "kind": "Job",
           "metadata": {"name": target["job_name"], "namespace": "isolab",
                        "annotations": {"dataset.isolabs.dev/reservation-sha256": reservation_sha}},
           "spec": {"suspend": True, "backoffLimit": 0, "parallelism": 1, "completions": 1,
                    "activeDeadlineSeconds": 3600,
                    "template": {"spec": {"restartPolicy": "Never", "automountServiceAccountToken": False,
                                          "containers": [{"image": "repo@sha256:" + "1" * 64,
                                                          "command": ["/usr/local/bin/crypto", "dataset", "start"]}]}}}}
    job_sha = write(root / "job.json", job)
    expiry = "2099-01-01T00:00:00Z"
    admission = {"schema": "crypto.ml-dataset.wave-admission/v1",
        "protocol_file_sha256": protocol_file_sha,
        "plan_sha256": plan_binding_sha, "plan_file_sha256": plan_file_sha,
        "job_sha256": job_sha,
        "image": "repo@sha256:" + "1" * 64, "artifact_prefix": "s3://bucket/test/",
        "region": "us-east-2", "wave_id": "wave-001",
        "worker_grant": {"uri": "s3://bucket/grant", "sha256": grant_sha, "expires_at": expiry},
        "budget": {"uri": "s3://bucket/budget", "sha256": budget_sha,
                   "expires_at": expiry, "region": "us-east-2", "cap_usd": 1}}
    write(root / "admission.json", admission)
    paths = ["--protocol", str(root / "protocol.json"),
             "--admission", str(root / "admission.json"), "--plan", str(root / "plan.json"),
             "--worker-grant", str(root / "grant.json"), "--budget", str(root / "budget.json"),
             "--reservation", str(root / "reservation.json"), "--job", str(root / "job.json")]
    return paths


class CampaignSubmitTests(unittest.TestCase):
    def test_checked_in_native_pilot_plan_and_protocol_digests(self):
        def tracked(path):
            return subprocess.run(["git", "show", "HEAD:" + path], cwd=ROOT,
                                  capture_output=True, check=True).stdout
        plan_bytes = tracked("research/ml_dataset_20261028/pilot-plan.json")
        protocol_bytes = tracked("research/ml_dataset_20261028/protocol.json")
        plan = json.loads(plan_bytes)
        protocol = json.loads(protocol_bytes)
        # These values were emitted by the native `crypto dataset preflight`
        # command for the checked-in pilot pair, not invented by this mirror.
        self.assertEqual(submit.native_plan_binding_sha256(plan),
                         "c905e878e3dcb29e407e743874d18b6b7091c097984f829820f26673ccd47f21")
        self.assertEqual(hashlib.sha256(protocol_bytes).hexdigest(),
                         "05a54dacc052b49bf7b9d6ffd776c461f49757e01488088c9a79bbf136f11604")
        self.assertEqual(plan["protocol_sha256"], hashlib.sha256(protocol_bytes).hexdigest())
        self.assertEqual(submit.protocol_body_sha256(protocol),
                         protocol["freeze"]["protocol_sha256"])

    def test_current_shape_draft_rejected_before_other_inputs_or_cloud_calls(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            draft = checker_tests.draft()
            write(root / "draft.json", draft)
            command = [sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                       "--dry-run", "--protocol", str(root / "draft.json")]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("full protocol admission refused", result.stderr)

    def test_admitted_dry_run_is_offline_and_byte_bound(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            paths = admitted_fixture(root)
            self.assertEqual(checker.validate(json.loads((root / "protocol.json").read_text()),
                                             admit=True,
                                             admission=json.loads((root / "admission.json").read_text())), [])
            result = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                     "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual(json.loads(result.stdout)["cloud_requests"], 0)
            self.assertEqual(json.loads(result.stdout)["status"], "local_consistency_only")
            self.assertFalse(json.loads(result.stdout)["launch_authorized"])
            (root / "budget.json").write_text('{"cap_usd":2,"region":"us-east-2"}\n')
            tampered = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                       "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(tampered.returncode, 2)
            self.assertIn("budget bytes differ", tampered.stderr)

    def test_rehashed_non_native_plan_is_rejected_before_local_pass(self):
        for field, value in (("seed", "invalid-native-u64"),
                             ("seed", -1), ("seed", 1 << 64), ("seed", True),
                             ("split_rule", "invalid-native-split"),
                             ("factor_base_size", 1), ("degree_bound", 33)):
            with self.subTest(field=field, value=value), tempfile.TemporaryDirectory() as name:
                root = pathlib.Path(name)
                paths = admitted_fixture(root)
                plan = json.loads((root / "plan.json").read_text())
                plan[field] = value
                plan_file_sha = write(root / "plan.json", plan)
                admission = json.loads((root / "admission.json").read_text())
                admission["plan_file_sha256"] = plan_file_sha
                write(root / "admission.json", admission)
                result = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                         "--dry-run", *paths], capture_output=True, text=True)
                self.assertEqual(result.returncode, 2)
                self.assertIn("plan ", result.stderr)

    def test_submit_not_supported_even_with_self_consistent_fixture(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            paths = admitted_fixture(root)
            marker = root / "called"
            fake = root / "kubectl"
            fake.write_text("#!/bin/sh\nprintf '%s\\n' \"$*\" >> '" + str(marker) + "'\n")
            fake.chmod(0o755)
            command = [sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                       "--dry-run", "--submit", *paths]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("unrecognized arguments: --submit", result.stderr)
            self.assertFalse(marker.exists())

    def test_unsuspended_or_unreviewed_pod_execution_field_rejected(self):
        with tempfile.TemporaryDirectory() as name:
            root = pathlib.Path(name)
            paths = admitted_fixture(root)
            job = json.loads((root / "job.json").read_text())
            job["spec"]["suspend"] = False
            job["spec"]["template"]["spec"]["hostNetwork"] = True
            job_sha = write(root / "job.json", job)
            admission = json.loads((root / "admission.json").read_text())
            admission["job_sha256"] = job_sha
            write(root / "admission.json", admission)
            result = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                     "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("must remain suspended", result.stderr)
            job["spec"]["suspend"] = True
            job_sha = write(root / "job.json", job)
            admission["job_sha256"] = job_sha
            write(root / "admission.json", admission)
            result = subprocess.run([sys.executable, str(HERE / "ml_dataset_campaign_submit.py"),
                                     "--dry-run", *paths], capture_output=True, text=True)
            self.assertEqual(result.returncode, 2)
            self.assertIn("unreviewed pod execution fields", result.stderr)


if __name__ == "__main__":
    unittest.main()
