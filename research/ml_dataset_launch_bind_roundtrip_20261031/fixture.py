#!/usr/bin/env python3
"""Offline synthetic launch-helper -> native budget bind compatibility fixture.

The snapshots are invented test data.  This invokes no Kubernetes, AWS, or
worker activation.  The resulting local grant is only in a temporary synthetic
ledger; the helper's no-grant-issued claim remains true for its own output.
"""

import argparse
import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
HELPER = ROOT / "scripts/ml_dataset_launch_reconcile.py"
RESOURCE = ("money_micro_usd", "output_bytes", "put_attempts", "read_attempts",
            "download_bytes", "local_bytes")


def save(path, obj):
    path.write_text(json.dumps(obj, indent=2, sort_keys=True) + "\n")


def run(args, good=True):
    result = subprocess.run(args, capture_output=True, text=True, check=False)
    if good and result.returncode:
        raise AssertionError(f"exit {result.returncode}: {args}: {result.stderr}")
    if not good and result.returncode == 0:
        raise AssertionError(f"unexpected success: {args}: {result.stdout}")
    return result


def resources(*values):
    assert len(values) == len(RESOURCE)
    return dict(zip(RESOURCE, values))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--crypto", type=pathlib.Path, required=True)
    parser.add_argument("--out", type=pathlib.Path, required=True)
    args = parser.parse_args()
    if args.out.exists():
        raise SystemExit(f"refusing existing output: {args.out}")
    with tempfile.TemporaryDirectory(prefix="ml-launch-bind-offline-") as tmp:
        base = pathlib.Path(tmp)
        # A fixed future deadline keeps the synthetic bytes replayable.
        deadline = 4_102_444_800
        sha = lambda ch: ch * 64
        target = {"cluster_identity": "arn:aws:eks:us-east-2:123456789012:cluster/offline-test",
                  "namespace": "isolab", "job_name": "offline-launch-test"}
        policy = {
            "schema": "crypto.ml-dataset.budget-policy/v1", "campaign_id": "offline-fixture",
            "deadline_unix_seconds": deadline,
            "caps": resources(200_000_000, 1_048_576, 1000, 2000, 8_388_608, 8_388_608),
            "prior": resources(188_388_800, 0, 0, 0, 0, 0),
            "prior_noncompute_reserve_micro_usd": 25_000_000,
            "noncompute_prices": {"put_micro_usd_per_1000": 5000,
                "read_micro_usd_per_1000": 400, "storage_micro_usd_per_gib_day": 1000,
                "download_micro_usd_per_gib": 100_000, "fixed_micro_usd": 1_000_000},
            "prior_usage_reconciled": True, "prior_evidence_sha256": [sha("1")],
            "region": "us-east-2", "pricing_region": "us-east-2",
            "price_evidence_sha256": sha("2"), "resource_envelope_sha256": sha("3"),
            "resource_lifetime_bound": True, "compute_rate_micro_usd_per_hour": 3600,
            "billable_resource_seconds": 1, "billable_resource_count": 1,
            "allocator_ledger_path": str(base / "campaign.sqlite"),
            "allocator_host_boot_id": pathlib.Path("/proc/sys/kernel/random/boot_id").read_text().strip(),
            "storage_horizon_days": 30,
            "allowed_remotes": [{"bucket": "setec-astronomy-artifacts", "prefix": "ml-dataset/test/"}],
            "coordination_prefix": "ml-dataset/test/budget/"}
        request = {"schema": "crypto.ml-dataset.budget-launch-request/v1",
            "launch_id": "offline-launch-1", "target": target,
            "worker_journal_path": str(base / "worker.sqlite"),
            "binding": {"source_commit": "a" * 40, "binary_sha256": sha("b"),
                "plan_sha256": sha("c"), "protocol_sha256": sha("d"),
                "shard_index": 0, "shard_count": 1},
            "expires_unix_seconds": deadline, "resource_envelope_sha256": sha("3"),
            "resources": resources(100, 131_072, 30, 50, 1_048_576, 1_048_576),
            "finalization": resources(0, 16_384, 10, 10, 262_144, 4096),
            "credential_role_arn": None}
        save(base / "policy.json", policy)
        save(base / "request.json", request)
        cmd = [str(args.crypto), "dataset", "budget"]
        reserved = json.loads(run(cmd + ["reserve-launch", "--create-ledger",
            "--policy", str(base / "policy.json"), "--ledger", str(base / "campaign.sqlite"),
            "--request", str(base / "request.json"), "--out", str(base / "reservation.json")]).stdout)
        assert reserved["cloud_requests"] == 0 and reserved["worker_grant_issued"] is False
        job_uid = "11111111-1111-4111-8111-111111111111"
        pod_uid = "22222222-2222-4222-8222-222222222222"
        node_uid = "33333333-3333-4333-8333-333333333333"
        containers = [{"name": "worker", "image": "example.invalid/crypto@sha256:" + sha("e")}]
        selector = {"node.kubernetes.io/instance-type": "m6i.large"}
        job = {"kind": "Job", "metadata": {"uid": job_uid, "resourceVersion": "11",
                "name": target["job_name"], "namespace": target["namespace"]},
            "spec": {"backoffLimit": 0, "parallelism": 1, "completions": 1,
                "activeDeadlineSeconds": 300,
                "template": {"spec": {"restartPolicy": "Never",
                    "automountServiceAccountToken": False, "containers": containers,
                    "nodeSelector": selector}}}, "status": {"active": 1}}
        pod = {"kind": "Pod", "metadata": {"uid": pod_uid, "resourceVersion": "12",
                "name": "offline-launch-test-pod", "namespace": target["namespace"],
                "ownerReferences": [{"kind": "Job", "name": target["job_name"],
                    "uid": job_uid, "controller": True}],
                "labels": {"batch.kubernetes.io/controller-uid": job_uid}},
            "spec": {"restartPolicy": "Never", "automountServiceAccountToken": False,
                "containers": containers, "nodeSelector": selector, "nodeName": "offline-node"},
            "status": {"phase": "Running"}}
        node = {"kind": "Node", "metadata": {"uid": node_uid, "resourceVersion": "13",
                "name": "offline-node", "labels": selector},
            "spec": {"providerID": "aws:///us-east-2a/i-0123456789abcdef0"}}
        for name, value in (("job", job), ("pods", [pod]), ("node", node)):
            save(base / f"{name}.json", value)
        run([sys.executable, str(HELPER), "--reservation", str(base / "reservation.json"),
            "--job", str(base / "job.json"), "--pods", str(base / "pods.json"),
            "--node", str(base / "node.json"), "--out", str(base / "reconciled.json")])
        reconciled = json.loads((base / "reconciled.json").read_text())
        assert reconciled["attestation_performed"] is False
        assert reconciled["grant_issued"] is False
        assert reconciled["resource_lifetime_verified"] is False
        assert reconciled["cloud_requests"] == 0
        candidate = reconciled["binding_candidate"]
        assert candidate["reservation_sha256"] == reserved["reservation_sha256"]
        save(base / "binding.json", candidate)
        before = hashlib.sha256((base / "campaign.sqlite").read_bytes()).hexdigest()
        tampered = dict(candidate, reservation_sha256=sha("f"))
        save(base / "tampered.json", tampered)
        rejected = run(cmd + ["bind-launch", "--policy", str(base / "policy.json"),
            "--ledger", str(base / "campaign.sqlite"), "--binding", str(base / "tampered.json"),
            "--out", str(base / "must-not-grant.json")], good=False)
        assert "launch binding differs" in rejected.stderr
        assert not (base / "must-not-grant.json").exists()
        assert hashlib.sha256((base / "campaign.sqlite").read_bytes()).hexdigest() == before
        bound = json.loads(run(cmd + ["bind-launch", "--policy", str(base / "policy.json"),
            "--ledger", str(base / "campaign.sqlite"), "--binding", str(base / "binding.json"),
            "--out", str(base / "grant.json")]).stdout)
        assert bound["grant"]["request"]["owner"] == candidate["owner"]
        assert bound["campaign"]["pending_launch_count"] == 0
        assert bound["campaign"]["bound_launch_count"] == 1
        assert bound["cloud_requests"] == 0 and bound["launched"] is False
        status = json.loads(run(cmd + ["status", "--policy", str(base / "policy.json"),
            "--ledger", str(base / "campaign.sqlite")]).stdout)
        assert status["pending_launch_count"] == 0 and status["bound_launch_count"] == 1
        report = {"schema": "crypto.ml-dataset.offline-launch-bind-fixture/v1",
            "synthetic_only": True, "attestation_performed": False,
            "native_binary_sha256": hashlib.sha256(args.crypto.read_bytes()).hexdigest(),
            "reconciler_sha256": hashlib.sha256(HELPER.read_bytes()).hexdigest(),
            "helper_grant_issued": False, "helper_cloud_requests": 0,
            "resource_lifetime_verified": False,
            "native_local_synthetic_grant_issued": True, "native_cloud_requests": 0,
            "reservation_sha256": reserved["reservation_sha256"],
            "observation_evidence_sha256": reconciled["evidence_sha256"],
            "grant_sha256": bound["grant_sha256"],
            "binding_candidate": candidate, "negative_tampered_digest_rejected": True,
            "negative_ledger_unchanged": True,
            "negative_error": rejected.stderr.strip(),
            "status_pending_launch_count": status["pending_launch_count"],
            "status_bound_launch_count": status["bound_launch_count"],
            "notes": "Invented snapshots and prices do not attest Kubernetes, actual cost, or worker lifetime."}
        save(args.out, report)
        print(json.dumps(report, indent=2, sort_keys=True))


if __name__ == "__main__":
    main()
