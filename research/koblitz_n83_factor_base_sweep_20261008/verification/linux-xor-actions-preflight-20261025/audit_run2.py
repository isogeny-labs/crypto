#!/usr/bin/env python3
"""Independently check the retained second hosted Linux XOR preflight."""

import argparse
import base64
import hashlib
import json
from pathlib import Path
import re
import zipfile


HERE = Path(__file__).resolve().parent
INPUTS = HERE.parent / "linux-xor-complete-inputs-20261025" / "manifest.json"
EVIDENCE = HERE / "run-40177842203-attempt-2"
RUN_ID = 40177842203
ARTIFACT_ID = 12593635829
HEAD = "9b6d5f4bbc4bb9c80cf22a1c58bbfb288804f723"
SOURCE = "acfcd8adfad8d45ddf9c5f799f9385120dce6cf4"
ZIP_SHA256 = "00713618c7c09188a9f75fef6ce6323af376428cc48d537491394c91429d1237"
MEMBERS = {
    "apt-install.log", "apt-update.log", "aws-identity.json",
    "cryptominisat5", "image.txt", "input-replay.json",
    "input-replay.stderr", "koblitz_n83_mixed_orbit_worker", "package.txt",
    "protocol-commit.txt", "s3-probe-exit.txt", "s3-probe.json",
    "s3-probe.sha256-check", "s3-probe.stderr", "smoke.json",
    "smoke.stderr", "smoke.stdout", "solver-ldd.txt",
    "solver-version.stderr", "solver-version.stdout", "source-commit.txt",
    "uname.txt", "verifier-gate.json", "worker-build.stderr",
    "worker-build.stdout", "worker.sha256",
}


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def read_json(path: Path) -> dict:
    return json.loads(path.read_bytes())


def audit(archive: Path) -> dict:
    raw = archive.read_bytes()
    digest = sha(raw)
    require(digest == ZIP_SHA256, "provider ZIP digest")
    metadata = read_json(EVIDENCE / "artifact_metadata.json")
    jobs = read_json(EVIDENCE / "provider_jobs.json")
    manifest_raw = INPUTS.read_bytes()
    manifest = json.loads(manifest_raw)
    require(metadata["id"] == ARTIFACT_ID
            and metadata["name"] == f"n83-linux-xor-preflight-{RUN_ID}-2"
            and metadata["workflow_run"]["id"] == RUN_ID
            and metadata["workflow_run"]["head_sha"] == HEAD
            and metadata["size_in_bytes"] == len(raw)
            and not metadata["expired"], "provider artifact identity")
    require(jobs["total_count"] == 1 and len(jobs["jobs"]) == 1,
            "single provider job")
    job = jobs["jobs"][0]
    require(job["run_id"] == RUN_ID and job["head_sha"] == HEAD
            and job["status"] == "completed" and job["conclusion"] == "success",
            "provider terminal success")
    steps = {row["name"]: row["conclusion"] for row in job["steps"]}
    for name in ("Install and smoke Linux XOR solver",
                 "Probe exact S3 input access before the worker build",
                 "Build pinned Linux external-model verifier",
                 "Fresh-fetch and replay all eleven S3 inputs",
                 "Preserve preflight receipts and binary"):
        require(steps.get(name) == "success", f"provider stage: {name}")
    with zipfile.ZipFile(archive) as bundle:
        names = bundle.namelist()
        require(len(names) == len(MEMBERS) and set(names) == MEMBERS,
                "exact unique ZIP members")
        files = {name: bundle.read(name) for name in MEMBERS}
    require(files["source-commit.txt"].strip().decode() == SOURCE
            and files["protocol-commit.txt"].strip().decode() == HEAD,
            "frozen source and protocol")
    identity = json.loads(files["aws-identity.json"])
    require(identity["Arn"]
            == "arn:aws:iam::590183823895:user/rho-gh-snapshot-readonly-20261024",
            "hosted AWS identity")
    first = manifest["entries"][0]
    probe = json.loads(files["s3-probe.json"])
    require(files["s3-probe-exit.txt"].strip() == b"0"
            and probe["ChecksumSHA256"]
            == base64.b64encode(bytes.fromhex(first["sha256"])).decode("ascii")
            and probe["ServerSideEncryption"] == "AES256"
            and b"s3-probe.xcnf: OK" in files["s3-probe.sha256-check"],
            "exact first-object S3 probe")
    smoke = json.loads(files["smoke.json"])
    require(smoke["schema"] == "n83.linux-xor-actions-preflight/v1"
            and smoke["solver_sha256"] == sha(files["cryptominisat5"])
            and smoke["package"] == files["package.txt"].decode().strip()
            and smoke["smoke_exit_code"] == 10
            and smoke["version_exit_code"] == 0
            and smoke["smoke_sat_line"] is True
            and smoke["selected_best_total_runtime"] is None,
            "solver bytes and qualification")
    for name, field in (("smoke.stdout", "smoke_stdout_sha256"),
                        ("smoke.stderr", "smoke_stderr_sha256"),
                        ("solver-version.stdout", "version_stdout_sha256"),
                        ("solver-version.stderr", "version_stderr_sha256")):
        require(sha(files[name]) == smoke[field], f"raw {name} digest")
    require(bool(re.search(rb"(?m)^s SATISFIABLE\s*$", files["smoke.stdout"])),
            "raw SAT smoke result")
    worker_sha = sha(files["koblitz_n83_mixed_orbit_worker"])
    replay = json.loads(files["input-replay.json"])
    gate = json.loads(files["verifier-gate.json"])
    require(files["worker.sha256"].decode().split()[0] == worker_sha
            and gate["worker_sha256"] == worker_sha,
            "worker binary bytes")
    require(replay["schema"] == "n83.linux-xor-complete-input-replay/v1"
            and replay["status"] == "PASS_COMPLETE_INPUT_REPLAY"
            and replay["curve"] == manifest["curve"]
            and replay["selected_best_total_runtime"] is None
            and len(replay["objects"]) == 11
            and {tuple(row[field] for field in ("key", "bytes", "sha256"))
                 for row in replay["objects"]}
            == {tuple(row[field] for field in ("key", "bytes", "sha256"))
                for row in manifest["entries"]},
            "exact complete S3 input replay")
    require(gate["schema"] == "n83.linux-xor-verifier-input-gate/v1"
            and gate["status"] == "PASS_REBUILT_XCNF_REJECTED_INCOMPLETE_MODEL"
            and gate["input_manifest_sha256"] == sha(manifest_raw)
            and gate["curve"] == manifest["curve"]
            and gate["selected_best_total_runtime"] is None,
            "exact verifier-input gate")
    return {
        "schema": "n83.linux-xor-actions-preflight-attempt-replay/v2",
        "status": "PASS_11_INPUTS_AND_VERIFIER_GATE_PROVIDER_ZIP",
        "run_id": RUN_ID,
        "artifact_id": metadata["id"],
        "artifact_zip_sha256": digest,
        "solver_sha256_verified_from_artifact": smoke["solver_sha256"],
        "worker_sha256_verified_from_artifact": worker_sha,
        "input_count": len(replay["objects"]),
        "artifact_s3_custody": "NOT_STORED",
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-zip", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.artifact_zip), indent=2, sort_keys=True))
