#!/usr/bin/env python3
"""One-shot dataset Job submission behind the full protocol admission gate.

Dry-run is offline. Submission needs explicit, byte-bound grant/budget and native
reservation records. A failed create is ambiguous: the intent is retained and
this command never retries. Reconcile the named Job before any further action.
"""

from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import os
import pathlib
import subprocess
import sys
import tempfile

import ml_dataset_launch_reconcile as launch

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))
import ml_dataset_full_protocol_check as protocol_check  # noqa: E402


def need(condition: bool, reason: str) -> None:
    if not condition:
        raise launch.ReconcileError(reason)


def checked_file(path: str) -> tuple[dict, str]:
    value, digest = launch.read_json(path)
    need(isinstance(value, dict), f"expected JSON object: {path}")
    return value, digest


def admitted_inputs(args: argparse.Namespace) -> dict:
    # This is deliberately the first gate. The draft is refused before reading
    # other inputs or calling any external process, even for --dry-run.
    protocol, protocol_file_sha = checked_file(args.protocol)
    errors = protocol_check.validate(protocol, admit=True)
    need(not errors, "full protocol admission refused: " + "; ".join(errors))

    for field in ("plan", "worker_grant", "budget", "reservation", "job"):
        need(bool(getattr(args, field)), f"--{field.replace('_', '-')} required after admission")
    admission = protocol["execution"]["admission"]
    plan, plan_sha = checked_file(args.plan)
    grant, grant_sha = checked_file(args.worker_grant)
    budget, budget_sha = checked_file(args.budget)
    reservation, _ = checked_file(args.reservation)
    job, job_sha = checked_file(args.job)
    need(plan_sha == admission["plan_sha256"], "plan bytes differ from admitted SHA256")
    need(grant_sha == admission["worker_grant"]["sha256"], "worker grant bytes differ from admitted SHA256")
    need(budget_sha == admission["budget"]["sha256"], "budget bytes differ from admitted SHA256")
    need(job_sha == admission.get("job_sha256"), "Job bytes differ from admitted SHA256")
    need(plan.get("schema") == "crypto.ml-dataset.plan/v1", "wrong full plan schema")
    need(plan.get("protocol_sha256") == protocol["freeze"]["protocol_sha256"],
         "plan protocol binding differs from frozen protocol")
    params = next(x["parameters"] for x in protocol["arms"] if x["id"] == "dataset")
    for plan_key, protocol_key in (("families", "families"), ("field_sizes", "n"),
                                   ("arities", "m"), ("solvers", "solvers")):
        need(plan.get(plan_key) == params[protocol_key], f"plan {plan_key} differs from protocol")
    for key in ("curve_seeds", "generators", "keys", "twists", "factor_base_size",
                "degree_bound", "work_limit"):
        need(plan.get(key) == params[key], f"plan {key} differs from protocol")

    reservation_sha = launch.reservation_digest(reservation)
    request = reservation["request"]
    binding = request["binding"]
    instrument = next(x["instrument"] for x in protocol["arms"] if x["id"] == "dataset")
    for key, expected in (("source_commit", instrument["commit"]),
                          ("binary_sha256", instrument["binary_sha256"]),
                          ("plan_sha256", plan_sha),
                          ("protocol_sha256", protocol["freeze"]["protocol_sha256"])):
        need(binding[key] == expected, f"reservation {key} differs from admission")
    need(request["launch_id"] == admission["wave_id"], "wave ID differs from reservation")
    need(request["target"]["namespace"] == "isolab", "unexpected Job namespace")
    need(request["expires_unix_seconds"] > dt.datetime.now(dt.timezone.utc).timestamp(),
         "reservation expired")
    need(type(request["resources"].get("money_micro_usd")) is int and
         0 < request["resources"]["money_micro_usd"] <= admission["budget"]["cap_usd"] * 1_000_000,
         "reserved money exceeds admitted wave cap")

    need(job.get("apiVersion") == "batch/v1" and job.get("kind") == "Job", "expected batch/v1 Job")
    meta = job.get("metadata") or {}
    target = request["target"]
    need(meta.get("name") == target["job_name"] and meta.get("namespace") == target["namespace"],
         "Job target differs from reservation")
    need((meta.get("annotations") or {}).get("dataset.isolabs.dev/reservation-sha256") == reservation_sha,
         "Job reservation annotation differs")
    spec = job.get("spec") or {}
    need(spec.get("backoffLimit") == 0 and spec.get("parallelism") == 1 and
         spec.get("completions") == 1, "Job retries or parallelism are not admitted")
    need(type(spec.get("activeDeadlineSeconds")) is int and
         0 < spec["activeDeadlineSeconds"] <= protocol["execution"]["timeout_seconds"],
         "Job active deadline exceeds protocol")
    pod = (spec.get("template") or {}).get("spec") or {}
    containers = pod.get("containers") or []
    need(len(containers) == 1 and containers[0].get("image") == admission["image"],
         "Job image differs from admitted digest")
    need(containers[0].get("command") == ["/usr/local/bin/crypto", "dataset", "start"],
         "Job does not start reviewed dataset worker")
    need(pod.get("restartPolicy") == "Never" and pod.get("automountServiceAccountToken") is False,
         "Job worker retry or token policy differs")
    # This is the pre-Job worker-access grant, not the native budget WorkerGrant
    # (which is minted only after the Job/Pod owner UIDs have been observed).
    need(grant.get("schema") == "crypto.ml-dataset.worker-access-admission/v1" and
         grant.get("status") == "applied_and_verified" and
         grant.get("region") == admission["region"] and
         isinstance(grant.get("role_arn"), str) and grant["role_arn"].startswith("arn:aws:iam::") and
         isinstance(grant.get("policy_sha256"), str) and
         protocol_check.SHA256.fullmatch(grant["policy_sha256"]),
         "worker access receipt is not applied and verified for admitted region")
    need(budget.get("schema") == "crypto.ml-dataset.campaign-budget-admission/v1" and
         budget.get("status") == "approved" and budget.get("region") == admission["region"] and
         budget.get("wave_id") == admission["wave_id"] and
         budget.get("cap_usd") == admission["budget"]["cap_usd"],
         "budget receipt is not approved for admitted wave and region")
    return {"schema": "crypto.ml-dataset.campaign-submit-preflight/v1",
            "protocol_file_sha256": protocol_file_sha, "plan_sha256": plan_sha,
            "worker_grant_sha256": grant_sha, "budget_sha256": budget_sha,
            "reservation_sha256": reservation_sha, "job_sha256": job_sha,
            "wave_id": admission["wave_id"], "job_name": target["job_name"],
            "namespace": target["namespace"]}


def exclusive_json(path: pathlib.Path, value: dict) -> None:
    data = json.dumps(value, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    with os.fdopen(fd, "wb") as out:
        out.write(data)
        out.flush()
        os.fsync(out.fileno())
    parent = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(parent)
    finally:
        os.close(parent)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--submit", action="store_true")
    for name in ("protocol", "plan", "worker-grant", "budget", "reservation", "job",
                 "intent", "kubeconfig", "context"):
        parser.add_argument("--" + name, required=name == "protocol")
    parser.add_argument("--kubectl", default="kubectl")
    args = parser.parse_args(argv)
    try:
        receipt = admitted_inputs(args)
        if args.dry_run:
            print(json.dumps({**receipt, "status": "admitted_offline_dry_run", "cloud_requests": 0},
                             sort_keys=True))
            return 0
        need(bool(args.intent and args.kubeconfig and args.context),
             "--submit requires --intent, --kubeconfig and --context")
        need(pathlib.Path(args.kubeconfig).is_file(), "kubeconfig missing")
        # Bind the exact checked bytes to create; never give kubectl a mutable
        # caller path. An exclusive durable intent prevents blind resubmission.
        _, job_bytes_sha = checked_file(args.job)
        need(job_bytes_sha == receipt["job_sha256"], "Job changed after preflight")
        exclusive_json(pathlib.Path(args.intent), {**receipt, "status": "create_attempt_started",
                                                   "created_at": dt.datetime.now(dt.timezone.utc).isoformat()})
        with tempfile.TemporaryDirectory(prefix="dataset-submit-") as temp:
            job_bytes = pathlib.Path(args.job).read_bytes()
            need(hashlib.sha256(job_bytes).hexdigest() == receipt["job_sha256"],
                 "Job changed after intent; reconcile intent before retry")
            manifest = pathlib.Path(temp, "job.json")
            manifest.write_bytes(job_bytes)
            result = subprocess.run([args.kubectl, "--kubeconfig", args.kubeconfig,
                                     "--context", args.context, "create", "-f", str(manifest)],
                                    stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                                    check=False)
        exclusive_json(pathlib.Path(args.intent + ".result.json"),
                       {**receipt, "create_exit_code": result.returncode,
                        "status": "create_returned_reconcile_job_uid"})
        return 0 if result.returncode == 0 else 3
    except (OSError, ValueError, TypeError, KeyError, IndexError) as exc:
        print(f"dataset submission refused: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
