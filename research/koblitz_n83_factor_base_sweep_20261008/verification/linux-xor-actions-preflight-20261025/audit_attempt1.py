#!/usr/bin/env python3
"""Independently replay the first hosted Linux XOR preflight failure artifact."""

import argparse
import base64
import hashlib
import json
from pathlib import Path
import re
import zipfile


HERE = Path(__file__).resolve().parent
RUN_ID = 40176531834
ARTIFACT_ID = 12592487092
HEAD = "642e49b23ce625c55b2dc5ef2feaa894fd4ebc49"
SOURCE = "acfcd8adfad8d45ddf9c5f799f9385120dce6cf4"
ZIP_SHA256 = "a7be830c192ac994c112a002f432f3583120b23af380d3d8b762a77ab564c560"
WORKER_SHA256 = "3d069ccd4b4adfb1784ed1364edc5bbdd2b656c9c7da416ebc8f22526c65d29c"
FLAGS = ["--threads", "1", "--random", "0", "--branchstr", "vmtf+vsids",
         "--autodisablegauss", "0", "--maxconfl", "20", "--maxtime", "30",
         "--maxmatrixcols", "25000", "--maxnummatrices", "10", "--verb", "1",
         "--verbstat", "1", "--printsol", "1"]
MEMBERS = {
    "apt-install.log", "apt-update.log", "image.txt", "input-replay.json",
    "koblitz_n83_mixed_orbit_worker", "package.txt", "protocol-commit.txt",
    "smoke.json", "smoke.stderr", "smoke.stdout", "solver-version.stderr",
    "solver-version.stdout", "source-commit.txt", "uname.txt",
    "worker-build.stderr", "worker-build.stdout", "worker.sha256",
}


def require(value: bool, reason: str) -> None:
    if not value:
        raise ValueError(reason)


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def audit(archive: Path) -> dict:
    raw_zip = archive.read_bytes()
    require(sha(raw_zip) == ZIP_SHA256, "provider artifact ZIP SHA-256")
    metadata = json.loads((HERE / "attempt-1/artifact_metadata.json").read_bytes())
    jobs = json.loads((HERE / "attempt-1/provider_jobs.json").read_bytes())
    custody = json.loads((HERE / "attempt-1/s3-custody.json").read_bytes())
    checksum = base64.b64encode(bytes.fromhex(ZIP_SHA256)).decode("ascii")
    require(custody["schema"] == "n83.linux-xor-preflight-s3-custody/v1"
            and custody["status"] == "PASS_FRESH_S3_REPLAY"
            and custody["bucket"] == "crypto-autoresearcher"
            and custody["key"].endswith(f"/preflight-{RUN_ID}/{ZIP_SHA256}.zip")
            and custody["bytes"] == len(raw_zip)
            and custody["sha256"] == ZIP_SHA256
            and custody["checksum_sha256_b64"] == checksum
            and custody["if_none_match"] == "*"
            and custody["selected_best_total_runtime"] is None,
            "write-once S3 custody identity")
    for operation in ("put_response", "head_response", "get_response"):
        response = custody[operation]
        require(response["ChecksumSHA256"] == checksum
                and response["ServerSideEncryption"] == "AES256",
                f"{operation}: S3 checksum and encryption")
    require(custody["head_response"]["ContentLength"] == len(raw_zip)
            and custody["get_response"]["ContentLength"] == len(raw_zip),
            "S3 content length")
    require(metadata["id"] == ARTIFACT_ID
            and metadata["name"] == f"n83-linux-xor-preflight-{RUN_ID}-1"
            and metadata["size_in_bytes"] == len(raw_zip)
            and metadata["workflow_run"]["id"] == RUN_ID
            and metadata["workflow_run"]["head_sha"] == HEAD
            and not metadata["expired"], "provider artifact identity")
    require(jobs["total_count"] == 1 and len(jobs["jobs"]) == 1,
            "one provider job")
    job = jobs["jobs"][0]
    require(job["run_id"] == RUN_ID and job["head_sha"] == HEAD
            and job["status"] == "completed" and job["conclusion"] == "failure",
            "provider terminal failure")
    steps = {row["name"]: row["conclusion"] for row in job["steps"]}
    require(steps["Install and smoke Linux XOR solver"] == "success"
            and steps["Build pinned Linux external-model verifier"] == "success"
            and steps["Fresh-fetch and replay all eleven S3 inputs"] == "failure"
            and steps["Preserve preflight receipts and binary"] == "success",
            "provider stage classification")
    with zipfile.ZipFile(archive) as bundle:
        require(len(bundle.namelist()) == len(MEMBERS)
                and set(bundle.namelist()) == MEMBERS, "exact artifact members")
        files = {name: bundle.read(name) for name in MEMBERS}
    require(files["source-commit.txt"].strip().decode() == SOURCE
            and files["protocol-commit.txt"].strip().decode() == HEAD
            and sha(files["koblitz_n83_mixed_orbit_worker"]) == WORKER_SHA256
            and files["worker.sha256"].decode().split()[0] == WORKER_SHA256,
            "frozen source and worker bytes")
    smoke = json.loads(files["smoke.json"])
    require(smoke["schema"] == "n83.linux-xor-actions-preflight/v1"
            and smoke["selected_best_total_runtime"] is None
            and smoke["flags"] == FLAGS
            and smoke["package"] == files["package.txt"].decode().strip()
            and smoke["package"] == "cryptominisat=5.11.15+dfsg1-1.1build1"
            and smoke["smoke_exit_code"] == 10
            and smoke["version_exit_code"] == 0
            and smoke["smoke_sat_line"] is True,
            "Linux solver qualification receipt")
    for name, key in (("smoke.stdout", "smoke_stdout_sha256"),
                      ("smoke.stderr", "smoke_stderr_sha256"),
                      ("solver-version.stdout", "version_stdout_sha256"),
                      ("solver-version.stderr", "version_stderr_sha256")):
        require(sha(files[name]) == smoke[key], f"{name}: raw stream digest")
    require(bool(re.search(rb"(?m)^s SATISFIABLE\s*$", files["smoke.stdout"]))
            and files["smoke.stderr"] == b""
            and b"CryptoMiniSat version 5.11.15" in files["solver-version.stdout"],
            "exact two-variable smoke result")
    require(files["input-replay.json"] == b""
            and files["apt-install.log"]
            and files["worker-build.stdout"] == b""
            and files["image.txt"], "retained setup and failed first input fetch")
    return {
        "schema": "n83.linux-xor-actions-preflight-attempt-replay/v1",
        "status": "PASS_PRE_S3_FAILURE_REPLAY",
        "run_id": RUN_ID,
        "artifact_id": ARTIFACT_ID,
        "artifact_zip_sha256": ZIP_SHA256,
        "solver_package": smoke["package"],
        "solver_sha256_from_host_receipt": smoke["solver_sha256"],
        "worker_sha256_verified_from_artifact": WORKER_SHA256,
        "s3_input_status": "FAILED_BEFORE_FIRST_OBJECT_REPLAY",
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--artifact-zip", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(audit(args.artifact_zip), indent=2, sort_keys=True))
