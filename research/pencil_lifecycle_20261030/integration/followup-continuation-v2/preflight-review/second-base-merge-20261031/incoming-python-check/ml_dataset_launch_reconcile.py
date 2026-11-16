#!/usr/bin/env python3
"""Offline, fail-closed reconciliation of one reserved dataset Job and Pod.

This reads bounded Kubernetes API snapshots. It performs no API call, grant
issuance, Job submission, or worker activation. A trusted live launcher must
obtain the snapshots and separately verify placement, lifetime, and delivery.
"""

import argparse
import hashlib
import json
import os
import pathlib
import re
import stat
import sys
import tempfile

LIMIT = 1024 * 1024
UID = re.compile(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
SHA = re.compile(r"^[0-9a-f]{64}$")
RESOURCE_SCHEMA = "crypto.ml-dataset.launch-observation/v1"


class ReconcileError(ValueError):
    pass


def need(condition, message):
    if not condition:
        raise ReconcileError(message)


def read_json(path):
    p = pathlib.Path(path)
    try:
        fd = os.open(p, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
    except OSError as e:
        raise ReconcileError(f"missing or symlinked input: {p}: {e}") from e
    with os.fdopen(fd, "rb") as f:
        details = os.fstat(f.fileno())
        need(stat.S_ISREG(details.st_mode), f"input is not a regular file: {p}")
        need(details.st_size <= LIMIT, f"input exceeds {LIMIT} bytes: {p}")
        raw = f.read(LIMIT + 1)
    need(len(raw) <= LIMIT, f"input exceeds {LIMIT} bytes: {p}")
    try:
        def no_duplicates(pairs):
            result = {}
            for key, value in pairs:
                need(key not in result, f"duplicate JSON field: {key}")
                result[key] = value
            return result

        def no_nonfinite(value):
            raise ReconcileError(f"nonfinite JSON number: {value}")

        return json.loads(raw, object_pairs_hook=no_duplicates,
                          parse_constant=no_nonfinite), hashlib.sha256(raw).hexdigest()
    except (ValueError, UnicodeError) as e:
        raise ReconcileError(f"invalid JSON: {p}: {e}") from e


def canonical(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False, allow_nan=False).encode("utf-8")


def sha(value):
    return hashlib.sha256(canonical(value)).hexdigest()


def ordered(obj, keys, label):
    need(isinstance(obj, dict) and set(obj) == set(keys), f"{label}: exact fields required")
    return {key: obj[key] for key in keys}


def reservation_digest(value):
    """Mirror serde_json field order for the native LaunchReservation/v1 type."""
    r = ordered(value, ("schema", "campaign_id", "policy_sha256", "request"), "reservation")
    q = ordered(r["request"], ("schema", "launch_id", "target", "worker_journal_path",
        "binding", "expires_unix_seconds", "resource_envelope_sha256", "resources",
        "finalization", "credential_role_arn"), "launch request")
    q["target"] = ordered(q["target"], ("cluster_identity", "namespace", "job_name"), "target")
    q["binding"] = ordered(q["binding"], ("source_commit", "binary_sha256", "plan_sha256",
        "protocol_sha256", "shard_index", "shard_count"), "run binding")
    resource_keys = ("money_micro_usd", "output_bytes", "put_attempts", "read_attempts",
        "download_bytes", "local_bytes")
    for key in ("resources", "finalization"):
        q[key] = ordered(q[key], resource_keys, key)
    r["request"] = q
    need(r["schema"] == "crypto.ml-dataset.budget-launch-reservation/v1", "wrong reservation schema")
    need(q["schema"] == "crypto.ml-dataset.budget-launch-request/v1", "wrong request schema")
    for name in ("policy_sha256",):
        need(isinstance(r[name], str) and SHA.fullmatch(r[name]), f"invalid {name}")
    for name in ("source_commit", "binary_sha256", "plan_sha256", "protocol_sha256"):
        val = q["binding"][name]
        size = 40 if name == "source_commit" else 64
        need(isinstance(val, str) and re.fullmatch(r"[0-9a-f]{%d}" % size, val), f"invalid {name}")
    return sha(r)


def identity(obj, kind):
    need(isinstance(obj, dict) and obj.get("kind") == kind, f"expected {kind} snapshot")
    meta = obj.get("metadata")
    need(isinstance(meta, dict), f"{kind} metadata missing")
    uid = meta.get("uid")
    need(isinstance(uid, str) and UID.fullmatch(uid), f"{kind} UID missing or invalid")
    need(isinstance(meta.get("resourceVersion"), str) and meta["resourceVersion"],
         f"{kind} resourceVersion missing")
    need(meta.get("deletionTimestamp") is None, f"{kind} is deleting")
    return meta, uid


def reconcile(reservation, job, pods, node):
    reservation_sha = reservation_digest(reservation)
    request = reservation["request"]
    target = request["target"]
    need(isinstance(target["cluster_identity"], str) and target["cluster_identity"].startswith("arn:aws:eks:"),
         "cluster identity missing")
    need(isinstance(pods, list) and len(pods) == 1, "exactly one Pod snapshot required")
    jm, job_uid = identity(job, "Job")
    need(jm.get("name") == target["job_name"] and jm.get("namespace") == target["namespace"],
         "Job differs from reserved target")
    js = job.get("spec")
    need(isinstance(js, dict) and js.get("backoffLimit") == 0 and js.get("parallelism") == 1
         and js.get("completions") == 1, "Job can retry or has unexpected concurrency")
    need(type(js.get("activeDeadlineSeconds")) is int and js["activeDeadlineSeconds"] > 0,
         "Job lacks a finite active deadline")
    need(js.get("suspend") is not True, "Job is suspended")
    status = job.get("status", {})
    need(isinstance(status, dict) and status.get("failed", 0) == 0 and status.get("succeeded", 0) == 0,
         "Job already terminal")
    conditions = status.get("conditions", [])
    need(isinstance(conditions, list) and not any(
        isinstance(item, dict) and item.get("type") in ("Complete", "Failed")
        and item.get("status") == "True" for item in conditions), "Job has terminal condition")
    template = js.get("template", {}).get("spec", {})
    need(template.get("restartPolicy") == "Never" and template.get("automountServiceAccountToken") is False,
         "Job has retrying container or worker API token")
    containers = template.get("containers")
    need(isinstance(containers, list) and len(containers) == 1,
         "Job must have one reviewed container")
    image = containers[0].get("image")
    need(isinstance(image, str) and re.search(r"@sha256:[0-9a-f]{64}$", image),
         "Job image is not digest pinned")
    selector = template.get("nodeSelector")
    need(isinstance(selector, dict) and isinstance(selector.get("node.kubernetes.io/instance-type"), str),
         "exact instance type selector missing")

    pod = pods[0]
    pm, pod_uid = identity(pod, "Pod")
    need(pm.get("namespace") == target["namespace"] and isinstance(pm.get("name"), str),
         "Pod namespace or name differs")
    refs = pm.get("ownerReferences")
    need(isinstance(refs, list) and len(refs) == 1 and refs[0].get("kind") == "Job"
         and refs[0].get("name") == target["job_name"] and refs[0].get("uid") == job_uid
         and refs[0].get("controller") is True, "Pod is not owned by exact Job UID")
    labels = pm.get("labels")
    need(isinstance(labels, dict) and labels.get("batch.kubernetes.io/controller-uid") == job_uid,
         "Pod controller UID label differs")
    ps = pod.get("spec")
    need(isinstance(ps, dict) and ps.get("restartPolicy") == "Never"
         and ps.get("automountServiceAccountToken") is False,
         "Pod startup policy differs from Job")
    need(ps.get("nodeSelector") == selector and ps.get("containers") == containers,
         "Pod container or placement selector differs from Job")
    pod_status = pod.get("status")
    need(isinstance(pod_status, dict) and pod_status.get("phase") in ("Pending", "Running"),
         "Pod is terminal or unknown")
    node_name = ps.get("nodeName")
    need(isinstance(node_name, str) and node_name, "Pod not scheduled")
    nm, _ = identity(node, "Node")
    need(nm.get("name") == node_name, "Pod node differs from observed Node")
    need(nm.get("labels", {}).get("node.kubernetes.io/instance-type") ==
         selector["node.kubernetes.io/instance-type"], "placed instance type differs from selector")
    provider = node.get("spec", {}).get("providerID")
    need(isinstance(provider, str) and re.fullmatch(r"aws:///[^/]+/i-[0-9a-f]{8,17}", provider),
         "Node lacks exact EC2 provider identity")
    evidence = {"schema": RESOURCE_SCHEMA, "reservation_sha256": reservation_sha,
        "cluster_identity": target["cluster_identity"], "job": job, "pods": pods, "node": node}
    evidence_sha = sha(evidence)
    binding = {"schema": "crypto.ml-dataset.budget-launch-binding/v1",
        "launch_id": request["launch_id"], "reservation_sha256": reservation_sha,
        "observed_target": target, "owner": {"job_uid": job_uid, "pod_uid": pod_uid},
        "observation_evidence_sha256": evidence_sha}
    return {"schema": "crypto.ml-dataset.launch-reconciliation/v1",
        "attestation_performed": False, "cloud_requests": 0, "grant_issued": False,
        "resource_lifetime_verified": False, "binding_candidate": binding,
        "evidence": evidence, "evidence_sha256": evidence_sha,
        "source_binding": request["binding"], "image": image,
        "placed_provider_id": provider}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ("reservation", "job", "pods", "node", "out"):
        parser.add_argument("--" + name, required=True)
    args = parser.parse_args(argv)
    try:
        reservation, _ = read_json(args.reservation)
        job, _ = read_json(args.job)
        pods, _ = read_json(args.pods)
        node, _ = read_json(args.node)
        result = reconcile(reservation, job, pods, node)
        out = pathlib.Path(args.out)
        data = canonical(result) + b"\n"
        need(not out.exists() and not out.is_symlink(), "output already exists")
        fd, temp = tempfile.mkstemp(prefix=".dataset-launch-", dir=out.parent)
        try:
            with os.fdopen(fd, "wb") as f:
                os.fchmod(f.fileno(), 0o600)
                f.write(data)
                f.flush()
                os.fsync(f.fileno())
            os.link(temp, out)
            dirfd = os.open(out.parent, os.O_RDONLY | os.O_DIRECTORY)
            try:
                os.fsync(dirfd)
            finally:
                os.close(dirfd)
        finally:
            os.unlink(temp)
        return 0
    except (OSError, ReconcileError, TypeError, KeyError, AttributeError, IndexError) as e:
        print(f"launch reconciliation refused: {e}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
