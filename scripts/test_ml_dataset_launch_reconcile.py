"""Offline launch observation checks; no Kubernetes or AWS calls."""

import copy
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys
import tempfile
import unittest

MODULE = pathlib.Path(__file__).with_name("ml_dataset_launch_reconcile.py")
SPEC = importlib.util.spec_from_file_location("ml_dataset_launch_reconcile", MODULE)
launch = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(launch)

JOB_UID = "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"
POD_UID = "bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb"
NODE_UID = "cccccccc-cccc-4ccc-8ccc-cccccccccccc"


def fixture():
    resources = dict(money_micro_usd=1000, output_bytes=100, put_attempts=2,
                     read_attempts=2, download_bytes=100, local_bytes=100)
    target = dict(cluster_identity="arn:aws:eks:us-east-2:123456789012:cluster/software-test",
                  namespace="isolab", job_name="dataset-launch-01")
    reservation = dict(schema="crypto.ml-dataset.budget-launch-reservation/v1",
                       campaign_id="test-campaign", policy_sha256="d" * 64,
                       request=dict(schema="crypto.ml-dataset.budget-launch-request/v1",
                           launch_id="test-launch", target=target,
                           worker_journal_path="/work/budget/worker.sqlite",
                           binding=dict(source_commit="a" * 40, binary_sha256="b" * 64,
                                        plan_sha256="c" * 64, protocol_sha256="e" * 64,
                                        shard_index=0, shard_count=1),
                           expires_unix_seconds=2000000000,
                           resource_envelope_sha256="f" * 64,
                           resources=resources, finalization=resources,
                           credential_role_arn=None))
    job = dict(kind="Job", metadata=dict(name=target["job_name"], namespace="isolab",
                                           uid=JOB_UID, resourceVersion="10"),
               spec=dict(backoffLimit=0, parallelism=1, completions=1,
                         activeDeadlineSeconds=600,
                         template=dict(spec=dict(restartPolicy="Never",
                                                 automountServiceAccountToken=False,
                                                 nodeSelector={"node.kubernetes.io/instance-type": "c6a.2xlarge"},
                                                 containers=[{"name": "dataset", "image": "repo@sha256:" + "1" * 64}]))),
               status=dict(active=1))
    pod = dict(kind="Pod", metadata=dict(name="dataset-launch-01-x", namespace="isolab",
                                          uid=POD_UID, resourceVersion="11",
                                          labels={"batch.kubernetes.io/controller-uid": JOB_UID},
                                          ownerReferences=[dict(kind="Job", name=target["job_name"],
                                                                uid=JOB_UID, controller=True)]),
               spec=dict(nodeName="node-01", restartPolicy="Never",
                         automountServiceAccountToken=False,
                         nodeSelector=copy.deepcopy(job["spec"]["template"]["spec"]["nodeSelector"]),
                         containers=copy.deepcopy(job["spec"]["template"]["spec"]["containers"])),
               status=dict(phase="Running"))
    node = dict(kind="Node", metadata=dict(name="node-01", uid=NODE_UID,
                                            resourceVersion="12",
                                            labels={"node.kubernetes.io/instance-type": "c6a.2xlarge"}),
                spec=dict(providerID="aws:///us-east-2a/i-1234567890abcdef0"))
    return reservation, job, [pod], node


class ReconcileTests(unittest.TestCase):
    def test_identity_chain_and_digest(self):
        reservation, job, pods, node = fixture()
        result = launch.reconcile(reservation, job, pods, node)
        self.assertEqual(result["binding_candidate"]["owner"],
                         {"job_uid": JOB_UID, "pod_uid": POD_UID})
        self.assertEqual(result["binding_candidate"]["reservation_sha256"],
                         launch.reservation_digest(reservation))
        self.assertEqual(result["binding_candidate"]["observation_evidence_sha256"],
                         launch.sha(result["evidence"]))
        self.assertFalse(result["attestation_performed"])
        self.assertFalse(result["resource_lifetime_verified"])

    def test_native_reservation_field_order_independent_of_input_order(self):
        reservation, *_ = fixture()
        expected = launch.reservation_digest(reservation)
        reversed_input = {key: reservation[key] for key in reversed(reservation)}
        self.assertEqual(launch.reservation_digest(reversed_input), expected)
        self.assertEqual(expected, hashlib.sha256(launch.canonical({
            "schema": reservation["schema"], "campaign_id": reservation["campaign_id"],
            "policy_sha256": reservation["policy_sha256"],
            "request": reservation["request"]})).hexdigest())

    def test_identity_and_retry_tampering_rejected(self):
        for mutate in (
            lambda r, j, p, n: p[0]["metadata"]["ownerReferences"][0].update(uid=POD_UID),
            lambda r, j, p, n: p[0]["metadata"]["labels"].update({"batch.kubernetes.io/controller-uid": POD_UID}),
            lambda r, j, p, n: p.append(copy.deepcopy(p[0])),
            lambda r, j, p, n: j["spec"].update(backoffLimit=1),
            lambda r, j, p, n: j["spec"]["template"]["spec"].update(automountServiceAccountToken=True),
            lambda r, j, p, n: j["spec"]["template"]["spec"]["containers"][0].update(image="repo:latest"),
            lambda r, j, p, n: n["metadata"]["labels"].update({"node.kubernetes.io/instance-type": "m6a.2xlarge"}),
            lambda r, j, p, n: n["spec"].update(providerID=""),
            lambda r, j, p, n: p[0]["spec"].update(nodeName="different"),
            lambda r, j, p, n: p[0]["spec"]["containers"][0].update(image="repo@sha256:" + "2" * 64),
            lambda r, j, p, n: j["spec"].update(suspend=True),
            lambda r, j, p, n: j["status"].update(conditions=[{"type": "Failed", "status": "True"}]),
            lambda r, j, p, n: j["metadata"].update(deletionTimestamp="2026-10-31T00:00:00Z"),
        ):
            with self.subTest(mutate=mutate):
                values = copy.deepcopy(fixture())
                mutate(*values)
                with self.assertRaises(launch.ReconcileError):
                    launch.reconcile(*values)

    def test_cli_output_is_exclusive_and_rejects_symlink_input(self):
        with tempfile.TemporaryDirectory() as tmp:
            paths = []
            for name, data in zip(("reservation", "job", "pods", "node"), fixture()):
                path = pathlib.Path(tmp, name + ".json")
                path.write_text(json.dumps(data))
                paths.extend(("--" + name, str(path)))
            out = pathlib.Path(tmp, "reconciliation.json")
            command = [sys.executable, str(MODULE), *paths, "--out", str(out)]
            self.assertEqual(subprocess.run(command, capture_output=True).returncode, 0)
            self.assertEqual(out.stat().st_mode & 0o777, 0o600)
            before = out.read_bytes()
            self.assertEqual(subprocess.run(command, capture_output=True).returncode, 2)
            self.assertEqual(out.read_bytes(), before)
            pathlib.Path(tmp, "job.json").rename(pathlib.Path(tmp, "job.real.json"))
            pathlib.Path(tmp, "job.json").symlink_to("job.real.json")
            self.assertEqual(subprocess.run(command, capture_output=True).returncode, 2)

    def test_duplicate_json_field_is_rejected(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = pathlib.Path(tmp, "duplicate.json")
            path.write_text('{"kind":"Job","kind":"Pod"}')
            with self.assertRaises(launch.ReconcileError):
                launch.read_json(path)


if __name__ == "__main__":
    unittest.main()
